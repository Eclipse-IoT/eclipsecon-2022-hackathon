//! Set the panicking behavior to reset
//!
//! This crate contains an implementation of `panic_fmt` that simply reset the chip.
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! use panic_reset as _;
//!
//! fn main() {
//!     panic!("argument is ignored");
//! }
//! ```
//!
//! # Breakable symbols
//!
//! With the panic handler being `#[inline(never)]` the symbol `rust_begin_unwind` will be
//! available to place a breakpoint on to halt when a panic is happening.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cortex_m::peripheral::SCB::sys_reset();
}
