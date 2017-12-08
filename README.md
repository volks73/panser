# Panser: A command line application for (de)serializing data

[About](#what-is-panser) | [Installation](#installation) | [Manual](https://volks73.github.io/panser/manpage.html) | [API](https://volks73.github.io/panser) | [Build](#build) | [Examples](#examples)  

## What is Panser?

The Panser project is a Command-Line Interface (CLI) application for (de)serializing data formats in a pipe-friendly manner. The project is primarily written in the [Rust](http://www.rust-lang.org) programming language. It can be installed on any [platform supported](https://forge.rust-lang.org/platform-support.html) by the Rust programming language, including Linux, macOS, and Windows. The idea is to have a single application for reading data in one format on stdin and writing the same data but in a different format to stdout. It is possible to read data from a file and write to another file, but the application is focused on creating streams of data that can be piped into a socket, such as a TCP stream. The primary motivator for the application is to read [JSON](http://www.json.org/) data and output to the [MessagePack](http://msgpack.org/index.html) format which could be used with a TCP stream to develop low-level Application Programming Interfaces (APIs) for network-enabled applications. The reverse is also a desired goal: reading in MessagePack data (binary, machine-readable) and transcoding it to JSON (text, human-readable).

After accomplishing the primary goal of transcoding between JSON and MessagePack (Msgpack) formats, additional formats were gradually added using the [serde](https://github.com/serde-rs/serde) project and related libraries. Almost all of the formats listed in the [Data Formats](https://serde.rs/#data-formats) section of the [Overview](https://serde.rs/) for the serde project are implemented. The intention is to add more formats as more crates are developed using the serde framework.

## Installation

Panser can be installed on any platform supported by the Rust programming language, including Linux, macOS, and Windows. It is possible to run Panser on Windows using the native command prompt (cmd.exe) or a terminal emulator, like [Mintty](https://mintty.github.io/) via [Cygwin](https://www.cygwin.com/).

### Windows

An installer (msi) with a pre-compiled binary is available with each [release](https://github.com/volks73/panser/releases). The installer will also add the installation location to the PATH system environment variable so panser can be executed from anywhere. Run the installer and follow the on-screen dialog to complete the installation.

It is also possible to install the application from source using Cargo. See the instructions for [installation via Cargo](#source) and use a command prompt (cmd.exe) or terminal emulator to execute the commands.

### macOS

Follow the instructions for [installation from source](#source).

### Linux

Follow the instructions for [installation from source](#source).

### Source

Download and install the following dependencies before installing the binary using Cargo.

- [Cargo](https://crates.io/), v0.17 or higher
- [Pandoc](http://pandoc.org), v1.19 or higher, optional
- [Rust](https://www.rust-lang.org/), v1.16 or higher

#### Application

Run the following commands from a terminal:

    $ git clone https://github.com/volks73/panser.git
    $ cd panser
    $ cargo install

Or obtain the source as an archive and run the following commands from a terminal:

    $ tar xf panser-#.#.#.tar.gz
    $ cd panser-#.#.#
    $ cargo install

where `#.#.#` is replaced with the version number of the source distribution, respectively. It might be desirable to change the install location by using the `--root` option with the `cargo install` command. See the `cargo install --help` for more information about installing a Rust binary crate using Cargo.


It might be desirable to change the install location by using the `--root` option with the `cargo install` command. See the `cargo install --help` for more information about installing a Rust binary crate using Cargo.

Note, if the panser binary was installed using cargo, then it can be uninstalled using `cargo uninstall panser`.

#### Documentation (Optional)

If the [Pandoc](http://pandoc.org) application was installed prior to installing from source via Cargo, i.e. `cargo install`, then a manpage in the [grofff](https://www.gnu.org/software/groff/) format is automatically created from the [markdown](http://pandoc.org/MANUAL.html#pandocs-markdown) "source" file in the `man` directory using pandoc as part of the build script (`build.rs`). Otherwise, the manpage can be built with the following command:

    $ pandoc -s -t man -o man/panser.1 man/panser.1.md 

Regardless if the manpage (`panser.1`) was manually or automatically generated, it must be must be manually installed with the following command:

    $ mkdir -p ~/.cargo/share/man/man1
    $ cp man/panser.1 ~/.cargo/share/man/man1

If uninstalling panser using Cargo, i.e. `cargo uninstall panser`, then the manpage must also be manually removed as follows:

    $ rm ~/.cargo/share/man/man1/panser.1

## Build

Download and install the same dependencies listed for [installing the application from source](#source), this includes the latest versions of [Rust](https://www.rust-lang.org), [Cargo](https://crates.io), and optionally [Pandoc](http://pandoc.org).

### Application

Run the following commands from a terminal:

    $ git clone https://github.com/volks73/panser.git
    $ cd panser
    $ cargo build

Or obtain the source as an archive and run the following commands from a terminal:

    $ tar xf panser-#.#.#.tar.gz
    $ cd panser-#.#.#
    $ cargo build

where `#.#.#` is replaced with the version number of the source distribution, respectively. The `--release` flag can be added to the cargo command to build a release application instead of a debug application. 

### Documentation

Documentation is available in two forms: (i) [API](#api) and (ii) [Manpage](#manpage). The API documentation is for the library/crate while the Manpage documentation is helpful for the executable/binary. 

#### [API](https://volks73.github.io/panser)

Obtain the appropriate source and run the following commands from the root directory of the project in a terminal:

    $ cargo doc --no-deps

The output will be available in the `target/doc` folder.

#### [Manpage](https://volks73.github.io/panser/manpage.html)

Obtain the appropriate source and run the following commands from the root directory of the project in a terminal to build the manpage in the [groff](https://www.gnu.org/software/groff/) and html formats:

    $ cargo build --release

Or,

    $ pandoc -s -t man -o man/panser.1 man/panser.1.md
    $ pandoc -s -t html -o manpage.html man/panser.1.md

When the `release` profile is used to build the binary, the manpage is automatically generated if pandoc is installed.

## Examples

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

## License

See the LICENSE file for more information about licensing and copyright.

