// main.rs
#![no_std]
#![no_main]

mod delay;
mod gpio;

use avr_device::entry;
use gpio::gpio::{configure_pin, write_pin, Mode};
use delay::delay::delay_ms;

#[entry]
fn main() -> ! {
    configure_pin(5, Mode::Output);

    loop {
        write_pin(5, true);
        delay_ms(1000);
        write_pin(5, false);
        delay_ms(1000);
    }
}





