pub mod mainwindow;

pub use mainwindow::MainWindow;

use eframe::egui;

pub trait Drawable {
    fn draw(&mut self, ctx: &egui::Context, eui: &mut egui::Ui);
}
