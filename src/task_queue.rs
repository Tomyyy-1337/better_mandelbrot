use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub struct TaskQueue<T> {
    tasks: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> TaskQueue<T> {
    pub fn len(&self) -> usize {
        self.tasks.lock().unwrap().len()
    }

    pub fn clear_queue(&self) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.clear();
    }

    pub fn new() -> TaskQueue<T> {
        TaskQueue {
            tasks: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }

    pub fn push(&self, task: T) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push_back(task);
        self.condvar.notify_one();
    }

    pub fn wait_for_task(&self) -> T {
        let mut tasks = self.tasks.lock().unwrap();
        while tasks.is_empty() {
            tasks = self.condvar.wait(tasks).unwrap();
        }
        tasks.pop_front().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn test_task_queue() {
        let task_queue = Arc::new(TaskQueue::new());
        assert_eq!(task_queue.len(), 0);
        task_queue.push(1);
        assert_eq!(task_queue.len(), 1);
        task_queue.push(2);
        assert_eq!(task_queue.len(), 2);
        assert_eq!(task_queue.wait_for_task(), 1);
        assert_eq!(task_queue.len(), 1);
        assert_eq!(task_queue.wait_for_task(), 2);
        assert_eq!(task_queue.len(), 0);

        let task_queue_clone = task_queue.clone();
        let t = std::thread::spawn(move || {
            task_queue_clone.wait_for_task();
        });

        sleep(Duration::from_millis(500));
        assert!(!t.is_finished());

        task_queue.push(1);
        assert!(t.join().is_ok());
    }
}
