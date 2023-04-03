#![windows_subsystem = "windows"]

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::{Body, Heading, Small};
use eframe::egui::{Context, FontData, FontFamily, FontId, Slider, Vec2};
use eframe::Theme::Light;
use eframe::{egui, Frame, IconData, NativeOptions};
use egui::TextStyle::*;
use portable_atomic::AtomicF64;
use windows_hotkeys::get_global_keystate;
use windows_hotkeys::keys::VKey;

use lyred::midi::{init, playback, KeyEvent, Mode};
use lyred::{data_new, Data, SPEED, IS_PLAY, PAUSE};

fn main() {
    let mut options = NativeOptions {
        default_theme: Light,
        resizable: false,
        initial_window_size: Some(Vec2::new(800.0, 600.0)),
        ..NativeOptions::default()
    };
    let icon_bytes = include_bytes!("../resources/lyre.ico");
    let icon_buffer = image::load_from_memory(icon_bytes)
        .unwrap();
    let icon = icon_buffer.to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let pixels = icon.into_vec();
    let icon_data = IconData {
        rgba: pixels,
        width: icon_width,
        height: icon_height,
    };
    options.icon_data = Some(icon_data);
    eframe::run_native("Lyred",
                       options,
                       Box::new(|_| Box::new(Player::default())))
        .unwrap();
}

pub struct Player {
    pub speed: Arc<AtomicF64>,
    pub tuned: bool,
    pub is_play: Arc<AtomicBool>,
    pub pause: Arc<AtomicBool>,
    pub state: String,
    pub opened_file: Data<Option<PathBuf>>,
    pub events: Data<Vec<KeyEvent>>,
    pub mode: Mode,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: SPEED.clone(),
            tuned: false,
            is_play: IS_PLAY.clone(),
            pause: PAUSE.clone(),
            state: format!("已停止播放"),
            opened_file: data_new(None),
            events: data_new(vec![]),
            mode: Mode::GenShin,
        }
    }
}

impl eframe::App for Player {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint();
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "font".to_owned(),
            FontData::from_static(include_bytes!("../resources/msyh.ttc")),
        );
        fonts.families
            .get_mut(&Proportional)
            .unwrap()
            .insert(0, "font".to_owned());
        fonts.families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
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

        let is_play = self.is_play.clone();
        let speed = self.speed.clone();
        let pause = self.pause.clone();
        let opened_file = self.opened_file.clone();
        let show_file = self.opened_file.clone();
        let events = self.events.clone();
        let mut m_speed = 1.0;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Lyred by Ykong1337");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("选择你的MIDI文件");
                if (ui.button("打开")).clicked() {
                    is_play.store(false, Ordering::Relaxed);
                    pause.store(false, Ordering::Relaxed);
                    init(opened_file, events.clone());
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
            ui.label(&format!("你的播放速度是: {}x", speed.load(Ordering::Relaxed)));
            ui.add(Slider::new(&mut m_speed, 0.1..=5.0).text("速度"));
            speed.store(m_speed, Ordering::Relaxed);
            if ui.button("- 0.1x").clicked() {
                if speed.load(Ordering::Relaxed) > 0.1 {
                    speed.fetch_sub(0.1, Ordering::Relaxed);
                }
            }
            if ui.button("+ 0.1x").clicked() {
                speed.fetch_add(0.1, Ordering::Relaxed);
            }
            ui.checkbox(&mut self.tuned, "开启自动调音");
            ui.separator();
            ui.label(&self.state);
            if get_global_keystate(VKey::Space) {
                pause.store(false, Ordering::Relaxed);
                if !is_play.load(Ordering::Relaxed) {
                    is_play.store(true, Ordering::Relaxed);
                    playback(
                        events.clone(),
                        self.tuned,
                        speed.clone(),
                        is_play.clone(),
                        pause.clone(),
                        self.mode.clone(),
                    );
                }
            }
            if get_global_keystate(VKey::Control) {
                pause.store(false, Ordering::Relaxed);
                is_play.store(false, Ordering::Relaxed);
            }
            if get_global_keystate(VKey::Back) {
                if !pause.load(Ordering::Relaxed) {
                    pause.store(true, Ordering::Relaxed);
                }
            }
            if is_play.load(Ordering::Relaxed) && !pause.load(Ordering::Relaxed) {
                self.state = format!("正在播放中...");
            }
            if !is_play.load(Ordering::Relaxed) {
                self.state = format!("已停止播放");
            }
            if pause.load(Ordering::Relaxed) && is_play.load(Ordering::Relaxed) {
                self.state = format!("已暂停播放");
            }
            ui.separator();
            ui.label("按下Space键开始播放 | 继续播放");
            ui.label("按下Backspace键暂停播放");
            ui.label("按下Ctrl键停止播放");
        });
    }
}
