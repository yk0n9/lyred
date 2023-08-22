#![windows_subsystem = "windows"]

use std::sync::atomic::Ordering;

use eframe::{IconData, NativeOptions};
use eframe::egui::Vec2;
use rdev::{Event, EventType};

use lyred::{BACK, CTRL, POOL, SPACE};
use lyred::ui::play::Play;

fn main() {
    run();
}

#[inline]
fn run() {
    let mut options = NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::new(400.0, 500.0)),
        ..NativeOptions::default()
    };
    let icon_data = IconData::try_from_png_bytes(include_bytes!("../resources/lyre.ico")).unwrap();
    options.icon_data = Some(icon_data);
    POOL.spawn(move || rdev::listen(callback).unwrap());
    eframe::run_native("Lyred", options, Box::new(|cc| Box::new(Play::new(cc)))).unwrap();
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(rdev::Key::Space) => SPACE.store(true, Ordering::Relaxed),
        EventType::KeyRelease(rdev::Key::Space) => SPACE.store(false, Ordering::Relaxed),
        EventType::KeyPress(rdev::Key::ControlLeft) => CTRL.store(true, Ordering::Relaxed),
        EventType::KeyRelease(rdev::Key::ControlLeft) => CTRL.store(false, Ordering::Relaxed),
        EventType::KeyPress(rdev::Key::Backspace) => BACK.store(true, Ordering::Relaxed),
        EventType::KeyRelease(rdev::Key::Backspace) => BACK.store(false, Ordering::Relaxed),
        _ => {}
    }
}