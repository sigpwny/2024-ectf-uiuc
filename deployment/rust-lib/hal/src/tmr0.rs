pub use max78000_pac::TMR;
use crate::gcr::CLOCK_HZ_PERIPHERAL;

/// Get the current time in microseconds (us)
pub fn get_time_us(tmr0: &TMR) -> u32 {
    return get_tick_count(tmr0) / (CLOCK_HZ_PERIPHERAL / 1_000_000);
}

/// Get the current tick count
pub fn get_tick_count(tmr0: &TMR) -> u32 {
    return tmr0.cnt().read().bits();
}

/// Configure TMR0
/// - 50MHz (peripheral clock, no prescaler)
/// - One-shot mode
/// - 32-bit cascade mode
/// - Range: 0x1 to 0xFFFFFFFF
pub fn config(tmr0: &TMR) {
    disable(tmr0);
    // Set the TMR0 clock source to the peripheral clock
    // Safety: The clksel_a field is 2 bits wide, which fits the value 0b00
    tmr0.ctrl1().modify(|_, w| unsafe {
        w.clksel_a().bits(0b00)
    });
    // Set 32-bit cascade mode
    tmr0.ctrl1().modify(|_, w| w .cascade().set_bit());
    tmr0.ctrl0().modify(|_, w| w
        .pol_a().clear_bit()        // Set polarity to active high
        .clkdiv_a().div_by_1()      // Set prescaler to divide by 1
        .mode_a().one_shot()        // Set one-shot mode
    );
    // Set initial count to 0x1
    tmr0.cnt().write(|w| unsafe { w.bits(0x1) });
    // Set the compare value to 0xFFFFFFFF
    tmr0.cmp().write(|w| unsafe { w.bits(0xFFFFFFFF) });
    // Enable timer clock source
    tmr0.ctrl0().modify(|_, w| w.clken_a().set_bit());
    while tmr0.ctrl1().read().clkrdy_a().bit_is_clear() { }
    // Enable timer
    enable(tmr0);
}

/// Disable the TMR0 peripheral
fn disable(tmr0: &TMR) {
    tmr0.ctrl0().modify(|_, w| w
        .en_a().clear_bit()
        .en_b().clear_bit()
    );
    while tmr0.ctrl1().read().clken_a().bit_is_set() { }
    while tmr0.ctrl1().read().clken_b().bit_is_set() { }
}

/// Enable the TMR0 peripheral
fn enable(tmr0: &TMR) {
    tmr0.ctrl0().modify(|_, w| w
        .en_a().set_bit()
        // .en_b().set_bit()
    );
    while tmr0.ctrl0().read().clken_a().bit_is_clear() { }
    // while p.TMR.ctrl0().clken_b().bit_is_clear() { }
}
