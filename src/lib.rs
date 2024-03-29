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

//! # Panser
//!
//! Panser is primarily a Command-Line Interface (CLI) application for (de)serializing data formats
//! in a UNIX, pipe-friendly manner, but much of the functionality is provided in the
//! library/crate. Since the binary is essentially a wrapper around the publicly exposed
//! functionality of the crate, documentation on using the binary is provided here in the API
//! documentation.
//!
//! ## Binary Usage
//!
//! ### Examples
//!
//! Convert [JSON](http://www.json.org) from stdin to [MessagePack](http://msgpack.org) (Msgpack)
//! and write to stdout. Panser converts JSON to Msgpack by default. See the `-h,--help` text for
//! more information and options. Specifically, see the `-f,--from` and `-t,--to` help text for
//! lists of supported formats. The `-r,--radix` option is used to display the serialized output as
//! a space-separated list of bytes, where each byte is a string with formatted with the specified
//! radix. If the `-r,--radix` was _not_ used, the serialized output would be written as binary
//! data to stdout and not be very human readable but easily piped to other applications.
//!
//! ```bash
//! $ echo '{"bool":true}' | panser --radix hex
//! 81 A4 62 6F 6F 6C C3
//! ```
//!
//! Pipe the output to another application. Here, the
//! [xxd](http://linuxcommand.org/man_pages/xxd1.html) application receives the serialized
//! MessagePack binary output from Panser and displays it using the C-style notation for bytes.
//! This demonstrates piping the output of Panser to another application, but the `-r,--radix`
//! option with the `hex` value serves a similar function to using the `xxd` application.
//!
//! ```bash
//! $ echo '{"bool":true"}' | panser | xxd -i
//!   0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3
//! ```
//!
//! Similarly, convert JSON from a file to Msgpack and write to stdout. If no file is specified,
//! then data is read continuously from stdin until End-of-File (EOF) is reached (Ctrl+D). The
//! short names and possible value for the `-r,--radix` option is used for more succinct usage.
//!
//! ```bash
//! $ panser -r h file.json
//! 81 A4 62 6F 6F 6C C3
//! ```
//!
//! Redirection can also be used.
//!
//! ```bash
//! $ panser -r h < file.json
//! 81 A4 62 6F 6F 6C C3
//! ```
//!
//! Write data to file instead of stdout. The output file will contain the binary MessagePack data.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser -o file.msgpack
//! $ cat file.msgpack | xxd -i
//!   0x82, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0xa6, 0x6e, 0x75, 0x6d, 0x62,
//!   0x65, 0x72, 0xcb, 0x3f, 0xf3, 0xbe, 0x76, 0xc8, 0xb4, 0x39, 0x58
//! ```
//!
//! If the `-r,--radix` option is used, then the contents of the output file would _not_ be
//! Msgpack data, but the space-separated list of bytes as numeric strings.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser -r h -o file.msgpack
//! $ cat file.msgpack
//! 82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58
//! ```
//!
//! Add size-based framing to the output. Size-based framing is prepending the total serialized
//! data length as an unsigned 32-bit integer in Big Endian (Network Order), and it is often used
//! to aid in buffering and creating stream-based applications. Note the first four bytes.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser -r h --sized-output
//! 00 00 00 17 82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58
//! ```
//!
//! The same can be done for input to remove the size-based framing. Note the use of the `-f` option to
//! indicate the input format is MessagePack and _not_ JSON. The first four bytes are removed.
//! Size-based framing can be added or removed from any supported format, not just MessagePack.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser --sized-output | panser -r h -f msgpack --sized-input
//! 82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58
//! ```
//!
//! Another form of framing data in a stream involves delimiting each frame with delimiter byte.
//! Panser can also handle delimiter-based framing of data. This uses the ASCII newline character
//! (`\n`, 10 dec, 0A hex, or 012 octal) as the delimiter.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser --delimited-output 10d | panser -r h -f msgpack --delimited-input 10d
//! 82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58$
//! ```
//!
//! Using the delimited input and output is also a neat way to create an interactive console for Panser.
//!
//! ```bash
//! $ panser -d 0Ah -t Hjson
//! {"bool":true"}
//! {
//!     "bool": true
//! }
//! {"bool":true,"number":1.234}
//! {
//!     "bool": true,
//!     "number": 1.234
//! }
//! ```
//!
//! If the output is a binary format, like Msgpack, the `-r,--radix` becomes very useful for
//! creating an interactive console.
//!
//! ```bash
//! $ panser -r h -d 0Ah
//! {"bool":true"}
//! 81 A4 62 6F 6F 6C C3
//! {"bool":true,"number":1.234}
//! 82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58
//! ```
//!
//! Data can be sent to a network device using the [nc](https://linux.die.net/man/1/nc) command. The JSON
//! will be transcoded to size-based framed MessagePack and streamed to the server at the IP
//! address and TCP port used with the `nc` command. This was actually the primary motivation for
//! creating the Panser application.
//!
//! ```bash
//! $ echo '{"bool":true,"numeber":1.234}' | panser --sized-output | nc 127.0.0.1 1234
//! ```
//!
//! ### Exit Codes
//!
//! | Code | Reason                             |
//! |------|------------------------------------|
//! | 0    | Success, no error                  |
//! | 1    | Failure, error transcoding         |
//! | 2    | Failure, generic error             |
//! | 3    | Failure, Input/Output (IO)         |
//! | 4    | Failure, error parsing integer     |
//! | 5    | Failure, error with UTF-8 encoding |

