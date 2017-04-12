// Copyright (c) 2017 Christopher R. Field. All rights reserved.

//! `panser` - A (de)serialization tool
//!
//! Parses command line flags and options and runs transcoding from one serialization format to
//! another.
//!
//! # Examples
//!
//! # Exit Codes
//!
//! | Code | Reason                                             |
//! |------|----------------------------------------------------|
//! | 0    | Success, no error                                  |
//! | 1    | Failure, error transcoding the Bincode format      |
//! | 2    | Failure, error transcoding the CBOR format         |
//! | 3    | Failure, error deserializing environment variables |
//! | 4    | Failure, generic error                             |
//! | 5    | Failure, error transcoding the Hjson format        |
//! | 6    | Failure, Input/Output (IO)                         |
//! | 7    | Failure, error transcoding the JSON format         |
//! | 8    | Failure, error decoding the MessagePack format     |
//! | 9    | Failure, error encoding the MessagePack format     |
//! | 10   | Failure, error transcoding the Pickle format       |
//! | 11   | Failure, error decoding the TOML format            |
//! | 12   | Failure, error encoding the TOML format            |
//! | 13   | Failure, error with UTF-8 encoding                 |
//! | 14   | Failure, error decoding the URL format             |
//! | 15   | Failure, error encoding the URL format             |
//! | 16   | Failure, error transcoding the YAML format         |

#[macro_use]
extern crate clap;
extern crate panser;

use clap::{App, Arg};
use panser::{FromFormat, ToFormat};
use std::io::Write;

/// The main entry point of the application. Parses command line options and starts the main
/// program.
fn main() {
    let matches = App::new("panser")
        .version(crate_version!())
        .about("An application for transcoding serialization formats.") 
        .arg(Arg::with_name("FILE")
            .help("A file to read as input instead of reading from STDIN. If a file extension exists, then it is used to determine the format of the serialized data contained within the file. If a file extension does not exist, then the '-f,--from' option should be used or JSON is assumed.")
            .index(1))
        .arg(Arg::with_name("framed-input")
            .help("Indicates the first four bytes of the input is an unsigned 32-bit integer in Big Endian (Network Order) indicating the total length of the serialized data.")
            .long("framed-input"))
        .arg(Arg::with_name("framed-output")
            .help("Prepends the total length of the serialized data as an unsigned 32-bit integer in Big Endian (Network Order).")
            .long("framed-output"))
        .arg(Arg::with_name("from")
            .help("The input format. [values: Bincode, CBOR, Envy, Hjson, JSON, Msgpack, Pickle, TOML, URL, YAML] [default: JSON]")
            .long("from")
            .short("f")
            .hide_possible_values(true)
            .possible_values(&FromFormat::possible_values())
            .takes_value(true))
        .arg(Arg::with_name("include-newline")
            .help("Writes the newline character (0x0A) to output at the end of serializing a message.")
            .long("include-newline")
            .short("n"))
        .arg(Arg::with_name("output")
            .help("A file to write the output instead of writing to STDOUT. If a file extension exists, then it is used to determined the format of the output serialized data. If a file extension does not exist, then the `-t,--to` option should be used or the MessagePack format is assumed.")
            .long("output")
            .short("o")
            .takes_value(true))
        .arg(Arg::with_name("to")
            .help("The output format. [values: Bincode, CBOR, Hjson, JSON, Msgpack, Pickle, TOML, URL, YAML] [default: Msgpack]")
            .long("to")
            .short("t")
            .hide_possible_values(true)
            .possible_values(&ToFormat::possible_values())
            .takes_value(true))
        .get_matches();
    let result = panser::run(
        matches.value_of("FILE"), 
        matches.value_of("output"), 
        value_t!(matches, "from", FromFormat).ok(),
        value_t!(matches, "to", ToFormat).ok(),
        matches.is_present("framed-input"),
        matches.is_present("framed-output"),
        matches.is_present("include-newline")
    );
    match result {
        Ok(_) => {
            std::process::exit(0);
        },
        Err(e) => {
            writeln!(&mut std::io::stderr(), "{}", e).expect("Writing to STDERR");
            std::process::exit(e.code());
        }
    }
}

