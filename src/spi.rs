#[cfg(feature = "atmega328p")]
pub mod atmega328p_spi {
    use core::ptr;

    const SPCR: *mut u8 = 0x4C as *mut u8; // SPI Control Register
    const SPSR: *mut u8 = 0x4D as *mut u8; // SPI Status Register
    const SPDR: *mut u8 = 0x4E as *mut u8; // SPI Data Register

    const SPE: u8 = 1 << 6;   // SPI Enable
    const MSTR: u8 = 1 << 4;  // Master mode
    const SPR0: u8 = 1 << 0;  // Clock rate select

    pub fn init_spi() {
        unsafe {
            ptr::write_volatile(SPCR, SPE | MSTR | SPR0); // Enable SPI, Master mode, Clock rate fosc/16
        }
    }

    pub fn transfer(data: u8) -> u8 {
        unsafe {
            ptr::write_volatile(SPDR, data); // Load data into SPI data register
            while ptr::read_volatile(SPSR) & (1 << 7) == 0 {} // Wait for transmission complete
            ptr::read_volatile(SPDR) // Return received data
        }
    }
}

#[cfg(feature = "esp8266")]
pub mod esp8266_spi {
    use core::ptr;

    const SPI_CMD: *mut u32 = 0x60000200 as *mut u32; // SPI Command Register
    const SPI_W0: *mut u32 = 0x60000240 as *mut u32;  // SPI Data Register

    pub fn init_spi() {
        // Stub for SPI initialization
    }

    pub fn transfer(data: u8) -> u8 {
        unsafe {
            ptr::write_volatile(SPI_W0, data as u32);
            ptr::write_volatile(SPI_CMD, 1);

            while ptr::read_volatile(SPI_CMD) & 1 != 0 {}
            ptr::read_volatile(SPI_W0) as u8
        }
    }
}