extern crate bincode;
extern crate byteorder;
extern crate envy;
extern crate rmp_serde;
extern crate serde;
extern crate serde_cbor;
//extern crate serde_hjson;
extern crate serde_json;
extern crate serde_pickle;
extern crate serde_urlencoded;
extern crate serde_yaml;
extern crate toml;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::num;
use std::result;
use std::str::{self, FromStr};

pub use self::panser::deserialize;
pub use self::panser::serialize;
pub use self::panser::transcode;
pub use self::panser::Panser;

mod panser;

/// A specialized `Result` type for panser operations.
pub type Result<T> = result::Result<T, Error>;

/// The available framing options.
#[derive(Clone, Copy, Debug)]
pub enum Framing {
    /// Prefix the total message size as an unsigned 32-bit integer.
    Sized,
    /// Separate, or delimit, each message with a byte, or char, as a flag.
    Delimited(u8),
}

/// The different output (serialization) formats.
///
/// Note, not all formats can be deserialized and serialized.
#[derive(Clone, Copy, Debug)]
pub enum ToFormat {
    Bincode,
    Cbor,
    Hjson,
    Json,
    Msgpack,
    Pickle,
    Toml,
    Url,
    Yaml,
}

impl ToFormat {
    pub fn possible_values() -> Vec<&'static str> {
        vec![
            "Bincode", "bincode", "BINCODE", "Cbor", "cbor", "CBOR", "Hjson", "hjson", "HJSON",
            "Json", "json", "JSON", "Msgpack", "msgpack", "MSGPACK", "Pickle", "pickle", "PICKLE",
            "Toml", "toml", "TOML", "Url", "url", "URL", "Yaml", "yaml", "YAML",
        ]
    }
}

impl FromStr for ToFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match &*s.to_string().to_lowercase() {
            "bincode" => Ok(ToFormat::Bincode),
            "cbor" => Ok(ToFormat::Cbor),
            "hjson" => Ok(ToFormat::Hjson),
            "json" => Ok(ToFormat::Json),
            "msgpack" => Ok(ToFormat::Msgpack),
            "pickle" => Ok(ToFormat::Pickle),
            "toml" => Ok(ToFormat::Toml),
            "url" => Ok(ToFormat::Url),
            "yaml" => Ok(ToFormat::Yaml),
            _ => Err("No Match"),
        }
    }
}

