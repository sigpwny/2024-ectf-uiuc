#![no_std]
#![no_main]

use panic_halt as _;
// use panic_semihosting as _;
use cortex_m_rt::entry;
use max78000_pac::Peripherals;
use libmsdk_sys as ffi;

#[entry]
fn main() -> ! {
    // Safety: We only steal once and we have exclusive access to the peripherals
    let peripherals: Peripherals = unsafe { Peripherals::steal() };
    system_clock_ipo_init(&peripherals);
    // Safety: Setup LED
    unsafe {
        ffi::LED_Init();
        ffi::LED_On(1);
    }
    loop {
        // ffi::LED_On(1);
        continue
    }
}

fn system_clock_ipo_init(peripherals: &Peripherals) {
    // Enable the Internal Primary Oscillator (IPO)
    peripherals.GCR.clkctrl().modify(|_, w| w.ipo_en().en());
    // Wait for IPO to be ready
    while peripherals.GCR.clkctrl().read().ipo_rdy().bit_is_clear() { }
    // Set the system clock to IPO and the divisor to 1
    peripherals.GCR.clkctrl().modify(|_, w| w
        .sysclk_sel().ipo()
        .sysclk_div().div1()
    );
    // Wait for the system clock to be ready
    while peripherals.GCR.clkctrl().read().sysclk_rdy().bit_is_clear() { }
}