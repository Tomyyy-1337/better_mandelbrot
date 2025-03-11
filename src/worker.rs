use std::sync::{mpsc::{Receiver, Sender}, Arc};

use crate::task_queue::TaskQueue;

pub struct Worker<T, R> {
    task_queue: Arc<TaskQueue<T>>,
    result_receiver: Receiver<R>,
    task_sender: Sender<T>,
}

impl<T, R> Worker<T, R> 
where 
    T: Send + 'static, 
    R: Send + 'static
{
    pub fn new(num_worker_threads: usize, worker_function: fn(T) -> R) -> Worker<T, R> {
        let (result_sender, result_receiver) = std::sync::mpsc::channel();
        let (task_sender, task_receiver) = std::sync::mpsc::channel();
        let task_queue = Arc::new(TaskQueue::new());

        Self::spawn_queue_buffer_thread(task_queue.clone(), task_receiver);
        
        for _ in 0..num_worker_threads {
            Self::spawn_worker_thread(worker_function, result_sender.clone(), task_queue.clone());
        }

        Worker {
            task_queue,
            result_receiver,
            task_sender,
        }
    }

    fn spawn_worker_thread(
        worker_function: fn(T) -> R,
        result_sender: Sender<R>,
        task_queue: Arc<TaskQueue<T>>
    ) {
        std::thread::spawn(move || {
            loop {
                let task = task_queue.wait_for_task();
                let result = worker_function(task);
                result_sender.send(result).unwrap();
            }
        });
    }

    fn spawn_queue_buffer_thread(
        task_queue: Arc<TaskQueue<T>>,
        task_receiver: Receiver<T>
    ) {
        std::thread::spawn(move || {
            for task in task_receiver.iter() {
                task_queue.push(task);
            }
        });
    }

    pub fn add_task(&self, task: T) {
        self.task_sender.send(task).unwrap();
    }

    pub fn add_tasks(&self, tasks: impl IntoIterator<Item = T>) {
        for task in tasks {
            self.task_sender.send(task).unwrap();
        }
    }

    /// Clear the task queue. Task that are currently being processed will not be interrupted.
    pub fn clear_queue(&self) {
        self.task_queue.clear_queue();
    }

    /// Write available results into the buffer and return the number of results written.
    /// If the buffer is too small to hold all available results, the remaining results will be left in the queue.
    pub fn recieve_results_in_buffer(&self, buffer: &mut [R]) -> usize {
        let mut indx = 0;
        while indx < buffer.len() {
            match self.result_receiver.try_recv() {
                Ok(result) => {
                    buffer[indx] = result;
                    indx += 1;
                }
                Err(_) => break,
            }
        }
        indx
    }

    // Recieve all available results and return them in a vector.
    pub fn receive_results(&self) -> Vec<R> {
        let mut results = Vec::new();
        while let Ok(result) = self.result_receiver.try_recv() {
            results.push(result);
        }
        results
    }

    pub fn current_queue_size(&self) -> usize {
        self.task_queue.len()
    }
}

