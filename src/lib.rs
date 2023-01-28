use std::sync::{Arc, Mutex};

use rdev::{EventType, Key, simulate};

pub mod midi;
pub mod maps;

pub type Data<T> = Arc<Mutex<T>>;

fn press(event: &EventType) {
    if let Ok(()) = simulate(event) {
        ()
    }
}

pub fn send(key: Key) {
    press(&EventType::KeyPress(key));
    press(&EventType::KeyRelease(key));
}