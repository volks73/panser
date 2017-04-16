# Panser: A command line application for (de)serializing data #

[About](#what-is-panser) | [Examples](#examples) | [Installation](#install) | [Build](#build)

## What is Panser? ##

The Panser project is a Command-Line Interface (CLI) application for (de)serializing data formats in a UNIX, pipe-friendly manner. The project is primarily written in the [Rust](http://www.rust-lang.org) programming language. The idea is to have a single application for reading data in one format on stdin and writing the same data but in a different format to stdout. It is possible to read data from a file and write to another file, but the application is focused on creating streams of data that can be piped into a socket, such as a TCP stream. The primary motivator for the application is to read [JSON](http://www.json.org/) data and output to the [MessagePack](http://msgpack.org/index.html) format which could be used with a TCP stream to develop low-level Application Programming Interfaces (APIs) for network-enabled applications. The reverse is also a desired goal: reading in MessagePack data (binary, machine-readable) and transcoding it to JSON (text, human-readable).

After accomplishing the primary goal of transcoding between JSON and MessagePack (Msgpack) formats, additional formats were gradually added using the [serde](https://github.com/serde-rs/serde) project and related libraries. Almost all of the formats listed in the [Data Formats](https://serde.rs/#data-formats) section of the [Overview](https://serde.rs/) for the serde project are implemented. The intention is to add more formats as more crates are developed using the serde framework.

## Examples ##

Convert [JSON](http://www.json.org) from stdin to [MessagePack](http://msgpack.org) (Msgpack) and write to stdout. Panser converts JSON to Msgpack by default. See the `-h,--help` text for more information and options. Specifically, see the `-f,--from` and `-t,--to` help text for lists of supported formats. The `-r,--radix` option is used to display the binary Msgpack format in a human readable format.

```bash
$ echo '{"bool":true}' | panser --radix hex
81 A4 62 6F 6F 6C C3
```

Similarly, convert JSON from a file to Msgpack and write to stdout. If no file is specified, then input data is read continuously from stdin. The file extension is used to determine the input data format unless the `-f,--from` option is explicitly used. Here, the `-r` option with the single character value is used to demonstrate a more succinct command line.

```bash
$ panser -r h file.json
81 A4 62 6F 6F 6C C3
```

Redirection can also be used.

```bash
$ panser -r h < file.json
81 A4 62 6F 6F 6C C3
```

Convert the JSON to a pretty, more human readable JSON. The [Hjson](https://hjson.org) format is a more human readable format.

```bash
$ echo '{"bool":true,"number":1.234}' | panser -t Hjson
{
    "bool": true,
    "number": 1.234
}
```

Write data to a file instead of stdout. The output file will contain the binary MessagePack data. The [xxd](http://linuxcommand.org/man_pages/xxd1.html) command is used to display the binary data as a [hex dump](https://en.wikipedia.org/wiki/Hex_dump). Using the `xxd` command is similar, but not identical, to using the `-r,--radix` option.

```bash
$ echo '{"bool":true,"number":1.234}' | panser -o file.msgpack
$ cat file.msgpack | xxd -i
  0x82, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0xa6, 0x6e, 0x75, 0x6d, 0x62,
  0x65, 0x72, 0xcb, 0x3f, 0xf3, 0xbe, 0x76, 0xc8, 0xb4, 0x39, 0x58
```

If the `-r,--radix` option is used, then the contents of the output file would _not_ be Msgpack data, but the space-separated list of bytes as numeric strings.

```bash
$ echo '{"bool":true,"number":1.234}' | panser -r h -o file.msgpack
$ cat file.msgpack
82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58
```

Write data to a file instead of stdout, but use the file extension to determine the format. The output file will contain the binary data in the [CBOR](http://cbor.io/) format.

```bash
$ echo '{"bool":true,"number":1.234}' | panser -o file.cbor
$ cat file.cbor | xxd -i
  0xa2, 0x64, 0x62, 0x6f, 0x6f, 0x6c, 0xf5, 0x66, 0x6e, 0x75, 0x6d, 0x62,
  0x65, 0x72, 0xfb, 0x3f, 0xf3, 0xbe, 0x76, 0xc8, 0xb4, 0x39, 0x58
```

Add size-based framing to the output. [Framing](https://en.wikipedia.org/wiki/Frame_(networking)) is a process to create specific frames of data from a continuous stream. It aids in buffering, reducing memory usage, and simplifying network handling code in applications. Size-based framing is prepending the total serialized data length as an unsigned 32-bit integer in Big Endian (Network Order) to the frame, or message.

```bash
$ echo '{"bool":true,"number":1.234}' | panser -r h --sized-output
00 00 00 17 82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58
```

The same can be done for input to remove the size-based framing. Note the use of the `-f` option to indicate the input format is Msgpack and _not_ JSON. The first four bytes are removed. Framing can be added or removed from any supported format, not just Msgpack.

```bash
$ echo '{"bool":true,"number":1.234}' | panser --sized-output | panser -r h -f msgpack --sized-input
82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58
```

An alternative form of framing uses a delimiter byte between frames, or messages. Panser can handle delimiter-based framing in a similar manner to size-based framing. The `--delimited-input` and `--delimited-output` options take a value that is a numeric string representation of a single byte. A radix suffix can be added to indicate the byte notation: (b) binary, (d) decimal, (h) hexadecimal, or (o) octal. If no radix suffix is used, then hexadecimal notation is assumed. Here, the ASCII newline character (`\n`, 1010b, 10d, 0Ah, and 012o) is used to delimit the binary data. However, the delimiter is not included in the output if the `-r,--radix` option is used.

```bash
$ echo '{"bool":true,"number":1.234}' | panser --delimited-output 0Ah | panser -r h -f msgpack --delimited-input 0Ah
82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58
```

The delimiter-based framing can be used to create an interactive console for Panser.

```bash
$ panser -d 0Ah -t Hjson
{"bool":true}
{
    "bool": true
}
{"bool":true,"number":1.234}
{
    "bool": true,
    "number": 1.234,
}
```

An interactive console with binary output, such as Msgpack, can be created with the delimiter-based framing and the `-r,--radix` option.

```bash
$ panser -r h -d 0Ah
{"bool":true"}
81 A4 62 6F 6F 6C C3
{"bool":true,"number":1.234}
82 A4 62 6F 6F 6C C3 A6 6E 75 6D 62 65 72 CB 3F F3 BE 76 C8 B4 39 58
```

Data can be sent to a network device using the [nc](https://linux.die.net/man/1/nc) command. The JSON will be transcoded to size-based framed Msgpack and streamed to the server at the IP address and TCP port used with the `nc` command. This was actually the primary motivation for creating the Panser application.

```bash
$ echo '{"bool":true,"numeber":1.234}' | panser --sized-output | nc 127.0.0.1 1234
```

Interestingly, Panser can be used in conjunction with the [wsta](https://github.com/esphen/wsta) application to send and receive data from a web socket server.

```bash
$ ehco '{"bool":true}' | panser | wsta 127.0.0.1:1234 | panser -f msgpack -t json
{"bool":true}
```

## Install ##

### Dependencies ###

- [Cargo](https://crates.io/), v0.17 or higher
- [Pandoc](http://pandoc.org), v1.18 or higher, optional
- [Rust](https://www.rust-lang.org/), v1.16 or higher

Download and install the latest version of [Rust](https://www.rust-lang.org) before proceeding. [Cargo](https://crates.io) will be installed automatically with Rust. [Pandoc](http://pandoc.org) is only need for installing and/or building the manual documentation, and it is optional.

### Repository ###

Obtain the source from the git repository and run the following commands from a terminal:

    $ git clone https://github.com/volks73/panser.git
    $ cd panser
    $ cargo install

It might be desirable to change the install location by using the `--root` option with the `cargo install` command. See the `cargo install --help` for more information about installing a Rust binary crate using Cargo.

### Source Distribution ###

Obtain the appropriate source distribution as an archive file and run the following commands from a terminal:

    $ tar xf panser-#.#.#.tar.gz
    $ cd panser-#.#.#
    $ cargo install

where `#.#.#` is replaced with the version number of the source distribution, respectively. It might be desirable to change the install location by using the `--root` option with the `cargo install` command. See the `cargo install --help` for more information about installing a Rust binary crate using Cargo.

### Documentation (Optional) ###

The manual must currently be installed manually. The [Pandoc]() application must be installed to convert the manual in markdown to the appropriate format. First install the application, then from the root directory of the project, run the following commands from a terminal:

    $ pandoc -s -t man -o man/panser.1 man/panser.1.md 
    $ cp man/panser.1 /usr/share/man/man1

## Build ##

Download and install the same dependencies listed for installing the application, this includes the latest versions of [Rust](https://www.rust-lang.org), [Cargo](https://crates.io), and optionally [Pandoc](http://pandoc.org).

### Application ###

Obtain the appropriate source from the repository and run the following commands from a terminal:

    $ git clone https://github.com/volks73/panser.git
    $ cd panser
    $ cargo build

Or obtain the source as an archive and run the following commands from a terminal:

    $ tar xf panser-#.#.#.tar.gz
    $ cd panser-#.#.#
    $ cargo build

where `#.#.#` is replaced with the version number of the source distribution, respectively. The `--release` flag can be added to the cargo command to build a release application instead of a debug application. 

### Documentation ###

Obtain the appropriate source and run the following commands from the root directory of the project in a terminal:

    $ pandoc -s -t man -o man/panser.1 man/panser.1.md

## License ##

See the LICENSE file for more information about licensing and copyright.

## Contributors ##

See the AUTHORS file for information about contributors. Contributors are listed alphabetically by family name.

