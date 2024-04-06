use crate::ui::{self, Drawable};

use eframe::{self, egui};

pub struct App {
    mainwindow: ui::MainWindow,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            mainwindow: ui::MainWindow::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |eui| {
            self.mainwindow.draw(ctx, eui);
        });
    }
}
