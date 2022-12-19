#![windows_subsystem = "windows"]

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use eframe::egui::{Context, FontId, Slider, Vec2};
use eframe::{egui, Frame, NativeOptions};
use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::{Body, Heading, Small};
use eframe::Theme::Light;
use enigo::{Enigo, Key, KeyboardControllable};
use lyred::midi::{c, init, KeyEvent, tune};
use egui::TextStyle::*;
use egui_file::FileDialog;
use windows_hotkeys::get_global_keystate;
use windows_hotkeys::keys::VKey;

fn main() {
    let options = NativeOptions {
        default_theme: Light,
        resizable: false,
        initial_window_size: Some(Vec2::new(800.0, 600.0)),
        ..NativeOptions::default()
    };
    eframe::run_native("Lyred", options, Box::new(|_| Box::new(Player::default())));
}

pub struct Player {
    pub speed: Arc<Mutex<f64>>,
    pub tuned: bool,
    pub is_play: Arc<Mutex<bool>>,
    pub pause: Arc<Mutex<bool>>,
    pub state: String,
    pub opened_file: Option<PathBuf>,
    pub open_file_dialog: Option<FileDialog>,
    pub events: Vec<KeyEvent>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: Arc::new(Mutex::new(1.0)),
            tuned: false,
            is_play: Arc::new(Mutex::new(false)),
            pause: Arc::new(Mutex::new(false)),
            state: format!("已停止播放"),
            opened_file: None,
            open_file_dialog: None,
            events: vec![],
        }
    }
}

impl Player {
    pub fn playback(message: Vec<KeyEvent>, tuned: bool, speed: Arc<Mutex<f64>>, is_play: Arc<Mutex<bool>>, pause: Arc<Mutex<bool>>) {
        let _ = thread::spawn(move || {
            let mut click = Enigo::new();
            let mut shift = 0;

            if tuned {
                shift = tune(message.clone());
            }

            let mut start_time = Local::now().timestamp_millis();
            let mut input_time = 0.0;
            for msg in message.iter() {
                if *pause.lock().unwrap() {
                    loop {
                        if !*pause.lock().unwrap() {
                            input_time = msg.delay;
                            start_time = Local::now().timestamp_millis();
                            break;
                        }
                    }
                }

                if !*is_play.lock().unwrap() {
                    break;
                }

                input_time += msg.delay / *speed.lock().unwrap();

                let playback_time = (Local::now().timestamp_millis() - start_time) as f64;
                let current_time = (input_time - playback_time) as u64;
                if current_time > 0 {
                    sleep(Duration::from_millis(current_time));
                }

                match c((msg.press as i32 + shift) as u8) {
                    Some(key) => {
                        click.key_down(Key::Layout(key));
                        click.key_up(Key::Layout(key));
                    }
                    _ => {}
                }
            }
            *is_play.lock().unwrap() = false;
        });
    }
}

impl eframe::App for Player {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint();
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("msyhbd.ttc")),
        );
        fonts
            .families
            .entry(Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());
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
        ].into();

        ctx.set_style(style);

        let is_play = Arc::clone(&self.is_play);
        let speed = Arc::clone(&self.speed);
        let pause = Arc::clone(&self.pause);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Lyred by Ykong1337");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("选择你的MIDI文件");
                if (ui.button("打开")).clicked() {
                    let filter = Box::new(|path: &Path| -> bool {
                        path.extension() == Some(OsStr::new("mid"))
                    });
                    let mut dialog = FileDialog::open_file(self.opened_file.clone()).filter(filter);
                    dialog.open();
                    self.open_file_dialog = Some(dialog);
                    *is_play.lock().unwrap() = false;
                    *pause.lock().unwrap() = false;
                }

                if let Some(dialog) = &mut self.open_file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            self.events = init(file.to_str().unwrap()).unwrap();
                            self.opened_file = Some(file);
                        }
                    }
                }
            });
            if let Some(path) = &self.opened_file {
                ui.label(&format!("你选择的是: {}", path.to_str().unwrap()));
            }
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
            if get_global_keystate(VKey::Return) {
                *pause.lock().unwrap() = false;
                if !*is_play.lock().unwrap() {
                    *is_play.lock().unwrap() = true;
                    Player::playback(self.events.clone(), self.tuned, Arc::clone(&self.speed), Arc::clone(&self.is_play), Arc::clone(&self.pause));
                }
            }
            if get_global_keystate(VKey::Shift) {
                *is_play.lock().unwrap() = false;
                *pause.lock().unwrap() = false;
            }
            if get_global_keystate(VKey::P) {
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
            ui.label("按下Enter键开始播放 | 继续播放");
            ui.label("按下P键暂停播放");
            ui.label("按下Shift键停止播放");
        });
    }
}