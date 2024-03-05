use max78000_pac::{GCR, LPGCR};

pub const CLOCK_HZ_IPO:         u32 = 100_000_000;
pub const CLOCK_HZ_ISO:         u32 = 60_000_000;
pub const CLOCK_HZ_SYSTEM:      u32 = CLOCK_HZ_IPO;
pub const CLOCK_HZ_PERIPHERAL:  u32 = CLOCK_HZ_SYSTEM / 2;

////////////////////////////////////////
// Oscillators
////////////////////////////////////////

/// Initialize the Internal Primary Oscillator (IPO)
pub fn ipo_init(gcr: &GCR) {
    gcr.clkctrl().modify(|_, w| w.ipo_en().en());
    while gcr.clkctrl().read().ipo_rdy().bit_is_clear() { }
}

/// Initialize the Internal Secondary Oscillator (ISO)
pub fn iso_init(gcr: &GCR) {
    gcr.clkctrl().modify(|_, w| w.iso_en().en());
    while gcr.clkctrl().read().iso_rdy().bit_is_clear() { }
}

/// Initialize the system clock to the IPO
pub fn system_clock_ipo_init(gcr: &GCR) {
    // Enable the Internal Primary Oscillator (IPO)
    ipo_init(gcr);
    // Set the system clock to IPO and the divisor to 1
    gcr.clkctrl().modify(|_, w| w
        .sysclk_sel().ipo()
        .sysclk_div().div1()
    );
    // Wait for the system clock to be ready
    while gcr.clkctrl().read().sysclk_rdy().bit_is_clear() { }
}

////////////////////////////////////////
// GPIOn Peripherals
////////////////////////////////////////

/// Enable the GPIO0 peripheral clock
pub fn mxc_gpio0_init(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.gpio0().clear_bit());
    while gcr.pclkdis0().read().gpio0().bit_is_set() { }
}

////////////////////////////////////////
// I2C1 Peripheral
////////////////////////////////////////

// Disable the I2C1 peripheral clock
pub fn mxc_i2c1_shutdown(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.i2c1().set_bit());
    while gcr.pclkdis0().read().i2c1().bit_is_clear() { }
}

// Reset the I2C1 peripheral
pub fn mxc_i2c1_reset(gcr: &GCR) {
    gcr.rst1().modify(|_, w| w.i2c1().set_bit());
    while gcr.rst1().read().i2c1().bit_is_clear() { }
}

// Enable the I2C1 peripheral clock
pub fn mxc_i2c1_enable_clock(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.i2c1().clear_bit());
    while gcr.pclkdis0().read().i2c1().bit_is_set() { }
}

////////////////////////////////////////
// TMR0 Peripheral
////////////////////////////////////////

/// Disable the TMR0 peripheral clock
pub fn mxc_tmr0_shutdown(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.tmr0().set_bit());
    while gcr.pclkdis0().read().tmr0().bit_is_clear() { }
}

/// Reset the TMR0 peripheral
pub fn mxc_tmr0_reset(gcr: &GCR) {
    gcr.rst0().modify(|_, w| w.tmr0().set_bit());
    while gcr.rst0().read().tmr0().bit_is_clear() { }
}

/// Enable the TMR0 peripheral clock
pub fn mxc_tmr0_enable_clock(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.tmr0().clear_bit());
    while gcr.pclkdis0().read().tmr0().bit_is_set() { }
}

////////////////////////////////////////
// TMR1 Peripheral
////////////////////////////////////////

/// Disable the TMR1 peripheral clock
pub fn mxc_tmr1_shutdown(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.tmr1().set_bit());
    while gcr.pclkdis0().read().tmr1().bit_is_clear() { }
}

/// Reset the TMR1 peripheral
pub fn mxc_tmr1_reset(gcr: &GCR) {
    gcr.rst0().modify(|_, w| w.tmr1().set_bit());
    while gcr.rst0().read().tmr1().bit_is_clear() { }
}

/// Enable the TMR1 peripheral clock
pub fn mxc_tmr1_enable_clock(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.tmr1().clear_bit());
    while gcr.pclkdis0().read().tmr1().bit_is_set() { }
}

////////////////////////////////////////
// TMR2 Peripheral
////////////////////////////////////////

/// Disable the TMR2 peripheral clock
pub fn mxc_tmr2_shutdown(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.tmr2().set_bit());
    while gcr.pclkdis0().read().tmr2().bit_is_clear() { }
}

/// Reset the TMR2 peripheral
pub fn mxc_tmr2_reset(gcr: &GCR) {
    gcr.rst0().modify(|_, w| w.tmr2().set_bit());
    while gcr.rst0().read().tmr2().bit_is_clear() { }
}

/// Enable the TMR1 peripheral clock
pub fn mxc_tmr2_enable_clock(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.tmr2().clear_bit());
    while gcr.pclkdis0().read().tmr2().bit_is_set() { }
}

////////////////////////////////////////
// TMR4 Peripheral
////////////////////////////////////////

/// Disable the TMR4 peripheral clock
pub fn mxc_tmr4_shutdown(lpgcr: &LPGCR) {
    lpgcr.pclkdis().modify(|_, w| w.tmr4().set_bit());
    while lpgcr.pclkdis().read().tmr4().bit_is_clear() { }
}

/// Reset the TMR4 peripheral
pub fn mxc_tmr4_reset(lpgcr: &LPGCR) {
    lpgcr.rst().modify(|_, w| w.tmr4().set_bit());
    while lpgcr.rst().read().tmr4().bit_is_clear() { }
}

/// Enable the TMR4 peripheral clock
pub fn mxc_tmr4_enable_clock(lpgcr: &LPGCR) {
    lpgcr.pclkdis().modify(|_, w| w.tmr4().clear_bit());
    while lpgcr.pclkdis().read().tmr4().bit_is_set() { }
}

////////////////////////////////////////
// TRNG Peripheral
////////////////////////////////////////

/// Disable the TRNG peripheral clock
pub fn mxc_trng_shutdown(gcr: &GCR) {
    gcr.pclkdis1().modify(|_, w| w.trng().set_bit());
    while gcr.pclkdis1().read().trng().bit_is_clear() { }
}

/// Reset the TRNG peripheral
pub fn mxc_trng_reset(gcr: &GCR) {
    gcr.rst0().modify(|_, w| w.trng().set_bit());
    while gcr.rst0().read().trng().bit_is_clear() { }
}

/// Enable the TRNG peripheral clock
pub fn mxc_trng_enable_clock(gcr: &GCR) {
    gcr.pclkdis1().modify(|_, w| w.trng().clear_bit());
    while gcr.pclkdis1().read().trng().bit_is_set() { }
}

////////////////////////////////////////
// UART0 Peripheral
////////////////////////////////////////

/// Disable the UART0 peripheral clock
pub fn mxc_uart0_shutdown(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.uart0().set_bit());
    while gcr.pclkdis0().read().uart0().bit_is_clear() { }
}

/// Reset the UART0 peripheral
pub fn mxc_uart0_reset(gcr: &GCR) {
    gcr.rst0().modify(|_, w| w.uart0().set_bit());
    while gcr.rst0().read().uart0().bit_is_clear() { }
}

/// Enable the UART0 peripheral clock
pub fn mxc_uart0_enable_clock(gcr: &GCR) {
    gcr.pclkdis0().modify(|_, w| w.uart0().clear_bit());
    while gcr.pclkdis0().read().uart0().bit_is_set() { }
}