impl fmt::Display for ToFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ToFormat::Bincode => write!(f, "Bincode"),
            ToFormat::Cbor => write!(f, "CBOR"),
            ToFormat::Hjson => write!(f, "Hjson"),
            ToFormat::Json => write!(f, "JSON"),
            ToFormat::Msgpack => write!(f, "Msgpack"),
            ToFormat::Pickle => write!(f, "Pickle"),
            ToFormat::Toml => write!(f, "TOML"),
            ToFormat::Url => write!(f, "URL"),
            ToFormat::Yaml => write!(f, "YAML"),
        }
    }
}

/// The different input (deserialization) formats.
///
/// Note, not all formats can be deserialized and serialized.
#[derive(Clone, Copy, Debug)]
pub enum FromFormat {
    Bincode,
    Cbor,
    Envy,
    Hjson,
    Json,
    Msgpack,
    Pickle,
    Toml,
    Url,
    Yaml,
}

impl FromFormat {
    pub fn possible_values() -> Vec<&'static str> {
        vec![
            "Bincode", "bincode", "BINCODE", "Cbor", "cbor", "CBOR", "Envy", "envy", "ENVY",
            "Hjson", "hjson", "HJSON", "Json", "json", "JSON", "Msgpack", "msgpack", "MSGPACK",
            "Pickle", "pickle", "PICKLE", "Toml", "toml", "TOML", "Url", "url", "URL", "Yaml",
            "yaml", "YAML",
        ]
    }
}

impl fmt::Display for FromFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromFormat::Bincode => write!(f, "Bincode"),
            FromFormat::Cbor => write!(f, "CBOR"),
            FromFormat::Envy => write!(f, "Envy"),
            FromFormat::Hjson => write!(f, "Hjson"),
            FromFormat::Json => write!(f, "JSON"),
            FromFormat::Msgpack => write!(f, "Msgpack"),
            FromFormat::Pickle => write!(f, "Pickle"),
            FromFormat::Toml => write!(f, "TOML"),
            FromFormat::Url => write!(f, "URL"),
            FromFormat::Yaml => write!(f, "YAML"),
        }
    }
}

impl FromStr for FromFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match &*s.to_string().to_lowercase() {
            "bincode" => Ok(FromFormat::Bincode),
            "cbor" => Ok(FromFormat::Cbor),
            "envy" => Ok(FromFormat::Envy),
            "hjson" => Ok(FromFormat::Hjson),
            "json" => Ok(FromFormat::Json),
            "msgpack" => Ok(FromFormat::Msgpack),
            "pickle" => Ok(FromFormat::Pickle),
            "toml" => Ok(FromFormat::Toml),
            "url" => Ok(FromFormat::Url),
            "yaml" => Ok(FromFormat::Yaml),
            _ => Err("No Match"),
        }
    }
}

/// The format, or radix, for displaying serialized binary data.
#[derive(Clone, Copy, Debug)]
pub enum Radix {
    /// Display data as a series of zeros (0) and ones (1).
    Binary,
    /// Display data as a series of decimal (integer) values.
    Decimal,
    /// Display data as a series of hexadecimal values.
    Hexadecimal,
    /// Display data as a series of octal values.
    Octal,
}

impl Radix {
    pub fn possible_values() -> Vec<&'static str> {
        vec![
            "b",
            "B",
            "bin",
            "Bin",
            "BIN",
            "binary",
            "Binary",
            "BINARY",
            "d",
            "D",
            "dec",
            "Dec",
            "DEC",
            "decimal",
            "Decimal",
            "DECIMAL",
            "h",
            "H",
            "hex",
            "Hex",
            "HEX",
            "hexadecimal",
            "Hexadecimal",
            "HEXADECIMAL",
            "o",
            "O",
            "oct",
            "Oct",
            "OCT",
            "octal",
            "Octal",
            "OCTAL",
        ]
    }
}

