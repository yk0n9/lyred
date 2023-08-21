use eframe::{App, Frame};
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
        self.show(ctx, &mut true);
    }
}
