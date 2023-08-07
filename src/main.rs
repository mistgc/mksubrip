pub mod ui;
pub mod writer;
pub mod app;
pub mod subrip;

pub use app::App;

fn main() {
    let opts = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui app", opts, Box::new(|cc| {
        Box::new(App::new(cc))
    }));
}
