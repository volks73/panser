// Copyright (C) 2017 Christopher R. Field. All rights reserved.

#[macro_use]
extern crate clap;
extern crate bincode;
extern crate byteorder;
extern crate envy;
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
use std::fs::File;
use std::io::{self, Cursor, ErrorKind, Read, Write};
use std::path::Path;
use std::str::{self, FromStr};
use std::sync::mpsc;
use std::thread;

fn transcode<W: Write>(input: &[u8], output: &mut W, from: FromFormat, to: ToFormat, framed: bool) -> Result<()> {
    let value = {
        match from {
            FromFormat::Bincode => bincode::deserialize::<serde_json::Value>(input)?,
            FromFormat::Cbor => serde_cbor::from_slice::<serde_json::Value>(input)?,
            FromFormat::Envy => envy::from_env::<serde_json::Value>()?,
            // Until the Hjson create is updated to work with serde v0.9 or newer, just use the
            // serde_json crate.
            FromFormat::Hjson => serde_json::from_slice::<serde_json::Value>(input)?,
            FromFormat::Json => serde_json::from_slice::<serde_json::Value>(input)?,
            FromFormat::Msgpack => rmp_serde::from_slice::<serde_json::Value>(input)?,
            FromFormat::Pickle => serde_pickle::from_slice::<serde_json::Value>(input)?,
            FromFormat::Toml => toml::from_slice::<serde_json::Value>(input)?,
            FromFormat::Url => serde_urlencoded::from_bytes::<serde_json::Value>(input)?,
            FromFormat::Yaml => serde_yaml::from_slice::<serde_json::Value>(input)?,
        }
    };
    let encoded_data = { 
        match to {
            ToFormat::Bincode => bincode::serialize(&value, bincode::Infinite)?,
            ToFormat::Cbor => serde_cbor::to_vec(&value)?,
            // Until the Hjson crate is updated to work with serde v0.9 or newer, use the
            // serde-json crate's pretty print for the Hjson format.
            ToFormat::Hjson => serde_json::to_vec_pretty(&value)?, 
            ToFormat::Json => serde_json::to_vec(&value)?,
            ToFormat::Msgpack => rmp_serde::to_vec(&value)?,
            ToFormat::Pickle => serde_pickle::to_vec(&value, true)?,
            ToFormat::Toml => toml::to_vec(&value)?,
            ToFormat::Url => serde_urlencoded::to_string(&value)?.into_bytes(),
            ToFormat::Yaml => serde_yaml::to_vec(&value)?,
        }
    };
    if framed {
        let mut frame_length = [0; 4];
        BigEndian::write_u32(&mut frame_length, encoded_data.len() as u32);
        output.write(&frame_length)?;
    }
    output.write(&encoded_data)?;
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

fn run(input: Option<&str>, output: Option<&str>, from: Option<FromFormat>, to: Option<ToFormat>, framed_input: bool, framed_output: bool) -> Result<()> {
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
        if let Ok(message) = message_rx.recv() {
            transcode(&message, &mut writer, from.unwrap_or({
                if let Some(i) = input {
                    if let Some(e) = Path::new(i).extension() {
                        FromFormat::from_str(
                            e.to_str().unwrap_or("json")
                        ).unwrap_or(FromFormat::Json)
                    } else {
                        FromFormat::Json
                    }
                } else {
                    FromFormat::Json
                }
            }),
            to.unwrap_or({
                if let Some(o) = output {
                    if let Some(e) = Path::new(o).extension() {
                        ToFormat::from_str(
                            e.to_str().unwrap_or("msgpack")
                        ).unwrap_or(ToFormat::Msgpack)
                    } else {
                        ToFormat::Msgpack
                    }
                } else {
                    ToFormat::Msgpack
                }
            }), 
            framed_output)?;
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
            .help("The input format. [values: Bincode, CBOR, Envy, Hjson, JSON, Msgpack, Pickle, TOML, URL, XML, YAML]")
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
            .help("The output format. [values: Bincode, CBOR, Hjson, JSON, Msgpack, Pickle, TOML, URL, YAML]")
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
        value_t!(matches, "from", FromFormat).ok(),
        value_t!(matches, "to", ToFormat).ok(),
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

