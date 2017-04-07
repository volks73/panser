extern crate panser;
extern crate serde;
extern crate serde_json;
extern crate serde_transcode;
extern crate rmp_serde;

use panser::{Error, Ok, Result};
use serde::ser::Serialize;
use std::io::{self, Write};

fn run() -> Result<Ok> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let value: serde_json::Value = serde_json::from_str(&input)?;
    value.serialize(&mut rmp_serde::Serializer::new(io::stdout())).map_err(|e| Error::MsgpackEncode(e))
}

fn main() {
    // TODO: Mask EOF error 
    // TODO: Change to continuously reading STDIN, see wsta project
    // TODO: Add `-f,--from` option
    // TODO: Add `-t,--to` option
    // TODO: Add `-o,--output` option
    let result = run();
    match result {
        Ok(_) => {
            println!(); // Print a newline so the prompt appears below the output
            std::process::exit(0);
        },
        Err(e) => {
            writeln!(&mut std::io::stderr(), "{}", e).expect("Writing to STDERR");
            std::process::exit(1);
        }
    }
}

