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
use super::{Error, Framing, FromFormat, Result, ToFormat};

/// Convert the input in a format to the output format.
///
/// Ideally, the input would have a more generic `Read` or `BufRead` type. The input is of type
/// `&[u8]` as opposed to a more generic `Read` or `BufRead` type like the output is a generic
/// `Write` type because not all serialization libraries currently support the `from_reader` method
/// or a method that deserializes from a reader, but all currently supported serialization
/// libraries do support the `from_slice` or similar method for deserialization from a slice of
/// bytes. 
pub fn transcode<W: Write>(input: &[u8], output: &mut W, from: FromFormat, to: ToFormat, framing: Option<Framing>) -> Result<()> {
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
    if let Some(f) = framing {
        match f {
            Framing::Sized => {
                let mut frame_length = [0; 4];
                BigEndian::write_u32(&mut frame_length, encoded_data.len() as u32);
                output.write(&frame_length)?;
            },
            _ => {},
        }
    }
    output.write(&encoded_data)?;
    if let Some(f) = framing {
        match f {
            Framing::Delimited(delimiter) => {
                output.write(&[delimiter; 1])?;
            },
            _ => {},
        }
    }
    output.flush()?;
    Ok(())
}

/// Reads exact length of bytes. 
///
/// This assumes the first four bytes of a message are the total data
/// length encoded as an unsigned 32-bit integer in Big Endian (Network Order). Reading continues
/// until the End-of-File (EOF) is reached.
///
/// Since the data is framed, the application can read messages as they as they are "streamed" into
/// the reader without having to read the entire stream or file into memory. Messages can be
/// transcoded as they arrive and continuous written to output.
fn read_exact<R: BufRead>(mut reader: R, message_tx: mpsc::Sender<Vec<u8>>) -> Result<()> {
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
}

/// Reads until a delimiter is reached.
///
/// Reading continues until the End-of-File (EOF) is reached.
///
/// Since the data is framed, the application can read messages as they as they are "streamed" into
/// the reader without having to read the entire stream or file into memory. Messages can be
/// transcoded as they arrive and continuous written to output.
fn read_until<R: BufRead>(mut reader: R, delimiter: u8, message_tx: mpsc::Sender<Vec<u8>>) -> Result<()> {
    loop {
        let mut buf = Vec::new();
        let bytes_count = reader.read_until(delimiter, &mut buf).map_err(|e| {
            match e.kind() {
                ErrorKind::UnexpectedEof => Error::Eof,
                _ => Error::Io(e)
            }
        })?;
        // If the `read_until` method is at the End-of-File (EOF), then it will return zero for the
        // number of bytes read and the buffer will be unmodified. In this case, that means an
        // empty Vec.
        if buf.is_empty() && bytes_count == 0 {
            break; // EOF
        } else {
            message_tx.send(buf).unwrap();
        }
    }
    Ok(())
}

fn read<R: BufRead>(mut reader: R, framing: Option<Framing>, message_tx: mpsc::Sender<Vec<u8>>) -> Result<()> {
    if let Some(f) = framing {
        match f {
            Framing::Sized => read_exact(reader, message_tx)?,
            Framing::Delimited(delimiter) => read_until(reader, delimiter, message_tx)?,
        }
    } else {
        // If framing is not used, then the end stream or file must be read before transcoding
        // begins. This is the only really universal way to transcode a non-framed stream since not
        // all serialization formats use framing.
        let mut buf = Vec::new();
        let bytes_count = reader.read_to_end(&mut buf)?;
        if bytes_count > 0 {
            if !buf.is_empty() {
                message_tx.send(buf).unwrap();
            }
        } 
    }
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
pub fn run(input: Option<&str>, output: Option<&str>, from: Option<FromFormat>, to: Option<ToFormat>, input_framing: Option<Framing>, output_framing: Option<Framing>) -> Result<()> {
    let (message_tx, message_rx) = mpsc::channel::<Vec<u8>>();
    // Use `BufRead` instead of `Read` to add additional reading methods, like `read_until`. The
    // `Send` trait is needed to move the reader to the read thread.
    let reader: Box<BufRead + Send> = {
        if let Some(i) = input {
            Box::new(BufReader::new(File::open(i)?))
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
        read(reader, input_framing, message_tx).or_else(|e| {
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
            output_framing)?;
        } else {
            break;
        }
    }
    handle.join()?;
    Ok(())
}

