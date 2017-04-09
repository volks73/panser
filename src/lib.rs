// Copyright (C) 2017 Christopher R. Field. All rights reserved.
extern crate serde_json;
extern crate rmp_serde;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::str;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
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

impl From<Box<Any + Send + 'static>> for Error {
    fn from(err: Box<Any + Send + 'static>) -> Error {
        Error::Generic(format!("{:?}", err))
    }
}

