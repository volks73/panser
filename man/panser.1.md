% PANSER(1)
% Christopher R. Field
% April 2017

# NAME

panser - A utility for (de)serializing data formats

# SYNOPSIS

panser [*options*] [*input-file*]...

# DESCRIPTION

The Panser project is a Command-Line Interface (CLI) application for (de)serializing data formats in a UNIX, pipe-friendly manner. The project is primarily written in the Rust programming language. The idea is to have a single application for reading data in one format on stdin and writing the same data but in a different format to stdout. It is possible to read data from a file and write to a file, but the application is focused on creating streams of data that can be piped into a socket, such as a TCP stream. The primary motivator for the application is to read JSON data and output to the MessagePack (Msgpack) format which could be used with a TCP stream to build a low-level Application Programming Interface (API) for a network-enabled application. The reverse is also a desired goal, reading in Msgpack data (binary, machine-readable) and transcoding it to JSON (text, human-readable).

# OPTIONS

-d, \--delimited=*DELIMITER*
:   Indicates each frame, or message, within a stream of data is separated by a delimiter byte and the same delimiter byte should be appended to the output after each frame, or message. The *DELIMITER* byte is specified as a string number. A radix suffix can be used to denote the notation: (b) binary, (d) decimal, (h) hexadecimal, or (o) octal. If no radix suffix is specified, then hexadecimal notation is assumed. For example, the ASCII newline character ('\n') can be supplied as the *DELIMITER* using any of the following values: 1010b, 10d, 0Ah, 012o, or 0A.

\--delimited-input=*DELIMITER*
:   Indicates each frame, or message, within a stream of data is separated by a delimiter byte. The *DELIMITER* byte is specified as a string number. A radix suffix can be used to denote the notation: (b) binary, (d) decimal, (h) hexadecimal, or (o) octal. If no radix suffix is specified, then hexadecimal notation is assumed. For example, the ASCII newline character ('\n') can be supplied as the *DELIMITER* using any of the following values: 1010b, 10d, 0Ah, 012o, or 0A.

\--delimited-output=*DELIMITER*
:   Appends the *DELIMITER* byte to the end of the transcode frame, or message. The *DELIMITER* byte is specified as a string number. A radix suffix can be used to denote the notation: (b) binary, (d) decimal, (h) hexadecimal, or (o) octal. If no radix suffix is specified, then hexadecimal notation is assumed. For example, the ASCII newline character ('\n') can be supplied as the *DELIMITER* using any of the following values: 1010b, 10d, 0Ah, 012o, or 0A.

-f *FORMAT*, \--from=*FORMAT*
:   Specify input format. *FORMAT* can be `Bincode`, `CBOR`, `Envy`, `Hjson`, `JSON`, `Msgpack`, `Pickle`, `TOML`, `URL`, or `YAML`. The *FORMAT* is case insensitive. The default is `JSON`.

-o *FILE*, \--output=*FILE*
:   Write output to *FILE* instead of *stdout*. If the `-t,--to` option is not used, the file extension for *FILE* is used to determine the format for the output.

-r *RADIX*, \--radix=*RADIX*
:   Changes the output to be a space-separated list of bytes, where each byte is a numeric string with the *RADIX*. The serialized input data is transcoded to the output format specified with the `-t,--to` option, but it is written to the output as a string. This si useful for debugging serialization formats and creating an interactive console with binary output data. Note, if delimited-basd framing is employed, the delimiter byte is _not_ included in the space-separated list of bytes. *RADIX* can be `b`, `bin`, `binary`, `d`, `dec`, `decimal`, `h`, `hex`, `hexadecimal`, `o`, `oct`, or `octal` and it is case insensitive.

-s, \--sized
:   Indicates the first four bytes of the input is an unsigned 32-bit integer in Big Endian (Network Order), which is the total size in bytes of the input frame, or message, and prepends the total size in bytes of the serialized data to the output frame, or message.

\--sized-input
:   Indicates the first four bytes of the input is an unsigned 32-bit integer in Big Endian (Network Order), which is the total size in bytes of the input frame, or message.

\--sized-output
:   Prepends the total size of the serialized data as an unsigned 32-bit integer in Big Endian (Network Order) to the output frame, or message.

-t *FORMAT*, \--to=*FORMAT*
:   Specify output format. *FORMAT* can be `Bincode`, `CBOR`, `Hjson`, `JSON`, `Msgpack`, `Pickle`, `TOML`, `URL`, or `YAML`. The *FORMAT* is case insensitive. The default is `Msgpack`.

