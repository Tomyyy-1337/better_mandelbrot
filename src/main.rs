use std::thread::{available_parallelism, sleep};

use f256::f256;
use fpdec::Decimal;

use mandelbrot_lib::{Task, Worker};

type Nums = f64;
// type Nums = Decimal;
// type Nums = f256;

fn main() {
    let available_threads = available_parallelism().unwrap().get();

    let width = 3440;
    let height = 1440;
    let chunk_resolution = 64;

    let width_in_chunks = width / chunk_resolution + 1;
    let height_in_chunks = height / chunk_resolution + 1;
    let chunk_size = Nums::from(4) / Nums::from(width_in_chunks);

    let num_tasks = width_in_chunks * height_in_chunks;

    let mut worker = Worker::new(
        available_threads - 1, 
        Task::calc_chunk
    );

    println!("Starting {} tasks...", num_tasks);
    let start = std::time::Instant::now();

    let new_tiles = (0..width_in_chunks)
        .flat_map(|x| (0..height_in_chunks).map(move |y| (x, y)))
        .map(|(x, y)| {
            let x = Nums::from(-2) + Nums::from(x) * Nums::from(chunk_size);
            let y = Nums::from(-2) + Nums::from(y) * Nums::from(chunk_size);
            Task {
                x,
                y,
                chunk_size,
                resolution: chunk_resolution,
                max_iter: 100,
            }
        });

    worker.add_tasks(new_tiles.clone());
    sleep(std::time::Duration::from_millis(1));
    println!("Starting 20 tasks...");
    let add_start = std::time::Instant::now();
    worker.add_tasks(new_tiles.take(20));

    println!("Dispatching 20 tasks took: {:?}", add_start.elapsed());

    let tasks_num = worker.wait_for_all_results().len();
    println!("{} Results received", tasks_num);
    println!("Calculation time: {:?}", start.elapsed());
}
