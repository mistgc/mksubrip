use crate::prelude::*;
use crate::ui::Drawable;
use crate::ui::Timeline;
use crate::Subrip;

pub struct NewSubripWindow {
    pub sig_created_subrip: Signal<Shared<Subrip>>,

    title: String,
    subrip_text: String,
    visible: bool,
    timeline: Option<Shared<Timeline>>,
}

impl Default for NewSubripWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl NewSubripWindow {
    pub fn new() -> Self {
        Self {
            sig_created_subrip: Signal::new(),
            title: "New Subrip".to_string(),
            subrip_text: String::new(),
            visible: false,
            timeline: None,
        }
    }

    pub fn toggle_visible(&mut self, _: &()) {
        self.visible = !self.visible;
    }

    fn create_subrip(&mut self) {
        use chrono::{Duration, NaiveTime};

        // TODO: get begin time from Time Line
        // let begin_time = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
        let begin_time = if let Some(timeline) = self.timeline.as_ref() {
            let timestamp_s = timeline.borrow().get_cursor_timestamp();
            NaiveTime::from_num_seconds_from_midnight_opt(timestamp_s as u32, 0).unwrap_or_default()
        } else {
            NaiveTime::default()
        };
        // Default duration
        let duration = Duration::new(5, 0).unwrap();
        let subrip = Subrip::new(&self.subrip_text, begin_time, duration);

        self.sig_created_subrip.emit(&Shared::new(subrip));
    }
}

impl Drawable for NewSubripWindow {
    fn draw(&mut self, ctx: &egui::Context, _eui: &mut egui::Ui) {
        if self.visible {
            egui::Window::new(&self.title)
                .collapsible(false)
                .show(ctx, |eui| {
                    eui.heading("Create a new subrip");
                    eui.text_edit_multiline(&mut self.subrip_text);
                    if eui.button("Submit").clicked() && self.subrip_text.as_str() != "" {
                        info!("Submited Text: {}", self.subrip_text.as_str());
                        self.create_subrip();
                        self.subrip_text.clear();
                    }
                });
        }
    }
}
