use mksubrip::App;

fn main() {
    env_logger::init();

    let mut opts = eframe::NativeOptions::default();
    opts.viewport = opts.viewport.with_inner_size([1024.0, 720.0]);
    let _ = eframe::run_native("mksubrip", opts, Box::new(|cc| Box::new(App::new(cc))));
}
