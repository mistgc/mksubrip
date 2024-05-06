pub mod subrip_block;

use crate::app::AppState;
use crate::prelude::*;
use crate::ui::Drawable;
use crate::ui::SubripBlock;
use crate::Subrip;

#[derive(Default)]
pub struct TimeLine {
    pub sig_video_seeked: Signal<f32>,

    app_state: Shared<AppState>,

    media_duration_s: i64,
    default_height: f32,
    granularity: Shared<f32>,
    stroke: egui::Stroke,
    subrip_blocks: Vec<SubripBlock>,
}

impl TimeLine {
    pub fn new(app_state: Shared<AppState>) -> Self {
        Self {
            app_state,
            default_height: 120.0,
            media_duration_s: 0,
            granularity: Shared::new(1.0),
            stroke: egui::Stroke::new(2.0, egui::Color32::from_hex("#555555").unwrap()),
            ..Self::default()
        }
    }

    fn set_granularity(&mut self, granularity: f32) {
        *self.granularity.borrow_mut() = granularity;
    }

    fn draw_timeline_cursor(
        &mut self,
        ctx: &egui::Context,
        painter: &egui::Painter,
        timeline_rect: egui::Rect,
    ) {
        if let Some(pointer_pos) = ctx.pointer_hover_pos() {
            if pointer_pos.x > timeline_rect.min.x
                && pointer_pos.x < timeline_rect.max.x
                && pointer_pos.y > timeline_rect.min.y
                && pointer_pos.y < timeline_rect.max.y
            {
                let p0 = Pos2 {
                    x: pointer_pos.x,
                    y: timeline_rect.min.y,
                };
                let p1 = Pos2 {
                    x: pointer_pos.x,
                    y: timeline_rect.max.y,
                };
                let p2 = Pos2 {
                    x: pointer_pos.x - 3.0,
                    y: timeline_rect.min.y,
                };
                let p3 = Pos2 {
                    x: pointer_pos.x + 3.0,
                    y: timeline_rect.min.y,
                };
                let p4 = Pos2 {
                    x: pointer_pos.x - 3.0,
                    y: timeline_rect.max.y,
                };
                let p5 = Pos2 {
                    x: pointer_pos.x + 3.0,
                    y: timeline_rect.max.y,
                };

                painter.line_segment([p0, p1], self.stroke);
                painter.line_segment([p2, p3], self.stroke);
                painter.line_segment([p4, p5], self.stroke);
            }
        }
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
}

impl Drawable for TimeLine {
    fn draw(&mut self, ctx: &egui::Context, eui: &mut egui::Ui) {
        let width = ctx.available_rect().width();
        *self.granularity.borrow_mut() = self.media_duration_s as f32 / width;
        let (resp, painter) = eui.allocate_painter(
            Vec2::new(width, self.default_height),
            egui::Sense::click_and_drag(),
        );
        let transform_to_screen = emath::RectTransform::from_to(
            egui::Rect::from_min_size(Pos2::ZERO, resp.rect.size()),
            resp.rect,
        );

        self.draw_timeline_cursor(ctx, &painter, resp.rect);

        for i in 10..(width as usize + 1) {
            if i % 10 == 0 {
                let p0 = transform_to_screen.transform_pos(Pos2 {
                    x: i as f32 - 8.0,
                    y: self.default_height / 2.0,
                });

                let p1 = transform_to_screen.transform_pos(Pos2 {
                    x: i as f32 - 2.0,
                    y: self.default_height / 2.0,
                });

                painter.line_segment([p0, p1], self.stroke);
            } else if i == width as usize + 1 {
            }
        }

        for i in self.subrip_blocks.iter_mut() {
            i.draw(ctx, eui);
        }
    }
}
