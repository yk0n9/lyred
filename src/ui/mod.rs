use std::path::Path;

use eframe::egui::{Context, SelectableLabel, Separator, Ui};
use eframe::{egui, App, Frame};

use crate::maps::MAP;
use crate::midi::{is_playing, CURRENT_MIDI};
use crate::ui::play::Play;
use crate::util::{vk_display, KEY_CODE};
use crate::COUNT;

pub mod play;

pub trait View {
    fn ui(&mut self, ui: &mut Ui);
}

impl App for Play {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
        let tracks_enable = self.tracks_enable;
        egui::Window::new("音轨")
            .scroll([true, true])
            .open(&mut self.tracks_enable)
            .show(ctx, |ui| {
                if tracks_enable {
                    egui::ScrollArea::both()
                        .auto_shrink([true, true])
                        .show(ui, |ui| {
                            for (enable, index, name) in self.midi.track_num.write().iter_mut() {
                                if ui
                                    .checkbox(enable, format!("Track {}: {}", index, name))
                                    .changed()
                                {
                                    self.notify_merge = true;
                                }
                            }
                        });
                }
            });
        let pitch_enable = self.pitch_enable;
        egui::Window::new("音调")
            .scroll([true, true])
            .open(&mut self.pitch_enable)
            .show(ctx, |ui| {
                if pitch_enable {
                    if ui.button("还原音调").clicked() {
                        self.midi.track_keys.write().iter_mut().for_each(|keys| {
                            keys.iter_mut().for_each(|key| {
                                key.key = key.backup;
                                key.real = 0;
                            });
                        });
                        self.notify_merge = true;
                    }
                    egui::ScrollArea::both()
                        .auto_shrink([true, true])
                        .show(ui, |ui| {
                            for (index, keys) in self.midi.track_keys.write().iter_mut().enumerate()
                            {
                                ui.collapsing(format!("Track {index}"), |ui| {
                                    ui.horizontal(|ui| {
                                        for key in keys.iter_mut() {
                                            ui.vertical(|ui| {
                                                ui.label(format!(
                                                    "{}{}",
                                                    if key.key > 0 {
                                                        "#"
                                                    } else if key.key < 0 {
                                                        "b"
                                                    } else {
                                                        ""
                                                    },
                                                    key.key.abs()
                                                ));
                                                if ui.button("升调").clicked() {
                                                    key.real += 1;
                                                    key.key += 1;
                                                    self.notify_merge = true;
                                                }
                                                if ui.button("降调").clicked() {
                                                    key.real -= 1;
                                                    key.key -= 1;
                                                    self.notify_merge = true;
                                                }
                                            });
                                            ui.add(Separator::default().vertical());
                                        }
                                    });
                                });
                            }
                        });
                }
            });
        if self.notify_merge && !is_playing() {
            self.midi
                .merge_tracks(&self.midi.current_range(), self.offset);
            self.notify_merge = false;
        }
        egui::Window::new("按键映射")
            .scroll([true, true])
            .open(&mut self.map_enable)
            .show(ctx, |ui| unsafe {
                for (i, level) in ['+', ' ', '-'].into_iter().enumerate() {
                    ui.separator();
                    for key in 0..7 {
                        let id = i * 7 + key;
                        egui::ComboBox::from_label(format!("{}{}", level, key + 1))
                            .selected_text(format!("{}", vk_display(MAP[id])))
                            .show_ui(ui, |ui| {
                                KEY_CODE
                                    .iter()
                                    .filter(|k| {
                                        self.config.function_key.pause.ne(*k)
                                            && self.config.function_key.play.ne(*k)
                                            && self.config.function_key.stop.ne(*k)
                                    })
                                    .for_each(|key| {
                                        ui.selectable_value(&mut MAP[id], *key, vk_display(*key));
                                    });
                            });
                    }
                }
            });

        egui::Window::new("MIDI列表")
            .scroll([true, true])
            .open(&mut self.dir_enable)
            .show(ctx, |ui| {
                let midis = self.midi.midis.read();
                if midis.is_empty() {
                    return;
                }
                for (index, midi_file) in midis.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let cond = CURRENT_MIDI.load().eq(&index);
                        if ui.button("▶").clicked() {
                            let midi = self.midi.clone();
                            midi.switch_midi(
                                index,
                                Path::new(self.config.midi_dir.0.read().as_str()).join(midi_file),
                            );
                            midi.playback_by(
                                self.config.midi_dir.0.read().as_str(),
                                self.offset,
                                self.play_mode,
                                self.mode,
                            );
                        }
                        let file = ui.add(SelectableLabel::new(cond, midi_file));
                        if file.clicked() {
                            let midi = self.midi.clone();
                            midi.switch_midi(
                                index,
                                Path::new(self.config.midi_dir.0.read().as_str()).join(midi_file),
                            );
                        }
                    });
                }
            });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        drop(COUNT.take());
        unsafe {
            self.config.map = MAP;
        }
        ron::to_string(&self.config)
            .inspect(|config| {
                std::fs::write("config.ron", config).ok();
            })
            .ok();
    }
}
