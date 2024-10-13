use std::ops::Deref;
use std::sync::Arc;

use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::*;
use eframe::egui::{FontId, Slider, Ui};
use eframe::{egui, CreationContext};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::font::load_fonts;
use crate::maps::{is_pressed, MAP};
use crate::midi::{Midi, State, PLAYING, SPEED, STATE};
use crate::ui::View;
use crate::util::{vk_display, KEY_CODE};
use crate::{COUNT, LOCAL, POOL, TIME_SHIFT};

#[derive(Debug, Clone)]
pub struct Play {
    pub midi: Midi,
    pub speed: f32,
    pub mode: Mode,
    pub play_mode: PlayMode,
    pub state: &'static str,
    pub tracks_enable: bool,
    pub pitch_enable: bool,
    pub map_enable: bool,
    pub dir_enable: bool,
    pub offset: i32,
    pub notify_merge: bool,
    pub config: Config,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayMode {
    Once,
    OneLoop,
    Loop,
    Random,
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
            play_mode: PlayMode::Once,
            state: "已停止",
            tracks_enable: false,
            pitch_enable: false,
            map_enable: false,
            dir_enable: false,
            offset: 0,
            notify_merge: false,
            config: Config::default(),
            control_key: ControlKey::default(),
            progress: 0,
        }
    }

    fn select_dir(&self) {
        let dir = self.config.midi_dir.0.clone();
        let midi = self.midi.clone();
        POOL.spawn(move || {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                *dir.write() = path.to_string_lossy().to_string();
                midi.get_midis_path(path);
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct MidiDir(pub Arc<RwLock<String>>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub midi_dir: MidiDir,
    pub function_key: FunctionKey,
    pub map: [u16; 21],
}

impl Serialize for MidiDir {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.read().deref())
    }
}

impl<'de> Deserialize<'de> for MidiDir {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = String::deserialize(deserializer)?;
        Ok(MidiDir(Arc::new(RwLock::new(path))))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            midi_dir: MidiDir(Arc::new(RwLock::new(String::new()))),
            function_key: Default::default(),
            map: unsafe { MAP },
        }
    }
}

impl View for Play {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| ui.heading("Lyred"));
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("选择MIDI文件").clicked() {
                STATE.store(State::Stop);
                self.offset = 0;
                self.midi.clone().init();
            }
            if ui.button("选择MIDI目录").clicked() {
                STATE.store(State::Stop);
                self.offset = 0;
                self.select_dir();
            }
            ui.toggle_value(&mut self.dir_enable, "MIDI列表");
            if ui.button("从MIDI转换").clicked() {
                if let Some(name) = self.midi.name.read().as_ref() {
                    self.midi.clone().convert_from_midi(name.to_string());
                }
            }
        });
        if let Some(name) = self.midi.name.read().as_ref() {
            ui.label(&format!("当前文件: {}", name));
        }
        let path = self.config.midi_dir.0.read();
        if !path.is_empty() {
            ui.label(format!("当前目录: {}", path.as_str()));
        }
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("选择模式:");
            ui.radio_value(&mut self.mode, Mode::GenShin, "GenShin");
            ui.radio_value(&mut self.mode, Mode::VRChat, "VRChat-中文吧");
        });
        ui.horizontal(|ui| {
            ui.label("播放模式:");
            ui.radio_value(&mut self.play_mode, PlayMode::Once, "单次");
            ui.radio_value(&mut self.play_mode, PlayMode::OneLoop, "单曲循环");
            ui.radio_value(&mut self.play_mode, PlayMode::Loop, "列表循环");
            ui.radio_value(&mut self.play_mode, PlayMode::Random, "列表随机");
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
        ui.horizontal(|ui| {
            ui.toggle_value(&mut self.tracks_enable, "音轨列表");
            ui.toggle_value(&mut self.pitch_enable, "音调列表");
            ui.toggle_value(&mut self.map_enable, "按键映射");
        });
        ui.separator();

        ui.label(self.state);
        if STATE.load() != State::Stop {
            self.progress = LOCAL.load();
            let count = unsafe { &*COUNT.as_ptr() };
            let current = self.progress;
            let len = count.len().saturating_sub(1);
            if len > 0 {
                if ui
                    .add(
                        Slider::new(&mut self.progress, 0..=len)
                            .show_value(false)
                            .text(format!(
                                "{:02}:{:02}/{:02}:{:02}",
                                count[current] / 60000000,
                                count[current] / 1000000 % 60,
                                count[len] / 60000000,
                                count[len] / 1000000 % 60
                            )),
                    )
                    .drag_stopped()
                {
                    TIME_SHIFT.store(true);
                    LOCAL.store(self.progress);
                }
            }
        }
        ui.separator();
        ui.label("按下 - 键减速");
        ui.label("按下 + 键加速");
        ui.horizontal(|ui| {
            ui.label("按下");
            egui::ComboBox::from_id_salt("Play")
                .selected_text(vk_display(self.config.function_key.play))
                .show_ui(ui, |ui| {
                    KEY_CODE
                        .iter()
                        .filter(|k| unsafe {
                            self.config.function_key.pause.ne(*k)
                                && self.config.function_key.stop.ne(*k)
                                && !MAP.contains(*k)
                        })
                        .for_each(|key| {
                            ui.selectable_value(
                                &mut self.config.function_key.play,
                                *key,
                                vk_display(*key),
                            );
                        })
                });
            ui.label("键开始播放 | 继续播放");
        });
        ui.horizontal(|ui| {
            ui.label("按下");
            egui::ComboBox::from_id_salt("Pause")
                .selected_text(vk_display(self.config.function_key.pause))
                .show_ui(ui, |ui| {
                    KEY_CODE
                        .iter()
                        .filter(|k| unsafe {
                            self.config.function_key.play.ne(*k)
                                && self.config.function_key.stop.ne(*k)
                                && !MAP.contains(*k)
                        })
                        .for_each(|key| {
                            ui.selectable_value(
                                &mut self.config.function_key.pause,
                                *key,
                                vk_display(*key),
                            );
                        })
                });
            ui.label("键暂停播放");
        });
        ui.horizontal(|ui| {
            ui.label("按下");
            egui::ComboBox::from_id_salt("Stop")
                .selected_text(vk_display(self.config.function_key.stop))
                .show_ui(ui, |ui| {
                    KEY_CODE
                        .iter()
                        .filter(|k| unsafe {
                            self.config.function_key.play.ne(*k)
                                && self.config.function_key.pause.ne(*k)
                                && !MAP.contains(*k)
                        })
                        .for_each(|key| {
                            ui.selectable_value(
                                &mut self.config.function_key.stop,
                                *key,
                                vk_display(*key),
                            );
                        })
                });
            ui.label("键停止播放");
        });
        ui.label("");
        ui.label("注意: 每±12个偏移量为一个八度");

        if is_pressed(self.config.function_key.play) {
            match STATE.load() {
                State::Stop => {
                    if !PLAYING.load() {
                        let midi = self.midi.clone();
                        midi.playback_by(
                            self.config.midi_dir.0.read().as_str(),
                            self.offset,
                            self.play_mode,
                            self.mode,
                        );
                    }
                }
                State::Pause => STATE.store(State::Playing),
                _ => {}
            }
        }
        if is_pressed(self.config.function_key.stop) {
            STATE.store(State::Stop);
        }
        if is_pressed(self.config.function_key.pause) {
            if let State::Playing = STATE.load() {
                STATE.store(State::Pause);
            }
        }

        self.state = match STATE.load() {
            State::Playing => "播放中...",
            State::Pause => "已暂停",
            State::Stop => "已停止",
        };
    }
}
