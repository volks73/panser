// Copyright (C) 2017 Christopher R. Field.
//
// This file is part of Panser.
//
// Panser is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// Panser is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with Panser.  If not, see <http://www.gnu.org/licenses/>.

use bincode;
//use envy;
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
use std::panic;
use std::path::Path;
use std::str::{self, FromStr};
use std::sync::mpsc;
use std::thread;
use super::{Error, Framing, FromFormat, Radix, Result, ToFormat};

type Sender = mpsc::Sender<serde_json::Value>;
type Receiver = mpsc::Receiver<serde_json::Value>;

/// A Builder for transcoding.
pub struct Panser {
    delimited_input: Option<String>,
    delimited_output: Option<String>,
    from: Option<FromFormat>,
    inputs: Option<Vec<String>>,
    output: Option<String>,
    radix: Option<Radix>,
    sized_input: bool,
    sized_output: bool,
    to: Option<ToFormat>,
}

impl Panser {
    /// Creates a new `Panser` with default options.
    ///
    /// The defaults are stdin for input, stdout for output, JSON for the from format, Msgpack for
    /// the to format, no delimited input, no delimited output, no sized input, and no sized
    /// output. The `Panser` struct is implemented following the Builder pattern. Methods can be
    /// chained to change the defaults.
    pub fn new() -> Panser {
        Panser {
            delimited_input: None,
            delimited_output: None,
            from: None,
            inputs: None,
            output: None,
            radix: None,
            sized_input: false,
            sized_output: false,
            to: None,
        }
    }

    /// Sets a delimiter byte for the input and changes to framed reading of the data.
    ///
    /// Data is read from the input source to the next delimiter byte. When the delimiter byte is
    /// reached, then all bytes up to the delimiter byte are transcoded. This continues until the
    /// End-of-File (EOF) is reached.
    pub fn delimited_input(mut self, delimited: Option<&str>) -> Self {
        self.delimited_input = delimited.map(|d| d.to_owned());
        self
    }

    /// Sets a delimiter byte for the output.
    ///
    /// The delimiter byte is appended to the output data.
    pub fn delimited_output(mut self, delimited: Option<&str>) -> Self {
        self.delimited_output = delimited.map(|d| d.to_owned());
        self
    }

    /// The format of the input.
    ///
    /// If `None`, which is the default, then the input format is assumed to be JSON.
    pub fn from(mut self, from: Option<FromFormat>) -> Self {
        self.from = from;
        self
    }

    /// The input source.
    ///
    /// If `None`, which is the default, then stdin is used as the source. The value is a path to
    /// a file.
    pub fn inputs(mut self, inputs: Option<Vec<&str>>) -> Self {
        self.inputs = inputs.map(|i| {
            i.iter().map(|f| String::from(*f)).collect::<Vec<String>>()
        });
        self
    }

    /// The output destination.
    ///
    /// If `None`, which is the default, then stdout is used as the destination. The value is
    /// a path to a file.
    pub fn output(mut self, output: Option<&str>) -> Self {
        self.output = output.map(|o| o.to_owned());
        self
    }

    /// Sets the written output to be a space-separated list of bytes represented as numeric
    /// strings with a specific radix, or notation.
    ///
    /// The data is still transcoded to the `to` format, but it is written to the output as
    /// a string. This is useful for debugging and creating an interactive console where humans are
    /// reading the serialized output.
    pub fn radix(mut self, radix: Option<Radix>) -> Self {
        self.radix = radix;
        self
    }

