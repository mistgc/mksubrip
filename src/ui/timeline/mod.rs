pub mod subrip_block;

use crate::app::AppState;
use crate::core::media_player::{self, Player};
use crate::prelude::*;
use crate::ui::Drawable;
use crate::ui::SubripBlock;
use crate::Subrip;

#[derive(Default)]
pub struct Timeline {
    pub sig_video_seeked: Signal<f32>,

    app_state: Shared<AppState>,

    media_duration_s: i64,
    default_height: f32,
    /// Refer to that one pixel means several seconds
    granularity: Shared<f32>,
    stroke: egui::Stroke,
    subrip_blocks: Vec<SubripBlock>,
    player: Option<Shared<Player>>,
    /// The range from begin timestamp to end timestamp in SECONDS
    /// example:
    /// ```rust
    /// // Means from 1000s to 10000s.
    /// duration_range = [1000, 10000];
    /// ```
    duration_range: [i64; 2],
}

impl Timeline {
    pub fn new(app_state: Shared<AppState>) -> Self {
        Self {
            app_state,
            default_height: 120.0,
            media_duration_s: 0,
            granularity: Shared::new(0.1),
            stroke: egui::Stroke::new(2.0, egui::Color32::from_hex("#555555").unwrap()),
            ..Self::default()
        }
    }

    fn set_granularity(&mut self, granularity: f32) {
        *self.granularity.borrow_mut() = granularity;
    }

    /// Calculate how may pixels one-second equals to.
    fn calc_sec_pixels(&self) -> f32 {
        let gran = self.get_granularity();

        1.0 / gran
    }

    /// Calculate the interval between two ticks.
    /// The minimum of the interval equals 8.0 pixels.
    fn calc_tick_step(&self) -> f32 {
        let sec_pixs = self.calc_sec_pixels();

        if sec_pixs < 8.0 {
            8.0
        } else if sec_pixs < 16.0 {
            16.0
        } else if sec_pixs < 32.0 {
            32.0
        } else {
            64.0
        }
    }

    fn draw_cursor(
        &mut self,
        _ctx: &egui::Context,
        painter: &egui::Painter,
        resp: &egui::Response,
    ) {
        use media_player::Streamer;
        if let Some(player) = self.player.as_ref() {
            let borrowed_player = player.borrow_mut();
            let video_streamer = borrowed_player.video_streamer.lock();
            let elapsed_s = video_streamer.elapsed_ms().get() / 1000;
            let duration_s = video_streamer.duration_ms() / 1000;
            let ratio = elapsed_s as f32 / duration_s as f32;
            let tick_step = self.calc_tick_step();
            let offset_ticks = (resp.rect.width() / tick_step * ratio).floor();
            let x = resp.rect.min.x + offset_ticks * tick_step;

            let p0 = Pos2 {
                x,
                y: resp.rect.min.y,
            };
            let p1 = Pos2 {
                x,
                y: resp.rect.max.y,
            };

            painter.line_segment([p0, p1], self.stroke);
        }
    }

    fn draw_hovered_cursor(
        &mut self,
        ctx: &egui::Context,
        painter: &egui::Painter,
        resp: &egui::Response,
    ) {
        if let Some(pointer_pos) = ctx.pointer_hover_pos() {
            if pointer_pos.x > resp.rect.min.x
                && pointer_pos.x < resp.rect.max.x
                && pointer_pos.y > resp.rect.min.y
                && pointer_pos.y < resp.rect.max.y
            {
                let p0 = Pos2 {
                    x: pointer_pos.x,
                    y: resp.rect.min.y,
                };
                let p1 = Pos2 {
                    x: pointer_pos.x,
                    y: resp.rect.max.y,
                };
                let p2 = Pos2 {
                    x: pointer_pos.x - 3.0,
                    y: resp.rect.min.y,
                };
                let p3 = Pos2 {
                    x: pointer_pos.x + 3.0,
                    y: resp.rect.min.y,
                };
                let p4 = Pos2 {
                    x: pointer_pos.x - 3.0,
                    y: resp.rect.max.y,
                };
                let p5 = Pos2 {
                    x: pointer_pos.x + 3.0,
                    y: resp.rect.max.y,
                };

                painter.line_segment([p0, p1], self.stroke);
                painter.line_segment([p2, p3], self.stroke);
                painter.line_segment([p4, p5], self.stroke);

                if resp.double_clicked() {
                    // TODO: the granularity will affect this functionality

                    let t = (pointer_pos.x - resp.rect.min.x) / resp.rect.width();
                    self.sig_video_seeked.emit(&t);
                    info!("Seek to {}", t);
                }
            }
        }
    }

