// Copyright (C) 2017 Christopher R. Field.
//
// This file is part of Panser.
//
// Panser is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// Panser is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with Panser.  If not, see <http://www.gnu.org/licenses/>.

use std::env;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn exe_path() -> PathBuf {
    Path::new(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable"))
        .join("target")
        .join("debug")
        .join(&env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME environment variable"))
}

#[test]
fn it_works() {
    let process = Command::new(exe_path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all("{\"bool\":true}".as_bytes()).expect("Write to stdin");
    let mut buf: Vec<u8> = Vec::new();
    process.stdout.expect("stdout").read_to_end(&mut buf).expect("Read from stdout");
    assert_eq!(buf, vec![0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3]);
}

#[test]
fn hex_radix_works() {
    let process = Command::new(exe_path())
        .arg("-r")
        .arg("h")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all("{\"bool\":true}".as_bytes()).expect("Write to stdin");
    let mut buf = String::new();
    process.stdout.expect("stdout").read_to_string(&mut buf).expect("Read from stdout");
    assert_eq!(&buf, "81 A4 62 6F 6F 6C C3 ");
}

#[test]
fn dec_radix_works() {
    let process = Command::new(exe_path())
        .arg("-r")
        .arg("d")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all("{\"bool\":true}".as_bytes()).expect("Write to stdin");
    let mut buf = String::new();
    process.stdout.expect("stdout").read_to_string(&mut buf).expect("Read from stdout");
    assert_eq!(&buf, "129 164 98 111 111 108 195 ");
}

#[test]
fn bin_radix_works() {
    let process = Command::new(exe_path())
        .arg("-r")
        .arg("b")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all("{\"bool\":true}".as_bytes()).expect("Write to stdin");
    let mut buf = String::new();
    process.stdout.expect("stdout").read_to_string(&mut buf).expect("Read from stdout");
    assert_eq!(&buf, "10000001 10100100 1100010 1101111 1101111 1101100 11000011 ");
}

#[test]
fn oct_radix_works() {
    let process = Command::new(exe_path())
        .arg("-r")
        .arg("o")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all("{\"bool\":true}".as_bytes()).expect("Write to stdin");
    let mut buf = String::new();
    process.stdout.expect("stdout").read_to_string(&mut buf).expect("Read from stdout");
    assert_eq!(&buf, "201 244 142 157 157 154 303 ");
}

#[test]
fn sized_output_works() {
    let process = Command::new(exe_path())
        .arg("--sized-output")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all("{\"bool\":true}".as_bytes()).expect("Write to stdin");
    let mut buf: Vec<u8> = Vec::new();
    process.stdout.expect("stdout").read_to_end(&mut buf).expect("Read from stdout");
    assert_eq!(buf, vec![0x00, 0x00, 0x00, 0x07, 0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3]);
}

#[test]
fn sized_input_works() {
    let process = Command::new(exe_path())
        .arg("--sized-input")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all(&vec![0x00, 0x00, 0x00, 0x0d, 0x7b, 0x22, 0x62, 0x6f, 0x6f, 0x6c, 0x22, 0x3a, 0x74, 0x72, 0x75, 0x65, 0x7d]).expect("Write to stdin");
    let mut buf: Vec<u8> = Vec::new();
    process.stdout.expect("stdout").read_to_end(&mut buf).expect("Read from stdout");
    assert_eq!(buf, vec![0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3]);
}

#[test]
fn sized_works() {
    let process = Command::new(exe_path())
        .arg("--sized")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all(&vec![0x00, 0x00, 0x00, 0x0d, 0x7b, 0x22, 0x62, 0x6f, 0x6f, 0x6c, 0x22, 0x3a, 0x74, 0x72, 0x75, 0x65, 0x7d]).expect("Write to stdin");
    let mut buf: Vec<u8> = Vec::new();
    process.stdout.expect("stdout").read_to_end(&mut buf).expect("Read from stdout");
    assert_eq!(buf, vec![0x00, 0x00, 0x00, 0x07, 0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3]);
}

#[test]
fn delimited_works() {
    let process = Command::new(exe_path())
        .arg("-d")
        .arg("0Ah")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all(&vec![0x7b, 0x22, 0x62, 0x6f, 0x6f, 0x6c, 0x22, 0x3a, 0x74, 0x72, 0x75, 0x65, 0x7d, 0x0A]).expect("Write to stdin");
    let mut buf: Vec<u8> = Vec::new();
    process.stdout.expect("stdout").read_to_end(&mut buf).expect("Read from stdout");
    assert_eq!(buf, vec![0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0x0A]);
}

#[test]
fn delimited_input_works() {
    let process = Command::new(exe_path())
        .arg("--delimited-input")
        .arg("0Ah")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all(&vec![0x7b, 0x22, 0x62, 0x6f, 0x6f, 0x6c, 0x22, 0x3a, 0x74, 0x72, 0x75, 0x65, 0x7d, 0x0A]).expect("Write to stdin");
    let mut buf: Vec<u8> = Vec::new();
    process.stdout.expect("stdout").read_to_end(&mut buf).expect("Read from stdout");
    assert_eq!(buf, vec![0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3]);
}

#[test]
fn delimited_output_works() {
    let process = Command::new(exe_path())
        .arg("--delimited-output")
        .arg("0Ah")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all(&vec![0x7b, 0x22, 0x62, 0x6f, 0x6f, 0x6c, 0x22, 0x3a, 0x74, 0x72, 0x75, 0x65, 0x7d]).expect("Write to stdin");
    let mut buf: Vec<u8> = Vec::new();
    process.stdout.expect("stdout").read_to_end(&mut buf).expect("Read from stdout");
    assert_eq!(buf, vec![0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0x0A]);
}

#[test]
fn delimited_input_sized_output_works() {
    let process = Command::new(exe_path())
        .arg("--delimited-input")
        .arg("0Ah")
        .arg("--sized-output")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all(&vec![0x7b, 0x22, 0x62, 0x6f, 0x6f, 0x6c, 0x22, 0x3a, 0x74, 0x72, 0x75, 0x65, 0x7d, 0x0A]).expect("Write to stdin");
    let mut buf: Vec<u8> = Vec::new();
    process.stdout.expect("stdout").read_to_end(&mut buf).expect("Read from stdout");
    assert_eq!(buf, vec![0x00, 0x00, 0x00, 0x07, 0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3]);
}

#[test]
fn sized_input_delimited_output_works() {
    let process = Command::new(exe_path())
        .arg("--sized-input")
        .arg("--delimited-output")
        .arg("0Ah")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Create process");
    process.stdin.expect("stdin").write_all(&vec![0x00, 0x00, 0x00, 0x0d, 0x7b, 0x22, 0x62, 0x6f, 0x6f, 0x6c, 0x22, 0x3a, 0x74, 0x72, 0x75, 0x65, 0x7d]).expect("Write to stdin");
    let mut buf: Vec<u8> = Vec::new();
    process.stdout.expect("stdout").read_to_end(&mut buf).expect("Read from stdout");
    assert_eq!(buf, vec![0x81, 0xa4, 0x62, 0x6f, 0x6f, 0x6c, 0xc3, 0x0A]);
}

