# Panser: A command line application for (de)serializing data #

Copyright (C) 2017 Christopher R. Field. All rights reserved.

This file is written in ASCII-encoded plain text using the [Github Flavored Markdown (GFM)](https://help.github.com/articles/github-flavored-markdown/) lightweight markup language.

## What is Panser? ##

The Panser project is a Command-Line Interface (CLI) application for (de)serializing data formats in a UNIX, pipe-friendly manner. The project is primarily written in the [Rust](http://www.rust-lang.org) programming language. The idea is to have a single application for reading data in one format on STDIN and writing the same data but in a different format to STDOUT. It is possible to read data from a file and write to a file, but the application is focused on creating streams of data that can be piped into a socket, such as a TCP stream. The primary motivator for the application is to read [JSON](http://www.json.org/) data and output to the [MessagePack](http://msgpack.org/index.html) format which could be used with a TCP stream to build a low-level Application Programming Interface (API) for a network-enabled application. The reverse is also a desired goal, reading in MessagePack data (binary, machine-readable) and transcoding it to JSON (text, human-readable).

After accomplishing the primary goal of transcoding between JSON and MessagePack (Msgpack) formats, additional formats were gradually added using the [serde](https://github.com/serde-rs/serde) project and related libraries. Almost all of the formats listed in the [Data Formats](https://serde.rs/#data-formats) section of the [Overview](https://serde.rs/) for the serde project are implemented. The intention is to add more formats as more crates are developed using the serde framework.

## Examples ##

The [xxd](http://linuxcommand.org/man_pages/xxd1.html) utility is used to display binary formats as a series of bytes in hex notation. 

Convert [JSON](http://www.json.org) from STDIN to [MessagePack](http://msgpack.org) (Msgpack) and output to STDOUT. Panser converts JSON to Msgpack by default. See the `-h,--help` text for more information and options. Specifically, see the `-f,--from` and `-t,--to` help text for lists of supported formats. 

```bash
$ echo '{"bool":true}' | panser | xxd -i
  0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3
$
```

Similarly, convert JSON from a file to Msgpack and output to STDOUT. If no file is specified, then input data is read continuously from STDIN. The file extension is used to determine the input data format unless the `-f,--from` option is explicitly used.

```bash
$ panser file.json | xxd -i
  0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3
$
```

Redirection can also be used.

```bash
$ panser < file.json | xxd -i
  0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3
$
```

Convert JSON to pretty, more human readable JSON. The [Hjson](https://hjson.org) format is a more human readable format. The `-n` adds a newline character to the end of the output to place the prompt on the next line.

```bash
$ echo '{"bool":true,"number":1.234}' | panser -n -t Hjson
{
    "bool": true,
    "number": 1.234
}
$
```

Write data to file instead of STDOUT. The output file will contain the binary MessagePack data.

```bash
$ echo '{"bool":true,"number":1.234}' | panser -o file.msgpack
```

Add framing to the output. Framing is prepending the total serialized data length as an unsigned 32-bit integer in Big Endian (Network ORder), and it is often used to aid in creating stream-based applications for buffering. Note the first four bytes.

```bash
$ echo '{"bool":true,"number":1.234}' | panser --framed-out | xxd -i
  0x00, 0x00, 0x00, 0x17, 0x82, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0xa6,
  0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0xcb, 0x3f, 0xf3, 0xbe, 0x76, 0xc8,
  0xb4, 0x39, 0x58
$
```

The same can be done for input to remove the framing. Note the use of the `-f` option to indicate the input format is MessagePack and _not_ JSON. The first four bytes are removed.  Framing can be added or removed from any supported format, not just MessagePack or other binary formats.

```bash
$ echo '{"bool":true,"number":1.234}' | panser --framed-out | panser -f msgpack --framed-input | xxd -i
  0x82, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0xa6, 0x6e, 0x75, 0x6d, 0x62,
  0x65, 0x72, 0xcb, 0x3f, 0xf3, 0xbe, 0x76, 0xc8, 0xb4, 0x39, 0x58
$
```

Send data to a network device using the [nc](https://linux.die.net/man/1/nc) command. The JSON will be transcoded to framed MessagePack and streamed to the server, or client at the IP address and Port used with the `nc` command. This was actually the primary motivation for creating the `panser` application.

```bash
$ echo '{"bool":true,"numeber":1.234}' | panser --framed-output | nc 127.0.0.1 1234
```

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

