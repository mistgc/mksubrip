use crate::prelude::*;
use crate::ui::Drawable;

pub struct ControlBar {
    pub sig_btn_play_clicked: Signal<()>,

    player: Option<Shared<media_player::Player>>,
}

impl Default for ControlBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlBar {
    pub fn new() -> Self {
        Self {
            sig_btn_play_clicked: Signal::new(),
            player: None,
        }
    }

    pub fn set_player(&mut self, player: &Shared<media_player::Player>) {
        self.player = Some(player.clone());
    }
}

impl Drawable for ControlBar {
    fn draw(&mut self, _ctx: &eframe::egui::Context, eui: &mut eframe::egui::Ui) {
        use media_player::PlayerState;
        use media_player::Streamer;

        let mut btn_str_icon = "◼";
        let mut label_text = String::from("");

        if let Some(player) = self.player.as_ref() {
            let borrowed_player = player.borrow_mut();
            let video_streamer = borrowed_player.video_streamer.lock();
            let state = borrowed_player.player_state.get();
            btn_str_icon = match state {
                PlayerState::Playing => "⏸",
                PlayerState::Paused => "▶",
                _ => "◼",
            };
            let elapsed_s = video_streamer.elapsed_ms().get() / 1000;
            let duration_s = video_streamer.duration_ms() / 1000;
            let elapsed_str = format!("{:02}:{:02}", elapsed_s / 60, elapsed_s % 60);
            let duration_str = format!("{:02}:{:02}", duration_s / 60, duration_s % 60);
            label_text = format!("{}/{}", elapsed_str, duration_str);
        }

        eui.horizontal(|eui| {
            let btn_play = eui.button(btn_str_icon);

            if btn_play.clicked() {
                self.sig_btn_play_clicked.emit(&());
            }

            eui.label(label_text);
        });
    }
}
