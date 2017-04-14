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
//! Convert [JSON](http://www.json.org) from stdin to [MessagePack](http://msgpack.org) (Msgpack)
//! and output to stdout. Panser converts JSON to Msgpack by default. See the `-h,--help` text for
//! more information and options. Specifically, see the `-f,--from` and `-t,--to` help text for
//! lists of supported formats. 
//!
//! ```bash
//! $ echo '{"bool":true}' | panser | xxd -i
//!   0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3
//! $
//! ```
//!
//! Similarly, convert JSON from a file to Msgpack and output to stdout. If no file is specified,
//! then input data is read continuously from stdin.
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
//! Write data to file instead of stdout. The output file will contain the binary MessagePack data.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser -o file.msgpack
//! ```
//!
//! Add sized-based framing to the output. Sized-based framing is prepending the total serialized data length as an unsigned 32-bit integer in Big Endian (Network Order), and it is often used to aid in buffering and creating stream-based applications. Note the first four bytes.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser --sized-output | xxd -i
//!   0x00, 0x00, 0x00, 0x17, 0x82, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0xa6,
//!   0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0xcb, 0x3f, 0xf3, 0xbe, 0x76, 0xc8,
//!   0xb4, 0x39, 0x58
//! $
//! ```
//!
//! The same can be done for input to remove the data size. Note the use of the `-f` option to
//! indicate the input format is MessagePack and _not_ JSON. The first four bytes are removed.
//! Sized-based framing can be added or removed from any supported format, not just MessagePack or
//! other binary formats.
//!
//! ```bash
//! $ echo '{"bool":true,"number":1.234}' | panser --sized-output | panser -f msgpack --sized-input | xxd -i
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
//! $ echo '{"bool":true,"numeber":1.234}' | panser --sized-output | nc 127.0.0.1 1234
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
//! | 10   | Failure, error parsing integer                     |
//! | 11   | Failure, error transcoding the Pickle format       |
//! | 12   | Failure, error decoding the TOML format            |
//! | 13   | Failure, error encoding the TOML format            |
//! | 14   | Failure, error with UTF-8 encoding                 |
//! | 15   | Failure, error decoding the URL format             |
//! | 16   | Failure, error encoding the URL format             |
//! | 17   | Failure, error transcoding the YAML format         |

#[macro_use]
extern crate clap;
extern crate panser;

use clap::{App, Arg};
use panser::{Framing, FromFormat, ToFormat};
use std::io::Write;

