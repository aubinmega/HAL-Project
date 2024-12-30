use core::ptr;

const DDRB: *mut u8 = 0x24 as *mut u8; // Data Direction Register B
const PORTB: *mut u8 = 0x25 as *mut u8; // Port B Data Register
const PINB5: u8 = 1 << 5; // Bit mask for Pin 5 on Port B

/// Set up the internal clock
pub fn setup_internal_clock() {
    unsafe {
        const CLKPR: *mut u8 = 0x61 as *mut u8; // Clock Prescaler Register
        ptr::write_volatile(CLKPR, 0x80); // Write CLKPCE (Clock Prescaler Change Enable)
        ptr::write_volatile(CLKPR, 0x00); // Set prescaler to no division (full speed)
    }
}

/// Configure GPIO for LED control
pub fn setup_gpio() {
    unsafe {
        let current_ddrb = ptr::read_volatile(DDRB);
        ptr::write_volatile(DDRB, current_ddrb | PINB5); // Set PINB5 as output
    }
}

/// Turn the LED on
pub fn led_on() {
    unsafe {
        let current_portb = ptr::read_volatile(PORTB);
        ptr::write_volatile(PORTB, current_portb | PINB5); // Set PINB5 high
    }
}

/// Turn the LED off
pub fn led_off() {
    unsafe {
        let current_portb = ptr::read_volatile(PORTB);
        ptr::write_volatile(PORTB, current_portb & !PINB5); // Set PINB5 low
    }
}

/// Delay function (blocking)
pub fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..16_000 {
            unsafe { ptr::read_volatile(0x00 as *const u8); }
        }
    }
}
