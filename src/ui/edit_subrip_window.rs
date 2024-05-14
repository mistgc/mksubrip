use crate::prelude::*;
use crate::ui::Drawable;
use crate::Subrip;

#[derive(Default)]
pub struct EditSubripWindow {
    subrip: Option<Shared<Subrip>>,

    visible: bool,
    text: String,
}

impl EditSubripWindow {
    pub fn new() -> Self {
        Self {
            subrip: None,
            visible: false,
            text: String::new(),
        }
    }

    pub fn edit_subrip(&mut self, subrip: &Shared<Subrip>) {
        self.set_subrip(subrip);
        self.visible = true;
    }

    pub fn set_subrip(&mut self, subrip: &Shared<Subrip>) {
        self.text = subrip.borrow().get_content();
        self.subrip = Some(subrip.clone());
    }

    pub fn submit(&mut self) {
        if let Some(subrip) = self.subrip.as_ref() {
            subrip.borrow_mut().set_content(self.text.as_str());
        }

        self.visible = false;
    }
}

impl Drawable for EditSubripWindow {
    fn draw(&mut self, ctx: &egui::Context, _eui: &mut egui::Ui) {
        if self.visible {
            let window = egui::Window::new("Edit Subrip Window").collapsible(false);
            window.show(ctx, |ui| {
                ui.text_edit_multiline(&mut self.text);

                if ui.button("Submit").clicked() {
                    self.submit();
                }
            });
        }
    }
}
