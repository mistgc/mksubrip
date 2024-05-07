use chrono::NaiveTime;

use crate::ui::Drawable;
use crate::{prelude::*, Subrip};

#[derive(Default)]
pub struct SubripListItem {
    pub(crate) subrip: Shared<Subrip>,

    begin_time_text: String,
    end_time_text: String,
    content_text: String,

    old_begin_time_text: String,
    old_end_time_text: String,
    old_content_text: String,
}

impl SubripListItem {
    pub fn new(subrip: Shared<Subrip>) -> Self {
        Self {
            subrip: subrip.clone(),
            content_text: subrip.borrow().get_content(),
            begin_time_text: subrip
                .borrow()
                .get_begin_time()
                .format("%H:%M:%S")
                .to_string(),
            end_time_text: subrip
                .borrow()
                .get_end_time()
                .format("%H:%M:%S")
                .to_string(),
            old_content_text: String::new(),
            old_begin_time_text: String::new(),
            old_end_time_text: String::new(),
        }
    }

    fn sync_data(&mut self) {
        if self.begin_time_text != self.old_begin_time_text {
            self.old_begin_time_text = self.begin_time_text.clone();
            if let Ok(naive_time) =
                NaiveTime::parse_from_str(self.old_begin_time_text.as_str(), "%H:%M:%S")
            {
                self.subrip.borrow_mut().set_begin_time(&naive_time);
            }
        }
        if self.end_time_text != self.old_end_time_text {
            self.old_end_time_text = self.end_time_text.clone();
            if let Ok(naive_time) =
                NaiveTime::parse_from_str(self.old_end_time_text.as_str(), "%H:%M:%S")
            {
                self.subrip.borrow_mut().set_end_time(&naive_time);
            }
        }
        if self.content_text != self.old_content_text {
            self.old_content_text = self.content_text.clone();
            self.subrip
                .borrow_mut()
                .set_content(self.old_content_text.as_str());
        }
    }
}

impl Drawable for SubripListItem {
    fn draw(&mut self, _ctx: &eframe::egui::Context, eui: &mut eframe::egui::Ui) {
        // Return directly if the subrip is loaded.
        if self.subrip.borrow_mut().is_loaded() {
            return;
        }

        let text_edit_width = 120.0;

        eui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |eui| {
            egui::TextEdit::singleline(&mut self.begin_time_text)
                .margin(egui::Vec2 { x: 10.0, y: 10.0 })
                .desired_width(text_edit_width)
                .show(eui);
            eui.add_space(eui.available_width() - text_edit_width);
            egui::TextEdit::singleline(&mut self.end_time_text)
                .margin(egui::Vec2 { x: 10.0, y: 10.0 })
                .desired_width(text_edit_width)
                .show(eui);
        });
        egui::TextEdit::multiline(&mut self.content_text)
            .desired_width(eui.available_width())
            .desired_rows(2)
            .show(eui);
        eui.separator();

        self.sync_data();
    }
}

impl From<Shared<Subrip>> for SubripListItem {
    fn from(value: Shared<Subrip>) -> Self {
        Self {
            subrip: value.clone(),
            content_text: value.borrow_mut().get_content(),
            begin_time_text: value
                .borrow_mut()
                .get_begin_time()
                .format("%H:%M:%S")
                .to_string(),
            end_time_text: value
                .borrow_mut()
                .get_end_time()
                .format("%H:%M:%S")
                .to_string(),
            old_content_text: String::new(),
            old_begin_time_text: String::new(),
            old_end_time_text: String::new(),
        }
    }
}
