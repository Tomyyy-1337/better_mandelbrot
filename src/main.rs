use std::thread::{available_parallelism, sleep};

use f256::f256;
use mandelbrot_lib::{
    Worker,
    mandelbrot::{F256Task, calc_chunk_f256},
};

fn main() {
    let w = Worker::new(4, |x: u64| x * x);
    w.add_tasks(0..10);
    drop(w);

    let available_threads = available_parallelism().unwrap().get();
    
    let width = 3440;
    let height = 1440;
    let chunk_resolution = 64;
    
    let width_in_chunks = width / chunk_resolution + 1;
    let height_in_chunks = height / chunk_resolution + 1;
    let chunk_size = f256::from(4) / f256::from(width_in_chunks);
    
    let mut num_tasks = width_in_chunks * height_in_chunks;
    
    let worker = Worker::new(
        available_threads - 1, 
        calc_chunk_f256
    );
    
    println!("Starting {} tasks...", num_tasks);
    let start = std::time::Instant::now();
    
    let new_tiles = (0..width_in_chunks)
        .flat_map(|x| (0..height_in_chunks).map(move |y| (x, y)))
        .map(|(x, y)| {
                let x = f256::from(-2) + f256::from(x) * chunk_size;
                let y = f256::from(-2) + f256::from(y) * chunk_size;
                F256Task {
                    x,
                    y,
                    chunk_size,
                    resolution: chunk_resolution,
                    max_iter: 100,
                }
            });
        
    worker.add_tasks(new_tiles.clone());
    sleep(std::time::Duration::from_millis(1));
    num_tasks += 20;
    println!("Starting 20 tasks...");
    let add_start = std::time::Instant::now();
    worker.add_tasks(new_tiles.take(20));

    println!("Dispatching 20 tasks took: {:?}", add_start.elapsed());

    let mut results = 0;
    while results < num_tasks {
        worker.wait_for_result();
        results += 1;
    }

    println!("Calculation time: {:?}", start.elapsed());
}
