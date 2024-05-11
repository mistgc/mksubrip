use crate::{prelude::*, Subrip};

use reqwest::blocking::multipart;
use std::path;

const MODEL_URL: [&str; 1] = ["/model/whisper"];

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum AIModelType {
    OpenaiWhisper = 0,
}

#[repr(usize)]
pub enum AIModelScale {
    Tiny = 0,
    Base,
    Large,
    // Specified(String),
}

pub struct AiTranslator {
    pub model_type: AIModelType,
    pub scale: AIModelScale,
}

impl Default for AiTranslator {
    fn default() -> Self {
        Self {
            model_type: AIModelType::OpenaiWhisper,
            scale: AIModelScale::Base,
        }
    }
}

impl AiTranslator {
    pub fn model(mut self, model: AIModelType) -> Self {
        self.model_type = model;

        self
    }

    pub fn scale(mut self, scale: AIModelScale) -> Self {
        self.scale = scale;

        self
    }

    pub fn translate(&self, file_path: &path::Path) -> Vec<Shared<Subrip>> {
        if let Ok(str) = self.request(file_path) {
            self.string_to_subrips(&str)
        } else {
            vec![]
        }
    }

    pub fn get_scale_str(&self) -> &str {
        match self.scale {
            AIModelScale::Tiny => "tiny",
            AIModelScale::Base => "base",
            AIModelScale::Large => "large",
        }
    }

    pub fn get_url(&self) -> &str {
        MODEL_URL[self.model_type as usize]
    }

    pub fn request(&self, file_path: &path::Path) -> Result<String> {
        if !file_path.exists() {
            return Err(anyhow!("{} is invalid", file_path.to_str().unwrap_or("")));
        }
        let model_url = self.get_url();
        let url = format!("{model_url}/{}", self.get_scale_str());
        let multipart = multipart::Form::new().file("data", file_path)?;

        match request::post_with_multipart(url, multipart) {
            Ok(result) => Ok(result.text()?),
            Err(err) => Err(anyhow!(err.to_string())),
        }
    }

    pub fn string_to_subrips(&self, _str: &str) -> Vec<Shared<Subrip>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use nom::Slice;

    use super::*;

    #[test]
    fn test_request() {
        let translator = AiTranslator {
            model_type: AIModelType::OpenaiWhisper,
            scale: AIModelScale::Base,
        };

        let text = translator
            .request(path::Path::new("data/test.mp3"))
            .unwrap();

        assert_eq!(text.as_str().slice(0..6), "'data'");
    }
}
