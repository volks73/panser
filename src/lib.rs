// Copyright (C) 2017 Christopher R. Field. All rights reserved.
extern crate bincode;
extern crate envy;
extern crate rmp_serde;
extern crate serde;
extern crate serde_cbor;
extern crate serde_hjson;
extern crate serde_json;
extern crate serde_pickle;
extern crate serde_urlencoded;
extern crate serde_xml;
extern crate serde_yaml;
extern crate toml;

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
    Cbor(serde_cbor::Error),
    Envy(envy::Error),
    Eof,
    Generic(String),
    Hjson(serde_hjson::Error),
    Io(io::Error),
    Json(serde_json::Error),
    MsgpackDecode(rmp_serde::decode::Error),
    MsgpackEncode(rmp_serde::encode::Error),
    Pickle(serde_pickle::Error),
    TomlDecode(toml::de::Error),
    TomlEncode(toml::ser::Error),
    Utf8(str::Utf8Error),
    UrlDecode(serde_urlencoded::de::Error),
    UrlEncode(serde_urlencoded::ser::Error),
    Xml(serde_xml::Error),
    Yaml(serde_yaml::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Bincode(ref message) => write!(f, "{}", message),
            Error::Cbor(ref message) => write!(f, "{}", message),
            Error::Envy(ref message) => write!(f, "{}", message),
            Error::Eof => write!(f, "End of file reached"),
            Error::Generic(ref message) => write!(f, "{}", message),
            Error::Hjson(ref message) => write!(f, "{}", message),
            Error::Io(ref message) => write!(f, "{}", message),
            Error::Json(ref message) => write!(f, "{}", message),
            Error::MsgpackDecode(ref message) => write!(f, "{}", message),
            Error::MsgpackEncode(ref message) => write!(f, "{}", message),
            Error::Pickle(ref message) => write!(f, "{}", message),
            Error::TomlDecode(ref message) => write!(f, "{}", message),
            Error::TomlEncode(ref message) => write!(f, "{}", message),
            Error::UrlDecode(ref message) => write!(f, "{}", message),
            Error::UrlEncode(ref message) => write!(f, "{}", message),
            Error::Utf8(ref message) => write!(f, "{}", message),
            Error::Xml(ref message) => write!(f, "{}", message),
            Error::Yaml(ref message) => write!(f, "{}", message),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Bincode(..) => "Bincode error",
            Error::Cbor(..) => "CBOR error",
            Error::Envy(..) => "Envy error",
            Error::Eof => "EOF error",
            Error::Generic(..) => "Generic error",
            Error::Hjson(..) => "Hjson error",
            Error::Io(..) => "IO error",
            Error::Json(..) => "JSON error",
            Error::MsgpackDecode(..) => "MessagePack decoding error",
            Error::MsgpackEncode(..) => "MessagePack encoding error",
            Error::Pickle(..) => "Pickle error",
            Error::TomlDecode(..) => "TOML decoding error",
            Error::TomlEncode(..) => "TOML encoding error",
            Error::UrlDecode(..) => "URL decoding error",
            Error::UrlEncode(..) => "URL encoding error",
            Error::Utf8(..) => "UTF-8 error",
            Error::Xml(..) => "XML error",
            Error::Yaml(..) => "YAML error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Bincode(ref err) => Some(err),
            Error::Cbor(ref err) => Some(err),
            Error::Envy(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Hjson(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::MsgpackDecode(ref err) => Some(err),
            Error::MsgpackEncode(ref err) => Some(err),
            Error::Pickle(ref err) => Some(err),
            Error::TomlDecode(ref err) => Some(err),
            Error::TomlEncode(ref err) => Some(err),
            Error::UrlDecode(ref err) => Some(err),
            Error::UrlEncode(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::Xml(ref err) => Some(err),
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

impl From<Box<Any + Send + 'static>> for Error {
    fn from(err: Box<Any + Send + 'static>) -> Error {
        Error::Generic(format!("{:?}", err))
    }
}
impl From<serde_hjson::Error> for Error {
    fn from(err: serde_hjson::Error) -> Error {
        Error::Hjson(err)
    }
}

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

impl From<serde_xml::Error> for Error {
    fn from(err: serde_xml::Error) -> Error {
        Error::Xml(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::Yaml(err)
    }
}

