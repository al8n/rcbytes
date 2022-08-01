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
//! Test using `Bytes` with an allocator that hands out "odd" pointers for
//! vectors (pointers where the LSB is set).

#![cfg(not(miri))] // Miri does not support custom allocators (also, Miri is "odd" by default with 50% chance)

use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr;

use rcbytes::Bytes;

#[global_allocator]
static ODD: Odd = Odd;

struct Odd;

unsafe impl GlobalAlloc for Odd {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() == 1 && layout.size() > 0 {
            // Allocate slightly bigger so that we can offset the pointer by 1
            let size = layout.size() + 1;
            let new_layout = match Layout::from_size_align(size, 1) {
                Ok(layout) => layout,
                Err(_err) => return ptr::null_mut(),
            };
            let ptr = System.alloc(new_layout);
            if !ptr.is_null() {
                ptr.offset(1)
            } else {
                ptr
            }
        } else {
            System.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.align() == 1 && layout.size() > 0 {
            let size = layout.size() + 1;
            let new_layout = match Layout::from_size_align(size, 1) {
                Ok(layout) => layout,
                Err(_err) => std::process::abort(),
            };
            System.dealloc(ptr.offset(-1), new_layout);
        } else {
            System.dealloc(ptr, layout);
        }
    }
}

#[test]
fn sanity_check_odd_allocator() {
    let vec = vec![33u8; 1024];
    let p = vec.as_ptr() as usize;
    assert!(p & 0x1 == 0x1, "{:#b}", p);
}

#[test]
fn test_bytes_from_vec_drop() {
    let vec = vec![33u8; 1024];
    let _b = Bytes::from(vec);
}

#[test]
fn test_bytes_clone_drop() {
    let vec = vec![33u8; 1024];
    let b1 = Bytes::from(vec);
    let _b2 = b1.clone();
}

#[test]
fn test_bytes_into_vec() {
    let vec = vec![33u8; 1024];

    // Test cases where kind == KIND_VEC
    let b1 = Bytes::from(vec.clone());
    assert_eq!(Vec::from(b1), vec);

    // Test cases where kind == KIND_ARC, ref_cnt == 1
    let b1 = Bytes::from(vec.clone());
    drop(b1.clone());
    assert_eq!(Vec::from(b1), vec);

    // Test cases where kind == KIND_ARC, ref_cnt == 2
    let b1 = Bytes::from(vec.clone());
    let b2 = b1.clone();
    assert_eq!(Vec::from(b1), vec);

    // Test cases where vtable = SHARED_VTABLE, kind == KIND_ARC, ref_cnt == 1
    assert_eq!(Vec::from(b2), vec);

    // Test cases where offset != 0
    let mut b1 = Bytes::from(vec.clone());
    let b2 = b1.split_off(20);

    assert_eq!(Vec::from(b2), vec[20..]);
    assert_eq!(Vec::from(b1), vec[..20]);
}
