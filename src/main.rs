// Copyright (C) 2017 Christopher R. Field. All rights reserved.

#[macro_use]
extern crate clap;
extern crate bincode;
extern crate byteorder;
extern crate panser;
extern crate rmp_serde;
extern crate serde;
extern crate serde_cbor;
extern crate serde_hjson;
extern crate serde_json;
extern crate serde_pickle;
extern crate serde_urlencoded;
extern crate serde_yaml;
extern crate toml;

use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use clap::{App, Arg};
use panser::{Error, FromFormat, Result, ToFormat};
use serde::Serialize;
use std::fs::File;
use std::io::{self, Cursor, ErrorKind, Read, Write};
use std::str;
use std::sync::mpsc;
use std::thread;

fn transcode<W: Write>(input: &[u8], output: &mut W, from: FromFormat, to: ToFormat, framed: bool) -> Result<()> {
    let value = {
        match from {
            FromFormat::Bincode => unimplemented!(),
            FromFormat::Bson => unimplemented!(),
            FromFormat::Cbor => serde_cbor::from_slice::<serde_json::Value>(input)?,
            FromFormat::Envy => unimplemented!(),
            FromFormat::Hjson => unimplemented!(),
            FromFormat::Json => serde_json::from_slice::<serde_json::Value>(input)?,
            FromFormat::Msgpack => rmp_serde::from_slice::<serde_json::Value>(input)?,
            FromFormat::Pickle => serde_pickle::from_slice::<serde_json::Value>(input)?,
            FromFormat::Redis => unimplemented!(),
            FromFormat::Toml => toml::from_slice::<serde_json::Value>(input)?,
            FromFormat::Url => serde_urlencoded::from_bytes::<serde_json::Value>(input)?,
            FromFormat::Xml => unimplemented!(),
            FromFormat::Yaml => serde_yaml::from_slice::<serde_json::Value>(input)?,
        }
    };
    let mut buf = Vec::new();
    match to {
        ToFormat::Bincode => value.serialize(&mut bincode::Serializer::new(&mut buf))?,
        ToFormat::Bson => unimplemented!(), 
        ToFormat::Cbor => {
            buf = serde_cbor::to_vec(&value)?;
        },
        ToFormat::Hjson => unimplemented!(),
        ToFormat::Json => value.serialize(&mut serde_json::Serializer::new(&mut buf))?,
        ToFormat::Msgpack => value.serialize(&mut rmp_serde::Serializer::new(&mut buf))?,
        ToFormat::Pickle => value.serialize(&mut serde_pickle::Serializer::new(&mut buf, true))?,
        ToFormat::Toml => {
            buf = toml::to_vec(&value)?;
        },
        ToFormat::Url => {
            buf = serde_urlencoded::to_string(&value)?.into_bytes();
        },
        ToFormat::Yaml => {
            buf = serde_yaml::to_vec(&value)?;
        },
    }
    if framed {
        let mut frame_length = [0; 4];
        BigEndian::write_u32(&mut frame_length, buf.len() as u32);
        output.write(&frame_length)?;
    }
    output.write(&buf)?;
    output.flush()?;
    Ok(())
}

fn read<R: Read + Send>(mut reader: R, framed: bool, message_tx: mpsc::Sender<Vec<u8>>) -> Result<()> {
    loop {
        if framed {
            let mut frame_length_buf = [0; 4];
            reader.read_exact(&mut frame_length_buf).map_err(|e| {
                match e.kind() {
                    ErrorKind::UnexpectedEof => Error::Eof,
                    _ => Error::Io(e)
                }
            })?;
            let mut frame_length_cursor = Cursor::new(frame_length_buf);
            let frame_length = frame_length_cursor.read_u32::<BigEndian>()?;
            let mut buf = vec![0; frame_length as usize];
            reader.read_exact(&mut buf).map_err(|e| {
                match e.kind() {
                    ErrorKind::UnexpectedEof => Error::Eof,
                    _ => Error::Io(e)
                }
            })?;
            message_tx.send(buf).unwrap();
        } else {
            let mut buf = Vec::new();
            let bytes_count = reader.read_to_end(&mut buf)?;
            if bytes_count > 0 {
                if !buf.is_empty() {
                    message_tx.send(buf).unwrap();
                }
            } else {
                // EOF
                break;
            }
        }
    }
    Ok(())
}

fn run(input: Option<&str>, output: Option<&str>, from: FromFormat, to: ToFormat, framed_input: bool, framed_output: bool) -> Result<()> {
    let (message_tx, message_rx) = mpsc::channel::<Vec<u8>>();
    let mut writer: Box<Write> = {
        if let Some(o) = output {
            Box::new(File::create(o)?)
        } else {
            Box::new(io::stdout())
        }
    };
    let reader: Box<Read + Send> = {
        if let Some(i) = input {
            Box::new(File::open(i)?)
        } else {
            Box::new(io::stdin())
        }
    };
    let handle = thread::spawn(move || {
        read(reader, framed_input, message_tx).or_else(|e| {
            match e {
                Error::Eof => Ok(()),
                _ => Err(e),
            }
        }).unwrap();
    });
    loop {
        if let Ok(input) = message_rx.recv() {
            transcode(&input, &mut writer, from, to, framed_output)?;
            writer.flush()?;
        } else {
            break;
        }
    }
    handle.join()?;
    Ok(())
}

fn main() {
    // TODO: Add interactive (-i) mode, maybe.
    // TODO: Add determining `from` format from file extension if present for input
    // TODO; Add determining `to` format from file extension if present for output
    // TODO: Add support for other formats
    let matches = App::new("panser")
        .version(crate_version!())
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
        .arg(Arg::with_name("from")
            .help("The input format. [values: Bincode, BSON, CBOR, Envy, Hjson, JSON, Msgpack, Pickle, Redis, TOML, URL, XML, YAML]")
            .long("from")
            .short("f")
            .hide_possible_values(true)
            .possible_values(&FromFormat::possible_values())
            .default_value("JSON")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .help("A file to write the output instead of writing to STDOUT. If a file extension exists, then it is used to determined the format of the output serialized data. If a file extension does not exist, then the `-t,--to` option should be used or the MessagePack format is assumed.")
            .long("output")
            .short("o")
            .takes_value(true))
        .arg(Arg::with_name("to")
            .help("The output format. [values: Bincode, BSON, CBOR, Hjson, JSON, Msgpack, Pickle, TOML, URL, YAML]")
            .long("to")
            .short("t")
            .hide_possible_values(true)
            .possible_values(&ToFormat::possible_values())
            .default_value("Msgpack")
            .takes_value(true))
        .get_matches();
    let result = run(
        matches.value_of("FILE"), 
        matches.value_of("output"), 
        value_t!(matches, "from", FromFormat).unwrap_or(FromFormat::Json),
        value_t!(matches, "to", ToFormat).unwrap_or(ToFormat::Msgpack),
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

