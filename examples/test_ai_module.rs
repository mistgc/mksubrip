use mksubrip::ai::*;
use mksubrip::prelude::*;

use std::path;

fn main() {
    let translator = AiTranslator {
        model_type: AIModelType::OpenaiWhisper,
        scale: AIModelScale::Base,
    };

    let text = translator
        .request(path::Path::new("data/test.mp3"))
        .unwrap_or_else(|err| {
            println!("{}", err);
            unreachable!()
        });
    println!("text: {}", text);

    assert_eq!(&text[0..7], "{\"data\"");

    let result = utils::json_str_to_subrips(text.as_str()).unwrap_or_else(|err| {
        println!("{}", err);
        unreachable!()
    });

    assert_eq!(result[0].borrow().get_index(), 1);
}
