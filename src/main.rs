pub mod app;
pub mod subrip;
pub mod ui;
pub mod writer;

pub use app::App;

fn main() {
    env_logger::init();

    let opts = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui app", opts, Box::new(|cc| Box::new(App::new(cc))));
}
