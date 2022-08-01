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

/// Iterator over the bytes contained by the buffer.
///
/// This struct is created by the [`iter`] method on [`Buf`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rcbytes::Bytes;
///
/// let buf = Bytes::from(&b"abc"[..]);
/// let mut iter = buf.into_iter();
///
/// assert_eq!(iter.next(), Some(b'a'));
/// assert_eq!(iter.next(), Some(b'b'));
/// assert_eq!(iter.next(), Some(b'c'));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`iter`]: trait.Buf.html#method.iter
/// [`Buf`]: trait.Buf.html
#[derive(Debug)]
pub struct IntoIter<T> {
    inner: T,
}

impl<T> IntoIter<T> {
    /// Creates an iterator over the bytes contained by the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcbytes::Bytes;
    ///
    /// let buf = Bytes::from_static(b"abc");
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    /// assert_eq!(iter.next(), Some(b'b'));
    /// assert_eq!(iter.next(), Some(b'c'));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub(crate) fn new(inner: T) -> IntoIter<T> {
        IntoIter { inner }
    }

    /// Consumes this `IntoIter`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rcbytes::{Buf, Bytes};
    ///
    /// let buf = Bytes::from(&b"abc"[..]);
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// let buf = iter.into_inner();
    /// assert_eq!(2, buf.remaining());
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
    /// use rcbytes::{Buf, Bytes};
    ///
    /// let buf = Bytes::from(&b"abc"[..]);
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// assert_eq!(2, iter.get_ref().remaining());
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
    /// use rcbytes::{Buf, BytesMut};
    ///
    /// let buf = BytesMut::from(&b"abc"[..]);
    /// let mut iter = buf.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// iter.get_mut().advance(1);
    ///
    /// assert_eq!(iter.next(), Some(b'c'));
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Buf> Iterator for IntoIter<T> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if !self.inner.has_remaining() {
            return None;
        }

        let b = self.inner.chunk()[0];
        self.inner.advance(1);

        Some(b)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.inner.remaining();
        (rem, Some(rem))
    }
}

impl<T: Buf> ExactSizeIterator for IntoIter<T> {}
