// Copyright (C) 2017 Christopher R. Field. All rights reserved.

#[macro_use]
extern crate clap;
extern crate panser;
extern crate serde;
extern crate serde_json;
extern crate serde_transcode;
extern crate rmp_serde;

use clap::{App, Arg};
use panser::Result;
use serde::ser::Serialize;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::sync::mpsc;
use std::thread;

fn transcode(input: &str) -> Result<()> {
    let value: serde_json::Value = serde_json::from_str(input)?;
    value.serialize(&mut rmp_serde::Serializer::new(io::stdout()))?;
    Ok(())
}

fn run(input: Option<&str>) -> Result<()> {
    if let Some(i) = input {
        run_file(i)
    } else {
        run_stdin()
    }
}

fn run_file(input: &str) -> Result<()> {
    let file = File::open(input)?;
    let mut buf_reader = BufReader::new(file);
    let mut buf = String::new();
    buf_reader.read_to_string(&mut buf)?;
    transcode(&buf)?;
    Ok(())
}

fn run_stdin() -> Result<()> {
    // Reading from STDIN should be conducted on a separate thread since it is blocking.
    let (message_tx, message_rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            let bytes_count = io::stdin().read_line(&mut buf).unwrap();
            if bytes_count > 0 {
                buf.pop(); // Remove trailing newline character (0xA)
                if !buf.is_empty() {
                    message_tx.send(buf).unwrap();
                }
            } else {
                // EOF reached
                break;
            }
        }
    });
    loop {
        if let Ok(input) = message_rx.recv() {
            transcode(&input)?;
            println!();
        } else {
            break;
        }
    }
    Ok(())
}

fn main() {
    // TODO: Add determining `from` format from file extension if present for input
    // TODO; Add determining `to` format from file extension if present for output
    // TODO: Add `--framed-input` flag, which indicates the input has a prepended message length as
    // a 32-bit integer
    // TODO: Add `--framed-output` flag, which prepents the length of the serialized data as
    // a 32-bit signed integer
    // TODO: Add `-f,--from` option
    // TODO: Add `-t,--to` option
    // TODO: Add `-o,--output` option. If not specified, output is written to STDOUT
    let matches = App::new("panser")
        .version(crate_version!())
        .author("Christopher R. Field <cfield2@gmail.com>")
        .about("An application for transcoding serialization formats.") 
        .arg(Arg::with_name("FILE")
             .help("A file to read as input instead of reading from STDIN. If a file extension exists, then it is used to determine the format of the serialized data contained within the file. If a file extension does not exist, then the '-f,--from' option should be used or JSON is assumed.")
             .index(1))
        .get_matches();
    let result = run(matches.value_of("FILE"));
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

