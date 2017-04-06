extern crate serde_json;
extern crate serde_transcode;

use std::io;

fn main() {
    let mut deserializer = serde_json::Deserializer::from_reader(io::stdin());
    let mut serializer = serde_json::Serializer::new(io::stdout());

    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
    println!("");
}

