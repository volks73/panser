// Copyright (C) 2017 Christopher R. Field. All rights reserved.
extern crate serde_json;
extern crate rmp_serde;

use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    MsgpackDecode(rmp_serde::decode::Error),
    MsgpackEncode(rmp_serde::encode::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref message) => write!(f, "{}", message),
            Error::Json(ref message) => write!(f, "{}", message),
            Error::MsgpackDecode(ref message) => write!(f, "{}", message),
            Error::MsgpackEncode(ref message) => write!(f, "{}", message),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(..) => "IO error",
            Error::Json(..) => "JSON error",
            Error::MsgpackDecode(..) => "MessagePack decoding error",
            Error::MsgpackEncode(..) => "MessagePack encoding error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::MsgpackDecode(ref err) => Some(err),
            Error::MsgpackEncode(ref err) => Some(err),
        }
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

