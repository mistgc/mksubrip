use crate::ui::Drawable;

pub struct ControlBar {}

impl Drawable for ControlBar {
    fn draw(&mut self, _ctx: &eframe::egui::Context, _eui: &mut eframe::egui::Ui) {}
}
