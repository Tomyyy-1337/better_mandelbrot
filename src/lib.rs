mod complex;

mod task_queue;

mod mandelbrot;
pub use crate::mandelbrot::Task;

mod worker_state;
pub use crate::worker_state::State;

mod worker;
pub use crate::worker::Worker;