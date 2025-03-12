use std::sync::{
    Arc,
    mpsc::{Receiver, Sender},
};

use crate::task_queue::TaskQueue;

enum Work<T> {
    Task(T),
    Terminate,
}

pub struct Worker<T, R> 
where
    T: Send + 'static,
    R: Send + 'static 
{
    task_queue: Arc<TaskQueue<Work<T>>>,
    result_receiver: Receiver<R>,
    task_sender: Sender<Work<T>>,
    num_worker_threads: usize,
} 

impl<T, R> Worker<T, R>
where
    T: Send + 'static,
    R: Send + 'static,
{
    /// Create a new worker with a given number of worker threads and a worker function and start the worker threads.
    pub fn new(num_worker_threads: usize, worker_function: fn(T) -> R) -> Worker<T, R> {
        let (result_sender, result_receiver) = std::sync::mpsc::channel();
        let (task_sender, task_receiver) = std::sync::mpsc::channel();
        let task_queue = Arc::new(TaskQueue::new());

        let mut worker_threads: Vec<std::thread::JoinHandle<()>> = (0..num_worker_threads.max(1))
            .map(|_| {
                Self::spawn_worker_thread(
                    worker_function,
                    result_sender.clone(),
                    task_queue.clone(),
                )
            })
            .collect();

        worker_threads.push(Self::spawn_queue_buffer_thread(task_queue.clone(), task_receiver));

        Worker {
            task_queue,
            result_receiver,
            task_sender,
            num_worker_threads,
        }
    }

    pub fn add_task(&self, task: T) {
        self.task_sender.send(Work::Task(task)).unwrap();
    }

    pub fn add_tasks(&self, tasks: impl IntoIterator<Item = T>) {
        for task in tasks {
            self.task_sender.send(Work::Task(task)).unwrap();
        }
    }

    /// Clear the task queue. Task that are currently being processed will not be interrupted.
    pub fn clear_queue(&self) {
        self.task_queue.clear_queue();
    }

    /// Write available results into the buffer and return the number of results written.
    /// If the buffer is too small to hold all available results, the remaining results will be left in the queue.
    pub fn receive_results_in_buffer(&self, buffer: &mut [R]) -> usize {
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

    /// Wait for the next result and return it. Blocks until a result is available.
    pub fn wait_for_result(&self) -> R {
        self.result_receiver.recv().unwrap()
    }

    /// Receive all available results and return them in a vector.
    pub fn receive_all_results(&self) -> Vec<R> {
        let mut results = Vec::new();
        while let Ok(result) = self.result_receiver.try_recv() {
            results.push(result);
        }
        results
    }

    pub fn current_queue_size(&self) -> usize {
        self.task_queue.len()
    }

    fn spawn_worker_thread(
        worker_function: fn(T) -> R,
        result_sender: Sender<R>,
        task_queue: Arc<TaskQueue<Work<T>>>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            loop {
                let task = task_queue.wait_for_task();
                match task {
                    Work::Terminate => break,
                    Work::Task(task) => {
                        let result = worker_function(task);
                        result_sender.send(result).unwrap();
                    }
                }
            }
        })
    }

    fn spawn_queue_buffer_thread(
        task_queue: Arc<TaskQueue<Work<T>>>,
        task_receiver: Receiver<Work<T>>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            for task in task_receiver.iter() {
                match task {
                    Work::Terminate => break,
                    Work::Task(_) => task_queue.push(task),
                }
            }
        })
    }
}

impl<T, R> Drop for Worker<T, R>
where
    T: Send + 'static,
    R: Send + 'static,
{
    /// Drop the worker and terminate all worker threads.
    fn drop(&mut self) {
        self.clear_queue();

        for _ in 0..self.num_worker_threads {
            self.task_queue.push(Work::Terminate);
        }
        self.task_sender.send(Work::Terminate).unwrap();
    }
}