use chrono::{DateTime, Duration, Utc};

use crate::ui::view::View;

#[derive(PartialEq, Eq)]
pub enum Format {
    SRT,
    ASS,
}

pub struct Subrip {
    pub format: Format,
    pub index: u32,
    begin_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    duration: Option<Duration>,
    content: Option<String>,
    state: Option<SubripState>,
}

#[derive(Default)]
struct SubripState {
    begin_time_text: String,
    end_time_text: String,
    content_text: String,
}

impl Default for Subrip {
    fn default() -> Self {
        Self {
            format: Format::SRT,
            index: 1,
            begin_time: None,
            end_time: None,
            duration: None,
            content: None,
            state: None,
        }
    }
}

impl Subrip {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            ..Self::default()
        }
    }

    /// Set begin time, end time and duration
    ///
    /// `end_time` = `begin_time` + `duration`
    pub fn set_time(&mut self, begin_time: DateTime<Utc>, duration: Duration) {
        self.begin_time = Some(begin_time);
        self.end_time = Some(begin_time + duration);
        self.duration = Some(duration);
    }

    /// Get text content
    pub fn get_content(&self) -> Option<String> {
        self.content.as_ref().cloned()
    }

    /// Get begin and end time
    pub fn get_time(&self) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
        (self.begin_time, self.end_time)
    }
}

impl View for Subrip {
    fn draw(&mut self, _ctx: &eframe::egui::Context, eui: &mut eframe::egui::Ui) {
        use eframe::egui;

        if self.state.is_none() {
            // initialize subrip state
            let mut beg_text = "".to_owned();
            let mut end_text = "".to_owned();
            let mut ctnt_text = "".to_owned();

            if let Some(beg_time) = &self.begin_time {
                beg_text = beg_time.time().format("%H:%M:%S").to_string();
            }

            if let Some(end_time) = &self.end_time {
                end_text = end_time.time().format("%H:%M:%S").to_string();
            }

            if let Some(ctnt) = &self.content {
                ctnt_text = ctnt.clone();
            }

            self.state = Some(SubripState {
                begin_time_text: beg_text,
                end_time_text: end_text,
                content_text: ctnt_text,
            });
        }

        if let Some(state) = &mut self.state {
            eui.label("Begin Time:");
            egui::TextEdit::singleline(&mut state.begin_time_text).show(eui);
            eui.label("End Time:");
            egui::TextEdit::singleline(&mut state.end_time_text).show(eui);
            egui::TextEdit::singleline(&mut state.content_text).show(eui);
        }
    }
}
