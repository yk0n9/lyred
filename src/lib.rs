use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::Arc;

use once_cell::sync::Lazy;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub mod convert;
pub mod font;
pub mod maps;
pub mod midi;
pub mod ui;
pub mod util;

pub const STOP: usize = 0;
pub const PLAYING: usize = 1;
pub const PAUSE: usize = 2;

pub static POOL: Lazy<Arc<ThreadPool>> = Lazy::new(|| {
    let pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();
    Arc::new(pool)
});

pub static TIME_SHIFT: AtomicBool = AtomicBool::new(false);
pub static LOCAL: AtomicUsize = AtomicUsize::new(usize::MAX);

pub static mut COUNT: Vec<usize> = Vec::new();
