use std::sync::{
    atomic::{AtomicBool, AtomicUsize},
    LazyLock,
};

use crossbeam::atomic::AtomicCell;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub mod convert;
pub mod font;
pub mod maps;
pub mod midi;
pub mod ui;
pub mod util;

pub static TIME_SHIFT: AtomicBool = AtomicBool::new(false);
pub static LOCAL: AtomicUsize = AtomicUsize::new(!0);
pub static COUNT: AtomicCell<Vec<usize>> = AtomicCell::new(vec![]);
pub static POOL: LazyLock<ThreadPool> =
    LazyLock::new(|| ThreadPoolBuilder::new().num_threads(2).build().unwrap());
