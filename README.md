# Panser: A command line application for (de)serializing data #

Copyright (C) 2017 Christopher R. Field. All rights reserved.

This file is written in ASCII-encoded plain text using the [Github Flavored Markdown (GFM)](https://help.github.com/articles/github-flavored-markdown/) lightweight markup language.

## What is Panser? ##

The Panser project is a Command-Line Interface (CLI) application for (de)serializing data formats in a UNIX, pipe-friendly manner. The project is primarily written in the [Rust](http://www.rust-lang.org) programming language. The idea is to have a single application for reading data in one format on STDIN and writing the same data but in a different format to STDOUT. It is possible to read data from a file and write to a file, but the application is focused on creating streams of data that can be piped into a socket, such as a TCP stream. The primary motivator for the application is to read [JSON](http://www.json.org/) data and output to the [MessagePack](http://msgpack.org/index.html) format which could be used with a TCP stream to build a low-level Application Programming Interface (API) for a network-enabled application. The reverse is also a desired goal, reading in MessagePack data (binary, machine-readable) and transcoding it to JSON (text, human-readable).

After accomplishing the primary goal of transcoding between JSON and MessagePack (Msgpack) formats, additional formats were gradually added using the [serde](https://github.com/serde-rs/serde) project and related libraries. Almost all of the formats listed in the [Data Formats](https://serde.rs/#data-formats) section of the [Overview](https://serde.rs/) for the serde project are implemented. The intention is to add more formats as more crates are developed using the serde framework.

## Usage ##

The contents of the `-h,--help` flag.

```text
USAGE:
    panser [FLAGS] [OPTIONS] [FILE]

FLAGS:
        --framed-input       Indicates the first four bytes of the input is an
                             unsigned 32-bit integer in Big Endian (Network
                             Order) indicating the total length of the
                             serialized data.
        --framed-output      Prepends the total length of the serialized data
                             as an unsigned 32-bit integer in Big Endian
                             (Network Order).
    -h, --help               Prints help information
    -n, --include-newline    Writes the newline character (0x0A) to output at
                             the end of serializing a message.
    -V, --version            Prints version information

OPTIONS:
    -f, --from <from>        The input format. [values: Bincode, CBOR, Envy,
                             Hjson, JSON, Msgpack, Pickle, TOML, URL, YAML]
                             [default: JSON]
    -o, --output <output>    A file to write the output instead of writing to
                             STDOUT. If a file extension exists, then it is
                             used to determined the format of the output
                             serialized data. If a file extension does not
                             exist, then the `-t,--to` option should be used or
                             the MessagePack format is assumed.
    -t, --to <to>            The output format. [values: Bincode, CBOR, Hjson,
                             JSON, Msgpack, Pickle, TOML, URL, YAML] [default:
                             Msgpack]

ARGS:
    <FILE>    A file to read as input instead of reading from STDIN. If a file
              extension exists, then it is used to determine the format of the
              serialized data contained within the file. If a file extension
              does not exist, then the '-f,--from' option should be used or
              JSON is assumed.
```

## Build ##

### Dependencies ###

- [Cargo](https://crates.io/), v0.17 or higher
- [Rust](https://www.rust-lang.org/), v1.16 or higher

### Quick-Start Instructions ###

Download and install the latest version of [Rust](https://www.rust-lang.org) before proceeding. [Cargo](https://crates.io) will be installed automatically with Rust.

#### Source Distribution ####

Obtain the appropriate source distribution as an archive file and run the following commands from a terminal:

    $ tar xf panser-#.#.#.tar.gz
    $ cd panser-#.#.#
    $ cargo build --release

where `#.#.#` is replaced with the version number of the source distribution, respectively.

## License ##

See the LICENSE file for a information about licensing and copyright.

## Contributors ##

See the AUTHORS file for information about contributors. Contributors are listed alphabetically by family name.

