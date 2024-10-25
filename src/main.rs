#![no_std]
#![no_main]

use core::ptr;
use avr_device::entry;
use panic_halt as _;

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB5: u8 = 1 << 5;

#[entry]
fn main() -> ! {
    setup();

    loop {
        led_on();
        delay_ms(1000);
        led_off();
        delay_ms(1000);
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
