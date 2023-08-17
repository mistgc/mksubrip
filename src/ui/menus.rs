use std::path::PathBuf;

#[derive(Default)]
pub struct FileMenus {
    pub import_file_path: Option<PathBuf>,
    pub state: FileMenusState,
}

#[derive(Default)]
pub struct FileMenusState {
    pub show_export_srt_file_win: bool,
}
