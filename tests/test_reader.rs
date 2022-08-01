// The code in this file is heavily based on [Carl Lerche's LRU implementation](https://github.com/tokio-rs/bytes).
//
// MIT License
//
// Copyright (c) 2022 Al Liu (https://github.com/al8n/rcbytes)
//
// Copyright (c) 2018 Carl Lerche (https://github.com/tokio-rs/bytes)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
#![warn(rust_2018_idioms)]
#![cfg(feature = "std")]

use std::io::{BufRead, Read};

use rcbytes::Buf;

#[test]
fn read() {
    let buf1 = &b"hello "[..];
    let buf2 = &b"world"[..];
    let buf = Buf::chain(buf1, buf2); // Disambiguate with Read::chain
    let mut buffer = Vec::new();
    buf.reader().read_to_end(&mut buffer).unwrap();
    assert_eq!(b"hello world", &buffer[..]);
}

#[test]
fn buf_read() {
    let buf1 = &b"hell"[..];
    let buf2 = &b"o\nworld"[..];
    let mut reader = Buf::chain(buf1, buf2).reader();
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert_eq!("hello\n", &line);
    line.clear();
    reader.read_line(&mut line).unwrap();
    assert_eq!("world", &line);
}
