#![windows_subsystem = "windows"]

use eframe::egui::Vec2;
use eframe::{IconData, NativeOptions};
use lyred::midi::{BACK, CTRL, SPACE};
use lyred::ui::Play;
use rdev::{listen, Event, EventType, Key};
use std::sync::atomic::Ordering;

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
    let icon_bytes: &[u8] = include_bytes!("../resources/lyre.ico");
    let icon_buffer = image::load_from_memory(icon_bytes).unwrap();
    let icon = icon_buffer.to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let pixels = icon.into_vec();
    let icon_data = IconData {
        rgba: pixels,
        width: icon_width,
        height: icon_height,
    };
    options.icon_data = Some(icon_data);
    rayon::spawn(move || {
        listen(callback).unwrap();
    });
    eframe::run_native("Lyred", options, Box::new(|cc| Box::new(Play::new(cc)))).unwrap();
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(Key::Space) => SPACE.store(true, Ordering::Relaxed),
        EventType::KeyRelease(Key::Space) => SPACE.store(false, Ordering::Relaxed),
        EventType::KeyPress(Key::ControlLeft) => CTRL.store(true, Ordering::Relaxed),
        EventType::KeyRelease(Key::ControlLeft) => CTRL.store(false, Ordering::Relaxed),
        EventType::KeyPress(Key::Backspace) => BACK.store(true, Ordering::Relaxed),
        EventType::KeyRelease(Key::Backspace) => BACK.store(false, Ordering::Relaxed),
        _ => {}
    }
}
