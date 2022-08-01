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

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![no_std]

//! Provides abstractions for working with bytes.
//!
//! The `bytes` crate provides an efficient byte buffer structure
//! ([`Bytes`](struct.Bytes.html)) and traits for working with buffer
//! implementations ([`Buf`], [`BufMut`]).
//!
//! [`Buf`]: trait.Buf.html
//! [`BufMut`]: trait.BufMut.html
//!
//! # `Bytes`
//!
//! `Bytes` is an efficient container for storing and operating on contiguous
//! slices of memory. It is intended for use primarily in networking code, but
//! could have applications elsewhere as well.
//!
//! `Bytes` values facilitate zero-copy network programming by allowing multiple
//! `Bytes` objects to point to the same underlying memory. This is managed by
//! using a reference count to track when the memory is no longer needed and can
//! be freed.
//!
//! A `Bytes` handle can be created directly from an existing byte store (such as `&[u8]`
//! or `Vec<u8>`), but usually a `BytesMut` is used first and written to. For
//! example:
//!
//! ```rust
//! use rcbytes::{BytesMut, BufMut};
//!
//! let mut buf = BytesMut::with_capacity(1024);
//! buf.put(&b"hello world"[..]);
//! buf.put_u16(1234);
//!
//! let a = buf.split();
//! assert_eq!(a, b"hello world\x04\xD2"[..]);
//!
//! buf.put(&b"goodbye world"[..]);
//!
//! let b = buf.split();
//! assert_eq!(b, b"goodbye world"[..]);
//!
//! assert_eq!(buf.capacity(), 998);
//! ```
//!
//! In the above example, only a single buffer of 1024 is allocated. The handles
//! `a` and `b` will share the underlying buffer and maintain indices tracking
//! the view into the buffer represented by the handle.
//!
//! See the [struct docs] for more details.
//!
//! [struct docs]: struct.Bytes.html
//!
//! # `Buf`, `BufMut`
//!
//! These two traits provide read and write access to buffers. The underlying
//! storage may or may not be in contiguous memory. For example, `Bytes` is a
//! buffer that guarantees contiguous memory, but a [rope] stores the bytes in
//! disjoint chunks. `Buf` and `BufMut` maintain cursors tracking the current
//! position in the underlying byte storage. When bytes are read or written, the
//! cursor is advanced.
//!
//! [rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)
//!
//! ## Relation with `Read` and `Write`
//!
//! At first glance, it may seem that `Buf` and `BufMut` overlap in
//! functionality with `std::io::Read` and `std::io::Write`. However, they
//! serve different purposes. A buffer is the value that is provided as an
//! argument to `Read::read` and `Write::write`. `Read` and `Write` may then
//! perform a syscall, which has the potential of failing. Operations on `Buf`
//! and `BufMut` are infallible.

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod buf;
pub use crate::buf::{Buf, BufMut};

mod bytes;
mod bytes_mut;
mod fmt;
pub use crate::bytes::Bytes;
pub use crate::bytes_mut::BytesMut;

// Optional Serde support
#[cfg(feature = "serde")]
mod serde;

#[inline(never)]
#[cold]
fn abort() -> ! {
    #[cfg(feature = "std")]
    {
        std::process::abort();
    }

    #[cfg(not(feature = "std"))]
    {
        struct Abort;
        impl Drop for Abort {
            fn drop(&mut self) {
                panic!();
            }
        }
        let _a = Abort;
        panic!("abort");
    }
}
