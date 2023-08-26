use eframe::egui;

use crate::ui::{self, view::View};

#[derive(Default)]
pub struct App {
    ui: ui::Ui,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |eui| {
            self.ui.draw(ctx, eui);
        });
    }
}
