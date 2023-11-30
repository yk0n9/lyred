use std::sync::atomic::Ordering;

use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::*;
use eframe::egui::{FontId, Slider, Ui};
use eframe::{egui, CreationContext};
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_BACK, VK_CONTROL, VK_SPACE};

use crate::font::load_fonts;
use crate::maps::is_pressed;
use crate::midi::{Midi, IS_PLAY, PAUSE, PLAYING, SPEED};
use crate::ui::View;
use crate::util::{vk_display, KEY_CODE};

#[derive(Debug, Clone)]
pub struct Play {
    pub midi: Midi,
    pub speed: f64,
    pub mode: Mode,
    pub state: &'static str,
    pub tracks_enable: bool,
    pub offset: i32,
    pub notify_merge: bool,
    pub function_keys: FunctionKeys,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FunctionKeys {
    pub play: VIRTUAL_KEY,
    pub pause: VIRTUAL_KEY,
    pub stop: VIRTUAL_KEY,
}

impl Default for FunctionKeys {
    fn default() -> Self {
        Self {
            play: VK_SPACE,
            pause: VK_BACK,
            stop: VK_CONTROL,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    GenShin,
    VRChat,
}

impl Play {
    pub fn new(cc: &CreationContext) -> Self {
        load_fonts(&cc.egui_ctx);
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(20.0, Proportional)),
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
            speed: 1.0,
            mode: Mode::GenShin,
            state: "已停止",
            tracks_enable: false,
            offset: 0,
            notify_merge: false,
            function_keys: FunctionKeys::default(),
        }
    }
}

impl View for Play {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| ui.heading("Lyred"));
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("选择MIDI文件");
            if ui.button("打开").clicked() {
                IS_PLAY.store(false, Ordering::Relaxed);
                PAUSE.store(false, Ordering::Relaxed);
                self.midi.clone().init();
                self.offset = 0;
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
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!(
                "偏移量: {} 命中率: {:.2}%",
                self.offset,
                self.midi.hit_rate.load(Ordering::Relaxed) * 100.0
            ));
            if ui.button("还原偏移量").clicked() {
                self.offset = 0;
                self.midi
                    .hit_rate
                    .store(self.midi.detect(self.offset), Ordering::Relaxed);
            }
        });
        if ui.button("向上调音").clicked() {
            self.offset += 1;
            self.midi
                .hit_rate
                .store(self.midi.detect(self.offset), Ordering::Relaxed);
        }
        if ui.button("向下调音").clicked() {
            self.offset -= 1;
            self.midi
                .hit_rate
                .store(self.midi.detect(self.offset), Ordering::Relaxed);
        }
        ui.toggle_value(&mut self.tracks_enable, "音轨列表");
        ui.separator();
        ui.label(self.state);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("按下");
            egui::ComboBox::from_id_source(0)
                .selected_text(vk_display(self.function_keys.play))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    KEY_CODE
                        .iter()
                        .filter(|k| {
                            **k != self.function_keys.pause.0 as u8
                                && **k != self.function_keys.stop.0 as u8
                        })
                        .for_each(|key| {
                            let vk = VIRTUAL_KEY(*key as u16);
                            ui.selectable_value(&mut self.function_keys.play, vk, vk_display(vk));
                        });
                });
            ui.label("键开始播放 | 继续播放");
        });
        ui.horizontal(|ui| {
            ui.label("按下");
            egui::ComboBox::from_id_source(1)
                .selected_text(vk_display(self.function_keys.pause))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    KEY_CODE
                        .iter()
                        .filter(|k| {
                            **k != self.function_keys.play.0 as u8
                                && **k != self.function_keys.stop.0 as u8
                        })
                        .for_each(|key| {
                            let vk = VIRTUAL_KEY(*key as u16);
                            ui.selectable_value(&mut self.function_keys.pause, vk, vk_display(vk));
                        });
                });
            ui.label("键暂停播放");
        });
        ui.horizontal(|ui| {
            ui.label("按下");
            egui::ComboBox::from_id_source(2)
                .selected_text(vk_display(self.function_keys.stop))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    KEY_CODE
                        .iter()
                        .filter(|k| {
                            **k != self.function_keys.play.0 as u8
                                && **k != self.function_keys.pause.0 as u8
                        })
                        .for_each(|key| {
                            let vk = VIRTUAL_KEY(*key as u16);
                            ui.selectable_value(&mut self.function_keys.stop, vk, vk_display(vk));
                        });
                });
            ui.label("键停止播放");
        });
        ui.label("");
        ui.label("注意: 每±12个偏移量为一个八度");

        if is_pressed(self.function_keys.play) {
            PAUSE.store(false, Ordering::Relaxed);
            if !PLAYING.load(Ordering::Relaxed) {
                IS_PLAY.store(true, Ordering::Relaxed);
                self.midi.clone().playback(self.offset, self.mode);
            }
        }
        if is_pressed(self.function_keys.stop) {
            PAUSE.store(false, Ordering::Relaxed);
            IS_PLAY.store(false, Ordering::Relaxed);
        }
        if is_pressed(self.function_keys.pause) {
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
