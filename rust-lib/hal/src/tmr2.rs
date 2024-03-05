pub use max78000_pac::TMR2;

/// Get the current tick count
pub fn get_tick_count(tmr2: &TMR2) -> u32 {
    return tmr2.cnt().read().bits();
}

/// Configure TMR2 as an INRO counter (for entropy)
/// - INRO
/// - Continuous mode
/// - 32-bit cascade mode
/// - Range: 0x1 to 0xFFFFFFFF
pub fn config(tmr2: &TMR2) {
    disable(tmr2);
    // Set the TMR2 clock source to the INRO
    // Safety: The clksel_a field is 2 bits wide, which fits the value 0b00
    tmr2.ctrl1().modify(|_, w| unsafe {
        w.clksel_a().bits(0b10)
    });
    // Set mode to cascade
    tmr2.ctrl1().modify(|_, w| w.cascade().set_bit());
    // Set 32-bit cascade mode
    tmr2.ctrl0().modify(|_, w| w
        .pol_a().clear_bit()        // Set polarity to active high
        .clkdiv_a().div_by_1()      // Set prescaler to divide by 1
        .mode_a().continuous()      // Set continuous mode
    );
    // Set initial count to 0x1
    tmr2.cnt().write(|w| unsafe { w.bits(0x1) });
    // Set the compare value to 0xFFFFFFFF
    tmr2.cmp().write(|w| unsafe { w.bits(0xFFFFFFFF) });
    // Enable timer clock source
    tmr2.ctrl0().modify(|_, w| w.clken_a().set_bit());
    while tmr2.ctrl1().read().clkrdy_a().bit_is_clear() { }
    // Enable timer
    enable(tmr2);
}

/// Disable the TMR2 peripheral
fn disable(tmr2: &TMR2) {
    tmr2.ctrl0().modify(|_, w| w
        .en_a().clear_bit()
        .en_b().clear_bit()
    );
    while tmr2.ctrl1().read().clken_a().bit_is_set() { }
    while tmr2.ctrl1().read().clken_b().bit_is_set() { }
}

/// Enable the TMR2 peripheral
fn enable(tmr2: &TMR2) {
    tmr2.ctrl0().modify(|_, w| w
        .en_a().set_bit()
    );
    while tmr2.ctrl0().read().clken_a().bit_is_clear() { }
}
