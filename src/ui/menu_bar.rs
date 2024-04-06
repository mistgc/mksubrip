use crate::prelude::*;
use crate::ui::Drawable;

use std::path::PathBuf;

pub struct MenuBar {
    pub sig_open_selected: Signal<PathBuf>,
    pub sig_export_srt_selected: Signal<()>,
}

#[derive(Default)]
struct MenuBarOutput {
    path_buf: Option<PathBuf>,
    show_export_win_srt: bool,
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            sig_open_selected: Signal::new(),
            sig_export_srt_selected: Signal::new(),
        }
    }

    fn nested_menus(eui: &mut egui::Ui) -> MenuBarOutput {
        let mut output = MenuBarOutput::default();

        if eui.button("OPEN").clicked() {
            if let Some(path_buf) = rfd::FileDialog::new()
                .add_filter("video", &["mp4", "mkv", "gif"])
                .pick_file()
            {
                output.path_buf = Some(path_buf);
                eui.close_menu();
            }
        }

        eui.menu_button("EXPORT", |eui| {
            if eui.button("SRT").clicked() {
                output.show_export_win_srt = true;
                eui.close_menu();
            }
        });

        output
    }
}

impl Drawable for MenuBar {
    fn draw(&mut self, _ctx: &egui::Context, eui: &mut egui::Ui) {
        if let Some(output) = eui.menu_button("📁", Self::nested_menus).inner {
            if let Some(path_buf) = output.path_buf {
                self.sig_open_selected.emit(&path_buf);
            }

            if output.show_export_win_srt {
                self.sig_export_srt_selected.emit(&());
            }
        }
    }
}
