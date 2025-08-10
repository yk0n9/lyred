#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::sync::Arc;

use eframe::egui::{IconData, Vec2, ViewportBuilder};
use eframe::NativeOptions;

use lyred::maps::MAP;
use lyred::ui::play::Play;

fn main() {
    run();
}

#[inline]
fn run() {
    let image = image::load_from_memory(include_bytes!("../resources/lyre.ico")).unwrap();
    let viewport = ViewportBuilder {
        resizable: Some(false),
        icon: Some(Arc::new(IconData {
            width: image.width(),
            height: image.height(),
            rgba: image.into_rgba8().into_raw(),
        })),
        inner_size: Some(Vec2::new(400.0, 600.0)),
        maximize_button: Some(false),
        ..Default::default()
    };
    let options = NativeOptions {
        viewport,
        ..NativeOptions::default()
    };
    eframe::run_native(
        "Lyred",
        options,
        Box::new(|cc| {
            let mut play = Play::new(cc);
            if let Ok(file) = std::fs::read_to_string("config.ron") {
                play.config = ron::from_str(&file).unwrap_or_default();
                let dir = play.config.midi_dir.0.read();
                if !dir.is_empty() {
                    play.midi.get_midis_path(Path::new(dir.as_str()));
                }
                unsafe {
                    MAP = play.config.map;
                }
            }
            Ok(Box::new(play))
        }),
    )
    .unwrap();
}
