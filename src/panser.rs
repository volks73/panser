// Copyright (c) 2017 Christopher R. Field. All rights reserved.

use bincode;
use envy;
use rmp_serde;
use serde_cbor;
use serde_json;
use serde_pickle;
use serde_urlencoded;
use serde_yaml;
use toml;

use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor, ErrorKind, Write};
use std::path::Path;
use std::str::{self, FromStr};
use std::sync::mpsc;
use std::thread;
use super::{Error, FromFormat, Result, ToFormat};

/// Convert the input in a format to the output format.
///
/// Ideally, the input would have a more generic `Read` or `BufRead` type. The input is of type
/// `&[u8]` as opposed to a more generic `Read` or `BufRead` type like the output is a generic
/// `Write` type because not all serialization libraries currently support the `from_reader` method
/// or a method that deserializes from a reader, but all currently supported serialization
/// libraries do support the `from_slice` or similar method for deserialization from a slice of
/// bytes. 
pub fn transcode<W: Write>(input: &[u8], output: &mut W, from: FromFormat, to: ToFormat, framed: bool, include_newline: bool) -> Result<()> {
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
    if include_newline {
        output.write(&[b'\n'])?;
    }
    output.flush()?;
    Ok(())
}

fn read<R: BufRead>(mut reader: R, framed: bool, message_tx: mpsc::Sender<Vec<u8>>) -> Result<()> {
    if framed {
        // If the input is framed, this means messages (input data) can be continuous read from
        // stdin or similar stream until End of File (EOF) is reached. Reaching EOF typically
        // occurs in a streaming capacity when stdout is closed by the proceeding application that
        // is piping (|) the output to the stdin of this application. Since the data is framed, the
        // application knows the number of bytes to read for each message without having to look
        // for a delimiting bytes, such as a newline (0x0A), or read the entire file to reach EOF
        // before transcoding. In other words, framed messages can be read as they stream into the
        // application.
        loop {
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
        }
    } else {
        // If the input is _not_ framed, the asumption is currently that the input is from a file,
        // either from a redirection (<) or as a first optional ARG from the command line. In the
        // case of a file, the entire contents should be read and then transcoded. This is not very
        // efficient, but unless framing is used or a delimiter is used within the file to separate
        // messages, this is the only universal way to transcode all serialization formats.
        let mut buf = Vec::new();
        let bytes_count = reader.read_to_end(&mut buf)?;
        if bytes_count > 0 {
            if !buf.is_empty() {
                message_tx.send(buf).unwrap();
            }
        } 
    }
    // TODO: Add delimited-based reading. The `read_until` function should be used instead of
    // framing or the `read_to_end` function. This would allow reading multiple messages from
    // a file or stream (stdin) without having to read the entire contents before transcoding,
    // similar to the frame reading.
    Ok(())
}

/// Create a producer-consumer architecture for reading and writing data. 
///
/// A separate thread is created and started for reading the input until until End-of-File (EOF) is
/// reached.
///
/// If input is `None`, then `stdin` is used for input. If output is `None`, then `stdout` is used
/// for output. If the input (from) format is `None`, then the format is determined from the file
/// extension if a file is provided and it has an extension. The default input format is JSON. If
/// the input format is not JSON and a file with an appropriate extension is _not_ used, then the
/// `from` parameter should not be `None`. A similar procedure is used for the output (to) format.
pub fn run(input: Option<&str>, output: Option<&str>, from: Option<FromFormat>, to: Option<ToFormat>, framed_input: bool, framed_output: bool, include_newline: bool) -> Result<()> {
    let (message_tx, message_rx) = mpsc::channel::<Vec<u8>>();
    // Use `BufRead` instead of `Read` to add additionally reading methods, like `read_until`.
    let reader: Box<BufRead + Send> = {
        if let Some(p) = input {
            Box::new(BufReader::new(File::open(p)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        }
    };
    let mut writer: Box<Write> = {
        if let Some(o) = output {
            Box::new(File::create(o)?)
        } else {
            Box::new(io::stdout())
        }
    };
    let handle = thread::spawn(move || {
        read(reader, framed_input, message_tx).or_else(|e| {
            match e {
                Error::Eof => {
                    Ok(())
                },
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
            framed_output,
            include_newline)?;
        } else {
            break;
        }
    }
    handle.join()?;
    Ok(())
}

