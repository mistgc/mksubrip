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
    state: Shared<TimelineState>,

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

#[derive(Default)]
struct TimelineState {
    pub width: f32,
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

    fn calc_ticks_range(&self) -> (u32, u32) {
        let sec_pixs = self.calc_sec_pixels();
        let tick_step = self.calc_tick_step();
        let duration_range = &self.duration_range;
        let interval = duration_range[1] - duration_range[0];
        let begin_tick = (duration_range[0] as f32 * sec_pixs / tick_step).round() as u32;
        let end_tick = begin_tick + (interval as f32 * sec_pixs / tick_step).round() as u32;

        (begin_tick, end_tick)
    }

    fn draw_cursor(
        &mut self,
        _ctx: &egui::Context,
        painter: &egui::Painter,
        resp: &egui::Response,
    ) {
        use media_player::Streamer;
        if let Some(player) = self.player.as_ref() {
            let borrowed_player = player.borrow();
            let video_streamer = borrowed_player.video_streamer.lock();
            let elapsed_s = video_streamer.elapsed_ms().get() / 1000;

            if utils::range_contains_timestamp(&self.duration_range, elapsed_s) {
                let tick_step = self.calc_tick_step();
                let sec_pixs = self.calc_sec_pixels();
                let cur_tick = (elapsed_s as f32 * sec_pixs / tick_step).round() as u32;
                let (begin_tick, _) = self.calc_ticks_range();
                let offset_tick = cur_tick - begin_tick;
                let offset_x = resp.rect.min.x + offset_tick as f32 * tick_step;
                let p0 = Pos2 {
                    x: offset_x,
                    y: resp.rect.min.y,
                };
                let p1 = Pos2 {
                    x: offset_x,
                    y: resp.rect.max.y,
                };

                painter.line_segment([p0, p1], self.stroke);
            }
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
        let sec_pixs = self.calc_sec_pixels();
        let (begin_tick, end_tick) = self.calc_ticks_range();

        for i in begin_tick..end_tick + 1 {
            if i % 5 == 0 {
                let p0 = Pos2 {
                    x: resp.rect.min.x + (i as f32 * tick_step).floor(),
                    y: resp.rect.min.y + 20.0,
                };
                let p1 = Pos2 {
                    x: resp.rect.min.x + (i as f32 * tick_step).floor(),
                    y: resp.rect.min.y + 40.0,
                };

                let total_seconds = (i as f32 * tick_step / sec_pixs) as i64;
                let mins = total_seconds / 60;
                let secs = total_seconds % 60;

                debug!("total_seconds = {total_seconds}, mins = {mins}, secs = {secs}");
                let galley = painter.layout_no_wrap(
                    format! {"{:02}:{:02}", mins, secs},
                    egui::FontId::default(),
                    egui::Color32::from_hex("#777777").unwrap(),
                );

                painter.galley(
                    Pos2 {
                        x: resp.rect.min.x + (i as f32 * tick_step).floor(),
                        y: resp.rect.min.y,
                    },
                    galley,
                    egui::Color32::from_hex("#777777").unwrap(),
                );

                painter.line_segment([p0, p1], self.stroke);
            } else if i == end_tick {
                let p0 = Pos2 {
                    x: resp.rect.min.x + (i as f32 * tick_step).floor(),
                    y: resp.rect.min.y + 20.0,
                };
                let p1 = Pos2 {
                    x: resp.rect.min.x + (i as f32 * tick_step).floor(),
                    y: resp.rect.min.y + 40.0,
                };

                let total_seconds = (i as f32 * tick_step / sec_pixs) as i64;
                let mins = total_seconds / 60;
                let secs = total_seconds % 60;

                debug!("total_seconds = {total_seconds}, mins = {mins}, secs = {secs}");
                let galley = painter.layout_no_wrap(
                    format! {"{:02}:{:02}", mins, secs},
                    egui::FontId::default(),
                    egui::Color32::from_hex("#777777").unwrap(),
                );

                painter.galley(
                    Pos2 {
                        x: resp.rect.min.x + (i as f32 * tick_step).floor(),
                        y: resp.rect.min.y,
                    },
                    galley,
                    egui::Color32::from_hex("#777777").unwrap(),
                );

                painter.line_segment([p0, p1], self.stroke);
            } else {
                let p0 = Pos2 {
                    x: resp.rect.min.x + (i as f32 * tick_step).floor(),
                    y: resp.rect.min.y + 20.0,
                };
                let p1 = Pos2 {
                    x: resp.rect.min.x + (i as f32 * tick_step).floor(),
                    y: resp.rect.min.y + 32.0,
                };

                painter.line_segment([p0, p1], self.stroke);
            }
        }
    }

    /// Poll and handle input events.
    fn update_input_event(&mut self, ctx: &egui::Context, resp: &egui::Response) {
        if ctx.rect_contains_pointer(resp.layer_id, resp.rect) {
            ctx.input(|i| {
                if i.smooth_scroll_delta[0] < 0.0 {
                    self.decrease_granularity();
                } else if i.smooth_scroll_delta[0] > 0.0 {
                    self.increase_granularity();
                }
            });
            ctx.input(|i| {
                self.move_duration_range(i.smooth_scroll_delta[1]);
                false
            });
        }
    }

    /// Update [`Timeline::duration_range`] when the screen(or window)'s width be changed.
    fn update_duration_range(&mut self, width: f32) {
        if self.state.borrow_mut().is_width_changed(width) {
            let gran = self.get_granularity();
            let begin_timestamp = self.duration_range[0];
            let end_timestamp = begin_timestamp + (gran * width) as i64;
            self.duration_range[1] = end_timestamp;
        }
    }

    /// Initialize [`Timeline`]
    fn init(&mut self) {
        // Granularity
        let width = self.app_state.borrow().screen_width;
        // The minimum of the granularity is 1.0
        let mut gran = self.media_duration_s as f32 / width;
        gran = utils::clamp(3.0, 0.016, gran);
        *self.granularity.borrow_mut() = gran;

        debug!("Granularity = {}", gran);

        // Duration Range
        self.duration_range = [0, (gran * width) as i64];
    }

    fn increase_granularity(&mut self) {
        let mut gran = *self.granularity.borrow() + 0.01;
        gran = gran.min(3.0);
        *self.granularity.borrow_mut() = gran;
    }

    fn decrease_granularity(&mut self) {
        let mut gran = *self.granularity.borrow() - 0.01;
        gran = gran.max(0.016);
        *self.granularity.borrow_mut() = gran;
    }

    fn move_duration_range(&mut self, delta: f32) {
        let sgn = -utils::sgn(delta);
        let sec_pixs = self.calc_sec_pixels();
        let tick_step = self.calc_tick_step();
        let tick_secs = tick_step / sec_pixs;
        let delta_seconds = (sgn as f32 * tick_secs) as i64;
        let interval = self.duration_range[1] - self.duration_range[0];

        self.duration_range[0] = 0.max(self.duration_range[0] + delta_seconds);
        self.duration_range[1] = self.duration_range[0] + interval;

        if self.duration_range[1] > self.media_duration_s {
            self.duration_range[1] = self.media_duration_s;
            self.duration_range[0] = self.duration_range[1] - interval;
        }
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
        self.init();
    }

    pub fn set_player(&mut self, player: &Shared<Player>) {
        self.player = Some(player.clone());
        self.init();
    }

    /// Get current timestamp pointed by the cursor of the timeline in SECONDS.
    /// In other words, get the elapsed duration in SECONDS.
    pub fn get_cursor_timestamp(&self) -> i64 {
        if let Some(player) = self.player.as_ref() {
            player.borrow().elapsed_ms() / 1000
        } else {
            0
        }
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
        self.state.borrow_mut().width = width;
        self.draw_cursor(ctx, &painter, &resp);
        self.draw_hovered_cursor(ctx, &painter, &resp);
        self.draw_ticks(ctx, &painter, &resp);

        for i in self.subrip_blocks.iter_mut() {
            // if i.is_containsed_in_range(&self.duration_range) {
            //     i.draw(ctx, eui);
            // }
            // i.draw(ctx, eui);
            i.draw_on_timeline(ctx, eui, &resp.rect);
        }
    }
}

impl TimelineState {
    pub fn is_width_changed(&self, width: f32) -> bool {
        self.width != width
    }
}
