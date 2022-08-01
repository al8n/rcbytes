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
use crate::BufMut;

use std::{cmp, io};

/// A `BufMut` adapter which implements `io::Write` for the inner value.
///
/// This struct is generally created by calling `writer()` on `BufMut`. See
/// documentation of [`writer()`](trait.BufMut.html#method.writer) for more
/// details.
#[derive(Debug)]
pub struct Writer<B> {
    buf: B,
}

pub fn new<B>(buf: B) -> Writer<B> {
    Writer { buf }
}

impl<B: BufMut> Writer<B> {
    /// Gets a reference to the underlying `BufMut`.
    ///
    /// It is inadvisable to directly write to the underlying `BufMut`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::BufMut;
    ///
    /// let buf = Vec::with_capacity(1024).writer();
    ///
    /// assert_eq!(1024, buf.get_ref().capacity());
    /// ```
    pub fn get_ref(&self) -> &B {
        &self.buf
    }

    /// Gets a mutable reference to the underlying `BufMut`.
    ///
    /// It is inadvisable to directly write to the underlying `BufMut`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::BufMut;
    ///
    /// let mut buf = vec![].writer();
    ///
    /// buf.get_mut().reserve(1024);
    ///
    /// assert_eq!(1024, buf.get_ref().capacity());
    /// ```
    pub fn get_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    /// Consumes this `Writer`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::BufMut;
    /// use std::io;
    ///
    /// let mut buf = vec![].writer();
    /// let mut src = &b"hello world"[..];
    ///
    /// io::copy(&mut src, &mut buf).unwrap();
    ///
    /// let buf = buf.into_inner();
    /// assert_eq!(*buf, b"hello world"[..]);
    /// ```
    pub fn into_inner(self) -> B {
        self.buf
    }
}

impl<B: BufMut + Sized> io::Write for Writer<B> {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        let n = cmp::min(self.buf.remaining_mut(), src.len());

        self.buf.put(&src[0..n]);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
