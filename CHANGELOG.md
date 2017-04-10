# Panser: Change Log

All notable changes to this project will be documented in this file, which is written in plain text (ASCII) [Github Flavored Markdown (GFM)](https://help.github.com/articles/github-flavored-markdown/) lightweight markup language. This project adheres to [Semantic Versioning](http://semver.org).

## [Unreleased][unreleased]

### Added

- The `-f,--from` option.
- The `-t,--to` option.
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
- Deserialization of the Msgpack format.
- Serialization to the JSON format.
- Serialization to the Bincode format.

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

