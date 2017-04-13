// Copyright (c) 2017 Christopher R. Field. All rights reserved.

//! `panser` - A (de)serialization tool
//!
//! Parses command line flags and options and runs transcoding from one serialization format to
//! another.
//!
//! # Examples
//!
//! The [xxd](http://linuxcommand.org/man_pages/xxd1.html) utility is used to display binary
//! formats as a series of bytes in hex notation. 
//!
//! Convert [JSON](http://www.json.org) from STDIN to [MessagePack](http://msgpack.org) (Msgpack)
//! and output to STDOUT. Panser converts JSON to Msgpack by default. See the `-h,--help` text for
//! more information and options. Specifically, see the `-f,--from` and `-t,--to` help text for
//! lists of supported formats. 
//!
//! ```bash
//! $ echo '{"bool":true}' | panser | xxd -i
//!   0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3
//! $
//! ```
//!
//! Similarly, convert JSON from a file to Msgpack and output to STDOUT. If no file is specified,
//! then input data is read continuously from STDIN.
//!
//! ```bash
//! $ panser file.json | xxd -i
//!   0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3
//! $
//! ```
//!
//! Redirection can also be used.
//!
//! ```bash
//! $ panser < file.json | xxd -i
//!   0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3
//! $
//! ```
//!
//! Convert JSON to pretty, more human readable JSON. The [Hjson](https://hjson.org) format is
//! a more human readable format. The `-n` adds a newline character to the end of the output to
//! place the prompt on the next line.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser -n -t Hjson
//! {
//!     "bool": true,
//!     "number": 1.234
//! }
//! $
//! ```
//!
//! Write data to file instead of STDOUT. The output file will contain the binary MessagePack data.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser -o file.msgpack
//! ```
//!
//! Add framing to the output. Framing is prepending the total serialized data length as an
//! unsigned 32-bit integer in Big Endian (Network ORder), and it is often used to aid in creating
//! stream-based applications for buffering. Note the first four bytes.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser --framed-out | xxd -i
//!   0x00, 0x00, 0x00, 0x17, 0x82, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0xa6,
//!   0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0xcb, 0x3f, 0xf3, 0xbe, 0x76, 0xc8,
//!   0xb4, 0x39, 0x58
//! $
//! ```
//!
//! The same can be done for input to remove the framing. Note the use of the `-f` option to
//! indicate the input format is MessagePack and _not_ JSON. The first four bytes are removed.
//! Framing can be added or removed from any supported format, not just MessagePack or other binary
//! formats.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser --framed-out | panser -f msgpack --framed-input | xxd -i
//!   0x82, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0xa6, 0x6e, 0x75, 0x6d, 0x62,
//!   0x65, 0x72, 0xcb, 0x3f, 0xf3, 0xbe, 0x76, 0xc8, 0xb4, 0x39, 0x58
//! $
//! ```
//!
//! Send data to a network device using the [nc](https://linux.die.net/man/1/nc) command. The JSON
//! will be transcoded to framed MessagePack and streamed to the server, or client at the IP
//! address and Port used with the `nc` command. This was actually the primary motivation for
//! creating the `panser` application.
//!
//! ```bash
//! $ echo '{"bool":true,"numeber":1.234}' | panser --framed-output | nc 127.0.0.1 1234
//! ```
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

