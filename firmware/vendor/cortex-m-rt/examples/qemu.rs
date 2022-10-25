// #![feature(stdsimd)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as semihosting;

extern crate panic_halt;

use cortex_m::asm;
use rt::entry;

#[entry]
fn main() -> ! {
    use core::fmt::Write;
    let x = 42;

    loop {
        asm::nop();

        // write something through semihosting interface
        let mut hstdout = semihosting::hio::hstdout().unwrap();
        write!(hstdout, "x = {}\n", x).unwrap();
        // exit from qemu
        semihosting::debug::exit(semihosting::debug::EXIT_SUCCESS);
    }
}
