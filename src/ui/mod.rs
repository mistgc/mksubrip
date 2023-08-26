pub mod menus;
mod monitor;
mod state;
pub mod view;

use std::fs;

use monitor::Monitor;

use chrono::prelude::*;
use eframe::egui;
use log::{debug, info};

use crate::{
    subrip::Subrip,
    writer::{SrtWriter, Writer},
};

pub struct Ui {
    monitor: Option<Monitor>,
    state: state::State,
    subrips: Vec<Subrip>,
    text: String,
    export_path: String,
    writer: Option<Box<dyn Writer>>,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            monitor: None,
            state: state::State::default(),
            subrips: Vec::new(),
            text: String::new(),
            export_path: String::new(),
            writer: None,
        }
    }
}

impl view::View for Ui {
    fn draw(&mut self, ctx: &eframe::egui::Context, eui: &mut eframe::egui::Ui) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Enter)) {
            self.state.show_create_new_subrip_win = !self.state.show_create_new_subrip_win;
        }

        // "new subrip" window
        egui::Window::new("create_new_subrip")
            .resizable(true)
            .open(&mut self.state.show_create_new_subrip_win)
            .default_pos([eui.available_width() / 2.0, eui.available_height() / 2.0])
            .show(ctx, |eui| {
                eui.heading("Create New Subrip");
                egui::TextEdit::multiline(&mut self.text)
                    .hint_text("Type something")
                    .show(eui);
                if eui.button("Submit").clicked() && self.text.as_str() != "" {
                    let mut subrip = Subrip::new(&self.text);
                    let current = match self.monitor.as_ref() {
                        Some(monitor) => match monitor.get_current_timestamp() {
                            Some(dt) => {
                                debug!(
                                    "Create
                                           a subrip that the begin time at {}",
                                    dt.time().to_string()
                                );

                                dt
                            }
                            None => DateTime::<Utc>::default(),
                        },
                        None => DateTime::<Utc>::default(),
                    };
                    subrip.set_time(current, chrono::Duration::seconds(10));
                    subrip.index = self.subrips.len() as u32 + 1;
                    self.subrips.push(subrip);
                    self.text.drain(..);
                }
            });

        // "export SRT format file" window
        egui::Window::new("export")
            .resizable(true)
            .open(&mut self.state.show_export_srt_file_win)
            .show(ctx, |eui| {
                eui.label("Export Path:");
                eui.text_edit_singleline(&mut self.export_path);
                if eui.button("Export").clicked() {
                    let file = fs::File::create(self.export_path.clone()).unwrap();
                    self.writer = Some(Box::new(SrtWriter::new(file).unwrap()));
                    self.writer
                        .as_mut()
                        .unwrap()
                        .write_multi(&mut self.subrips)
                        .unwrap();
                }
            });

        egui::SidePanel::left("l1")
            .resizable(false)
            .min_width(30.0)
            .show_inside(eui, |eui| {
                eui.heading("l1");
                match eui.menu_button("📁", Self::nested_menus).inner {
                    Some(file_menus) => {
                        if let Some(path) = file_menus.import_file_path {
                            info!("Import file path: {}", path.to_str().unwrap());
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
                        if file_menus.state.show_export_srt_file_win {
                            self.state.show_export_srt_file_win = true;
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

impl Ui {
    pub fn nested_menus(eui: &mut eframe::egui::Ui) -> menus::FileMenus {
        let mut fm = menus::FileMenus::default();

        if eui.button("OPEN").clicked() {
            if let Some(path_buf) = rfd::FileDialog::new()
                .add_filter("video", &["mp4", "mkv", "gif"])
                .pick_file()
            {
                fm.import_file_path = Some(path_buf);
            }

            eui.close_menu();
        }

        eui.menu_button("EXPORT", |eui| {
            if eui.button("SRT").clicked() {
                fm.state.show_export_srt_file_win = true;
                eui.close_menu();
            }
        });

        fm
    }
}
