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
//! Utilities for working with buffers.
//!
//! A buffer is any structure that contains a sequence of bytes. The bytes may
//! or may not be stored in contiguous memory. This module contains traits used
//! to abstract over buffers as well as utilities for working with buffer types.
//!
//! # `Buf`, `BufMut`
//!
//! These are the two foundational traits for abstractly working with buffers.
//! They can be thought as iterators for byte structures. They offer additional
//! performance over `Iterator` by providing an API optimized for byte slices.
//!
//! See [`Buf`] and [`BufMut`] for more details.
//!
//! [rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)
//! [`Buf`]: trait.Buf.html
//! [`BufMut`]: trait.BufMut.html

mod buf_impl;
mod buf_mut;
mod chain;
mod iter;
mod limit;
#[cfg(feature = "std")]
mod reader;
mod take;
mod uninit_slice;
mod vec_deque;
#[cfg(feature = "std")]
mod writer;

pub use self::buf_impl::Buf;
pub use self::buf_mut::BufMut;
pub use self::chain::Chain;
pub use self::iter::IntoIter;
pub use self::limit::Limit;
pub use self::take::Take;
pub use self::uninit_slice::UninitSlice;

#[cfg(feature = "std")]
pub use self::{reader::Reader, writer::Writer};
