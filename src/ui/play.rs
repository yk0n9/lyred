use std::sync::atomic::Ordering;

use eframe::{CreationContext, egui};
use eframe::egui::{Context, FontId, Slider, Ui};
use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::*;
use windows_hotkeys::get_global_keystate;
use windows_hotkeys::keys::VKey;

use crate::font::load_fonts;
use crate::midi::{IS_PLAY, Midi, PAUSE, PLAYING, SPEED};
use crate::ui::{Module, View};

#[derive(Debug, Clone)]
pub struct Play<'a> {
    midi: Midi,
    tuned: bool,
    speed: f64,
    mode: Mode,
    state: &'a str,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    GenShin,
    VRChat,
}

impl Play<'_> {
    pub fn new(cc: &CreationContext) -> Self {
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
            midi: Midi::new(),
            tuned: false,
            speed: 1.0,
            mode: Mode::GenShin,
            state: "已停止",
        }
    }
}

impl Module for Play<'_> {
    fn name(&self) -> &'static str {
        "Lyred"
    }

    fn show(&mut self, ctx: &Context, _open: &mut bool) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}

impl View for Play<'_> {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| ui.heading("Lyred"));
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("选择MIDI文件");
            if ui.button("打开").clicked() {
                IS_PLAY.store(false, Ordering::Relaxed);
                PAUSE.store(false, Ordering::Relaxed);
                self.midi.clone().init();
            }
            if ui.button("从MIDI转换").clicked() {
                if let Some(name) = self.midi.clone().name.lock().unwrap().as_ref() {
                    self.midi.clone().convert_from_midi(name.to_string());
                }
            }
        });
        if let Some(name) = self.midi.clone().name.lock().unwrap().as_ref() {
            ui.label(&format!("当前文件: {}", name));
        }
        ui.separator();
        ui.label("选择模式");
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, Mode::GenShin, "GenShin");
            ui.radio_value(&mut self.mode, Mode::VRChat, "VRChat-中文吧");
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.add(Slider::new(&mut self.speed, 0.1..=5.0).prefix("播放速度:"));
            if ui.button("还原").clicked() {
                self.speed = 1.0;
            }
        });
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
        ui.label(self.state);
        ui.separator();
        ui.label("按下Space键开始播放 | 继续播放");
        ui.label("按下Backspace键暂停播放");
        ui.label("按下Ctrl键停止播放");

        if get_global_keystate(VKey::Space) {
            PAUSE.store(false, Ordering::Relaxed);
            if !PLAYING.load(Ordering::Relaxed) {
                IS_PLAY.store(true, Ordering::Relaxed);
                self.midi.clone().playback(self.tuned, self.mode);
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
            self.state = "播放中...";
        }
        if !IS_PLAY.load(Ordering::Relaxed) {
            self.state = "已停止";
        }
        if IS_PLAY.load(Ordering::Relaxed) && PAUSE.load(Ordering::Relaxed) {
            self.state = "已暂停";
        }
    }
}