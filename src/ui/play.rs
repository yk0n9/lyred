use std::sync::atomic::Ordering;

use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::*;
use eframe::egui::{FontId, Slider, Ui};
use eframe::{egui, CreationContext};
use serde::{Deserialize, Serialize};

use crate::font::load_fonts;
use crate::maps::is_pressed;
use crate::midi::{Midi, State, SPEED, STATE};
use crate::ui::View;
use crate::util::{vk_display, KEY_CODE};
use crate::{COUNT, LOCAL, TIME_SHIFT};

#[derive(Debug, Clone)]
pub struct Play {
    pub midi: Midi,
    pub speed: f32,
    pub mode: Mode,
    pub state: &'static str,
    pub tracks_enable: bool,
    pub offset: i32,
    pub notify_merge: bool,
    pub function_key: FunctionKey,
    pub control_key: ControlKey,
    pub progress: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ControlKey {
    pub add: bool,
    pub sub: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FunctionKey {
    pub play: u16,
    pub pause: u16,
    pub stop: u16,
}

impl Default for FunctionKey {
    fn default() -> Self {
        Self {
            play: 32,
            pause: 8,
            stop: 17,
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
            function_key: FunctionKey::default(),
            control_key: ControlKey::default(),
            progress: 0,
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
                STATE.store(State::Stop);
                self.midi.clone().init();
                self.offset = 0;
            }
            if ui.button("从MIDI转换").clicked() {
                if let Some(name) = self.midi.name.read().as_ref() {
                    self.midi.clone().convert_from_midi(name.to_string());
                }
            }
        });
        if let Some(name) = self.midi.name.read().as_ref() {
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
        SPEED.store(self.speed);
        ui.horizontal(|ui| {
            let sub = is_pressed(189) || is_pressed(109);
            if !sub {
                self.control_key.sub = false;
            }
            if ui.button("减速0.1x").clicked() || sub != self.control_key.sub {
                self.control_key.sub = sub;
                if SPEED.load() > 0.1 {
                    self.speed -= 0.1;
                    SPEED.store(self.speed);
                }
            }
            let add = is_pressed(187) || is_pressed(107);
            if !add {
                self.control_key.add = false;
            }
            if ui.button("加速0.1x").clicked() || add != self.control_key.add {
                self.control_key.add = add;
                self.speed += 0.1;
                SPEED.store(self.speed);
            }
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!(
                "偏移量: {} 命中率: {:.2}%",
                self.offset,
                self.midi.hit_rate.load() * 100.0
            ));
            if ui.button("还原偏移量").clicked() {
                self.offset = 0;
                self.midi.hit_rate.store(self.midi.detect(self.offset));
            }
        });
        if ui.button("向上调音").clicked() {
            self.offset += 1;
            self.midi.hit_rate.store(self.midi.detect(self.offset));
        }
        if ui.button("向下调音").clicked() {
            self.offset -= 1;
            self.midi.hit_rate.store(self.midi.detect(self.offset));
        }
        ui.toggle_value(&mut self.tracks_enable, "音轨列表");
        ui.separator();
        ui.label(self.state);
        if STATE.load() != State::Stop {
            self.progress = LOCAL.load(Ordering::Relaxed);
            let count = unsafe { &*COUNT.as_ptr() };
            if ui
                .add(
                    Slider::new(&mut self.progress, 0..=count.len() - 1)
                        .show_value(false)
                        .text(format!(
                            "{:02}:{:02}/{:02}:{:02}",
                            count[LOCAL.load(Ordering::Relaxed)] / 60000000,
                            count[LOCAL.load(Ordering::Relaxed)] / 1000000 % 60,
                            count[count.len() - 1] / 60000000,
                            count[count.len() - 1] / 1000000 % 60
                        )),
                )
                .drag_stopped()
            {
                TIME_SHIFT.store(true, Ordering::Relaxed);
                LOCAL.store(self.progress, Ordering::Relaxed);
            }
        }
        ui.separator();
        ui.label("按下 - 键减速");
        ui.label("按下 + 键加速");
        ui.horizontal(|ui| {
            ui.label("按下");
            egui::ComboBox::from_id_source(0)
                .selected_text(vk_display(self.function_key.play))
                .show_ui(ui, |ui| {
                    KEY_CODE
                        .iter()
                        .filter(|k| **k != self.function_key.pause && **k != self.function_key.stop)
                        .for_each(|key| {
                            ui.selectable_value(
                                &mut self.function_key.play,
                                *key,
                                vk_display(*key),
                            );
                        })
                });
            ui.label("键开始播放 | 继续播放");
        });
        ui.horizontal(|ui| {
            ui.label("按下");
            egui::ComboBox::from_id_source(1)
                .selected_text(vk_display(self.function_key.pause))
                .show_ui(ui, |ui| {
                    KEY_CODE
                        .iter()
                        .filter(|k| **k != self.function_key.play && **k != self.function_key.stop)
                        .for_each(|key| {
                            ui.selectable_value(
                                &mut self.function_key.pause,
                                *key,
                                vk_display(*key),
                            );
                        })
                });
            ui.label("键暂停播放");
        });
        ui.horizontal(|ui| {
            ui.label("按下");
            egui::ComboBox::from_id_source(2)
                .selected_text(vk_display(self.function_key.stop))
                .show_ui(ui, |ui| {
                    KEY_CODE
                        .iter()
                        .filter(|k| **k != self.function_key.play && **k != self.function_key.pause)
                        .for_each(|key| {
                            ui.selectable_value(
                                &mut self.function_key.stop,
                                *key,
                                vk_display(*key),
                            );
                        })
                });
            ui.label("键停止播放");
        });
        ui.label("");
        ui.label("注意: 每±12个偏移量为一个八度");

        if is_pressed(self.function_key.play) {
            match STATE.load() {
                State::Stop => {
                    if LOCAL.load(Ordering::Relaxed) == !0 {
                        STATE.store(State::Playing);
                        self.midi.clone().playback(self.offset, self.mode);
                    }
                }
                State::Pause => STATE.store(State::Playing),
                _ => {}
            }
        }
        if is_pressed(self.function_key.stop) {
            STATE.store(State::Stop);
        }
        if is_pressed(self.function_key.pause) {
            if let State::Playing = STATE.load() {
                STATE.store(State::Pause);
            }
        }

        self.state = match STATE.load() {
            State::Playing => "播放中...",
            State::Pause => "已暂停",
            _ => "已停止",
        };
    }
}
