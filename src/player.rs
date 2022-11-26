use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use eframe::egui::{Context, FontId, Vec2};
use eframe::{egui, Frame, NativeOptions};
use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::{Body, Heading, Small};
use eframe::Theme::Light;
use enigo::{Enigo, Key, KeyboardControllable};
use rs_midi_player::midi::{c, init, KeyEvent, tune};
use egui::TextStyle::*;
use egui_file::FileDialog;
use windows_hotkeys::get_global_keystate;
use windows_hotkeys::keys::VKey;

fn main() {
    let options = NativeOptions {
        default_theme: Light,
        resizable: false,
        initial_window_size: Some(Vec2::new(800., 600.)),
        ..NativeOptions::default()
    };
    eframe::run_native("Player", options, Box::new(|_| Box::new(Player::default())));
}

pub struct Player {
    pub speed: f64,
    pub tuned: bool,
    pub pause: bool,
    pub kill: bool,
    pub opened_file: Option<PathBuf>,
    pub open_file_dialog: Option<FileDialog>,
    pub events: Vec<KeyEvent>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 1.0,
            tuned: false,
            pause: false,
            kill: false,
            opened_file: None,
            open_file_dialog: None,
            events: vec![],
        }
    }
}

impl Player {
    pub fn playback(&mut self, message: Vec<KeyEvent>) {
        let mut click = Enigo::new();
        let mut shift = 0;

        if self.tuned {
            shift = tune(message.clone());
        }

        let start_time = Local::now().timestamp_millis();
        let mut input_time = 0.;
        for msg in message.into_iter() {
            if get_global_keystate(VKey::Shift) {
                break;
            }
            if get_global_keystate(VKey::Up) {
                self.speed += 0.1;
            }
            if get_global_keystate(VKey::Down) {
                self.speed -= 0.1;
            }

            input_time += msg.delay / self.speed;

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
    }
}

impl eframe::App for Player {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint();
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Midi-Player by Ykong1337");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Select your MIDI File");
                if (ui.button("Open")).clicked() {
                    let mut dialog = FileDialog::open_file(self.opened_file.clone());
                    dialog.open();
                    self.open_file_dialog = Some(dialog);
                }

                if let Some(dialog) = &mut self.open_file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            self.opened_file = Some(file);
                        }
                    }
                }
            });
            if let Some(path) = &self.opened_file {
                ui.label(&format!("You select {}", path.to_str().unwrap()));
                self.events = init(path.to_str().unwrap()).unwrap();
                self.kill = false;
            }
            ui.separator();
            ui.label(&format!("Your play speed = {}x", self.speed));
            if ui.button("- 0.1x").clicked() {
                self.speed -= 0.1;
            }
            if ui.button("+ 0.1x").clicked() {
                self.speed += 0.1;
            }
            ui.checkbox(&mut self.tuned, "Whether to tune");
            ui.separator();

            if ui.button("Start Playback").clicked() || get_global_keystate(VKey::Space) {
                self.playback(self.events.clone());
            }
        });
    }
}