#![no_std]

use core::panic::PanicInfo;
use max78000_pac as pac;
use max78000_hal::{gcr, gpio0, gpio2, i2c1, tmr0, uart0};

pub mod secure_comms;

pub enum Led {
    Red = 0,
    Green = 1,
    Blue = 2,
}

pub struct Board {
    // pub peripherals: pac::Peripherals,
    pub gcr: pac::GCR,
    pub gpio0: pac::GPIO0,
    pub gpio2: pac::GPIO2,
    pub tmr0: pac::TMR,
    pub tmr1: pac::TMR1,
    pub uart0: pac::UART,
}

impl Board {
    /// Create a new Board instance
    pub fn new() -> Self {
        // Safety: We only steal the peripherals once and we have exclusive access
        let p: pac::Peripherals = unsafe { pac::Peripherals::steal() };
        // Initialize the system clock
        gcr::system_clock_ipo_init(&p.GCR);
        // Initialize GPIO
        gcr::mxc_gpio0_init(&p.GCR);
        // Initialize UART0 for host communication
        gcr::mxc_uart0_shutdown(&p.GCR);
        gcr::mxc_uart0_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_UART0);
        uart0::config(&p.UART);
        // TODO: Initialize I2C1
        gcr::mxc_i2c1_shutdown(&p.GCR);
        gcr::mxc_i2c1_enable_clock(&p.GCR);
        // ...
        // Initialize TMR0 as continuous 32-bit system tick timer
        gcr::mxc_tmr0_shutdown(&p.GCR);
        gcr::mxc_tmr0_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_TMR0);
        tmr0::config_as_systick(&p.TMR);
        // Initialize LEDs
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_LED0);
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_LED1);
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_LED2);

        // Return the Board instance
        Board {
            // peripherals: p,
            gcr: p.GCR,
            gpio0: p.GPIO0,
            gpio2: p.GPIO2,
            tmr0: p.TMR,
            tmr1: p.TMR1,
            uart0: p.UART,
        }
    }

    /// Host debugging is only enabled in debug builds
    /// Output sent via UART0
    #[cfg(all(debug_assertions, not(feature = "semihosting")))]
    pub fn send_host_debug(&self, message: &[u8]) {
        uart0::send_bytes(&self.uart0, b"%debug ");
        uart0::send_bytes(&self.uart0, message);
        uart0::send_bytes(&self.uart0, b"%\r\n");
    }
    
    /// Host debugging is only enabled in debug builds
    /// Output sent via semihosting (requires a debugger to be attached)
    #[cfg(all(debug_assertions, feature = "semihosting"))]
    pub fn send_host_debug(&self, message: &[u8]) {
        use cortex_m_semihosting::{heprint, syscall};
        heprint!("%debug ");
        // Safety: Required to print type &[u8] to the host
        unsafe { syscall!(WRITE, 1, message.as_ptr(), message.len()) };
        heprint!("%\r\n");
    }

    /// Host debugging is disabled in release builds, so do nothing
    #[cfg(not(debug_assertions))]
    pub fn send_host_debug(&self, _message: &[u8]) {
        cortex_m::asm::nop();
    }

    pub fn led_on(&self, led: Led) {
        match led {
            Led::Red => gpio2::set_out(&self.gpio2, gpio2::GPIO2_CFG_LED0.pins),
            Led::Green => gpio2::set_out(&self.gpio2, gpio2::GPIO2_CFG_LED1.pins),
            Led::Blue => gpio2::set_out(&self.gpio2, gpio2::GPIO2_CFG_LED2.pins),
        }
    }

    pub fn led_off(&self, led: Led) {
        match led {
            Led::Red => gpio2::clr_out(&self.gpio2, gpio2::GPIO2_CFG_LED0.pins),
            Led::Green => gpio2::clr_out(&self.gpio2, gpio2::GPIO2_CFG_LED1.pins),
            Led::Blue => gpio2::clr_out(&self.gpio2, gpio2::GPIO2_CFG_LED2.pins),
        }
    }

    pub fn led_toggle(&self, led: Led) {
        match led {
            Led::Red => gpio2::toggle_out(&self.gpio2, gpio2::GPIO2_CFG_LED0.pins),
            Led::Green => gpio2::toggle_out(&self.gpio2, gpio2::GPIO2_CFG_LED1.pins),
            Led::Blue => gpio2::toggle_out(&self.gpio2, gpio2::GPIO2_CFG_LED2.pins),
        }
    }
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    // Safety: We're panicking, nothing is safe anymore
    let p = unsafe { pac::Peripherals::steal() };
    loop {
        // Blink the red LED to indicate a panic
        gpio2::set_out(&p.GPIO2, gpio2::GPIO2_CFG_LED0.pins);
        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }
        gpio2::clr_out(&p.GPIO2, gpio2::GPIO2_CFG_LED0.pins);
        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }
    }
}