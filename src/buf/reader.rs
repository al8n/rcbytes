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
use crate::Buf;

use std::{cmp, io};

/// A `Buf` adapter which implements `io::Read` for the inner value.
///
/// This struct is generally created by calling `reader()` on `Buf`. See
/// documentation of [`reader()`](trait.Buf.html#method.reader) for more
/// details.
#[derive(Debug)]
pub struct Reader<B> {
    buf: B,
}

pub fn new<B>(buf: B) -> Reader<B> {
    Reader { buf }
}

impl<B: Buf> Reader<B> {
    /// Gets a reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::Buf;
    ///
    /// let buf = b"hello world".reader();
    ///
    /// assert_eq!(b"hello world", buf.get_ref());
    /// ```
    pub fn get_ref(&self) -> &B {
        &self.buf
    }

    /// Gets a mutable reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    pub fn get_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    /// Consumes this `Reader`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::Buf;
    /// use std::io;
    ///
    /// let mut buf = b"hello world".reader();
    /// let mut dst = vec![];
    ///
    /// io::copy(&mut buf, &mut dst).unwrap();
    ///
    /// let buf = buf.into_inner();
    /// assert_eq!(0, buf.remaining());
    /// ```
    pub fn into_inner(self) -> B {
        self.buf
    }
}

impl<B: Buf + Sized> io::Read for Reader<B> {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        let len = cmp::min(self.buf.remaining(), dst.len());

        Buf::copy_to_slice(&mut self.buf, &mut dst[0..len]);
        Ok(len)
    }
}

impl<B: Buf + Sized> io::BufRead for Reader<B> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(self.buf.chunk())
    }
    fn consume(&mut self, amt: usize) {
        self.buf.advance(amt)
    }
}
