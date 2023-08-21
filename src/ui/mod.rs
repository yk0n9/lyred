use eframe::{App, egui, Frame};
use eframe::egui::{Context, Ui};

use crate::ui::play::Play;

pub mod play;

pub trait View {
    fn ui(&mut self, ui: &mut Ui);
}

pub trait Module: View {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &Context, open: &mut bool);
}

impl App for Play<'_> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default()
            .show(ctx, |ui| self.ui(ui));
        egui::Window::new("音轨").open(&mut self.tracks_enable).show(ctx, |ui| {
            for (enable, index) in self.midi.track_num.lock().unwrap().iter_mut() {
                ui.checkbox(enable, format!("Track {}", index));
            }
        });
    }
}
