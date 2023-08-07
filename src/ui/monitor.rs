use eframe::egui;
use crate::ui::view::View;

pub struct Monitor {
    player: Option<egui_video::Player>,
    ctx: egui::Context,
    media_path: Option<String>,
}

impl Monitor {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            player: None,
            ctx: ctx.clone(),
            media_path: None,
        }
    }

    pub fn set_media_path(mut self, path: impl Into<String>) -> Self {
        self.media_path = Some(path.into());
        // if self.player.is_some() {
        //     self.player.take();
        // }
        let player = egui_video::Player::new(&self.ctx, self.media_path.as_ref().unwrap()).unwrap();
        self.player = Some(player);

        self
    }
}

impl View for Monitor {
    fn draw(&mut self,ctx: &eframe::egui::Context, eui: &mut eframe::egui::Ui) {
        let width = self.player.as_ref().unwrap().width;
        let height = self.player.as_ref().unwrap().height;

        self.player.as_mut().unwrap().ui(eui, [eui.available_width(), eui.available_height()]);
    }
}
