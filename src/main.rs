use std::thread::available_parallelism;

use f256::f256;
use mandelbrot_lib::{
    Worker,
    mandelbrot::{F256Task, calc_chunk_f256},
};

fn main() {
    let available_threads = available_parallelism().unwrap().get();

    let width = 3440;
    let height = 1440;
    let chunk_resolution = 64;

    let width_in_chunks = width / chunk_resolution + 1;
    let height_in_chunks = height / chunk_resolution + 1;
    let chunk_size = f256::from(4) / f256::from(width_in_chunks);

    let num_tasks = width_in_chunks * height_in_chunks;

    let worker = Worker::new(
        available_threads, 
        calc_chunk_f256
    );

    let start = std::time::Instant::now();
    println!("Starting {} tasks...", num_tasks);
    for x in 0..width_in_chunks {
        for y in 0..height_in_chunks {
            let x = f256::from(-2) + f256::from(x) * chunk_size;
            let y = f256::from(-2) + f256::from(y) * chunk_size;
            worker.add_task(F256Task {
                x,
                y,
                chunk_size,
                resolution: chunk_resolution,
                max_iter: 200,
            });
        }
    }

    println!("Waiting for results...");

    let mut results = 0;
    while results < num_tasks {
        worker.wait_for_result();
        results += 1;
    }
    println!("Time taken: {:?}", start.elapsed());
}
