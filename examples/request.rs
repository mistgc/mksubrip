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

    let result = translator.json_str_to_subrips(text.as_str());

    match result {
        Ok(subrips) => println!("{}", subrips[0].borrow().get_index()),
        Err(err) => println!("{}", err),
    }
}
