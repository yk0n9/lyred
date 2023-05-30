#![windows_subsystem = "windows"]

use eframe::egui::Vec2;
use eframe::{IconData, NativeOptions};
use lyred::ui::Play;

fn main() {
    run();
}

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
    eframe::run_native("Lyred", options, Box::new(|cc| Box::new(Play::new(cc)))).unwrap();
}
