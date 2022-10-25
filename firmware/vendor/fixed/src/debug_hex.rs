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

#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{AtomicU32, Ordering};
use core::{
    cell::Cell,
    fmt::{Debug, Formatter, Result as FmtResult, Write},
};

// This is an ugly hack to check whether a `Formatter` has `debug_lower_hex` or
// `debug_upper_hex`.
//
// We do a dummy write with format string "{:x?}" to get `debug_lower_hex`, and
// a dummy write with format string "{:X?}" to get `debug_upper_hex`. Each time,
// we get the flags using the deprecated `Formatter::flags`.
//
// If AtomicU32 is supported, we cache the flags.

#[cfg(target_has_atomic = "32")]
static LOWER_FLAGS: AtomicU32 = AtomicU32::new(0);
#[cfg(target_has_atomic = "32")]
static UPPER_FLAGS: AtomicU32 = AtomicU32::new(0);

fn get_flags(f: &Formatter) -> u32 {
    #[allow(deprecated)]
    f.flags()
}

struct StoreFlags(Cell<u32>);

impl Debug for StoreFlags {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.set(get_flags(f));
        Ok(())
    }
}

struct Discard;

impl Write for Discard {
    fn write_str(&mut self, _s: &str) -> FmtResult {
        Ok(())
    }
}

pub enum IsDebugHex {
    No,
    Lower,
    Upper,
}

pub fn is_debug_hex(f: &Formatter) -> IsDebugHex {
    let flags = get_flags(f);
    // avoid doing unnecessary work if flags is zero
    if flags == 0 {
        return IsDebugHex::No;
    }

    let (lower_mask, upper_mask) = get_flag_masks();

    if flags & lower_mask != 0 {
        IsDebugHex::Lower
    } else if flags & upper_mask != 0 {
        IsDebugHex::Upper
    } else {
        IsDebugHex::No
    }
}

#[cfg(target_has_atomic = "32")]
fn load_cache() -> Option<(u32, u32)> {
    let cached_lower = LOWER_FLAGS.load(Ordering::Relaxed);
    let cached_upper = UPPER_FLAGS.load(Ordering::Relaxed);

    if cached_lower == u32::MAX || cached_upper == u32::MAX {
        // error was detected, so no need to repeat the error generation
        return Some((0, 0));
    }
    if cached_lower == 0 || cached_upper == 0 {
        return None;
    }
    Some((cached_lower, cached_upper))
}

#[cfg(target_has_atomic = "32")]
fn store_cache(lower: u32, upper: u32) {
    LOWER_FLAGS.store(lower, Ordering::Relaxed);
    UPPER_FLAGS.store(upper, Ordering::Relaxed);
}

#[cfg(not(target_has_atomic = "32"))]
fn load_cache() -> Option<(u32, u32)> {
    None
}

#[cfg(not(target_has_atomic = "32"))]
fn store_cache(_lower: u32, _upper: u32) {}

fn get_flag_masks() -> (u32, u32) {
    if let Some(cached) = load_cache() {
        return cached;
    }

    let store_flags = StoreFlags(Cell::new(0));
    if write!(Discard, "{:x?}", store_flags).is_err() {
        store_cache(u32::MAX, u32::MAX);
        return (0, 0);
    }
    let lower_flags = store_flags.0.get();
    if write!(Discard, "{:X?}", store_flags).is_err() {
        store_cache(u32::MAX, u32::MAX);
        return (0, 0);
    }
    let upper_flags = store_flags.0.get();
    let lower_mask = lower_flags & !upper_flags;
    let upper_mask = upper_flags & !lower_flags;
    store_cache(lower_mask, upper_mask);
    (lower_mask, upper_mask)
}
