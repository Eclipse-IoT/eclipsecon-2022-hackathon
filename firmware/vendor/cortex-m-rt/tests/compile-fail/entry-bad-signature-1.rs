#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::entry;

#[entry]
fn foo() {}
//~^ ERROR `#[entry]` function must have signature `[unsafe] fn() -> !`
