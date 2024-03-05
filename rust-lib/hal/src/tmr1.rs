pub use max78000_pac::TMR1;
use crate::gcr::CLOCK_HZ_ISO;

/// Get the current time in microseconds (us)
pub fn get_time_us(tmr1: &TMR1) -> u32 {
    return get_tick_count(tmr1) / (CLOCK_HZ_ISO / 1_000_000);
}

/// Get the current tick count
pub fn get_tick_count(tmr1: &TMR1) -> u32 {
    return tmr1.cnt().read().bits();
}

/// Convert microseconds (us) to ticks
pub fn us_to_ticks(us: u32) -> u32 {
    return us * (CLOCK_HZ_ISO / 1_000_000);
}

/// Configure TMR1 as transaction one-shot timer
/// - 60MHz (ISO, no prescaler)
/// - One-shot mode
/// - 32-bit cascade mode
/// - Range: 0x1 to 0xFFFFFFFF
pub fn config(tmr1: &TMR1) {
    disable(tmr1);
    // Set the TMR1 clock source to the ISO
    // Safety: The clksel_a field is 2 bits wide, which fits the value 0b00
    tmr1.ctrl1().modify(|_, w| unsafe {
        w.clksel_a().bits(0b01)
    });
    // Set 32-bit cascade mode
    tmr1.ctrl1().modify(|_, w| w .cascade().set_bit());
    tmr1.ctrl0().modify(|_, w| w
        .pol_a().clear_bit()        // Set polarity to active high
        .clkdiv_a().div_by_1()      // Set prescaler to divide by 1
        .mode_a().one_shot()        // Set one-shot mode
    );
    // Set initial count to 0x1
    tmr1.cnt().write(|w| unsafe { w.bits(0x1) });
    // Set the compare value to 0xFFFFFFFF
    tmr1.cmp().write(|w| unsafe { w.bits(0xFFFFFFFF) });
    // Enable timer clock source
    tmr1.ctrl0().modify(|_, w| w.clken_a().set_bit());
    while tmr1.ctrl1().read().clkrdy_a().bit_is_clear() { }
    // Enable timer
    enable(tmr1);
}

/// Disable the TMR1 peripheral
fn disable(tmr1: &TMR1) {
    tmr1.ctrl0().modify(|_, w| w
        .en_a().clear_bit()
        .en_b().clear_bit()
    );
    while tmr1.ctrl1().read().clken_a().bit_is_set() { }
    while tmr1.ctrl1().read().clken_b().bit_is_set() { }
}

/// Enable the TMR1 peripheral
fn enable(tmr1: &TMR1) {
    tmr1.ctrl0().modify(|_, w| w
        .en_a().set_bit()
    );
    while tmr1.ctrl0().read().clken_a().bit_is_clear() { }
}