    /// Create a producer-consumer architecture for reading and writing data. 
    ///
    /// A separate thread is created and started for reading the input until End-of-File (EOF) is
    /// reached. If reading stdin, Ctrl+D can be used to force an EOF.
    ///
    /// If input is `None`, then stdin is used for input. If output is `None`, then stdout is used
    /// for output. If the input (from) format is `None`, then the format is determined from the file
    /// extension if a file is provided and it has an extension. The default input format is JSON. If
    /// the input format is not JSON and a file with an appropriate extension is _not_ used, then the
    /// `from` parameter should not be `None`. A similar procedure is used for the output (to) format.
    pub fn run(self) -> Result<()> {
        let (tx, rx) = mpsc::channel::<serde_json::Value>();
        // Use `BufRead` instead of `Read` to add additional reading methods, like `read_until`. The
        // `Send` trait is needed to move the reader to the read thread.
        let readers: Vec<Box<BufRead + Send>> = {
            if let Some(i) = self.inputs.as_ref() {
                // There has to be a way to do this with map and collect.
                let mut files: Vec<Box<BufRead + Send>> = Vec::new();
                for f in i {
                    files.push(Box::new(BufReader::new(File::open(f)?)));
                }
                files
            } else {
                vec![Box::new(BufReader::new(io::stdin()))]
            }
        };
        let writer: Box<Write> = {
            if let Some(o) = self.output.as_ref() {
                Box::new(File::create(o)?)
            } else {
                Box::new(io::stdout())
            }
        };
        let froms = {
            if let Some(files) = self.inputs.as_ref() {
                files.iter()
                    .map(|f| {
                        self.from.unwrap_or({
                            if let Some(e) = Path::new(f).extension() {
                                FromFormat::from_str(
                                    e.to_str().unwrap_or("json")
                                ).unwrap_or(FromFormat::Json)
                            } else {
                                FromFormat::Json
                            }
                        })
                    })
                    .collect()
            } else {
                vec![self.from.unwrap_or(FromFormat::Json)]
            }
        };
        let to = self.to.unwrap_or({
            if let Some(o) = self.output.as_ref() {
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
        });
        let input_framing = self.delimited_input.as_ref().map_or_else(|| {
            if self.sized_input {
                Ok(Some(Framing::Sized))
            } else {
                Ok(None)
            }
        }, to_framing_delimited)?;
        let output_framing = self.delimited_output.as_ref().map_or_else(|| {
            if self.sized_output {
                Ok(Some(Framing::Sized))
            } else {
                Ok(None)
            }
        }, to_framing_delimited)?;
        // Set the panic hook to do nothing. This suppresses the
        //
        // >thread '<unnamed>' panicked at 'Box<Any>', src/panser.rs:223
        // >note: Run with `RUST_BACKTRACE=1` for a backtrace."
        //
        // message when an error occurs. The read thread panics on an error, but it is not really
        // a panic. This ensure the application exits in a clean fashion everytime and with the
        // appropriate error code without having to implement redundant error message printing and
        // exiting. This probably shold be changed in the future to user a verbose flag to re-enable
        // the full panic message when debugging.
        panic::set_hook(Box::new(|_|{}));
        let handle = thread::spawn(move || {
            for r in readers.into_iter().zip(froms) {
                let (reader, from) = r;
                let result = read(reader, from, input_framing, &tx).or_else(|e| {
                    match e {
                        Error::Eof => {
                            Ok(())
                        },
                        _ => Err(e),
                    }
                });
                // Do not use `unwrap` for the read function result. Using `unwrap` yields an
                // `Unknown error: Any` message, which is not very useful. The `unwrap` method for
                // a Result creates a custom string from the Error value, it does not pass the actual
                // Error value. The type of error is lost, so when the panic occurs within this
                // thread and it is "caught" by the `join` method later, the unwrapped panic
                // message appears as a string and fails to be cast to the Error type. This yields
                // a default Generic Error that is printed to stderr as "Unknown error: Any". The
                // workaround to print more useful error message on an error from the read thread
                // without having to write redundant error handling and keep the error type
                // information is to use the panic! macro and pass the Error value directly to the
                // Result of the `join` message. A Generic Error is still created with the `From`
                // trait, but the Error type can be determined and a more appropriate error message
                // with more useful information can be created.
                //
                // There is probably a better way to do all of this, but I have not found it yet.
                match result {
                    Ok(_) => {},
                    Err(e) => panic!(e),
                }
            }
        });
        write(writer, to, output_framing, self.radix, rx)?;
        handle.join()?;
        Ok(())
    }

    /// Indicates the first four bytes is the total data length and changes to framed reading of
    /// the data.
    ///
    /// The first four bytes are read as an unsigned 32-bit integer (u32) in Big Endian (Network
    /// Order). Then N number of bytes are read, where N is the size converted from the first
    /// four bytes. Once N bytes are read, all bytes up to the size are transcoded. This
    /// continues until the End-of-File (EOF) is reached.
    pub fn sized_input(mut self, sized: bool) -> Self {
        self.sized_input = sized;
        self
    }

    /// Prepends the length of the data to the output.
    ///
    /// The size of the output is prepended as an unsigned 32-bit integer (u32) in Big Endian
    /// (Network Order).
    pub fn sized_output(mut self, sized: bool) -> Self {
        self.sized_output = sized;
        self
    }

    /// The format of the output.
    ///
    /// If `None`, which is the default, then the output format is assumed to be MessagePack
    /// (Msgpack).
    pub fn to(mut self, to: Option<ToFormat>) -> Self {
        self.to = to;
        self
    }
}

/// Deserialize to a universal, arbitrary value.
///
/// The `serde_json::Value` type is used as a container for an arbitrary deserialized value. All
/// formats are deserialized to a `serde_json::Value` type.
pub fn deserialize(input: &[u8], from: FromFormat) -> Result<serde_json::Value> {
    Ok({
        match from {
            FromFormat::Bincode => bincode::deserialize::<serde_json::Value>(input)?,
            FromFormat::Cbor => serde_cbor::from_slice::<serde_json::Value>(input)?,
            FromFormat::Envy => unimplemented!(),
            //FromFormat::Envy => envy::from_env::<serde_json::Value>()?,
            // TODO: Change to use Hjson serde library. Until the Hjson crate is updated to work
            // with serde v0.9 or newer, the serde_json create is used. The Hjson crate currently
            // uses serde v0.8 and causes compiler errors.
            FromFormat::Hjson => serde_json::from_slice::<serde_json::Value>(input)?,
            FromFormat::Json => serde_json::from_slice::<serde_json::Value>(input)?,
            FromFormat::Msgpack => rmp_serde::from_slice::<serde_json::Value>(input)?,
            FromFormat::Pickle => serde_pickle::from_slice::<serde_json::Value>(input)?,
            FromFormat::Toml => toml::from_slice::<serde_json::Value>(input)?,
            FromFormat::Url => serde_urlencoded::from_bytes::<serde_json::Value>(input)?,
            FromFormat::Yaml => serde_yaml::from_slice::<serde_json::Value>(input)?,
        }
    })
}

/// Serialize from a universal, arbitrary value.
///
/// The `serde_json::Value` type is used as a container for an arbitrary value that can be
/// serialized to any format.
pub fn serialize(value: serde_json::Value, to: ToFormat) -> Result<Vec<u8>> {
    Ok({ 
        match to {
            ToFormat::Bincode => bincode::serialize(&value, bincode::Infinite)?,
            ToFormat::Cbor => serde_cbor::to_vec(&value)?,
            // TODO: Change to use Hjson serde library. Until the Hjson crate is updated to work
            // with serde v0.9 or newer, the serde_json create is used. The Hjson crate currently
            // uses serde v0.8 and causes compiler errors.
            ToFormat::Hjson => serde_json::to_vec_pretty(&value)?, 
            ToFormat::Json => serde_json::to_vec(&value)?,
            ToFormat::Msgpack => rmp_serde::to_vec(&value)?,
            ToFormat::Pickle => serde_pickle::to_vec(&value, true)?,
            ToFormat::Toml => toml::to_vec(&value)?,
            ToFormat::Url => serde_urlencoded::to_string(&value)?.into_bytes(),
            ToFormat::Yaml => serde_yaml::to_vec(&value)?,
        }
    })
}

/// Convert the input in one format to the output of another format.
///
/// This does allocate memory, as not all serde-based libraries support allocation-free
/// transcoding. However, if used in a producer-consumer architecture with framing,
/// the memory usage should be minimized.
///
/// # Example
///
/// ```rust
/// extern crate panser;
///
/// use panser::{FromFormat, ToFormat};
///
/// fn main() {
///     let input = "{\"bool\":true}";
///     let output = panser::transcode(
///         input.as_bytes(), 
///         FromFormat::Json,
///         ToFormat::Msgpack
///     ).unwrap();
///     assert_eq!(output, vec![0x81, 0xA4, 0x62, 0x6F, 0x6F, 0x6C, 0xC3]);
/// }
/// ```
pub fn transcode(input: &[u8], from: FromFormat, to: ToFormat) -> Result<Vec<u8>> {
    serialize(deserialize(input, from)?, to)
}


/// Converts a string to a delimiter byte.
///
/// A delimiter byte can be specified as a string using the following notation: 1010b for binary,
/// 10d for decimal, 0Ah for hexadecimal, and 012o for octal. All four of these notations will
/// yield the ASCII newline character for the delimiter byte. If no radix suffix (`b`, `d`, `h`, or
/// `o`) is present at the end of the string, then hexadecimal is assumed.
///
/// # Errors
///
/// A `ParseInt` error will occur if the string cannot be converted to a u8 (byte) value.
fn to_framing_delimited(s: &String) -> Result<Option<Framing>> {
    let value = match s.chars().last().unwrap() {
        'b' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 2)?,
        'd' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 10)?,
        'h' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 16)?,
        'o' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 8)?,
        _ => u8::from_str_radix(&s, 16)?,
    };
    Ok(Some(Framing::Delimited(value)))
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
fn read_exact<R: BufRead>(mut reader: R, from: FromFormat, tx: &Sender) -> Result<()> {
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
        tx.send(deserialize(&buf, from)?).unwrap();
    }
}

