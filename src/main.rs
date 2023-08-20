#![windows_subsystem = "windows"]

use eframe::egui::Vec2;
use eframe::{IconData, NativeOptions};
use lyred::ui::Play;

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
    eframe::run_native("Lyred", options, Box::new(|cc| Box::new(Play::new(cc)))).unwrap();
}
