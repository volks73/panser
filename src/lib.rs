// Copyright (C) 2017 Christopher R. Field. All rights reserved.
extern crate bincode;
extern crate serde;
extern crate serde_json;
extern crate rmp_serde;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::str::{self, FromStr};
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub enum ToFormat {
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

impl ToFormat {
    pub fn possible_values() -> Vec<&'static str> {
        vec![
            "Bincode", "bincode", "BINCODE",
            "Bson", "bson", "BSON",
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
            "bson" => Ok(ToFormat::Bson),
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
            ToFormat::Bson => write!(f, "BSON"),
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

impl FromFormat {
    pub fn possible_values() -> Vec<&'static str> {
        vec![
            "Bincode", "bincode", "BINCODE",
            "Bson", "bson", "BSON",
            "Cbor", "cbor", "CBOR",
            "Envy", "envy", "ENVY",
            "Hjson", "hjson", "HJSON",
            "Json", "json", "JSON",
            "Msgpack", "msgpack", "MSGPACK",
            "Pickle", "pickle", "PICKLE",
            "Redis", "redis", "REDIS",
            "Toml", "toml", "TOML",
            "Url", "url", "URL",
            "Xml", "xml", "XML",
            "Yaml", "yaml", "YAML",
        ]
    }
}

impl fmt::Display for FromFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromFormat::Bincode => write!(f, "Bincode"),
            FromFormat::Bson => write!(f, "BSON"),
            FromFormat::Cbor => write!(f, "CBOR"),
            FromFormat::Envy => write!(f, "Envy"),
            FromFormat::Hjson => write!(f, "Hjson"),
            FromFormat::Json => write!(f, "JSON"),
            FromFormat::Msgpack => write!(f, "Msgpack"),
            FromFormat::Pickle => write!(f, "Pickle"),
            FromFormat::Redis => write!(f, "Redis"),
            FromFormat::Toml => write!(f, "TOML"),
            FromFormat::Url => write!(f, "URL"),
            FromFormat::Xml => write!(f, "XML"),
            FromFormat::Yaml => write!(f, "YAML"),
        }
    }
}

impl FromStr for FromFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match &*s.to_string().to_lowercase() {
            "bincode" => Ok(FromFormat::Bincode),
            "bson" => Ok(FromFormat::Bson),
            "cbor" => Ok(FromFormat::Cbor),
            "envy" => Ok(FromFormat::Envy),
            "hjson" => Ok(FromFormat::Hjson),
            "json" => Ok(FromFormat::Json),
            "msgpack" => Ok(FromFormat::Msgpack),
            "pickle" => Ok(FromFormat::Pickle),
            "redis" => Ok(FromFormat::Redis),
            "toml" => Ok(FromFormat::Toml),
            "url" => Ok(FromFormat::Url),
            "xml" => Ok(FromFormat::Xml),
            "yaml" => Ok(FromFormat::Yaml),
            _ => Err("No Match")
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Bincode(bincode::Error),
    Eof,
    Generic(String),
    Io(io::Error),
    Json(serde_json::Error),
    MsgpackDecode(rmp_serde::decode::Error),
    MsgpackEncode(rmp_serde::encode::Error),
    Utf8(str::Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Bincode(ref message) => write!(f, "{}", message),
            Error::Eof => write!(f, "End of file reached"),
            Error::Generic(ref message) => write!(f, "{}", message),
            Error::Io(ref message) => write!(f, "{}", message),
            Error::Json(ref message) => write!(f, "{}", message),
            Error::MsgpackDecode(ref message) => write!(f, "{}", message),
            Error::MsgpackEncode(ref message) => write!(f, "{}", message),
            Error::Utf8(ref message) => write!(f, "{}", message),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Bincode(..) => "Bincode error",
            Error::Eof => "EOF error",
            Error::Generic(..) => "Generic error",
            Error::Io(..) => "IO error",
            Error::Json(..) => "JSON error",
            Error::MsgpackDecode(..) => "MessagePack decoding error",
            Error::MsgpackEncode(..) => "MessagePack encoding error",
            Error::Utf8(..) => "UTF-8 error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Bincode(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::MsgpackDecode(ref err) => Some(err),
            Error::MsgpackEncode(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
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

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::Bincode(err)
    }
}

impl From<Box<Any + Send + 'static>> for Error {
    fn from(err: Box<Any + Send + 'static>) -> Error {
        Error::Generic(format!("{:?}", err))
    }
}

