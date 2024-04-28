use crate::core::media_player::{AudioDevice, Player};
use crate::prelude::*;
use crate::ui::Drawable;

pub struct Moniter {
    pub sig_media_loaded: Signal<()>,
    pub sig_media_duration_s_changed: Signal<i64>,

    pub(crate) ctx: Option<egui::Context>,
    pub(crate) player: Option<Player>,
    pub(crate) audio_device: Option<AudioDevice>,
    pub(crate) media_path: String,
}

impl Default for Moniter {
    fn default() -> Self {
        Self::new()
    }
}

impl Moniter {
    pub fn new() -> Self {
        Self {
            sig_media_loaded: Signal::new(),
            sig_media_duration_s_changed: Signal::new(),

            ctx: None,
            player: None,
            audio_device: None,
            media_path: String::new(),
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn set_media_path(&mut self, path: &std::path::PathBuf) {
        if let Some(str) = path.to_str() {
            self.media_path = str.to_string();
            if let Some(ctx) = &self.ctx {
                if let Ok(player) = media_player::Player::new(ctx, &self.media_path) {
                    self.sig_media_duration_s_changed
                        .emit(&(player.duration_ms / 1000));
                    if let Ok(audio_device) = media_player::AudioDevice::new() {
                        self.audio_device = Some(audio_device);
                        if let Ok(player_with_audio) =
                            player.with_audio(self.audio_device.as_mut().unwrap())
                        {
                            self.player = Some(player_with_audio);
                        }
                    } else {
                        self.player = Some(player);
                    }
                } else {
                    error!("{} is invalid!", &self.media_path);
                }
            } else {
                error!("The field `ctx` of ui::Moniter is None!");
            }
        }
    }

    pub fn set_ctx(&mut self, ctx: &egui::Context) {
        self.ctx = Some(ctx.clone());
    }

    pub fn seek(&mut self, t: f32) {
        if let Some(player) = &mut self.player {
            player.seek(t);
        } else {
            error!("The field `player` of ui::Moniter is None!");
        }
    }

    pub fn current_timestamp(&self) -> i64 {
        use crate::core::media_player::Streamer;

        if let Some(player) = &self.player {
            player.video_streamer.lock().elapsed_ms().get() / 1000
        } else {
            error!("The field `player` of ui::Moniter is None!");

            0
        }
    }

    pub fn get_media_duration(&self) -> i64 {
        use crate::core::media_player::Streamer;

        if let Some(player) = &self.player {
            player.video_streamer.lock().duration_ms() / 1000
        } else {
            error!("The field `player` of ui::Moniter is None!");

            0
        }
    }
}

impl Drawable for Moniter {
    fn draw(&mut self, _ctx: &egui::Context, eui: &mut egui::Ui) {
        if let Some(player) = &mut self.player {
            player.ui(eui, eui.available_size());
        }
    }
}
