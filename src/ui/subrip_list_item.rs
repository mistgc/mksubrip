use chrono::NaiveTime;

use crate::ui::Drawable;
use crate::{prelude::*, Subrip};

#[derive(Default)]
pub struct SubripListItem {
    pub sig_begin_time_changed: Signal<NaiveTime>,
    pub sig_end_time_changed: Signal<NaiveTime>,
    pub sig_content_changed: Signal<String>,

    begin_time_text: String,
    end_time_text: String,
    content_text: String,

    old_begin_time_text: String,
    old_end_time_text: String,
    old_content_text: String,
}

impl SubripListItem {
    pub fn new(
        content_text: impl Into<String>,
        begin_time_text: impl Into<String>,
        end_time_text: impl Into<String>,
    ) -> Self {
        Self {
            sig_begin_time_changed: Signal::new(),
            sig_end_time_changed: Signal::new(),
            sig_content_changed: Signal::new(),
            content_text: content_text.into(),
            begin_time_text: begin_time_text.into(),
            end_time_text: end_time_text.into(),
            old_content_text: String::new(),
            old_begin_time_text: String::new(),
            old_end_time_text: String::new(),
        }
    }

    fn check_text(&mut self) {
        if self.begin_time_text != self.old_begin_time_text {
            self.old_begin_time_text = self.begin_time_text.clone();
            if let Ok(naive_time) =
                NaiveTime::parse_from_str(self.old_begin_time_text.as_str(), "%H:%M:%S")
            {
                self.sig_begin_time_changed.emit(&naive_time);
            }
        }
        if self.end_time_text != self.old_end_time_text {
            self.old_end_time_text = self.end_time_text.clone();
            if let Ok(naive_time) =
                NaiveTime::parse_from_str(self.old_end_time_text.as_str(), "%H:%M:%S")
            {
                self.sig_end_time_changed.emit(&naive_time);
            }
        }
        if self.content_text != self.old_content_text {
            self.old_content_text = self.content_text.clone();
            self.sig_content_changed.emit(&self.old_content_text);
        }
    }
}

impl Drawable for SubripListItem {
    fn draw(&mut self, _ctx: &eframe::egui::Context, eui: &mut eframe::egui::Ui) {
        let text_edit_width = 120.0;

        eui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |eui| {
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
            .show(eui);
        eui.separator();

        self.check_text();
    }
}

impl From<&Subrip> for SubripListItem {
    fn from(value: &Subrip) -> Self {
        Self::new(
            value.get_content(),
            value.get_begin_time().format("%H:%M:%S").to_string(),
            value.get_end_time().format("%H:%M:%S").to_string(),
        )
    }
}
