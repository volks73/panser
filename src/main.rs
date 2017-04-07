extern crate serde;
extern crate serde_json;
extern crate serde_transcode;
extern crate rmp_serde;

use serde::ser::Serialize;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    //let input = "{\"bool\":true}";
    //let iter = input.bytes().map(Ok);
    //let mut deserializer = serde_json::Deserializer::from_reader(io::stdin());
    //let mut deserializer = serde_json::Deserializer::from_iter(iter);
    io::stdin().read_line(&mut input).unwrap();
    let value: serde_json::Value = serde_json::from_str(&input).unwrap();
    let result = value.serialize(&mut rmp_serde::Serializer::new(io::stdout()));
    println!();
    match result {
        Ok(_) => {
            std::process::exit(0);
        },
        Err(e) => {
            writeln!(&mut std::io::stderr(), "{}", e).expect("Writing to STDERR");
            std::process::exit(1);
        }
    }
}

