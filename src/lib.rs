use std::sync::atomic::{AtomicBool, AtomicUsize};

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

pub static POOL: Lazy<ThreadPool> =
    Lazy::new(|| ThreadPoolBuilder::new().num_threads(2).build().unwrap());

pub static TIME_SHIFT: AtomicBool = AtomicBool::new(false);
pub static LOCAL: AtomicUsize = AtomicUsize::new(!0);

pub static mut COUNT: Vec<usize> = Vec::new();