impl FromStr for Radix {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match &*s.to_string().to_lowercase() {
            "b" => Ok(Radix::Binary),
            "B" => Ok(Radix::Binary),
            "bin" => Ok(Radix::Binary),
            "binary" => Ok(Radix::Binary),
            "d" => Ok(Radix::Decimal),
            "D" => Ok(Radix::Decimal),
            "dec" => Ok(Radix::Decimal),
            "decimal" => Ok(Radix::Decimal),
            "h" => Ok(Radix::Hexadecimal),
            "H" => Ok(Radix::Hexadecimal),
            "hex" => Ok(Radix::Hexadecimal),
            "hexadecimal" => Ok(Radix::Hexadecimal),
            "o" => Ok(Radix::Octal),
            "O" => Ok(Radix::Octal),
            "oct" => Ok(Radix::Octal),
            "octal" => Ok(Radix::Octal),
            _ => Err("No match"),
        }
    }
}

impl fmt::Display for Radix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Radix::Binary => write!(f, "b, bin, or binary"),
            Radix::Decimal => write!(f, "d, dec, or decimal"),
            Radix::Hexadecimal => write!(f, "h, hex, or hexadecimal"),
            Radix::Octal => write!(f, "o, oct, or octal"),
        }
    }
}

/// The error type for panser-releated operations and associated traits.
///
/// Errors mostly originate from the dependencies, but custom instances of Error can be crated with
/// the `Generic` variant and a message.
#[derive(Debug)]
pub enum Error {
    /// Decoding/encoding of the Bincode format failed.
    Bincode(bincode::Error),
    /// Decoding/encoding of the CBOR format failed.
    Cbor(serde_cbor::Error),
    Envy(envy::Error),
    /// End-of-File (EOF) reached.
    Eof,
    /// A generic or custom error occurred. The message should contain the detailed information.
    Generic(String),
    //Hjson(serde_hjson::Error),
    /// An I/O operation failed.
    Io(io::Error),
    /// Decoding/encoding of the JSON format failed.
    Json(serde_json::Error),
    /// Decoding of the MessagePack format failed.
    MsgpackDecode(rmp_serde::decode::Error),
    /// Encoding of the MessagePack format failed.
    MsgpackEncode(rmp_serde::encode::Error),
    /// Converting a string to an integer failed.
    ParseInt(num::ParseIntError),
    /// Decoding/encoding of the Pickle format failed.
    Pickle(serde_pickle::Error),
    /// Decoding of the TOML format failed.
    TomlDecode(toml::de::Error),
    /// Encoding of the TOML format failed.
    TomlEncode(toml::ser::Error),
    /// A UTF8 operation failed.
    Utf8(str::Utf8Error),
    /// Decoding from a URL failed.
    UrlDecode(serde_urlencoded::de::Error),
    /// Encoding from a URL failed.
    UrlEncode(serde_urlencoded::ser::Error),
    /// Decoding/encoding of the YAML format failed.
    Yaml(serde_yaml::Error),
}

