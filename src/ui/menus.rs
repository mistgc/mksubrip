use std::path::PathBuf;

#[derive(Default)]
pub struct FileMenus {
    pub file_path: Option<PathBuf>,
}

impl FileMenus {
    pub fn nested_menus(eui: &mut eframe::egui::Ui) -> Self {
        let mut fm = FileMenus::default();

        if eui.button("OPEN").clicked() {
            if let Some(path_buf) = rfd::FileDialog::new()
                .add_filter("video", &["mp4", "mkv", "gif"])
                .pick_file()
            {
                fm.file_path = Some(path_buf);
            }

            eui.close_menu();
        }

        fm
    }
}
