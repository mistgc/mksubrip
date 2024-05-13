use crate::prelude::*;

use chrono::{Duration, NaiveTime};

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub enum SubripFormat {
    #[default]
    SRT,
}

#[derive(Debug, Clone, Default)]
pub struct SubripState {
    pub is_loaded: bool,
    pub is_deleted: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Subrip {
    pub state: SubripState,

    pub format: SubripFormat,
    pub index: u32,
    pub begin_time: NaiveTime,
    pub end_time: NaiveTime,
    pub content: String,
}

impl Subrip {
    pub fn new(
        content: impl Into<String>,
        begin_time: impl Into<NaiveTime>,
        duration: Duration,
    ) -> Self {
        let begin_time = begin_time.into();
        Self {
            state: SubripState::default(),
            format: SubripFormat::SRT,
            index: 0,
            begin_time,
            end_time: begin_time + duration,
            content: content.into(),
        }
    }

    pub fn from_vec_str(str: [&str; 4]) -> Result<Self> {
        let index = str[0].parse::<u32>()?;
        let start = utils::naive_time_from_str(str[1])?;
        let end = utils::naive_time_from_str(str[2])?;
        let text = str[3];

        Ok(Self {
            format: SubripFormat::SRT,
            index,
            begin_time: start,
            end_time: end,
            content: text.to_string(),
            state: SubripState::default(),
        })
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
        self.state.is_loaded
    }

    pub fn set_loading(&mut self, flag: bool) {
        self.state.is_loaded = flag;
    }

    pub fn delete(&mut self) {
        self.state.is_deleted = true;
    }

    pub fn is_deleted(&self) -> bool {
        self.state.is_deleted
    }

    pub fn toggle_loading(&mut self) {
        self.state.is_loaded = !self.state.is_loaded;
    }
}

impl PartialEq for Subrip {
    fn eq(&self, other: &Self) -> bool {
        self.content.as_str() == other.content.as_str()
            && self.index == other.index
            && self.begin_time == other.begin_time
            && self.end_time == other.end_time
            && self.format == other.format
            && self.state == other.state
    }
}

impl PartialEq for SubripState {
    fn eq(&self, other: &Self) -> bool {
        self.is_deleted == other.is_deleted && self.is_loaded == other.is_loaded
    }
}
