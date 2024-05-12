use nom::Slice;

fn main() {
    let json_str = r#"
{"data": [
    {"index": 1, "start": "00:05.001", "end": "00:10.001", "text": "Hello, World!"}
]}"#;
    println!("{}", json_str);
    let value: serde_json::Value = serde_json::from_str(json_str).unwrap();

    println!("{}", value["data"][0]["start"].as_str().unwrap());
    let start = value["data"][0]["start"].as_str().unwrap().to_string();
    let min = start.as_str().slice(0..2).parse::<u32>().unwrap();
    let sec = start.as_str().slice(3..5).parse::<u32>().unwrap();
    let nano = start.as_str().slice(6..9).parse::<u32>().unwrap();

    let time =
        chrono::NaiveTime::from_num_seconds_from_midnight_opt(min * 60 + sec, nano * 10E5 as u32)
            .unwrap();
    println!("{:?}", time);
}
