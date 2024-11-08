// delay.rs
pub mod delay {
    use core::ptr;

    pub fn delay_ms(ms: u16) {
        for _ in 0..ms {
            for _ in 0..16_000 {
                unsafe { ptr::read_volatile(0x00 as *const u8); }
            }
        }
    }
}