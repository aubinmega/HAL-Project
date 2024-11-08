// gpio.rs
pub mod gpio {
    use core::ptr;

    const DDRB: *mut u8 = 0x24 as *mut u8;
    const PORTB: *mut u8 = 0x25 as *mut u8;

    pub enum Mode {
        Input,
        Output,
    }

    pub fn configure_pin(pin: u8, mode: Mode) {
        unsafe {
            let current_ddrb = ptr::read_volatile(DDRB);
            match mode {
                Mode::Input => ptr::write_volatile(DDRB, current_ddrb & !(1 << pin)),
                Mode::Output => ptr::write_volatile(DDRB, current_ddrb | (1 << pin)),
            }
        }
    }

    pub fn write_pin(pin: u8, value: bool) {
        unsafe {
            let current_portb = ptr::read_volatile(PORTB);
            if value {
                ptr::write_volatile(PORTB, current_portb | (1 << pin));
            } else {
                ptr::write_volatile(PORTB, current_portb & !(1 << pin));
            }
        }
    }
}