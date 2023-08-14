/// a component that can be draw
pub trait View {
    fn draw(&mut self, ctx: &eframe::egui::Context, eui: &mut eframe::egui::Ui);
}
