#![no_std]               // Disables the standard library since we’re working with a microcontroller without an OS.
#![no_main]              // Indicates we provide our own entry function (`main`), rather than the standard Rust `main`.

use core::ptr;           // For safe access to read and write registers
use avr_device::entry;   // Provides the entry function for AVR microcontrollers
use panic_halt as _;     // Panic handler to stop the program in case of an error

// Register addresses and bit definitions for controlling pins on the microcontroller
const DDRB: *mut u8 = 0x24 as *mut u8;    // Direction register for port B
const PORTB: *mut u8 = 0x25 as *mut u8;   // Register for writing values to port B
const PINB5: u8 = 1 << 5;                 // Bit corresponding to pin PB5 (D13 on an Arduino Uno)

#[entry]
fn main() -> ! {
    setup();  // Initial configuration of the pin

    // Main loop to blink the LED
    loop {
        led_on();           // Turns the LED on by setting PB5 to high
        delay_ms(1000);     // Wait for 1 second

        led_off();          // Turns the LED off by setting PB5 to low
        delay_ms(1000);     // Wait for another second
    }
}

/// Configures pin D13 (PB5) as an output
fn setup() {
    unsafe {
        // Sets PB5 as output by setting the corresponding bit in DDRB
        let current_ddrb = ptr::read_volatile(DDRB);
        ptr::write_volatile(DDRB, current_ddrb | PINB5);
    }
}

/// Turns the LED on by setting PB5 to 1
fn led_on() {
    unsafe {
        // Sets PB5 to high (high voltage) to turn on the LED
        let current_portb = ptr::read_volatile(PORTB);
        ptr::write_volatile(PORTB, current_portb | PINB5);
    }
}

/// Turns the LED off by setting PB5 to 0
fn led_off() {
    unsafe {
        // Sets PB5 to low (low voltage) to turn off the LED
        let current_portb = ptr::read_volatile(PORTB);
        ptr::write_volatile(PORTB, current_portb & !PINB5);
    }
}

/// Delay function to introduce a delay in milliseconds
fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..16_000 {  // Approximately 1 ms for a 16 MHz processor
            // Reading from a null address as a NOP (no operation) to prevent compiler optimization
            unsafe { ptr::read_volatile(0x00 as *const u8); }
        }
    }
}
