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
use crate::{Buf, Bytes};

use core::cmp;

/// A `Buf` adapter which limits the bytes read from an underlying buffer.
///
/// This struct is generally created by calling `take()` on `Buf`. See
/// documentation of [`take()`](trait.Buf.html#method.take) for more details.
#[derive(Debug)]
pub struct Take<T> {
    inner: T,
    limit: usize,
}

pub fn new<T>(inner: T, limit: usize) -> Take<T> {
    Take { inner, limit }
}

impl<T> Take<T> {
    /// Consumes this `Take`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::{Buf, BufMut};
    ///
    /// let mut buf = b"hello world".take(2);
    /// let mut dst = vec![];
    ///
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"he"[..]);
    ///
    /// let mut buf = buf.into_inner();
    ///
    /// dst.clear();
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"llo world"[..]);
    /// ```
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::Buf;
    ///
    /// let buf = b"hello world".take(2);
    ///
    /// assert_eq!(11, buf.get_ref().remaining());
    /// ```
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::{Buf, BufMut};
    ///
    /// let mut buf = b"hello world".take(2);
    /// let mut dst = vec![];
    ///
    /// buf.get_mut().advance(2);
    ///
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"ll"[..]);
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Returns the maximum number of bytes that can be read.
    ///
    /// # Note
    ///
    /// If the inner `Buf` has fewer bytes than indicated by this method then
    /// that is the actual number of available bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::Buf;
    ///
    /// let mut buf = b"hello world".take(2);
    ///
    /// assert_eq!(2, buf.limit());
    /// assert_eq!(b'h', buf.get_u8());
    /// assert_eq!(1, buf.limit());
    /// ```
    pub fn limit(&self) -> usize {
        self.limit
    }

    /// Sets the maximum number of bytes that can be read.
    ///
    /// # Note
    ///
    /// If the inner `Buf` has fewer bytes than `lim` then that is the actual
    /// number of available bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::{Buf, BufMut};
    ///
    /// let mut buf = b"hello world".take(2);
    /// let mut dst = vec![];
    ///
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"he"[..]);
    ///
    /// dst.clear();
    ///
    /// buf.set_limit(3);
    /// dst.put(&mut buf);
    /// assert_eq!(*dst, b"llo"[..]);
    /// ```
    pub fn set_limit(&mut self, lim: usize) {
        self.limit = lim
    }
}

impl<T: Buf> Buf for Take<T> {
    fn remaining(&self) -> usize {
        cmp::min(self.inner.remaining(), self.limit)
    }

    fn chunk(&self) -> &[u8] {
        let bytes = self.inner.chunk();
        &bytes[..cmp::min(bytes.len(), self.limit)]
    }

    fn advance(&mut self, cnt: usize) {
        assert!(cnt <= self.limit);
        self.inner.advance(cnt);
        self.limit -= cnt;
    }

    fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        assert!(len <= self.remaining(), "`len` greater than remaining");

        let r = self.inner.copy_to_bytes(len);
        self.limit -= len;
        r
    }
}