/// Reads until a delimiter is reached.
///
/// Reading continues until the End-of-File (EOF) is reached.
///
/// Since the data is framed, the application can read messages as they as they are "streamed" into
/// the reader without having to read the entire stream or file into memory. Messages can be
/// transcoded as they arrive and continuous written to output.
fn read_until<R: BufRead>(mut reader: R, from: FromFormat, delimiter: u8, tx: &Sender) -> Result<()> {
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
            tx.send(deserialize(&buf, from)?).unwrap();
        }
    }
    Ok(())
}

/// The producer loop for reading (input) and writing (output) serialized data.
///
/// Determines the appropriating reading paradiagm based on the framing.
fn read<R: BufRead>(mut reader: R, from: FromFormat, framing: Option<Framing>, tx: &Sender) -> Result<()> {
    if let Some(f) = framing {
        match f {
            Framing::Sized => read_exact(reader, from, &tx)?,
            Framing::Delimited(delimiter) => read_until(reader, from, delimiter, &tx)?,
        }
    } else {
        // If framing is not used, then the end of the stream or file must be read before transcoding
        // begins. This is the only real universal way to transcode a non-framed stream.
        let mut buf = Vec::new();
        let bytes_count = reader.read_to_end(&mut buf)?;
        if bytes_count > 0 {
            if !buf.is_empty() {
                tx.send(deserialize(&buf, from)?).unwrap();
            }
        } 
    }
    Ok(())
}

