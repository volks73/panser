# Panser: Change Log

All notable changes to this project will be documented in this file, which is written in plain text (ASCII) [Github Flavored Markdown (GFM)](https://help.github.com/articles/github-flavored-markdown/) lightweight markup language. This project adheres to [Semantic Versioning](http://semver.org).

## [Unreleased][unreleased]

### Added

- The `-o,--output` option to write output other than STDOUT.
- The optional `FILE` argument to read from a file instead of STDIN.
- The `-n,--suppress-newline` flag to suppress output of the newline character (0x0A) if needed.

## [0.0.2] - 2017-04-08

### Added

- Better error handling
- Reading STDIN interactively.

## [0.0.1] - 2017-04-07

### Added

- Reading JSON from STDIN.
- Transcoding from [JSON](http://www.json.org) to [MessagePack](http://www.msgpack.org).

