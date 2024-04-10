use crate::{
    ui::{self, Drawable},
    Subrip,
};

use eframe::{self, egui};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct AppState {
    pub subrips: Vec<Rc<RefCell<Subrip>>>,
}

pub struct App {
    state: Rc<RefCell<AppState>>,
    mainwindow: ui::MainWindow,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let app_state = Rc::new(RefCell::new(AppState::default()));
        Self {
            state: app_state.clone(),
            mainwindow: ui::MainWindow::new(app_state.clone()),
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
