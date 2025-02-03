use std::sync::LazyLock;

use crossbeam::atomic::AtomicCell;
use parking_lot::RwLock;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub mod convert;
pub mod font;
pub mod maps;
pub mod midi;
pub mod ui;
pub mod util;

pub static TIME_SHIFT: AtomicCell<bool> = AtomicCell::new(false);
pub static LOCAL: AtomicCell<usize> = AtomicCell::new(0);
pub static COUNT: RwLock<Vec<usize>> = RwLock::new(vec![]);
pub static POOL: LazyLock<ThreadPool> =
    LazyLock::new(|| ThreadPoolBuilder::new().num_threads(2).build().unwrap());
