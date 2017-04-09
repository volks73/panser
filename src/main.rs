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

fn transcode(input: &str, output: Option<&str>) -> Result<()> {
    let out: Box<Write> = output.map_or(Box::new(io::stdout()), |o| {
        Box::new(File::create(o).unwrap())
    });
    let value: serde_json::Value = serde_json::from_str(input)?;
    value.serialize(&mut rmp_serde::Serializer::new(out))?;
    Ok(())
}

fn run(input: Option<&str>, output: Option<&str>, suppress_newline: bool) -> Result<()> {
    if let Some(i) = input {
        run_file(i, output)
    } else {
        run_stdin(output, suppress_newline)
    }
}

fn run_file(input: &str, output: Option<&str>) -> Result<()> {
    let file = File::open(input)?;
    let mut buf_reader = BufReader::new(file);
    let mut buf = String::new();
    buf_reader.read_to_string(&mut buf)?;
    transcode(&buf, output)?;
    Ok(())
}

fn run_stdin(output: Option<&str>, suppress_newline: bool) -> Result<()> {
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
            transcode(&input, output)?;
            if suppress_newline {
                io::stdout().flush()?;
            } else {
                println!();
            }
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
        .arg(Arg::with_name("output")
             .help("A file to write the output instead of writing to STDOUT. If a file extension exists, then it is used to determined the format of the output serialized data. If a file extension does not exist, then the `-t,--to` opton should be used or MessagePack is assumed.")
             .long("output")
             .short("o")
             .takes_value(true))
        .arg(Arg::with_name("suppress-newline")
             .help("Suppresses writing a newline character (0x0A) at the end of the output. By default, a newline character is appended to the output written to STDOUT. This can be a problem in a some instances when piping binary data to other commands or applications.")
             .long("suppress-newline")
             .short("n"))
        .get_matches();
    let result = run(matches.value_of("FILE"), matches.value_of("output"), matches.is_present("suppress-newline"));
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

