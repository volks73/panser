// Copyright (C) 2017 Christopher R. Field. All rights reserved.

#[macro_use]
extern crate clap;
extern crate byteorder;
extern crate panser;
extern crate serde;
extern crate serde_json;
extern crate serde_transcode;
extern crate rmp_serde;

use byteorder::{ByteOrder, BigEndian};
use clap::{App, Arg};
use panser::Result;
use serde::ser::Serialize;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::str;
use std::sync::mpsc;
use std::thread;

fn transcode(input: &[u8], output: Option<&str>, framed: bool) -> Result<()> {
    let mut out: Box<Write> = output.map_or(Box::new(io::stdout()), |o| {
        Box::new(File::create(o).unwrap())
    });
    let value: serde_json::Value = serde_json::from_slice(input)?;
    let mut buf = Vec::new();
    value.serialize(&mut rmp_serde::Serializer::new(&mut buf))?;
    if framed {
        let mut frame_length = [0; 4];
        BigEndian::write_u32(&mut frame_length, buf.len() as u32);
        out.write(&frame_length)?;
    }
    out.write(&buf)?;
    out.flush()?;
    Ok(())
}

fn run(input: Option<&str>, output: Option<&str>, suppress_newline: bool, framed_input: bool, framed_output: bool) -> Result<()> {
    if let Some(i) = input {
        run_file(i, output, framed_input, framed_output)
    } else {
        run_stdin(output, suppress_newline, framed_output)
    }
}

fn run_file(input: &str, output: Option<&str>, framed_input: bool, framed_output: bool) -> Result<()> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    if framed_input {
        let mut frame_length = [0; 4];
        reader.read_exact(&mut frame_length)?;
    }
    reader.read_to_end(&mut buf)?;
    transcode(&buf, output, framed_output)?;
    Ok(())
}

fn run_stdin(output: Option<&str>, suppress_newline: bool, framed_output: bool) -> Result<()> {
    // Reading from STDIN should be conducted on a separate thread since it is blocking.
    let (message_tx, message_rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            // TODO: Add reading input when the `--framed-input` flag is specified.
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
            transcode(input.as_bytes(), output, framed_output)?;
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
    // TODO: Add `-f,--from` option
    // TODO: Add `-t,--to` option
    // TODO: Add support for other formats
    let matches = App::new("panser")
        .version(crate_version!())
        .author("Christopher R. Field <cfield2@gmail.com>")
        .about("An application for transcoding serialization formats.") 
        .arg(Arg::with_name("FILE")
             .help("A file to read as input instead of reading from STDIN. If a file extension exists, then it is used to determine the format of the serialized data contained within the file. If a file extension does not exist, then the '-f,--from' option should be used or JSON is assumed.")
             .index(1))
        .arg(Arg::with_name("framed-input")
             .help("Indicates the first four bytes of the input is an unsigned 32-bit integer in Big Endian (Network Order) indicating the total length of the serialized data.")
             .long("framed-input"))
        .arg(Arg::with_name("framed-output")
             .help("Prepends the total length of the serialized data as an unsigned 32-bit integer in Big Endian (Network Order).")
             .long("framed-output"))
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
    let result = run(
        matches.value_of("FILE"), 
        matches.value_of("output"), 
        matches.is_present("suppress-newline"), 
        matches.is_present("framed-input"),
        matches.is_present("framed-output")
    );
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

