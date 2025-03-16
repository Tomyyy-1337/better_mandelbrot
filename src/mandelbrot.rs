use crate::{complex::Complex, worker::State};

pub struct Task<T> {
    pub x: T,
    pub y: T,
    pub chunk_size: T,
    pub resolution: u32,
    pub max_iter: u32,
}

impl<T> Task<T>
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::AddAssign
        + PartialOrd
        + From<u32>,
{
    pub fn calc_chunk(task: Task<T>, state: &State) -> Option<Vec<u32>> {
        let mut results = Vec::with_capacity((task.resolution * task.resolution) as usize);
        let step_size = task.chunk_size / T::from(task.resolution);
        for i in 0..task.resolution {
            for j in 0..task.resolution {
                let a = task.x + (step_size * T::from(i));
                let b = task.y + (step_size * T::from(j));
                results.push(Self::calc_mandelbrot(a, b, task.max_iter));
            }
        }
        Some(results)
    }

    fn calc_mandelbrot(a: T, b: T, max_iter: u32) -> u32 {
        let mut z = Complex::new(T::from(0), T::from(0));
        let c = Complex::new(a, b);
        let mut iter = 0;
        while z.norm() <= T::from(4) && iter < max_iter {
            z *= z;
            z += c;
            iter += 1;
        }
        iter
    }
}