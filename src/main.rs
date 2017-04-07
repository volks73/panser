extern crate serde;
extern crate serde_json;
extern crate serde_transcode;
extern crate rmp_serde;

use serde::ser::Serialize;
use std::error::Error;
use std::io::{self, Write};

// TODO: Create lib.rs and add Error enum type

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let value: serde_json::Value = serde_json::from_str(&input)?;
    let result = value.serialize(&mut rmp_serde::Serializer::new(io::stdout()));
    println!();
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e))
    }
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
            std::process::exit(0);
        },
        Err(e) => {
            writeln!(&mut std::io::stderr(), "{}", e).expect("Writing to STDERR");
            std::process::exit(1);
        }
    }
}

