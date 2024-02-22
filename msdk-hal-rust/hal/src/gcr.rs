pub use max78000_pac::GCR;

/// Initialize the system clock to the Internal Primary Oscillator (IPO)
pub fn system_clock_ipo_init(gcr: &GCR) {
    // Enable the Internal Primary Oscillator (IPO)
    gcr.clkctrl().modify(|_, w| w.ipo_en().en());
    // Wait for IPO to be ready
    while gcr.clkctrl().read().ipo_rdy().bit_is_clear() { }
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
    // TODO: Configure GPIO callback initialization (4x32 null function pointers)
    // See msdk/Libraries/PeriphDrivers/Source/GPIO/gpio_common.c, MXC_GPIO_Common_Init
    while gcr.pclkdis0().read().gpio0().bit_is_set() { }
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