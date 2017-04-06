# Panser: A command line application for deserializing and serializing data #

Copyright (C) 2017 Christopher R. Field. All rights reserved.

This file is written in ASCII-encoded plain text using the [Github Flavored Markdown (GFM)](https://help.github.com/articles/github-flavored-markdown/) lightweight markup language.

## What is Panser? ##

The Panser project is a Command-Line Interface (CLI) application for deserializing and serializing data formats in a UNIX pipe-friendly manner. The project is primarily written in the [Rust](http://www.rust-lang.org) programming language. The idea is to have a single application for reading in data in one format on STDIN and writing the same data but in a different format to STDOUT. It is possible to read data from a file and write into a file, but application is focused on creating streams of data that can be piped into a socket, such as a TCP connection or UDP socket. A primary motivator for the application is to read in [JSON](http://www.json.org/) data and output a binary format, such as [MessagePack](http://msgpack.org/index.html), which could be streamed to TCP connection. The reverse is also a desired goal, reading in MessagePack data (binary, machine-readable) and transcoding it to JSON (text, human-readable). The application should aid in the development of network-focused Application Programming Interfaces (APIs) that use binary data.

## Build ##

### Dependencies ###

- [Cargo](https://crates.io/), v0.12.0 or higher
- [Rust](https://www.rust-lang.org/), v1.10.0 or higher

### Quick-Start Instructions ###

Download and install the latest version of [Rust](https://www.rust-lang.org) before proceeding. [Cargo](https://crates.io) will be installed automatically with Rust.

#### Source Distribution ####

Obtain the appropriate source distribution as an archive file and run the following commands from a terminal:

    $ tar xf panser-#.#.#.tar.gz
    $ cd panser-#.#.#
    $ cargo build --release

where `#.#.#` is replaced with the version number of the source distribution, respectively.

## Contacts ##

## License ##

See the LICENSE file for a information about licensing and copyright.

## Contributors ##

See the AUTHORS file for information about contributors. Contributors are listed alphabetically by family name.

