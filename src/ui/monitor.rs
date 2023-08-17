use crate::ui::view::View;
use chrono::prelude::*;
use eframe::egui;

pub struct Monitor {
    player: Option<egui_video::Player>,
    audio_device: Option<egui_video::AudioDevice>,
    ctx: egui::Context,
    media_path: Option<String>,
}

impl Monitor {
    pub fn new(ctx: &egui::Context) -> Self {
        let mut audio_device = None;
        if let Ok(sdl) = sdl2::init() {
            if let Ok(audio) = sdl.audio() {
                if let Ok(device) = egui_video::init_audio_device(&audio) {
                    audio_device = Some(device);
                }
            }
        }

        Self {
            player: None,
            ctx: ctx.clone(),
            media_path: None,
            audio_device,
        }
    }

    pub fn set_media_path(&mut self, path: impl Into<String>) {
        self.media_path = Some(path.into());
        let mut player =
            egui_video::Player::new(&self.ctx, self.media_path.as_ref().unwrap()).unwrap();

        if let Some(audio_device) = &mut self.audio_device {
            player = player.with_audio(audio_device).unwrap();
        }

        self.player = Some(player);
    }

    /// Get the current timestamp of the media
    pub fn get_current_timestamp(&self) -> Option<DateTime<Utc>> {
        use egui_video::Streamer;
        let player = self.player.as_ref()?;
        let elapsed_ms = player.video_streamer.lock().elapsed_ms().get();

        Some(Utc.timestamp_millis_opt(elapsed_ms).unwrap())
    }
}

impl View for Monitor {
    fn draw(&mut self, ctx: &eframe::egui::Context, eui: &mut eframe::egui::Ui) {
        let width = self.player.as_ref().unwrap().width;
        let height = self.player.as_ref().unwrap().height;

        self.player
            .as_mut()
            .unwrap()
            .ui(eui, [eui.available_width(), eui.available_height()]);
    }
}
