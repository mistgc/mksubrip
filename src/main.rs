use mksubrip::App;

fn main() {
    env_logger::init();

    let opts = eframe::NativeOptions::default();
    let _ = eframe::run_native("mksubrip", opts, Box::new(|cc| Box::new(App::new(cc))));
}
