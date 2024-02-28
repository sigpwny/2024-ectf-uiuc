pub use max78000_pac::UART;

/// Write a byte to UART0
pub fn write_byte(uart0: &UART, byte: u8) {
    while is_tx_full(uart0) { }
    // Safety: The data field is 8 bits wide, which fits the byte
    uart0.fifo().write(|w| unsafe { w.data().bits(byte) });
}

/// Write bytes to UART0
pub fn write_bytes(uart0: &UART, bytes: &[u8]) {
    for byte in bytes {
        write_byte(uart0, *byte);
    }
}

/// Read a byte from UART0
pub fn read_byte(uart0: &UART) -> u8 {
    while is_rx_empty(uart0) { }
    return uart0.fifo().read().data().bits();
}

/// Read bytes from UART0
pub fn read_bytes(uart0: &UART, bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        *byte = read_byte(uart0);
    }
}

/// Checks if the UART0 TX FIFO is full
pub fn is_tx_full(uart0: &UART) -> bool {
    return uart0.status().read().tx_full().bit_is_set();
}

/// Checks if the UART0 RX FIFO is empty
pub fn is_rx_empty(uart0: &UART) -> bool {
    return uart0.status().read().rx_em().bit_is_set();
}

/// Configure UART0 for 115200 baud, 8N1 operation
pub fn config(uart0: &UART) {
    // Set RX threshold to 1 byte
    // Safety: The rx_thd_val field is 4 bits wide, which fits the value 1
    uart0.ctrl().write(|w| unsafe { w.rx_thd_val().bits(1) });
    // Set the UART clock and configure 115200 baud 8N1
    uart0.ctrl().write(|w| w
        .ucagm().set_bit()
        .bclksrc().peripheral_clock()
        .bclken().set_bit()
        .stopbits().clear_bit()
        .char_size()._8bits()
        .par_en().clear_bit()
    );
    // The Peripheral Clock is 50MHz (System Clock IPO 100MHz / 2)
    // 50MHz / 115200 ~= 434
    // Safety: The clkdiv field is 19 bits wide, which fits the divisor 434
    uart0.clkdiv().write(|w| unsafe { w.clkdiv().bits(434) });
    // Wait until baud clock is ready
    while !uart0.ctrl().read().bclkrdy().bit_is_set() {}
}