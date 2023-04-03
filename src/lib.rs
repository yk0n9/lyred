use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use once_cell::sync::Lazy;
use portable_atomic::AtomicF64;

use rdev::{simulate, EventType, Key};

pub mod maps;
pub mod midi;

pub static SPEED: Lazy<Arc<AtomicF64>> = Lazy::new(|| Arc::new(AtomicF64::new(1.0)));
pub static IS_PLAY: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));
pub static PAUSE: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));

pub type Data<T> = Arc<Mutex<T>>;

#[inline]
pub fn data_new<T>(val: T) -> Data<T> {
    Arc::new(Mutex::new(val))
}

#[inline]
fn press(event: &EventType) {
    simulate(event).unwrap();
}

#[inline]
pub fn send(key: Key) {
    press(&EventType::KeyPress(key));
    press(&EventType::KeyRelease(key));
}
