# Panser: Change Log

All notable changes to this project will be documented in this file, which is written in plain text (ASCII) [Github Flavored Markdown (GFM)](https://help.github.com/articles/github-flavored-markdown/) lightweight markup language. This project adheres to [Semantic Versioning](http://semver.org).

## [Unreleased][unreleased]

### Added

- Documentation to public enums and functions.
- GitHub Pages for API documentation.
- HTML format for manpage.
- Build script to generate manpage on release build.

### Changed

- Documentation structure to use library/crate instead of binary.

### Fixed

- Examples in documentation that resulted in failed tests.

## [0.5.0] - 2017-11-16

### Added

- Colorized error statements. See [Issue #6](https://github.com/volks73/panser/issues/6).

### Changed

- The error statement format and error descriptions to be more human readable.

## [0.4.1] - 2017-05-30

### Added

- Integration tests.
- Serde v1.0 support for dependencies.

### [0.4.0] - 2017-04-17

### Added

- Deserializing (reading) multiple files and serializing (writing) to a single file. See [Issue #3](https://github.com/volks73/panser/issues/3).
- The `serialize` and `deserialize` function to the library.
- The `Error[#]` prefix to error messages, where `#` is the error/exit code.

### Fixed

- Error messages printing panic information. See [Issue #5](https://github.com/volks73/panser/issues/5).

## [0.3.0] - 2017-04-16

### Added

- The `-r,--radix` option to the command line interface (CLI). See [Issue #2](https://github.com/volks73/panser/issues/2).

### Changed

- The `transcode` function to return the serialized bytes instead of writing to the output. See [Issue #1](https://github.com/volks73/panser/issues/1).

### Removed

- Examples from the manual.

## [0.2.0] - 2017-04-15

### Added

- The `-d,--delimited` option.
- The `--delimited-input` option.
- The `--delimited-output` option.
- The `-s,--sized` flag.

### Changed

- The `--framed-input` flag to `--sized-input` to indicate the framing is by data size.
- The `--framed-output` flag to `--size-output` to indicate the framing is by data size.
- License from MIT to GPLv3.

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

