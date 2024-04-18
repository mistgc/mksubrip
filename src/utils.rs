use crate::prelude::*;

pub fn new_rect(x: f32, y: f32, w: f32, h: f32) -> egui::Rect {
    egui::Rect::from_points(&[Pos2 { x, y }, Pos2 { x: w, y: h }])
}

pub fn clamp<T: std::cmp::PartialOrd>(max: T, min: T, value: T) -> T {
    if value < min {
        min
    } else if value > max {
        return max;
    } else {
        return value;
    }
}
