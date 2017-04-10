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

// TODO: Add `possible_values` method, which returns a slice of strings that are the possible values.
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

// TODO: Add `possible_values` method, which returns a slice of strings that are the possible values.
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

