#![no_std]
#![no_main]

use core::ptr;
use avr_device::entry;
use panic_halt as _;

mod usart; // Include the USART module
use crate::usart::usart::{init_usart, transmit, receive}; // Import functions

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB5: u8 = 1 << 5;

#[entry]
fn main() -> ! {
    setup();
    init_usart(9600); // Initialize USART with a baud rate of 9600

    loop {
        led_on();
        transmit(b'L');
        transmit(b'E');
        transmit(b'D');
        transmit(b' ');
        transmit(b'O');
        transmit(b'N');
        transmit(b'\n');
        delay_ms(1000);

        led_off();
        transmit(b'L');
        transmit(b'E');
        transmit(b'D');
        transmit(b' ');
        transmit(b'O');
        transmit(b'F');
        transmit(b'F');
        transmit(b'\n');
        delay_ms(1000);

        let received = receive(); // Echo received data back
        transmit(received);
    }
}

fn setup() {
    unsafe {
        let current_ddrb = ptr::read_volatile(DDRB);
        ptr::write_volatile(DDRB, current_ddrb | PINB5);
    }
}

fn led_on() {
    unsafe {
        let current_portb = ptr::read_volatile(PORTB);
        ptr::write_volatile(PORTB, current_portb | PINB5);
    }
}

fn led_off() {
    unsafe {
        let current_portb = ptr::read_volatile(PORTB);
        ptr::write_volatile(PORTB, current_portb & !PINB5);
    }
}

fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..16_000 {
            unsafe { ptr::read_volatile(0x00 as *const u8); }
        }
    }
}
