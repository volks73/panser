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
//! in a UNIX, pipe-friendly manner, but much of the functionality is provided in the library.

extern crate bincode;
extern crate byteorder;
//extern crate envy;
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
use std::str::{self, FromStr};
use std::result;

pub use self::panser::deserialize;
pub use self::panser::Panser;
pub use self::panser::serialize;
pub use self::panser::transcode;

mod panser;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub enum Framing {
    Sized,
    Delimited(u8),
}

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
            "Bincode", "bincode", "BINCODE",
            "Cbor", "cbor", "CBOR",
            "Hjson", "hjson", "HJSON",
            "Json", "json", "JSON",
            "Msgpack", "msgpack", "MSGPACK",
            "Pickle", "pickle", "PICKLE",
            "Toml", "toml", "TOML",
            "Url", "url", "URL",
            "Yaml", "yaml", "YAML",
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
            _ => Err("No Match")
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
            "Bincode", "bincode", "BINCODE",
            "Cbor", "cbor", "CBOR",
            "Envy", "envy", "ENVY",
            "Hjson", "hjson", "HJSON",
            "Json", "json", "JSON",
            "Msgpack", "msgpack", "MSGPACK",
            "Pickle", "pickle", "PICKLE",
            "Toml", "toml", "TOML",
            "Url", "url", "URL",
            "Yaml", "yaml", "YAML",
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
            _ => Err("No Match")
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Radix {
    Binary,
    Decimal,
    Hexadecimal,
    Octal,
}

impl Radix {
    pub fn possible_values() -> Vec<&'static str> {
        vec![
            "b", "B", "bin", "Bin", "BIN", "binary", "Binary", "BINARY",
            "d", "D", "dec", "Dec", "DEC", "decimal", "Decimal", "DECIMAL",
            "h", "H", "hex", "Hex", "HEX", "hexadecimal", "Hexadecimal", "HEXADECIMAL",
            "o", "O", "oct", "Oct", "OCT", "octal", "Octal", "OCTAL",
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
            _ => Err("No match")
        }
    }
}

impl fmt::Display for Radix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            Radix::Binary => write!(f, "b, bin, or binary"),
            Radix::Decimal => write!(f, "d, dec, or decimal"),
            Radix::Hexadecimal => write!(f, "h, hex, or hexadecimal"),
            Radix::Octal => write!(f, "o, oct, or octal"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Bincode(bincode::Error),
    Cbor(serde_cbor::Error),
    //Envy(envy::Error),
    Eof,
    Generic(String),
    //Hjson(serde_hjson::Error),
    Io(io::Error),
    Json(serde_json::Error),
    MsgpackDecode(rmp_serde::decode::Error),
    MsgpackEncode(rmp_serde::encode::Error),
    ParseInt(num::ParseIntError),
    Pickle(serde_pickle::Error),
    TomlDecode(toml::de::Error),
    TomlEncode(toml::ser::Error),
    Utf8(str::Utf8Error),
    UrlDecode(serde_urlencoded::de::Error),
    UrlEncode(serde_urlencoded::ser::Error),
    Yaml(serde_yaml::Error),
}

impl Error {
    pub fn code(&self) -> i32 {
        match *self {
            Error::Bincode(..) => 1,
            Error::Cbor(..) => 1,
            //Error::Envy(..) => 1,
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
            //Error::Envy(ref message) => write!(f, "{}", message),
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
            //Error::Envy(..) => "Envy error",
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

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Bincode(ref err) => Some(err),
            Error::Cbor(ref err) => Some(err),
            //Error::Envy(ref err) => Some(err),
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

//impl From<envy::Error> for Error {
    //fn from(err: envy::Error) -> Error {
        //Error::Envy(err)
    //}
//}

impl From<Box<Any + Send + 'static>> for Error {
    fn from(err: Box<Any + Send + 'static>) -> Error {
        err.downcast_ref::<Error>().map_or(
            Error::Generic(format!("Unknown error: {:?}", err)), 
            |e| Error::Generic(format!("{}", e))
        )
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

