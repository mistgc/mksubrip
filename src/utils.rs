use crate::prelude::*;

pub use lerp::lerp;
pub use lerp::Lerp;

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

pub fn range_contains_timestamp(range: &[i64; 2], timestamp: i64) -> bool {
    timestamp >= range[0] && timestamp < range[1]
}

mod lerp {
    pub trait Lerp {
        type Output;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output;
    }

    pub fn lerp<T: Lerp>(a: &T, b: &T, t: f32) -> T::Output {
        a.lerp(b, t)
    }

    impl Lerp for f32 {
        type Output = f32;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            self + (other - self) * t
        }
    }

    impl Lerp for u64 {
        type Output = u64;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            self + ((other - self) as f64 * t as f64) as u64
        }
    }

    impl Lerp for i64 {
        type Output = i64;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            self + ((other - self) as f64 * t as f64) as i64
        }
    }

    impl Lerp for u32 {
        type Output = u32;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            self + ((other - self) as f32 * t) as u32
        }
    }

    impl Lerp for i32 {
        type Output = i32;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            self + ((other - self) as f32 * t) as i32
        }
    }

    impl Lerp for u16 {
        type Output = u16;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            self + ((other - self) as f32 * t) as u16
        }
    }

    impl Lerp for i16 {
        type Output = i16;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            self + ((other - self) as f32 * t) as i16
        }
    }

    impl Lerp for u8 {
        type Output = u8;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            self + ((other - self) as f32 * t) as u8
        }
    }

    impl Lerp for i8 {
        type Output = i8;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            self + ((other - self) as f32 * t) as i8
        }
    }

    impl Lerp for eframe::egui::Pos2 {
        type Output = Self;

        fn lerp(&self, other: &Self, t: f32) -> Self::Output {
            let x = lerp(&self.x, &other.x, t);
            let y = lerp(&self.y, &other.y, t);

            eframe::egui::Pos2 { x, y }
        }
    }
}
