use std::thread::available_parallelism;

use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot_lib::{Task, Worker};

#[cfg(not(any(feature = "quad", feature = "octo")))]
type Num = f64;
#[cfg(feature = "quad")]
type Num = fpdec::Decimal;
#[cfg(feature = "octo")]
type Num = f256::f256;

fn criterion_benchmark(c: &mut Criterion) {    
    let mut c = c.benchmark_group("repaint");
    c.sample_size(10);

    let available_threads = available_parallelism().unwrap().get();
    let mut worker = Worker::new(
        available_threads - 1, 
        Task::calc_chunk
    );


    let type_name = std::any::type_name::<Num>();
    
    c.bench_function(format!("{type_name} Repaint"), |b| b.iter(|| repaint::<Num>(&mut worker)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn repaint<Num>(worker: &mut Worker<Task<Num>, Vec<u32>>) 
where 
    Num: Send + 'static
        + Copy
        + std::ops::Mul<Output = Num>
        + std::ops::Add<Output = Num>
        + std::ops::Sub<Output = Num>
        + std::ops::Div<Output = Num>
        + std::ops::AddAssign
        + PartialOrd
        + From<u32>
        + From<i32>
{
    let width = 3440;
    let height = 1440;
    let chunk_resolution = 64;

    let width_in_chunks = width / chunk_resolution + 1;
    let height_in_chunks = height / chunk_resolution + 1;
    let chunk_size = Num::from(4) / Num::from(width_in_chunks);

    let new_tiles = (0..width_in_chunks)
        .flat_map(|x| (0..height_in_chunks).map(move |y| (x, y)))
        .map(|(x, y)| {
            let x = Num::from(-2) + Num::from(x) * Num::from(chunk_size);
            let y = Num::from(-2) + Num::from(y) * Num::from(chunk_size);
            Task {
                x,
                y,
                chunk_size,
                resolution: chunk_resolution,
                max_iter: 200,
            }
        });

    worker.add_tasks(new_tiles);

    worker.wait_for_all_results();
}