impl Error {
    /// Gets an error code related to the error.
    ///
    /// This is useful as a return, or exit, code for a command line application, where a non-zero
    /// integer indicates a failure in the application. It can also be used for quickly and easily
    /// teseting equality between two errors.
    pub fn code(&self) -> i32 {
        match *self {
            Error::Bincode(..) => 1,
            Error::Cbor(..) => 1,
            Error::Envy(..) => 1,
            Error::Eof => 0, // Not actually an error
            Error::Generic(..) => 2,
            //Error::Hjson(..) => 1,
            Error::Io(..) => 3,
            Error::Json(..) => 1,
            Error::MsgpackDecode(..) => 1,
            Error::MsgpackEncode(..) => 1,
            Error::ParseInt(..) => 4,
            Error::Pickle(..) => 1,
            Error::TomlDecode(..) => 1,
            Error::TomlEncode(..) => 1,
            Error::Utf8(..) => 5,
            Error::UrlDecode(..) => 1,
            Error::UrlEncode(..) => 1,
            Error::Yaml(..) => 1,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Bincode(ref err) => write!(f, "{}", err),
            Error::Cbor(ref err) => write!(f, "{}", err),
            Error::Envy(ref message) => write!(f, "{}", message),
            Error::Eof => write!(f, "End of file reached"),
            Error::Generic(ref message) => write!(f, "{}", message),
            //Error::Hjson(ref message) => write!(f, "{}", message),
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Json(ref err) => write!(f, "{}", err),
            Error::MsgpackDecode(ref err) => write!(f, "{}", err),
            Error::MsgpackEncode(ref err) => write!(f, "{}", err),
            Error::ParseInt(ref err) => write!(f, "{}", err),
            Error::Pickle(ref err) => write!(f, "{}", err),
            Error::TomlDecode(ref err) => write!(f, "{}", err),
            Error::TomlEncode(ref err) => write!(f, "{}", err),
            Error::UrlDecode(ref err) => write!(f, "{}", err),
            Error::UrlEncode(ref err) => write!(f, "{}", err),
            Error::Utf8(ref err) => write!(f, "{}", err),
            Error::Yaml(ref err) => write!(f, "{}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Bincode(..) => "Bincode",
            Error::Cbor(..) => "CBOR",
            Error::Envy(..) => "Envy error",
            Error::Eof => "EOF",
            Error::Generic(..) => "Generic",
            //Error::Hjson(..) => "Hjson error",
            Error::Io(..) => "IO",
            Error::Json(..) => "JSON",
            Error::MsgpackDecode(..) => "MessagePack decoding",
            Error::MsgpackEncode(..) => "MessagePack encoding",
            Error::ParseInt(..) => "Parse integer",
            Error::Pickle(..) => "Pickle",
            Error::TomlDecode(..) => "TOML decoding",
            Error::TomlEncode(..) => "TOML encoding",
            Error::UrlDecode(..) => "URL decoding",
            Error::UrlEncode(..) => "URL encoding",
            Error::Utf8(..) => "UTF-8",
            Error::Yaml(..) => "YAML",
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::Bincode(ref err) => Some(err),
            Error::Cbor(ref err) => Some(err),
            Error::Envy(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            //Error::Hjson(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::MsgpackDecode(ref err) => Some(err),
            Error::MsgpackEncode(ref err) => Some(err),
            Error::ParseInt(ref err) => Some(err),
            Error::Pickle(ref err) => Some(err),
            Error::TomlDecode(ref err) => Some(err),
            Error::TomlEncode(ref err) => Some(err),
            Error::UrlDecode(ref err) => Some(err),
            Error::UrlEncode(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::Yaml(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::Bincode(err)
    }
}

impl From<serde_cbor::Error> for Error {
    fn from(err: serde_cbor::Error) -> Error {
        Error::Cbor(err)
    }
}

impl From<envy::Error> for Error {
    fn from(err: envy::Error) -> Error {
        Error::Envy(err)
    }
}

impl From<Box<dyn Any + Send + 'static>> for Error {
    fn from(err: Box<dyn Any + Send + 'static>) -> Error {
        err.downcast_ref::<Error>()
            .map_or(Error::Generic(format!("Unknown error: {:?}", err)), |e| {
                Error::Generic(format!("{}", e))
            })
    }
}

//impl From<serde_hjson::Error> for Error {
//fn from(err: serde_hjson::Error) -> Error {
//Error::Hjson(err)
//}
//}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(err: rmp_serde::encode::Error) -> Error {
        Error::MsgpackEncode(err)
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(err: rmp_serde::decode::Error) -> Error {
        Error::MsgpackDecode(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<serde_pickle::Error> for Error {
    fn from(err: serde_pickle::Error) -> Error {
        Error::Pickle(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Error {
        Error::TomlEncode(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::TomlDecode(err)
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(err: serde_urlencoded::ser::Error) -> Error {
        Error::UrlEncode(err)
    }
}

impl From<serde_urlencoded::de::Error> for Error {
    fn from(err: serde_urlencoded::de::Error) -> Error {
        Error::UrlDecode(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::Yaml(err)
    }
}
