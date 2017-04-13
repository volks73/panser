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

-f *FORMAT*, \--from=*FORMAT*
:   Specify input format.  *FORMAT* can be `Bincode`, `CBOR`, `Envy`, `Hjson`, `JSON`, `Msgpack`, `Pickle`, `TOML`, `URL`, or `YAML`. The *FORMAT* is case insensitive. The default is `JSON`.

-t *FORMAT*, \--to=*FORMAT*
:   Specify output format.  *FORMAT* can be `Bincode`, `CBOR`, `Hjson`, `JSON`, `Msgpack`, `Pickle`, `TOML`, `URL`, or `YAML`. The *FORMAT* is case insensitive. The default is `Msgpack`.

\--framed-input
:   Indicates the first four bytes of the input is an unsigned 32-bit integer in Big Endian (Network ORder) indicating the total length of the serialzied data.

\--framed-output
:   Prepends the total length of the serialized data as an unsigned 32-bit integer in Big Endian (Network Order).

\--include-newline
:   Write the newline character (0x0A) to output at the end of transcoding the data.

-o *FILE*, \--output=*FILE*
:   Write output to *FILE* instead of *stdout*. If the `-t,--to` option is not used, the file extension for *FILE* is used to determine the format for the output.

# EXAMPLES

Convert some JSON input to the Msgpack binary format. This is the default.

    echo '{"bool":true}' | panser

Convert some JSON input from a file to the Msgpack binary format.

    panser file.json

This is equivalent to using a redirection of the file to stdin.

    panser < file.json

Add framing to the output.

    echo '{"bool":true,"number":1.234} | panser --framed-output

Remove framing from input.

    panser -f Msgpack --framed-input framed.msgpack

# SEE ALSO

