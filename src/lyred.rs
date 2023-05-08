#![windows_subsystem = "windows"]

use std::path::PathBuf;
use std::sync::atomic::Ordering;

use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::{Body, Heading, Small};
use eframe::egui::{Context, FontId, Slider, Vec2};
use eframe::Theme::Light;
use eframe::{CreationContext, egui, Frame, IconData, NativeOptions};
use egui::TextStyle::*;
use lyred::font::load_fonts;
use windows_hotkeys::get_global_keystate;
use windows_hotkeys::keys::VKey;

use lyred::convert::convert_from_midi;
use lyred::midi::{init, playback, KeyEvent, Mode};
use lyred::{data_new, Data, IS_PLAY, PAUSE, SPEED};

fn main() {
    let mut options = NativeOptions {
        default_theme: Light,
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
    eframe::run_native(
        "Lyred",
        options,
        Box::new(|cc| Box::new(Player::new(cc))),
    )
        .unwrap();
}

pub struct Player {
    pub speed: f64,
    pub tuned: bool,
    pub state: String,
    pub opened_file: Data<Option<PathBuf>>,
    pub events: Data<Vec<KeyEvent>>,
    pub mode: Mode,
}

impl Player {
    fn new(cc: &CreationContext) -> Self {
        load_fonts(&cc.egui_ctx);
        let mut style = (*cc.egui_ctx.style()).clone();
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
        cc.egui_ctx.set_style(style);

        Self {
            speed: 1.0,
            tuned: false,
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Lyred by Ykong1337");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("选择你的MIDI文件");
                if ui.button("打开").clicked() {
                    IS_PLAY.store(false, Ordering::Relaxed);
                    PAUSE.store(false, Ordering::Relaxed);
                    init(self.opened_file.clone(), self.events.clone());
                }
                if ui.button("从MIDI转换").clicked() {
                    if let Some(path) = self.opened_file.clone().lock().unwrap().as_ref() {
                        let name = path.file_stem().unwrap().to_string_lossy().to_string();
                        convert_from_midi(name, self.events.clone());
                    }
                }
            });
            if let Some(path) = self.opened_file.clone().lock().unwrap().as_ref() {
                ui.label(&format!("你选择的是: {}", path.to_str().unwrap()));
            }
            ui.separator();
            ui.label("选择你的模式");
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.mode, Mode::GenShin, "GenShin");
                ui.radio_value(&mut self.mode, Mode::VRChat, "VRChat-中文吧");
            });
            ui.separator();
            ui.label(&format!(
                "你的播放速度是: {}x",
                SPEED.load(Ordering::Relaxed)
            ));
            ui.add(Slider::new(&mut self.speed, 0.1..=5.0).text("速度"));
            SPEED.store(self.speed, Ordering::Relaxed);
            ui.horizontal(|ui| {
                if ui.button("减速0.1x").clicked() {
                    if SPEED.load(Ordering::Relaxed) > 0.1 {
                        self.speed -= 0.1;
                        SPEED.store(self.speed, Ordering::Relaxed);
                    }
                }
                if ui.button("加速0.1x").clicked() {
                    self.speed += 0.1;
                    SPEED.store(self.speed, Ordering::Relaxed);
                }
            });
            ui.checkbox(&mut self.tuned, "开启自动调音");
            ui.separator();
            ui.label(&self.state);
            if get_global_keystate(VKey::Space) {
                PAUSE.store(false, Ordering::Relaxed);
                if !IS_PLAY.load(Ordering::Relaxed) {
                    IS_PLAY.store(true, Ordering::Relaxed);
                    playback(
                        self.events.clone(),
                        self.tuned,
                        SPEED.clone(),
                        IS_PLAY.clone(),
                        PAUSE.clone(),
                        self.mode.clone(),
                    );
                }
            }
            if get_global_keystate(VKey::Control) {
                PAUSE.store(false, Ordering::Relaxed);
                IS_PLAY.store(false, Ordering::Relaxed);
            }
            if get_global_keystate(VKey::Back) {
                if !PAUSE.load(Ordering::Relaxed) {
                    PAUSE.store(true, Ordering::Relaxed);
                }
            }
            if IS_PLAY.load(Ordering::Relaxed) && !PAUSE.load(Ordering::Relaxed) {
                self.state = format!("正在播放中...");
            }
            if !IS_PLAY.load(Ordering::Relaxed) {
                self.state = format!("已停止播放");
            }
            if PAUSE.load(Ordering::Relaxed) && IS_PLAY.load(Ordering::Relaxed) {
                self.state = format!("已暂停播放");
            }
            ui.separator();
            ui.label("按下Space键开始播放 | 继续播放");
            ui.label("按下Backspace键暂停播放");
            ui.label("按下Ctrl键停止播放");
        });
    }
}
