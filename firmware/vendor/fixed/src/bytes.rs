// Copyright © 2018–2022 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

use core::{marker::PhantomData, slice};

#[derive(Clone, Copy, Debug)]
pub struct Bytes<'a> {
    ptr: *const u8,
    len: usize,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> Bytes<'a> {
    #[inline]
    pub const fn new(bytes: &'a [u8]) -> Bytes<'a> {
        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            phantom: PhantomData,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn slice(self) -> &'a [u8] {
        // SAFETY: points to a valid slice
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    #[inline]
    pub const fn len(self) -> usize {
        self.len
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    #[inline]
    pub const fn get(self, i: usize) -> u8 {
        assert!(i < self.len, "index out of bounds");
        let ptr = self.ptr.wrapping_add(i);
        // SAFETY: points to a valid slice, and bounds already checked
        unsafe { *ptr }
    }

    #[inline]
    pub const fn split(self, i: usize) -> (Bytes<'a>, Bytes<'a>) {
        let end_len = match self.len().checked_sub(i) {
            Some(s) => s,
            None => panic!("index out of bounds"),
        };
        (
            Bytes {
                ptr: self.ptr,
                len: i,
                phantom: PhantomData,
            },
            Bytes {
                ptr: self.ptr.wrapping_add(i),
                len: end_len,
                phantom: PhantomData,
            },
        )
    }
}
