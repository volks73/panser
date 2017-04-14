# Panser: Change Log

All notable changes to this project will be documented in this file, which is written in plain text (ASCII) [Github Flavored Markdown (GFM)](https://help.github.com/articles/github-flavored-markdown/) lightweight markup language. This project adheres to [Semantic Versioning](http://semver.org).

## [Unreleased][unreleased]

### Added

- The `--delimited-input` option.
- The `--delimited-output` option.

### Changed

- The `--framed-input` flag to `--sized-input` to indicate the framing is by data size.
- The `--framed-output` flag to `--size-output` to indicate the framing is by data size.

## [0.1.1] - 2017-04-12

### Added

- Examples to the documentation.
- Installation directions to documentation.
- Error code as process exit code.

### Changed

- The organization to have the functionality available in the library.

## [0.1.0] - 2017-04-10

### Added

- The `-n,--include-newline` flag.
- The `-f,--from` option.
- The `-t,--to` option.
- Determining the input format from the file extension if a file is given
- Determining the output format from the file extension if the `-o,--output` option is used.
- Case insensitivity to the `-f,--from` option.
- Case insensitivity to the `-t,--to` option
- Deserialization of the Envy format.
- Deserialization of the URL format.
- Serialization of the URL format.
- Deserialization of the CBOR format.
- Serialization of the CBOR format.
- Deserialization of the YAML format.
- Serialization of the YAML format.
- Deserialization of the Pickle format.
- Serialization of the Pickle format.
- Deserialization of the TOML format.
- Serialization of the TOML format.
- Deserialization of the Bincode format.
- Serialization of the Bincode format.
- Deserialization of the Msgpack format.
- Serialization of the JSON format.

### Fixed

- Panic when reading framed input and EOF reached.

## [0.0.3] - 2017-04-09

### Added

- The `--framed-input` flag.
- The `--framed-output` flag.
- The `-o,--output` option to write output other than STDOUT.
- The optional `FILE` argument to read from a file instead of STDIN.

### Changed

- Handling of STDIN to be more conventional.

### Fixed

- Panics when file not found.

### Removed

- Explicitly STDIN interactivity.

## [0.0.2] - 2017-04-08

### Added

- Better error handling
- Reading STDIN interactively.

## [0.0.1] - 2017-04-07

### Added

- Reading JSON from STDIN.
- Transcoding from [JSON](http://www.json.org) to [MessagePack](http://www.msgpack.org).

