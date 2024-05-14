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
        match self.request(file_path) {
            Ok(str) => utils::json_str_to_subrips(&str).unwrap_or_default(),
            Err(err) => {
                error!("{}", err.to_string());
                vec![]
            }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_str_to_subrips() {
        let text = r#"{
    "data": [{
        "index": "1",
        "start": "00:05.001",
        "end": "00:10.001",
        "text": "Hello, World!"
    }]
}
"#;

        let subrips = utils::json_str_to_subrips(text).unwrap();

        assert_eq!(subrips[0].borrow().get_index(), 1);
    }
}
