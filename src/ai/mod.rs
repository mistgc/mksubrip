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
            self.json_str_to_subrips(&str).unwrap_or_default()
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

    pub fn json_str_to_subrips(&self, json_str: &str) -> Result<Vec<Shared<Subrip>>> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        if let Some(data) = value["data"].as_array() {
            let mut result = vec![];
            for subrip_json in data.iter() {
                let index = subrip_json["index"].as_str().unwrap();
                let start = subrip_json["start"].as_str().unwrap();
                let end = subrip_json["end"].as_str().unwrap();
                let text = subrip_json["text"].as_str().unwrap();
                let subrip = Subrip::from_vec_str([&index, &start, &end, &text])?;

                result.push(Shared::new(subrip));
            }

            Ok(result)
        } else {
            error!("json_str is invalid...");

            Err(anyhow!("json_str is invalid..."))
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::Slice;

    use super::*;

    #[test]
    fn test_string_to_subrips() {
        let translator = AiTranslator {
            model_type: AIModelType::OpenaiWhisper,
            scale: AIModelScale::Base,
        };

        let text = translator
            .request(path::Path::new("data/test.mp3"))
            .unwrap();

        assert_eq!(text.as_str().slice(0..7), "{\"data\"");

        let subrips = translator.json_str_to_subrips(text.as_str()).unwrap();

        assert_eq!(subrips[0].borrow().get_index(), 1);
    }
}
