mod complex;

mod task_queue;

mod mandelbrot;
pub use crate::mandelbrot::Task;

mod worker;
pub use crate::worker::{Worker, State};
