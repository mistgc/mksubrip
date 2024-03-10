use crate::prelude::*;
use crate::ui::Drawable;

pub struct MainWindow {}

impl Drawable for MainWindow {
    fn draw(&mut self, _ctx: &egui::Context, eui: &mut egui::Ui) {
        egui::TopBottomPanel::bottom("b1")
            .resizable(true)
            .min_height(300.0)
            .show_inside(eui, |eui| {
                eui.heading("b1");
            });

        egui::SidePanel::left("l1")
            .resizable(false)
            .min_width(30.0)
            .show_inside(eui, |eui| {
                eui.heading("l1");
            });

        // subrip(caption) list area
        egui::SidePanel::right("r1")
            .resizable(true)
            .min_width(400.0)
            .show_inside(eui, |eui| {
                eui.heading("r1");
            });

        egui::TopBottomPanel::bottom("b2")
            .resizable(true)
            .min_height(50.0)
            .show_inside(eui, |eui| {
                eui.heading("b2");
            });

        // monitor area
        egui::CentralPanel::default().show_inside(eui, |eui| {
            eui.heading("c1");
        });
    }
}