    /// Draw ticks on [`Timeline`].
    fn draw_ticks(&mut self, _ctx: &egui::Context, painter: &egui::Painter, resp: &egui::Response) {
        let tick_step = self.calc_tick_step();
        let rect = resp.rect;
        let count = (rect.width() / tick_step).round() as u32;

        for i in 0..count {
            if i % 5 == 0 {
                let p0 = Pos2 {
                    x: rect.min.x + (i as f32 * tick_step).floor(),
                    y: rect.min.y,
                };
                let p1 = Pos2 {
                    x: rect.min.x + (i as f32 * tick_step).floor(),
                    y: rect.min.y + 20.0,
                };

                painter.line_segment([p0, p1], self.stroke);
            } else {
                let p0 = Pos2 {
                    x: rect.min.x + (i as f32 * tick_step).floor(),
                    y: rect.min.y,
                };
                let p1 = Pos2 {
                    x: rect.min.x + (i as f32 * tick_step).floor(),
                    y: rect.min.y + 12.0,
                };

                painter.line_segment([p0, p1], self.stroke);
            }
        }
    }

    /// Poll and handle input events.
    fn update_input_event(&mut self, ctx: &egui::Context, _resp: &egui::Response) {
        if ctx.input(|i| i.key_released(egui::Key::A)) {
            self.decrease_granularity();
        } else if ctx.input(|i| i.key_released(egui::Key::B)) {
            self.increase_granularity();
        }
    }

    /// Update [`Timeline::duration_range`] when the screen(or window)'s width be changed.
    fn update_duration_range(&mut self, width: f32) {
        let gran = self.get_granularity();
        let begin_timestamp = self.duration_range[0];
        let end_timestamp = begin_timestamp + (gran * width) as i64;
        self.duration_range[1] = end_timestamp;
    }

    /// Initialize [`Timeline`]
    fn init(&mut self) {
        // Granularity
        let width = self.app_state.borrow().screen_width;
        // The minimum of the granularity is 1.0
        let mut gran = self.media_duration_s as f32 / width;
        gran = utils::clamp(3.0, 0.01, gran);
        *self.granularity.borrow_mut() = gran;

        debug!("Granularity = {}", gran);

        // Duration Range
        self.duration_range = [0, (gran * width) as i64];
    }

    fn increase_granularity(&mut self) {
        let mut gran = *self.granularity.borrow() + 1.0;
        gran = gran.min(3.0);
        *self.granularity.borrow_mut() = gran;
    }

    fn decrease_granularity(&mut self) {
        let mut gran = *self.granularity.borrow() - 1.0;
        gran = gran.max(0.01);
        *self.granularity.borrow_mut() = gran;
    }

    pub fn get_granularity(&self) -> f32 {
        *self.granularity.borrow()
    }

    pub fn add_block_from_subrip(&mut self, subrip: &Shared<Subrip>) {
        let mut block = SubripBlock::new(subrip.clone());
        block.set_granularity(self.granularity.clone());

        self.subrip_blocks.push(block);
    }

    pub fn set_media_duration_s(&mut self, duration_s: &i64) {
        info!("ui::TimeLine::media_duration_s = {}", duration_s);

        self.media_duration_s = *duration_s;
    }

    pub fn set_player(&mut self, player: &Shared<Player>) {
        self.player = Some(player.clone());
        self.init();
    }
}

impl Drawable for Timeline {
    fn draw(&mut self, ctx: &egui::Context, eui: &mut egui::Ui) {
        let width = ctx.available_rect().width();
        let (resp, painter) = eui.allocate_painter(
            Vec2::new(width, self.default_height),
            egui::Sense::click_and_drag(),
        );

        self.update_input_event(ctx, &resp);
        self.update_duration_range(width);
        self.draw_cursor(ctx, &painter, &resp);
        self.draw_hovered_cursor(ctx, &painter, &resp);
        self.draw_ticks(ctx, &painter, &resp);

        for i in self.subrip_blocks.iter_mut() {
            i.draw(ctx, eui);
        }
    }
}
