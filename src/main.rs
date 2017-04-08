extern crate panser;
extern crate serde;
extern crate serde_json;
extern crate serde_transcode;
extern crate rmp_serde;

use panser::Result;
use serde::ser::Serialize;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;

fn run() -> Result<()> {
    let (message_tx, message_rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            // TODO: Add better error handling
            io::stdin().read_line(&mut input).unwrap();
            if input.is_empty() {
                break;
            } else {
                message_tx.send(input).unwrap();
            }
        }
    });
    loop {
        if let Ok(input) = message_rx.recv() {
            let value: serde_json::Value = serde_json::from_str(&input)?;
            value.serialize(&mut rmp_serde::Serializer::new(io::stdout()))?;
            // The println serves two purposes, adds a new for the next input since the serialize
            // to STDOUT does not include a newline character at the end and it causes the flushing
            // of the STDOUT buffer so that the output is actually written to STDOUT.
            println!();
        } else {
            break;
        }
    }
    Ok(())
}

fn main() {
    // TODO: Add optional argument for input file. If not specified, then input is read from STDIN
    // TODO: Add determining `from` format from file extension if present for input
    // TODO; Add determining `to` format from file extension if present for output
    // TODO: Add `--framed-input` flag, which indicates the input has a prepended message length as
    // a 32-bit integer
    // TODO: Add `--framed-output` flag, which prepents the length of the serialized data as
    // a 32-bit signed integer
    // TODO: Add `-f,--from` option
    // TODO: Add `-t,--to` option
    // TODO: Add `-o,--output` option. If not specified, output is written to STDOUT
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

