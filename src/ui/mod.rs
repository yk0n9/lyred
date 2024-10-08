use eframe::egui::{Context, Separator, Ui};
use eframe::{egui, App, Frame};

use crate::midi::is_playing;
use crate::ui::play::Play;
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
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        drop(COUNT.take());
        ron::to_string(&self.function_key)
            .inspect(|config| {
                std::fs::write("config.ron", config).ok();
            })
            .ok();
    }
}
