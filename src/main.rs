use std::{thread::{available_parallelism, sleep}, time::Duration};

use mandelbrot_lib::Worker;
use rand::random_range;

fn main() {
    let available_threads = available_parallelism().unwrap().get();

    let worker = Worker::new(
        available_threads - 2,
        worker_function
    );


    let start_tasks = 0..48;
    worker.add_tasks(start_tasks);

    let long_running_tasks = (0..48).map(|_| random_range(40..=50));
    worker.add_tasks(long_running_tasks);

    let mut results = vec![0; 200];
    let mut len;

    loop {
        println!("\n====================================================================");
        println!("Current queue size: {}", worker.current_queue_size());
        println!("====================================================================");
        sleep(Duration::from_millis(750));

        let start_read_time = std::time::Instant::now();
        // let results = worker.receive_results();
        len = worker.recieve_results_in_buffer(&mut results);
        println!("Read time: {:?}", start_read_time.elapsed());

        for result in results.iter().take(len) {
            println!("Task completed result: {}", result);
        }
        
        println!("Adding a new task");
        let add_task_time = std::time::Instant::now();
        worker.add_task(random_range(30..=52));
        worker.add_tasks([10,11,12,13]);
        worker.add_tasks(14..40);
        println!("Add task time: {:?}", add_task_time.elapsed());
    }
}
  
fn worker_function(num: u64) -> u64 {
    fib(num)
} 

fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}
