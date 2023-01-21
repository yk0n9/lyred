#![windows_subsystem = "windows"]

use anyhow::Result;
use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::{Body, Heading, Small};
use eframe::egui::{Context, FontId, Slider, Vec2};
use eframe::Theme::Light;
use eframe::{egui, Frame, IconData, NativeOptions};
use egui::TextStyle::*;
use lyred::midi::{init, KeyEvent, Mode, playback};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use windows_hotkeys::get_global_keystate;
use windows_hotkeys::keys::VKey;

fn main() -> Result<()> {
    let mut options = NativeOptions {
        default_theme: Light,
        resizable: false,
        initial_window_size: Some(Vec2::new(800.0, 600.0)),
        ..NativeOptions::default()
    };
    let icon_bytes = include_bytes!("../resources/lyre.ico");
    let icon_buffer = image::load_from_memory(icon_bytes).ok().unwrap();
    let icon = icon_buffer.to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let pixels = icon.into_vec();
    let icon_data = IconData {
        rgba: pixels,
        width: icon_width,
        height: icon_height,
    };
    options.icon_data = Some(icon_data);
    eframe::run_native("Lyred", options, Box::new(|_| Box::new(Player::default())));

    Ok(())
}

pub struct Player {
    pub speed: Arc<Mutex<f64>>,
    pub tuned: bool,
    pub is_play: Arc<Mutex<bool>>,
    pub pause: Arc<Mutex<bool>>,
    pub state: String,
    pub opened_file: Arc<Mutex<Option<PathBuf>>>,
    pub events: Arc<Mutex<Vec<KeyEvent>>>,
    pub mode: Mode,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: Arc::new(Mutex::new(1.0)),
            tuned: false,
            is_play: Arc::new(Mutex::new(false)),
            pause: Arc::new(Mutex::new(false)),
            state: format!("已停止播放"),
            opened_file: Arc::new(Mutex::new(None)),
            events: Arc::new(Mutex::new(vec![])),
            mode: Mode::GenShin,
        }
    }
}

impl eframe::App for Player {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint();
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../resources/msyhbd.ttc")),
        );
        fonts
            .families
            .entry(Proportional)
            .or_default()
            .insert(0, "font".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("font".to_owned());
        ctx.set_fonts(fonts);
        let mut style = (*ctx.style()).clone();

        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
            .into();

        ctx.set_style(style);

        let is_play = Arc::clone(&self.is_play);
        let speed = Arc::clone(&self.speed);
        let pause = Arc::clone(&self.pause);
        let opened_file = Arc::clone(&self.opened_file);
        let show_file = opened_file.clone();
        let events = Arc::clone(&self.events);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Lyred by Ykong1337");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("选择你的MIDI文件");
                if (ui.button("打开")).clicked() {
                    *is_play.lock().unwrap() = false;
                    *pause.lock().unwrap() = false;
                    init(opened_file, events);
                }
            });
            if let Some(path) = show_file.lock().unwrap().clone() {
                ui.label(&format!("你选择的是: {}", path.to_str().unwrap()));
            }
            ui.separator();
            ui.label("选择你的模式");
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.mode, Mode::GenShin, "GenShin");
                ui.radio_value(&mut self.mode, Mode::VRChat, "VRChat-中文吧");
            });
            ui.separator();
            ui.label(&format!("你的播放速度是: {}x", *speed.lock().unwrap()));
            ui.add(Slider::new(&mut *speed.lock().unwrap(), 0.1..=5.0).text("速度"));
            if ui.button("- 0.1x").clicked() {
                if *speed.lock().unwrap() > 0.1 {
                    *speed.lock().unwrap() -= 0.1;
                }
            }
            if ui.button("+ 0.1x").clicked() {
                *speed.lock().unwrap() += 0.1;
            }
            ui.checkbox(&mut self.tuned, "开启自动调音");
            ui.separator();
            ui.label(&self.state);
            if get_global_keystate(VKey::Space) {
                *pause.lock().unwrap() = false;
                if !*is_play.lock().unwrap() {
                    *is_play.lock().unwrap() = true;
                    playback(
                        Arc::clone(&self.events),
                        self.tuned,
                        Arc::clone(&self.speed),
                        Arc::clone(&self.is_play),
                        Arc::clone(&self.pause),
                        self.mode.clone(),
                    );
                }
            }
            if get_global_keystate(VKey::Control) {
                *is_play.lock().unwrap() = false;
                *pause.lock().unwrap() = false;
            }
            if get_global_keystate(VKey::Back) {
                if !*pause.lock().unwrap() {
                    *pause.lock().unwrap() = true;
                }
            }
            if *is_play.lock().unwrap() && !*pause.lock().unwrap() {
                self.state = format!("正在播放中...");
            }
            if !*is_play.lock().unwrap() {
                self.state = format!("已停止播放");
            }
            if *pause.lock().unwrap() && *is_play.lock().unwrap() {
                self.state = format!("已暂停播放");
            }
            ui.separator();
            ui.label("按下Space键开始播放 | 继续播放");
            ui.label("按下Backspace键暂停播放");
            ui.label("按下Ctrl键停止播放");
        });
    }
}
