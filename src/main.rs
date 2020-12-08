#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

// The prelude just exports all HAL traits anonymously which makes
// all trait methods available.  This is probably something that
// should always be added.
use arduino_uno::prelude::*;

// Define the entry-point for the application.  This can only be
// done once in the entire dependency tree.
#[arduino_uno::entry]
fn main() -> ! {
    // Get the peripheral singletons for interacting with them.
    let dp = arduino_uno::Peripherals::take().unwrap();

    unimplemented!()
}