/// The main entry point of the application. Parses command line options and starts the main
/// program.
fn main() {
    // TODO: Add `-d,--delimited` option, which takes a value. The value is byte in hex notation,
    // 0x## or possibly just ##, where ## is a hexadecimal digit (0-9 or A-F). When specified, this
    // option enables continuous reading of input data where individual messages of serialized data
    // are delimited by a "delimter" byte. The same delimiter byte is appended to the output. This
    // applies the same delimiter to the input and output. The `--delimited-input` and
    // `--delimited-output` options can be used to specify different delimiter bytes for the input
    // and output, or if only one of the two streams are or should be delimited. 

    // TODO: Add `-s,--sized` flag. This indicates the length of each message is the first
    // four bytes of the input and the output should have the length of each message prepended to
    // the data. The `--sized-input` and `--sized-output` flags can be used to
    // independently specify the framing by length for the input and output.

    let matches = App::new("panser")
        .version(crate_version!())
        .about("An application for transcoding serialization formats.") 
        .arg(Arg::with_name("delimited-input")
             .help("Indicates a complete message is delimited by the specified byte value. The delimiter byte can be specified as a (b) binary, (d) decimal, (h) hexadecimal, or (o) octal string value by using the character as a suffix. For example, '0Ah' would be the newline character in ASCII hex. If no notation suffix is used, then hexadecimal notation is assumed for the byte value. This option cannot be used with the '--sized-input' flag.")
             .long("delimited-input")
             .conflicts_with("sized-input")
             .takes_value(true))
        .arg(Arg::with_name("delimited-output")
             .help("Appends the delimiter byte to the message. The delimiter byte can be specified as a (b) binary, (d) decimal, (h) hexadecimal, or (o) octal string value by using the character as a suffix. For example, '0Ah' would be the newline character in ASCII hex. If no notation suffix is used, then hexadecimal notation is assumed for the byte value. This option cannot be used with the '--sized-input' flag.")
             .long("delimited-output")
             .conflicts_with("sized-output")
             .takes_value(true))
        .arg(Arg::with_name("FILE")
            .help("A file to read as input instead of reading from stdin. If a file extension exists, then it is used to determine the format of the serialized data contained within the file. If a file extension does not exist, then the '-f,--from' option should be used or JSON is assumed.")
            .index(1))
        .arg(Arg::with_name("from")
            .help("The input format. [values: Bincode, CBOR, Envy, Hjson, JSON, Msgpack, Pickle, TOML, URL, YAML] [default: JSON]")
            .long("from")
            .short("f")
            .hide_possible_values(true)
            .possible_values(&FromFormat::possible_values())
            .takes_value(true))
        .arg(Arg::with_name("output")
            .help("A file to write the output instead of writing to stdout. If a file extension exists, then it is used to determined the format of the output serialized data. If a file extension does not exist, then the `-t,--to` option should be used or the MessagePack format is assumed.")
            .long("output")
            .short("o")
            .takes_value(true))
        .arg(Arg::with_name("sized-input")
            .help("Indicates the first four bytes of the input is an unsigned 32-bit integer in Big Endian (Network Order) indicating the total length of the serialized data. This flag cannot be used when the '--delimited-input' option is specified.")
            .long("sized-input")
            .conflicts_with("delimited-input"))
        .arg(Arg::with_name("sized-output")
            .help("Prepends the total length of the serialized data as an unsigned 32-bit integer in Big Endian (Network Order). This flag cannot be used when the '--delimited-output' option is specified.")
            .long("sized-output")
            .conflicts_with("delimited-output"))
        .arg(Arg::with_name("to")
            .help("The output format. [values: Bincode, CBOR, Hjson, JSON, Msgpack, Pickle, TOML, URL, YAML] [default: Msgpack]")
            .long("to")
            .short("t")
            .hide_possible_values(true)
            .possible_values(&ToFormat::possible_values())
            .takes_value(true))
        .get_matches();
    let input_framing = matches.value_of("delimited-input").map_or_else(|| {
        if matches.is_present("sized-input") {
            Some(Framing::Sized)
        } else {
            None
        }
    }, |s| {
        let value = match s.chars().last().unwrap() {
            'b' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 2).unwrap(),
            'd' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 10).unwrap(),
            'h' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 16).unwrap(),
            'o' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 8).unwrap(),
            _ => u8::from_str_radix(s, 16).unwrap(),
        };
        Some(Framing::Delimited(value))
    });
    let output_framing = matches.value_of("delimited-output").map_or_else(|| {
        if matches.is_present("sized-output") {
            Some(Framing::Sized)
        } else {
            None
        }
    }, |s| {
        let value = match s.chars().last().unwrap() {
            'b' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 2).unwrap(),
            'd' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 10).unwrap(),
            'h' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 16).unwrap(),
            'o' => u8::from_str_radix(&s.chars().take(s.len() - 1).collect::<String>(), 8).unwrap(),
            _ => u8::from_str_radix(s, 16).unwrap(),
        };
        Some(Framing::Delimited(value))
    });
    let result = panser::run(
        matches.value_of("FILE"), 
        matches.value_of("output"), 
        value_t!(matches, "from", FromFormat).ok(),
        value_t!(matches, "to", ToFormat).ok(),
        input_framing,
        output_framing
    );
    match result {
        Ok(_) => {
            std::process::exit(0);
        },
        Err(e) => {
            writeln!(&mut std::io::stderr(), "{}", e).expect("Writing to stderr");
            std::process::exit(e.code());
        }
    }
}

