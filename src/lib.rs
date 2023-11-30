use std::sync::Arc;

use once_cell::sync::Lazy;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub mod convert;
pub mod font;
pub mod maps;
pub mod midi;
pub mod ui;
pub mod util;

pub static POOL: Lazy<Arc<ThreadPool>> = Lazy::new(|| {
    let pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();
    Arc::new(pool)
});
