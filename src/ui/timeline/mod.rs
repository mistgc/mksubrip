pub mod subrip_block;

use crate::app::AppState;
use crate::prelude::*;
use crate::ui::Drawable;
use crate::ui::SubripBlock;
use crate::Subrip;

#[derive(Default)]
pub struct TimeLine {
    // pub sig_value_changed: Signal<f32>,
    app_state: Rc<RefCell<AppState>>,

    default_height: f32,
    granularity: Rc<Cell<f32>>,
    stroke: egui::Stroke,
    audio_data: Vec<f32>,
    audio_duration: i64,
    subrip_blocks: Vec<SubripBlock>,
}

impl TimeLine {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            app_state,
            default_height: 120.0,
            audio_duration: 1440,
            granularity: Rc::new(Cell::new(1.0)),
            stroke: egui::Stroke::new(2.0, egui::Color32::from_hex("#555555").unwrap()),
            ..Self::default()
        }
    }

    pub fn set_audio_duration(&mut self, audio_duration: i64) {
        self.audio_duration = audio_duration;
    }

    pub fn set_granularity(&mut self, granularity: f32) {
        self.granularity.set(granularity);
    }

    pub fn set_audio_data(&mut self, data: Vec<f32>) {
        self.audio_data = data;
    }

    pub fn add_subrip_block(&mut self, subrip_block: SubripBlock) {
        self.subrip_blocks.push(subrip_block);
    }

    pub fn add_block_from_subrip(&mut self, subrip: &Rc<RefCell<Subrip>>) {
        let block = SubripBlock::new(subrip.clone());
        self.subrip_blocks.push(block);
    }
}

impl Drawable for TimeLine {
    fn draw(&mut self, ctx: &egui::Context, eui: &mut egui::Ui) {
        let width = ctx.available_rect().width();
        let _height = self.default_height;

        let (resp, painter) = eui.allocate_painter(
            egui::vec2(width, self.default_height),
            egui::Sense::click_and_drag(),
        );
        let to_screen = emath::RectTransform::from_to(
            egui::Rect::from_min_size(Pos2::ZERO, resp.rect.size()),
            resp.rect,
        );
        let ch = resp.rect.height() / 2.0;
        let mut sum = 0.0;

        for i in self.audio_data.iter() {
            let p1 = to_screen.transform_pos(Pos2 { x: sum, y: ch - i });
            let p2 = to_screen.transform_pos(Pos2 { x: sum, y: ch + i });

            painter.add(egui::Shape::LineSegment {
                points: [p1, p2],
                stroke: self.stroke,
            });

            sum += self.granularity.get();
        }

        for i in self.subrip_blocks.iter_mut() {
            i.draw(ctx, eui);
        }
    }
}
