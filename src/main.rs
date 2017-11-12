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

//! `panser` - A (de)serialization tool
//!
//! Parses command line flags and options and runs transcoding from one serialization format to
//! another.
//!
//! # Examples
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
//! # Exit Codes
//!
//! | Code | Reason                                             |
//! |------|----------------------------------------------------|
//! | 0    | Success, no error                                  |
//! | 1    | Failure, error transcoding                         |
//! | 2    | Failure, generic error                             |
//! | 3    | Failure, Input/Output (IO)                         |
//! | 4    | Failure, error parsing integer                     |
//! | 5    | Failure, error with UTF-8 encoding                 |

extern crate ansi_term;
extern crate atty;
#[macro_use] extern crate clap;
extern crate panser;

use ansi_term::Colour;
use clap::{App, Arg};
use panser::{FromFormat, Panser, Radix, ToFormat};
use std::error::Error;
use std::io::Write;

const ERROR_COLOR: Colour = Colour::Fixed(9); // bright red

/// The main entry point of the application. Parses command line options and starts the main
/// program.
fn main() {
    // Based on documentation for the ansi_term crate, Windows 10 supports ANSI escape characters,
    // but it must be enabled first. The ansi_term crate provides a function for enabling ANSI
    // support in Windows, but it is conditionally compiled and only exists for Windows builds. To
    // avoid build errors on non-windows platforms, a cfg guard should be put in place.
    #[cfg(windows)] ansi_term::enable_ansi_support().unwrap();

    let matches = App::new("panser")
        .version(crate_version!())
        .about("An application for transcoding serialization formats.") 
        .arg(Arg::with_name("delimited")
             .help("Inidcates a complete message is delimited by the specified byte value and the byte should be appended to the output of each message. This is equivalent to using the '--delimited-input' and '--delimited-output' options with the same value. The delimiter byte can be specified as a (b) binary, (d) decimal, (h) hexadecimal, or (o) octal string value by using the character as a radix suffix. For example, '0Ah' would be the ASCII newline character specified as a hexadecimal string value. If no radix suffix is specified, then hexadecimal notation is assumed. This option cannot be used with the '--sized', '--sized-input', or '--sized-output' flags.")
             .long("delimited")
             .short("d")
             .conflicts_with("delimited-input")
             .conflicts_with("delimited-output")
             .conflicts_with("sized")
             .conflicts_with("sized-input")
             .conflicts_with("sized-output")
             .takes_value(true))
        .arg(Arg::with_name("delimited-input")
             .help("Indicates a complete message is delimited by the specified byte value. The delimiter byte can be specified as a (b) binary, (d) decimal, (h) hexadecimal, or (o) octal string value by using the character as a radix suffix. For example, '0Ah' would be the ASCII newline character specified as a hexadecimal string value. If no radix suffix is used, then hexadecimal notation is assumed. This option cannot be used with the '--sized', '--sized-input', or '--delimited' options.")
             .long("delimited-input")
             .conflicts_with("delimited")
             .conflicts_with("sized")
             .conflicts_with("sized-input")
             .takes_value(true))
        .arg(Arg::with_name("delimited-output")
             .help("Appends the delimiter byte to the message. The delimiter byte can be specified as a (b) binary, (d) decimal, (h) hexadecimal, or (o) octal string value by using the character as a radix suffix. For example, '0Ah' would be the ASCII newline character specified as a hexadecimal string value. If no radix suffix is used, then hexadecimal notation is assumed. This option cannot be used with the '--sized', '--sized-output', or '--delimited' options.")
             .long("delimited-output")
             .conflicts_with("delimited")
             .conflicts_with("sized")
             .conflicts_with("sized-output")
             .takes_value(true))
        .arg(Arg::with_name("FILES")
            .help("The files to read as input instead of reading from stdin. Unless the '-f,--from' option is used, the file extension for each file will be used to determine the input data format. If a file extension does not exist, the data format is assumed to be JSON. If the '-f,--from' option is used, then the same input data format is used for deserialization regardless of the file extensions.")
            .index(1)
            .multiple(true))
        .arg(Arg::with_name("from")
            .help("The input format. The value is case insensitive. [values: Bincode, CBOR, Envy, Hjson, JSON, Msgpack, Pickle, TOML, URL, YAML] [default: JSON]")
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
        .arg(Arg::with_name("radix")
             .help("Changes the output to be a space-separated list of bytes, where each byte is represented as a numeric string based on the radix value. The serialized input data is transcoded to the format specified with the '-t,--to' option, but it is written to the output as a string. This is useful for debugging serialization formats and creating an interactive console. Note, if delimited-based framing is employed, the delimiter byte is not included in the space-separated list of bytes. The radix value can be the first letter of the possible values ('b', 'd', 'h', or 'o') and the value is case insensitive. [values: bin, dec, hex, oct] [default: hex]")
             .long("radix")
             .short("r")
             .hide_possible_values(true)
             .possible_values(&Radix::possible_values())
             .takes_value(true))
        .arg(Arg::with_name("sized")
            .help("Indicates the first four bytes of the input is an unsigned 32-bit integer in Big Endian (Network Order) that is the total size of the serialized data, and the data size should be prepended to the output. This flag cannot be used with the '--delimited', '--delimited-input', '--delimited-output', '--sized-input', or '--sized-output' options.")
            .long("sized")
            .short("s")
            .conflicts_with("delimited")
            .conflicts_with("delimited-input")
            .conflicts_with("delimited-output")
            .conflicts_with("sized-input")
            .conflicts_with("sized-output"))
        .arg(Arg::with_name("sized-input")
            .help("Indicates the first four bytes of the input is an unsigned 32-bit integer in Big Endian (Network Order) indicating the total length of the serialized data. This flag cannot be used with the '--delimited', '--delimited-input', or '--sized' options.")
            .long("sized-input")
            .conflicts_with("delimited")
            .conflicts_with("delimited-input")
            .conflicts_with("sized"))
        .arg(Arg::with_name("sized-output")
            .help("Prepends the total length of the serialized data as an unsigned 32-bit integer in Big Endian (Network Order). This flag cannot be used with the '--delimited', '--delimited-output', or '--sized' options.")
            .long("sized-output")
            .conflicts_with("delimited")
            .conflicts_with("delimited-output")
            .conflicts_with("sized"))
        .arg(Arg::with_name("to")
            .help("The output format. The value is case insensitive. [values: Bincode, CBOR, Hjson, JSON, Msgpack, Pickle, TOML, URL, YAML] [default: Msgpack]")
            .long("to")
            .short("t")
            .hide_possible_values(true)
            .possible_values(&ToFormat::possible_values())
            .takes_value(true))
        .get_matches();
    let result = Panser::new()
        .delimited_output(matches.value_of("delimited-output").or(matches.value_of("delimited")))
        .delimited_input(matches.value_of("delimited-input").or(matches.value_of("delimited")))
        .from(value_t!(matches, "from", FromFormat).ok())
        .inputs(matches.values_of("FILES").map(|v| v.collect::<Vec<&str>>()))
        .output(matches.value_of("output"))
        .radix(value_t!(matches, "radix", Radix).ok())
        .sized_input(matches.is_present("sized-input") || matches.is_present("sized"))
        .sized_output(matches.is_present("sized-output") || matches.is_present("sized"))
        .to(value_t!(matches, "to", ToFormat).ok())
        .run();
    match result {
        Ok(_) => {
            std::process::exit(0);
        },
        Err(e) => {
            let mut tag = format!("Error[{}] ({})", e.code(), e.description());
            if atty::is(atty::Stream::Stderr) {
                tag = ERROR_COLOR.paint(tag).to_string()
            }
            writeln!(&mut std::io::stderr(), "{}: {}", tag, e)
                .expect("Writing to stderr");
            std::process::exit(e.code());
        }
    }
}

