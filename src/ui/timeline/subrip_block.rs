use chrono::Timelike;

use crate::prelude::*;
use crate::ui::Drawable;
use crate::Subrip;

const BORDER_NORMAL_WIDTH: f32 = 1.0;
const BORDER_HOVER_RANGE: f32 = 8.0;
const BORDER_HOVERED_WIDTH: f32 = 2.0;
const BLOCK_HEIGHT: f32 = 50.0;

// FIXME:
// 1. The display for data of the subrip is exceptional.
// 2. Dropping bounds is exceptional.
pub struct SubripBlock {
    state: SubripBlockState,

    granularity: Shared<f32>,
    subrip: Shared<Subrip>,
}

pub struct SubripBlockState {
    pos: Pos2,
    offset: f32,
    rect: egui::Rect,
    body_dragging: bool,
    body_drag_start: Pos2,
    left_dragging: bool,
    left_drag_start: Pos2,
    right_dragging: bool,
    right_drag_start: Pos2,
}

impl SubripBlock {
    pub fn new(data: Shared<Subrip>) -> Self {
        Self {
            state: SubripBlockState::new(),
            subrip: data,
            granularity: Shared::new(1.0),
        }
    }

    pub fn set_granularity(&mut self, granularity: Shared<f32>) {
        self.granularity = granularity;
    }

    /// Get the begin timestamp and end timestamp of [`Subrip`] in SECONDS
    pub fn get_duration_range(&self) -> [i64; 2] {
        let borrowed_subrip = self.subrip.borrow();
        let begin_timestamp_s = borrowed_subrip.get_begin_time().num_seconds_from_midnight() as i64;
        let end_timestamp_s = begin_timestamp_s + borrowed_subrip.get_duration().num_seconds();

        [begin_timestamp_s, end_timestamp_s]
    }

    /// Check if the duration range is contained in the given range.
    pub fn is_containsed_in_range(&self, range: &[i64; 2]) -> bool {
        utils::range_contains_subrange(range, &self.get_duration_range())
    }

    fn is_hovered_left(&self, resp: &egui::Response) -> bool {
        let rect = self.state.rect;
        let hover_pos = resp.hover_pos().unwrap_or_default();

        hover_pos.x >= rect.left() && hover_pos.x <= rect.left() + BORDER_HOVER_RANGE
    }

    fn is_hovered_right(&self, resp: &egui::Response) -> bool {
        let rect = self.state.rect;
        let hover_pos = resp.hover_pos().unwrap_or_default();

        hover_pos.x >= rect.left() + rect.width() - BORDER_HOVER_RANGE
            && hover_pos.x <= rect.left() + rect.width()
    }

    fn is_hovered_body(&self, resp: &egui::Response) -> bool {
        !self.is_hovered_left(resp) && !self.is_hovered_right(resp) && resp.hovered()
    }

