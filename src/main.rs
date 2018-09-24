#![no_main]
#![no_std]

extern crate cortex_r_rt;
extern crate panic_abort;

use cortex_r_rt::{entry,exception};

#[exception]
fn SVCall() {
    static mut COUNTER: u32 = 0;
    
    loop {
        *COUNTER += 100;
    }
}

#[entry]
fn main() -> ! {
    static mut COUNT: u32 = 0;

    loop {
        *COUNT += 1;
    }
}
