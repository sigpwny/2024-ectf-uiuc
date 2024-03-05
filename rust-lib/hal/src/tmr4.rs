pub use max78000_pac::TMR4;

/// Get the current tick count
pub fn get_tick_count(tmr4: &TMR4) -> u16 {
    return tmr4.cnt().read().bits() as u16;
}

/// Configure TMR4 as an IBRO counter (for entropy)
/// - IBRO
/// - Continuous mode
/// - Range: 0x1 to 0xFFFF
pub fn config(tmr4: &TMR4) {
    disable(tmr4);
    // Set the TMR4 clock source to the IBRO
    // Safety: The clksel_a field is 2 bits wide, which fits the value 0b00
    tmr4.ctrl1().modify(|_, w| unsafe {
        w.clksel_a().bits(0b00)
    });
    tmr4.ctrl0().modify(|_, w| w
        .pol_a().clear_bit()        // Set polarity to active high
        .clkdiv_a().div_by_1()      // Set prescaler to divide by 1
        .mode_a().continuous()      // Set to continuous mode
    );
    // Set initial count to 0x1
    tmr4.cnt().write(|w| unsafe { w.bits(0x1) });
    // Set the compare value to 0xFFFF
    tmr4.cmp().write(|w| unsafe { w.bits(0xFFFF) });
    // Enable timer clock source
    tmr4.ctrl0().modify(|_, w| w.clken_a().set_bit());
    while tmr4.ctrl1().read().clkrdy_a().bit_is_clear() { }
    // Enable timer
    enable(tmr4);
}

/// Disable the TMR4 peripheral
fn disable(tmr4: &TMR4) {
    tmr4.ctrl0().modify(|_, w| w
        .en_a().clear_bit()
        .en_b().clear_bit()
    );
    while tmr4.ctrl1().read().clken_a().bit_is_set() { }
}

/// Enable the TMR4 peripheral
fn enable(tmr4: &TMR4) {
    tmr4.ctrl0().modify(|_, w| w
        .en_a().set_bit()
    );
    while tmr4.ctrl0().read().clken_a().bit_is_clear() { }
}
