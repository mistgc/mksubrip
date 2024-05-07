use crate::{
    prelude::*,
    ui::{self, Drawable},
    Subrip,
};

use eframe::{self, egui};

#[derive(Default)]
pub struct AppState {
    pub subrips: Vec<Shared<Subrip>>,
    pub screen_width: f32,
    pub screen_height: f32,
}

pub struct App {
    state: Shared<AppState>,
    mainwindow: ui::MainWindow,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let app_state = Shared::new(AppState {
            screen_width: 1024.0,
            screen_height: 720.0,
            ..Default::default()
        });
        Self {
            state: app_state.clone(),
            mainwindow: ui::MainWindow::new(app_state.clone()),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |eui| {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE);
            self.mainwindow.draw(ctx, eui);
        });
    }
}
