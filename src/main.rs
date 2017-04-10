// Copyright (C) 2017 Christopher R. Field. All rights reserved.

#[macro_use]
extern crate clap;
extern crate byteorder;
extern crate panser;
extern crate serde;
extern crate serde_json;
extern crate rmp_serde;

use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use clap::{App, Arg};
use panser::{Error, Result};
use serde::ser::Serialize;
use std::fs::File;
use std::io::{self, Cursor, ErrorKind, Read, Write};
use std::str::{self, FromStr};
use std::sync::mpsc;
use std::thread;

enum ToFormat {
    Bincode,
    Bson,
    Cbor,
    Hjson,
    Json,
    Msgpack,
    Pickle,
    Toml,
    Url,
    Yaml,
}

impl FromStr for ToFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Bincode" => Ok(ToFormat::Bincode),
            "bincode" => Ok(ToFormat::Bincode),
            "BINCODE" => Ok(ToFormat::Bincode),
            "Bson" => Ok(ToFormat::Bson),
            "bson" => Ok(ToFormat::Bson),
            "BSON" => Ok(ToFormat::Bson),
            "Cbor" => Ok(ToFormat::Cbor),
            "cbor" => Ok(ToFormat::Cbor),
            "CBOR" => Ok(ToFormat::Cbor),
            "Hjson" => Ok(ToFormat::Hjson),
            "hjson" => Ok(ToFormat::Hjson),
            "HJSON" => Ok(ToFormat::Hjson),
            "Json" => Ok(ToFormat::Json),
            "json" => Ok(ToFormat::Json),
            "JSON" => Ok(ToFormat::Json),
            "Msgpack" => Ok(ToFormat::Msgpack),
            "msgpack" => Ok(ToFormat::Msgpack),
            "MSGPACK" => Ok(ToFormat::Msgpack),
            "MessagePack" => Ok(ToFormat::Msgpack),
            "Pickle" => Ok(ToFormat::Pickle),
            "pickle" => Ok(ToFormat::Pickle),
            "PICKLE" => Ok(ToFormat::Pickle),
            "Toml" => Ok(ToFormat::Toml),
            "toml" => Ok(ToFormat::Toml),
            "TOML" => Ok(ToFormat::Toml),
            "Url" => Ok(ToFormat::Url),
            "url" => Ok(ToFormat::Url),
            "URL" => Ok(ToFormat::Url),
            "Yaml" => Ok(ToFormat::Yaml),
            "yaml" => Ok(ToFormat::Yaml),
            "YAML" => Ok(ToFormat::Yaml),
            _ => Err("No Match")
        }
    }
}

enum FromFormat {
    Bincode,
    Bson,
    Cbor,
    Envy,
    Hjson,
    Json,
    Msgpack,
    Pickle,
    Redis,
    Toml,
    Url,
    Xml,
    Yaml,
}

impl FromStr for FromFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Bincode" => Ok(FromFormat::Bincode),
            "bincode" => Ok(FromFormat::Bincode),
            "BINCODE" => Ok(FromFormat::Bincode),
            "Bson" => Ok(FromFormat::Bson),
            "bson" => Ok(FromFormat::Bson),
            "BSON" => Ok(FromFormat::Bson),
            "Cbor" => Ok(FromFormat::Cbor),
            "cbor" => Ok(FromFormat::Cbor),
            "CBOR" => Ok(FromFormat::Cbor),
            "Envy" => Ok(FromFormat::Envy),
            "envy" => Ok(FromFormat::Envy),
            "ENVY" => Ok(FromFormat::Envy),
            "Hjson" => Ok(FromFormat::Hjson),
            "hjson" => Ok(FromFormat::Hjson),
            "HJSON" => Ok(FromFormat::Hjson),
            "Json" => Ok(FromFormat::Json),
            "json" => Ok(FromFormat::Json),
            "JSON" => Ok(FromFormat::Json),
            "Msgpack" => Ok(FromFormat::Msgpack),
            "msgpack" => Ok(FromFormat::Msgpack),
            "MSGPACK" => Ok(FromFormat::Msgpack),
            "MessagePack" => Ok(FromFormat::Msgpack),
            "Pickle" => Ok(FromFormat::Pickle),
            "pickle" => Ok(FromFormat::Pickle),
            "PICKLE" => Ok(FromFormat::Pickle),
            "Redis" => Ok(FromFormat::Redis),
            "redis" => Ok(FromFormat::Redis),
            "REDIS" => Ok(FromFormat::Redis),
            "Toml" => Ok(FromFormat::Toml),
            "toml" => Ok(FromFormat::Toml),
            "TOML" => Ok(FromFormat::Toml),
            "Url" => Ok(FromFormat::Url),
            "url" => Ok(FromFormat::Url),
            "URL" => Ok(FromFormat::Url),
            "Xml" => Ok(FromFormat::Xml),
            "xml" => Ok(FromFormat::Xml),
            "XML" => Ok(FromFormat::Xml),
            "Yaml" => Ok(FromFormat::Yaml),
            "yaml" => Ok(FromFormat::Yaml),
            "YAML" => Ok(FromFormat::Yaml),
            _ => Err("No Match")
        }
    }
}


fn transcode<W: Write>(input: &[u8], output: &mut W, framed: bool) -> Result<()> {
    let value: serde_json::Value = serde_json::from_slice(input)?;
    let mut buf = Vec::new();
    value.serialize(&mut rmp_serde::Serializer::new(&mut buf))?;
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
            let mut buf = Vec::with_capacity(frame_length as usize);
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
                buf.pop(); // Remove trailing newline (0x0A) character
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

fn run(input: Option<&str>, output: Option<&str>, framed_input: bool, framed_output: bool) -> Result<()> {
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
        read(reader, framed_input, message_tx).map_err(|e| {
            match e {
                Error::Eof => Ok(()),
                _ => Err(e),
            }
        }).unwrap();
    });
    loop {
        if let Ok(input) = message_rx.recv() {
            transcode(&input, &mut writer, framed_output)?;
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
        .arg(Arg::with_name("from")
            .help("The input format.")
            .long("from")
            .short("f")
            .possible_values(&["Bincode", "BSON", "CBOR", "Envy", "Hjson", "JSON", "Msgpack", "Pickle", "Redis", "TOML", "URL", "XML", "YAML"])
            .default_value("JSON")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .help("A file to write the output instead of writing to STDOUT. If a file extension exists, then it is used to determined the format of the output serialized data. If a file extension does not exist, then the `-t,--to` option should be used or the MessagePack format is assumed.")
            .long("output")
            .short("o")
            .takes_value(true))
        .arg(Arg::with_name("to")
            .help("The output format.")
            .long("to")
            .short("t")
            .possible_values(&["Bincode", "BSON", "CBOR", "Hjson", "JSON", "Msgpack", "Pickle", "TOML", "URL", "YAML"])
            .default_value("Msgpack")
            .takes_value(true))
        .get_matches();
    let result = run(
        matches.value_of("FILE"), 
        matches.value_of("output"), 
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

