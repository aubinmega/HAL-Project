pub mod usart {
    use core::ptr;

    const UBRR0H: *mut u8 = 0xC5 as *mut u8; // High byte of baud rate
    const UBRR0L: *mut u8 = 0xC4 as *mut u8; // Low byte of baud rate
    const UCSR0A: *mut u8 = 0xC0 as *mut u8; // Control and Status Register A
    const UCSR0B: *mut u8 = 0xC1 as *mut u8; // Control and Status Register B
    const UCSR0C: *mut u8 = 0xC2 as *mut u8; // Control and Status Register C
    const UDR0: *mut u8 = 0xC6 as *mut u8;   // USART I/O Data Register

    const RXEN0: u8 = 1 << 4; // Receiver Enable bit
    const TXEN0: u8 = 1 << 3; // Transmitter Enable bit

    const UCSZ01: u8 = 1 << 2; // Character Size bit 1
    const UCSZ00: u8 = 1 << 1; // Character Size bit 0

    /// Initialize USART with the given baud rate
    pub fn init_usart(baud_rate: u16) {
        let ubrr = ((16_000_000 / (16 * baud_rate as u32)) - 1) as u16;
        unsafe {
            ptr::write_volatile(UBRR0H, (ubrr >> 8) as u8); // Set baud rate high byte
            ptr::write_volatile(UBRR0L, ubrr as u8);        // Set baud rate low byte
            ptr::write_volatile(UCSR0B, RXEN0 | TXEN0);     // Enable receiver and transmitter
            ptr::write_volatile(UCSR0C, UCSZ01 | UCSZ00);   // 8-bit data, no parity, 1 stop bit
        }
    }

    /// Transmit a byte via USART
    pub fn transmit(data: u8) {
        unsafe {
            while ptr::read_volatile(UCSR0A) & (1 << 5) == 0 {} // Wait for empty transmit buffer
            ptr::write_volatile(UDR0, data); // Send data
        }
    }

    /// Receive a byte via USART
    pub fn receive() -> u8 {
        unsafe {
            while ptr::read_volatile(UCSR0A) & (1 << 7) == 0 {} // Wait for data to be received
            ptr::read_volatile(UDR0) // Return received data
        }
    }
}
