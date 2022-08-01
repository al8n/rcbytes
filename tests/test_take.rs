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

use rcbytes::buf::Buf;
use rcbytes::Bytes;

#[test]
fn long_take() {
    // Tests that get a take with a size greater than the buffer length will not
    // overrun the buffer. Regression test for #138.
    let buf = b"hello world".take(100);
    assert_eq!(11, buf.remaining());
    assert_eq!(b"hello world", buf.chunk());
}

#[test]
fn take_copy_to_bytes() {
    let mut abcd = Bytes::copy_from_slice(b"abcd");
    let abcd_ptr = abcd.as_ptr();
    let mut take = (&mut abcd).take(2);
    let a = take.copy_to_bytes(1);
    assert_eq!(Bytes::copy_from_slice(b"a"), a);
    // assert `to_bytes` did not allocate
    assert_eq!(abcd_ptr, a.as_ptr());
    assert_eq!(Bytes::copy_from_slice(b"bcd"), abcd);
}

#[test]
#[should_panic]
fn take_copy_to_bytes_panics() {
    let abcd = Bytes::copy_from_slice(b"abcd");
    abcd.take(2).copy_to_bytes(3);
}
