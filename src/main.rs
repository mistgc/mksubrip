use mksubrip::prelude::*;
use mksubrip::App;

fn main() {
    env_logger::init();

    let mut opts = eframe::NativeOptions::default();
    opts.viewport = opts.viewport.with_inner_size([1024.0, 720.0]);
    let _ = eframe::run_native(
        "mksubrip",
        opts,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "simsun".to_owned(),
                egui::FontData::from_static(include_bytes!("/usr/share/fonts/TTF/simsun.ttc")),
            );
            // Put my font first (highest priority) for proportional text:
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "simsun".to_owned());

            // Put my font as last fallback for monospace:
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("simsun".to_owned());
            let ctx = &cc.egui_ctx;
            ctx.set_fonts(fonts);
            Box::new(App::new(cc))
        }),
    );
}
