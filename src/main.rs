extern crate serde_json;
extern crate serde_transcode;

use std::io;

fn main() {
    let input = r#"
    {
        "a boolean": true,
        "an array": [3, 2, 1]
    }"#;
    let iter = input.bytes().map(Ok);
    let mut deserializer = serde_json::Deserializer::from_iter(iter);
    let mut serializer = serde_json::Serializer::new(io::stdout());

    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
}

