pub struct State {
    pub show_create_new_subrip_win: bool,
    pub show_export_srt_file_win: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            show_create_new_subrip_win: false,
            show_export_srt_file_win: false,
        }
    }
}
