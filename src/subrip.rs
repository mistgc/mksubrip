use chrono::{DateTime, Utc, Duration};

pub enum Format {
    SRT,
    ASS,
}

pub struct Subrip {
    format: Format,
    index: u32,
    begin_time: Option<DateTime<Utc>>,
    duration: Option<Duration>,
    content: Option<String>,
}

impl Default for Subrip {
    fn default() -> Self {
        Self {
            format: Format::SRT,
            index: 1,
            begin_time: None,
            duration: None,
            content: None,
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

    pub fn set_time(&mut self, begin_time: DateTime<Utc>, duration: Duration) {
        self.begin_time = Some(begin_time);
        self.duration = Some(duration);
    }
}
