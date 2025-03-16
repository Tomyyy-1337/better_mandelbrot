use std::thread::sleep;

use mandelbrot_lib::{State, Worker};
use multi_compare::c;

fn main() {
    let mut worker = Worker::new(4, task);

    worker.add_tasks([10000; 4]);   
    
    sleep(std::time::Duration::from_secs(1));
    
    println!("Cancelling all tasks");
    worker.clear_queue();
    
    println!("Waiting for all results");
    worker.wait_for_all_results();
    
    worker.add_tasks([10000; 4]);   
    worker.wait_for_all_results();
}

fn task(n: u64, state: &State) -> Option<u64> {
    println!("Task: {}", n);
    let mut sum = 0;
    for i in 0..n {
        if state.is_cancelled() {
            println!("Task: {} cancelled", n);
            return None;
        }
        for j in 0..n {
            for k in 0..n {
                if c!(i < j < k) {
                    sum += 1;
                } else {
                    sum -= 1;
                }
            }
        }
    }
    println!("Task: {} done", n);
    Some(sum)
}