    fn get_granularity(&self) -> f32 {
        *self.granularity.borrow()
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

    pub fn is_deleted(&self) -> bool {
        self.subrip.borrow().is_deleted()
    }

    pub fn draw_on_timeline(
        &mut self,
        ctx: &egui::Context,
        eui: &mut egui::Ui,
        timeline_rect: &egui::Rect,
        duration_range: &[i64; 2],
    ) {
        let mut subrip = self.subrip.borrow_mut();
        let begin_timestamp = subrip.begin_time.num_seconds_from_midnight() as i64;
        let end_timestamp = subrip.end_time.num_seconds_from_midnight() as i64;
        if begin_timestamp > duration_range[1] || end_timestamp < duration_range[0] {
            return;
        }

        let sec_pixs = self.calc_sec_pixels();
        let delta_pixs = (begin_timestamp - duration_range[0]) as f32 * sec_pixs;
        let duration_pixs = (end_timestamp - begin_timestamp) as f32 * sec_pixs;
        let exposed_pixs = if delta_pixs < 0.0 {
            duration_pixs + delta_pixs
        } else {
            duration_pixs
        };
        let x = delta_pixs.max(0.0);
        self.state.pos.x = x;

        let ctnt = subrip.get_content();
        let width = exposed_pixs;
        let height = BLOCK_HEIGHT;
        let paint_rect = utils::new_rect(
            timeline_rect.left() + self.state.pos.x,
            timeline_rect.top() + 64.0 + self.state.pos.y,
            timeline_rect.left() + self.state.pos.x + width,
            timeline_rect.top() + 64.0 + self.state.pos.y + height,
        );
        self.state.set_rect(paint_rect);
        let mut child_ui = eui.child_ui(paint_rect, egui::Layout::default());
        let (resp, painter) = child_ui.allocate_painter(
            Vec2::new(width + BORDER_NORMAL_WIDTH * 2.0, height),
            egui::Sense::click_and_drag(),
        );

        painter.rect_filled(
            egui::Rect::from_points(&[
                [paint_rect.left() + BORDER_NORMAL_WIDTH, paint_rect.top()].into(),
                [
                    paint_rect.left() + width - BORDER_NORMAL_WIDTH,
                    paint_rect.top() + paint_rect.height(),
                ]
                .into(),
            ]),
            egui::Rounding::default(),
            egui::Color32::from_hex("#222").unwrap(),
        );

        if resp.hovered() {
            painter.rect_filled(
                egui::Rect::from_points(&[
                    [paint_rect.left() + BORDER_NORMAL_WIDTH, paint_rect.top()].into(),
                    [
                        paint_rect.left() + width - BORDER_NORMAL_WIDTH,
                        paint_rect.top() + paint_rect.height(),
                    ]
                    .into(),
                ]),
                egui::Rounding::default(),
                egui::Color32::from_hex("#555").unwrap(),
            );
        }

        painter.rect_filled(
            egui::Rect::from_points(&[
                [paint_rect.left(), paint_rect.top()].into(),
                [
                    paint_rect.left() + BORDER_NORMAL_WIDTH,
                    paint_rect.top() + paint_rect.height(),
                ]
                .into(),
            ]),
            egui::Rounding::default(),
            egui::Color32::GRAY,
        );

        if self.is_hovered_left(&resp) {
            painter.rect_filled(
                egui::Rect::from_points(&[
                    [paint_rect.left(), paint_rect.top()].into(),
                    [
                        paint_rect.left() + BORDER_HOVERED_WIDTH,
                        paint_rect.top() + paint_rect.height(),
                    ]
                    .into(),
                ]),
                egui::Rounding::default(),
                egui::Color32::WHITE,
            );
        }

        painter.rect_filled(
            egui::Rect::from_points(&[
                [paint_rect.left() + paint_rect.width(), paint_rect.top()].into(),
                [
                    paint_rect.left() + paint_rect.width() + 1.0,
                    paint_rect.top() + paint_rect.height(),
                ]
                .into(),
            ]),
            egui::Rounding::default(),
            egui::Color32::GRAY,
        );

        let galley = painter.layout(
            ctnt,
            egui::FontId::default(),
            egui::Color32::WHITE,
            width - 10.0,
        );

        painter.galley(
            Pos2::new(paint_rect.left() + 5.0, paint_rect.top()),
            galley,
            egui::Color32::PLACEHOLDER,
        );

        if self.is_hovered_right(&resp) {
            painter.rect_filled(
                egui::Rect::from_points(&[
                    [
                        paint_rect.left() + paint_rect.width() - BORDER_HOVERED_WIDTH,
                        paint_rect.top(),
                    ]
                    .into(),
                    [
                        paint_rect.left() + paint_rect.width(),
                        paint_rect.top() + paint_rect.height(),
                    ]
                    .into(),
                ]),
                egui::Rounding::default(),
                egui::Color32::WHITE,
            );
        }

        if self.is_hovered_left(&resp) && resp.drag_started() {
            if let Some(new_drag_start_pos) = resp.interact_pointer_pos() {
                self.state.left_dragging = true;
                self.state.left_drag_start = new_drag_start_pos;
                self.state.body_dragging = false;
                self.state.right_dragging = false;
            }
        } else if self.is_hovered_right(&resp) && resp.drag_started() {
            if let Some(new_drag_start_pos) = resp.interact_pointer_pos() {
                self.state.right_dragging = true;
                self.state.right_drag_start = new_drag_start_pos;
                self.state.body_dragging = false;
                self.state.left_dragging = false;
            }
        } else if self.is_hovered_body(&resp) && resp.drag_started() {
            if let Some(new_drag_start_pos) = resp.interact_pointer_pos() {
                self.state.body_dragging = true;
                self.state.body_drag_start = new_drag_start_pos;
                self.state.left_dragging = false;
                self.state.right_dragging = false;
            }
        }

        if self.state.body_dragging {
            if let Some(new_drag_new_pos) = resp.interact_pointer_pos() {
                let drag_delta = new_drag_new_pos.x - self.state.body_drag_start.x;
                // let time_delta = drag_delta * (*self.granularity.borrow());
                let time_delta = drag_delta / self.calc_sec_pixels();
                subrip.add_begin_delta(time_delta);
                subrip.add_end_delta(time_delta);
                // self.state.pos.x += drag_delta;
                self.state.body_drag_start.x = new_drag_new_pos.x;
            }
        } else if self.state.left_dragging {
            if let Some(new_drag_new_pos) = resp.interact_pointer_pos() {
                let drag_delta = new_drag_new_pos.x - self.state.left_drag_start.x;
                // let time_delta = drag_delta * (*self.granularity.borrow());
                let time_delta = drag_delta / self.calc_sec_pixels();
                // self.state.pos.x += time_delta;
                subrip.add_begin_delta(time_delta);
                self.state.left_drag_start.x = new_drag_new_pos.x;
            }
        } else if self.state.right_dragging {
            if let Some(new_drag_new_pos) = resp.interact_pointer_pos() {
                let drag_delta = new_drag_new_pos.x - self.state.right_drag_start.x;
                // let time_delta = drag_delta * (*self.granularity.borrow());
                let time_delta = drag_delta / self.calc_sec_pixels();
                subrip.add_end_delta(time_delta);
                self.state.right_drag_start.x = new_drag_new_pos.x;
            }
        }

        if resp.drag_stopped() {
            self.state.body_drag_start = Pos2 { x: 0.0, y: 0.0 };
            self.state.left_drag_start = Pos2 { x: 0.0, y: 0.0 };
            self.state.right_drag_start = Pos2 { x: 0.0, y: 0.0 };
            self.state.body_dragging = false;
            self.state.left_dragging = false;
            self.state.right_dragging = false;
        }

        if resp.hovered() {
            if let Some(pointer_pos) = ctx.pointer_latest_pos() {
                if paint_rect.contains(pointer_pos) {
                    egui::show_tooltip_at(
                        ctx,
                        resp.id.with("__tooltip"),
                        Some(pointer_pos),
                        |ui| {
                            ui.vertical(|ui| {
                                let begin_time = subrip.get_begin_time();
                                let end_time = subrip.get_end_time();
                                let content = subrip.get_content();

                                ui.label(format!(
                                    "{} --> {}",
                                    begin_time.format("%M:%S"),
                                    end_time.format("%M:%S"),
                                ));
                                ui.label(content);
                            });
                        },
                    );
                }
            }
        }
    }
}

impl Drawable for SubripBlock {
    fn draw(&mut self, ctx: &egui::Context, eui: &mut egui::Ui) {
        let cursor_rect = eui.cursor();
        let mut subrip = self.subrip.borrow_mut();
        let ctnt = subrip.get_content();
        let duration = subrip.get_duration().num_seconds();
        let width = duration as f32 / *self.granularity.borrow();
        let height = BLOCK_HEIGHT;
        let rect = utils::new_rect(
            cursor_rect.left() + self.state.pos.x,
            cursor_rect.top() + self.state.pos.y,
            cursor_rect.left() + self.state.pos.x + width,
            cursor_rect.top() + self.state.pos.y + height,
        );
        self.state.set_rect(rect);
        let mut child_ui = eui.child_ui(rect, egui::Layout::default());
        let (resp, painter) = child_ui.allocate_painter(
            Vec2::new(width + BORDER_NORMAL_WIDTH * 2.0, height),
            egui::Sense::click_and_drag(),
        );

        painter.rect_filled(
            egui::Rect::from_points(&[
                [rect.left() + BORDER_NORMAL_WIDTH, rect.top()].into(),
                [
                    rect.left() + width - BORDER_NORMAL_WIDTH,
                    rect.top() + rect.height(),
                ]
                .into(),
            ]),
            egui::Rounding::default(),
            egui::Color32::from_hex("#222").unwrap(),
        );

        if resp.hovered() {
            painter.rect_filled(
                egui::Rect::from_points(&[
                    [rect.left() + BORDER_NORMAL_WIDTH, rect.top()].into(),
                    [
                        rect.left() + width - BORDER_NORMAL_WIDTH,
                        rect.top() + rect.height(),
                    ]
                    .into(),
                ]),
                egui::Rounding::default(),
                egui::Color32::from_hex("#555").unwrap(),
            );
        }

        painter.rect_filled(
            egui::Rect::from_points(&[
                [rect.left(), rect.top()].into(),
                [
                    rect.left() + BORDER_NORMAL_WIDTH,
                    rect.top() + rect.height(),
                ]
                .into(),
            ]),
            egui::Rounding::default(),
            egui::Color32::GRAY,
        );

        if self.is_hovered_left(&resp) {
            painter.rect_filled(
                egui::Rect::from_points(&[
                    [rect.left(), rect.top()].into(),
                    [
                        rect.left() + BORDER_HOVERED_WIDTH,
                        rect.top() + rect.height(),
                    ]
                    .into(),
                ]),
                egui::Rounding::default(),
                egui::Color32::WHITE,
            );
        }

        painter.rect_filled(
            egui::Rect::from_points(&[
                [rect.left() + rect.width(), rect.top()].into(),
                [rect.left() + rect.width() + 1.0, rect.top() + rect.height()].into(),
            ]),
            egui::Rounding::default(),
            egui::Color32::GRAY,
        );

        let galley = painter.layout(
            ctnt,
            egui::FontId::default(),
            egui::Color32::WHITE,
            width - 10.0,
        );

        painter.galley(
            Pos2::new(rect.left() + 5.0, rect.top()),
            galley,
            egui::Color32::PLACEHOLDER,
        );

        if self.is_hovered_right(&resp) {
            painter.rect_filled(
                egui::Rect::from_points(&[
                    [
                        rect.left() + rect.width() - BORDER_HOVERED_WIDTH,
                        rect.top(),
                    ]
                    .into(),
                    [rect.left() + rect.width(), rect.top() + rect.height()].into(),
                ]),
                egui::Rounding::default(),
                egui::Color32::WHITE,
            );
        }

        if self.is_hovered_left(&resp) && resp.drag_started() {
            if let Some(new_drag_start_pos) = resp.interact_pointer_pos() {
                self.state.left_dragging = true;
                self.state.left_drag_start = new_drag_start_pos;
                self.state.body_dragging = false;
                self.state.right_dragging = false;
            }
        } else if self.is_hovered_right(&resp) && resp.drag_started() {
            if let Some(new_drag_start_pos) = resp.interact_pointer_pos() {
                self.state.right_dragging = true;
                self.state.right_drag_start = new_drag_start_pos;
                self.state.body_dragging = false;
                self.state.left_dragging = false;
            }
        } else if self.is_hovered_body(&resp) && resp.drag_started() {
            if let Some(new_drag_start_pos) = resp.interact_pointer_pos() {
                self.state.body_dragging = true;
                self.state.body_drag_start = new_drag_start_pos;
                self.state.left_dragging = false;
                self.state.right_dragging = false;
            }
        }

        // if resp.drag_started() {
        //     if self.is_hovered_left(&resp) {
        //         if let Some(new_drag_start_pos) = resp.interact_pointer_pos() {
        //             self.state.left_dragging = true;
        //             self.state.left_drag_start = new_drag_start_pos;
        //             self.state.body_dragging = false;
        //             self.state.right_dragging = false;
        //         }
        //     } else if self.is_hovered_right(&resp) {
        //         if let Some(new_drag_start_pos) = resp.interact_pointer_pos() {
        //             self.state.right_dragging = true;
        //             self.state.right_drag_start = new_drag_start_pos;
        //             self.state.body_dragging = false;
        //             self.state.left_dragging = false;
        //         }
        //     } else if self.is_hovered_body(&resp) {
        //         if let Some(new_drag_start_pos) = resp.interact_pointer_pos() {
        //             self.state.body_dragging = true;
        //             self.state.body_drag_start = new_drag_start_pos;
        //             self.state.left_dragging = false;
        //             self.state.right_dragging = false;
        //         }
        //     }
        // }

        if self.state.body_dragging {
            if let Some(new_drag_new_pos) = resp.interact_pointer_pos() {
                let drag_delta = new_drag_new_pos.x - self.state.body_drag_start.x;
                let time_delta = drag_delta * (*self.granularity.borrow());
                subrip.add_begin_delta(time_delta);
                subrip.add_end_delta(time_delta);
                self.state.pos.x += drag_delta;
                self.state.body_drag_start.x = new_drag_new_pos.x;
            }
        } else if self.state.left_dragging {
            if let Some(new_drag_new_pos) = resp.interact_pointer_pos() {
                let drag_delta = new_drag_new_pos.x - self.state.left_drag_start.x;
                let time_delta = drag_delta * (*self.granularity.borrow());
                self.state.pos.x += time_delta;
                subrip.add_begin_delta(time_delta);
                self.state.left_drag_start.x = new_drag_new_pos.x;
            }
        } else if self.state.right_dragging {
            if let Some(new_drag_new_pos) = resp.interact_pointer_pos() {
                let drag_delta = new_drag_new_pos.x - self.state.right_drag_start.x;
                subrip.add_end_delta(drag_delta * (*self.granularity.borrow()));
                self.state.right_drag_start.x = new_drag_new_pos.x;
            }
        }

        if resp.drag_stopped() {
            self.state.body_drag_start = Pos2 { x: 0.0, y: 0.0 };
            self.state.left_drag_start = Pos2 { x: 0.0, y: 0.0 };
            self.state.right_drag_start = Pos2 { x: 0.0, y: 0.0 };
            self.state.body_dragging = false;
            self.state.left_dragging = false;
            self.state.right_dragging = false;
        }

        if resp.hovered() {
            if let Some(_pointer_pos) = ctx.pointer_latest_pos() {}
        }
    }
}

impl Default for SubripBlockState {
    fn default() -> Self {
        Self::new()
    }
}

impl SubripBlockState {
    pub fn new() -> Self {
        Self {
            offset: 0.0,
            pos: Pos2 { x: 0.0, y: 0.0 },
            rect: utils::new_rect(0.0, 0.0, 0.0, 0.0),
            body_dragging: false,
            body_drag_start: Pos2 { x: 0.0, y: 0.0 },
            left_dragging: false,
            left_drag_start: Pos2 { x: 0.0, y: 0.0 },
            right_dragging: false,
            right_drag_start: Pos2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn set_rect(&mut self, rect: egui::Rect) {
        self.rect = rect;
    }
}
