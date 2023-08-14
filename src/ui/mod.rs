pub mod menus;
mod monitor;
mod state;
pub mod view;

use monitor::Monitor;

use eframe::egui;

use crate::subrip::Subrip;

pub struct Ui {
    monitor: Option<Monitor>,
    state: state::State,
    subrips: Vec<Subrip>,
    text: String,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            monitor: None,
            state: state::State::default(),
            subrips: Vec::new(),
            text: String::new(),
        }
    }
}

impl view::View for Ui {
    fn draw(&mut self, ctx: &eframe::egui::Context, eui: &mut eframe::egui::Ui) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Enter)) {
            self.state.show_create_new_subrip_win = !self.state.show_create_new_subrip_win;
        }

        if self.state.show_create_new_subrip_win {
            egui::Window::new("create_new_subrip")
                .resizable(true)
                .default_pos([eui.available_width() / 2.0, eui.available_height() / 2.0])
                .show(ctx, |eui| {
                    eui.heading("Create New Subrip");
                    egui::TextEdit::multiline(&mut self.text).hint_text("Type something").show(eui);
                    if eui.button("Submit").clicked() && self.text.as_str() != "" {
                        let mut subrip = Subrip::new(&self.text);
                        subrip.set_time(chrono::Utc::now(), chrono::Duration::seconds(10));
                        self.subrips.push(subrip);
                        self.text.drain(..);
                    }
                });
        }

        egui::SidePanel::left("l1")
            .resizable(false)
            .min_width(30.0)
            .show_inside(eui, |eui| {
                eui.heading("l1");
                match eui.menu_button("📁", menus::FileMenus::nested_menus).inner {
                    Some(file_menus) => {
                        if let Some(path) = file_menus.file_path {
                            match self.monitor.as_mut() {
                                Some(monitor) => {
                                    monitor.set_media_path(path.to_string_lossy().to_string());
                                }
                                None => {
                                    let mut monitor = Monitor::new(ctx);
                                    monitor.set_media_path(path.to_string_lossy().to_string());
                                    self.monitor = Some(monitor);
                                }
                            }
                        }
                    }
                    None => {}
                }
            });
        egui::TopBottomPanel::bottom("b1")
            .resizable(true)
            .min_height(300.0)
            .show_inside(eui, |eui| {
                eui.heading("b1");
            });

        // subrip(caption) list area
        egui::SidePanel::right("r1")
            .resizable(true)
            .min_width(400.0)
            .show_inside(eui, |eui| {
                eui.heading("r1");
                self.subrips.iter_mut().for_each(|i| {
                    i.draw(ctx, eui);
                });
            });

        egui::TopBottomPanel::bottom("b2")
            .resizable(true)
            .min_height(50.0)
            .show_inside(eui, |eui| {
                eui.heading("b2");
            });

        // monitor area
        egui::CentralPanel::default().show_inside(eui, |eui| {
            if self.monitor.is_some() {
                self.monitor.as_mut().unwrap().draw(ctx, eui);
            }
        });
    }
}
