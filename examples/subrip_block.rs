use chrono::{NaiveTime, TimeDelta};
use mksubrip::prelude::*;
use mksubrip::ui::{Drawable, SubripBlock};
use mksubrip::Subrip;

fn main() {
    let opt = eframe::NativeOptions::default();
    let subrip = Shared::new(Subrip::new(
        "hello world.",
        NaiveTime::from_hms_opt(0, 0, 10).unwrap(),
        TimeDelta::seconds(180),
    ));
    let _ = eframe::run_simple_native("subrip block", opt, move |ctx, _eframe| {
        let mut block = SubripBlock::new(subrip.clone());
        egui::CentralPanel::default().show(ctx, |ui| {
            block.draw(ctx, ui);
        });
    });
}
