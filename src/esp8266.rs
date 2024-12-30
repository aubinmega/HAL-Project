pub fn setup_gpio(pin: u8) {
    unsafe {
        let enable_reg = GPIO_ENABLE_W1TS;
        ptr::write_volatile(enable_reg, 1 << pin);
    }
}

pub fn set_gpio_high(pin: u8) {
    unsafe {
        let set_reg = GPIO_OUT_W1TS;
        ptr::write_volatile(set_reg, 1 << pin);
    }
}

pub fn set_gpio_low(pin: u8) {
    unsafe {
        let clear_reg = GPIO_OUT_W1TC;
        ptr::write_volatile(clear_reg, 1 << pin);
    }
}
