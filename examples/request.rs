use mksubrip::ai::*;

use std::path;

fn main() {
    let translator = AiTranslator {
        model_type: AIModelType::OpenaiWhisper,
        scale: AIModelScale::Base,
    };

    let text = translator
        .request(path::Path::new("data/test.mp3"))
        .unwrap();

    println!("{text}");
}
