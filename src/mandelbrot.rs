use crate::complex::ComplexF256;
use f256::f256;

fn calc_mandelbrot_f256(a: f256, b: f256, max_iter: u64) -> u64 {
    let mut z = ComplexF256::new(f256::from(0), f256::from(0));
    let c = ComplexF256::new(a, b);
    let mut iter = 0;
    while z.norm() <= f256::from(4) && iter < max_iter {
        z.square();
        z.add(&c);
        iter += 1;
    }
    iter
}

pub struct F256Task {
    pub x: f256,
    pub y: f256,
    pub chunk_size: f256,
    pub resolution: u64,
    pub max_iter: u64,
}

pub fn calc_chunk_f256(task: F256Task) -> Vec<u64> {
    let mut results = Vec::new();
    let step_size = task.chunk_size / f256::from(task.resolution);
    for i in 0..task.resolution {
        for j in 0..task.resolution {
            let a = task.x + (step_size * f256::from(i));
            let b = task.y + (step_size * f256::from(j));
            results.push(calc_mandelbrot_f256(a, b, task.max_iter));
        }
    }
    results
}