/// Writes the serialized output data.
///
/// If the `display` is `None`, then the data is written "as-is". This means serialized binary
/// data, like the MessagePack format, are written as binary data and may not be human readable.
/// However, if the `display` is a `Radix` value, then the serialized output data is written as
/// a space-separated list of bytes, where each byte is a string formatted using the radix. This
/// can be used to visual, or display, serialized binary data in a more human readable fashion.
fn write_data<W: Write>(mut writer: W, data: &[u8], radix: Option<Radix>) -> Result<()> {
    if let Some(r) = radix {
        for byte in data.iter() {
            match r {
                Radix::Binary => write!(&mut writer, "{:b} ", byte)?,
                Radix::Decimal => write!(&mut writer, "{} ", byte)?,
                Radix::Hexadecimal => write!(&mut writer, "{:0X} ", byte)?,
                Radix::Octal => write!(&mut writer, "{:o} ", byte)?,
            }
        }
    } else {
        writer.write(&data)?;
    }
    Ok(())
}

/// The consumer loop for the producer-consumer architecture for reading (input) and writing
/// (output).
///
/// The consumer loop listens for serialized messages from the producer (input) loop. When
/// a message is received, the serialized input data is transcoded based on the `from` format to
/// the serialized output data based on the `to` format. After transcoding, the serialized output
/// data is written to the output with the `writer` based on the `framing`.
///
/// The `display` value is ignored for writing the delimiter if delimited-based framing is used.
/// This makes it easier to create an interactive console with the application.
fn write<W: Write>(mut writer: W, to: ToFormat, framing: Option<Framing>, radix: Option<Radix>, rx: Receiver) -> Result<()> {
    loop {
        if let Ok(data) = rx.recv() {
            let encoded_data = serialize(data, to)?;
            if let Some(f) = framing {
                match f {
                    Framing::Sized => {
                        let mut frame_length = [0; 4];
                        BigEndian::write_u32(&mut frame_length, encoded_data.len() as u32);
                        write_data(&mut writer, &frame_length, radix)?;
                    },
                    _ => {},
                }
            }
            write_data(&mut writer, &encoded_data, radix)?;
            if let Some(f) = framing {
                match f {
                    Framing::Delimited(delimiter) => {
                        // The delimiter should _not_ be written as a string if there is some
                        // display value. An ASCII newline character ('\n') is often used as
                        // a delimiter to create an interactive console. If the newline character
                        // is written as a string byte, then the cursor will not appear after
                        // space-separated list of bytes of the output. It is awkward looking. This
                        // ensures the delimiter is always written as binary data and the cursor is
                        // printed on the following line of the output when creating an interactive
                        // console.
                        writer.write(&[delimiter; 1])?;
                    },
                    _ => {},
                }
            }
            writer.flush()?;
        } else {
            break;
        }
    }
    Ok(())
}

