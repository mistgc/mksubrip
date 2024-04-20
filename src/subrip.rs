use chrono::{Duration, NaiveTime};

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub enum SubripFormat {
    #[default]
    SRT,
}

#[derive(Debug, Default)]
pub struct Subrip {
    format: SubripFormat,
    index: u32,
    begin_time: NaiveTime,
    end_time: NaiveTime,
    content: String,
    is_loaded: bool,
}

impl Subrip {
    pub fn new(
        content: impl Into<String>,
        begin_time: impl Into<NaiveTime>,
        duration: Duration,
    ) -> Self {
        let begin_time = begin_time.into();
        Self {
            format: SubripFormat::SRT,
            index: 0,
            begin_time,
            end_time: begin_time + duration,
            content: content.into(),
            is_loaded: false,
        }
    }

    pub fn set_index(&mut self, index: u32) {
        self.index = index;
    }

    pub fn set_format(&mut self, format: SubripFormat) {
        self.format = format;
    }

    pub fn set_begin_time(&mut self, naive_time: &NaiveTime) {
        self.begin_time = *naive_time;
    }

    pub fn set_end_time(&mut self, naive_time: &NaiveTime) {
        self.end_time = *naive_time;
    }

    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn get_format(&self) -> SubripFormat {
        self.format
    }

    pub fn get_begin_time(&self) -> NaiveTime {
        self.begin_time
    }

    pub fn get_end_time(&self) -> NaiveTime {
        self.end_time
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn get_content_ref(&self) -> &String {
        &self.content
    }

    pub fn get_duration(&self) -> Duration {
        self.end_time - self.begin_time
    }

    pub fn add_begin_delta(&mut self, delta: f32) {
        self.begin_time += chrono::TimeDelta::seconds(delta as i64);
    }

    pub fn add_end_delta(&mut self, delta: f32) {
        self.end_time += chrono::TimeDelta::seconds(delta as i64);
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    pub fn set_loading(&mut self, flag: bool) {
        self.is_loaded = flag;
    }

    pub fn toggle_loading(&mut self) {
        self.is_loaded = !self.is_loaded;
    }
}
