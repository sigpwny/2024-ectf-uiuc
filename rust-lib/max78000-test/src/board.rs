#![no_std]

use core::panic::PanicInfo;
use max78000_pac as pac;
use max78000_hal::{*};

pub mod secure_comms;

pub enum Led {
    Red = 0,
    Green = 1,
    Blue = 2,
}

pub struct Board {
    // pub peripherals: pac::Peripherals,
    pub flc: pac::FLC,
    pub gcr: pac::GCR,
    pub gpio0: pac::GPIO0,
    pub gpio2: pac::GPIO2,
    pub i2c1: pac::I2C1,
    pub tmr0: pac::TMR,
    pub tmr1: pac::TMR1,
    pub trng: pac::TRNG,
    pub uart0: pac::UART,
}

impl Board {
    /// Create a new Board instance
    pub fn new() -> Self {
        // Safety: We only steal the peripherals once and we have exclusive access
        let p: pac::Peripherals = unsafe { pac::Peripherals::steal() };
        // Initialize the system clock
        gcr::system_clock_ipo_init(&p.GCR);
        // Disable ICC
        p.ICC0.ctrl().modify(|_, w| w.en().clear_bit());
        // Initialize GPIO
        gcr::mxc_gpio0_init(&p.GCR);
        // Initialize UART0 for host communication
        gcr::mxc_uart0_shutdown(&p.GCR);
        gcr::mxc_uart0_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_UART0);
        uart0::config(&p.UART);
        // Initialize TMR0 as continuous 32-bit system tick timer
        gcr::mxc_tmr0_shutdown(&p.GCR);
        gcr::mxc_tmr0_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_TMR0);
        tmr0::config_as_systick(&p.TMR);
        // Initialize TRNG
        gcr::mxc_trng_shutdown(&p.GCR);
        gcr::mxc_trng_enable_clock(&p.GCR);
        // Initialize FLC
        flc::config(&p.FLC);
        // Initialize LEDs
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_LED0);
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_LED1);
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_LED2);

        // Return the Board instance
        Board {
            // peripherals: p,
            flc: p.FLC,
            gcr: p.GCR,
            gpio0: p.GPIO0,
            gpio2: p.GPIO2,
            i2c1: p.I2C1,
            tmr0: p.TMR,
            tmr1: p.TMR1,
            trng: p.TRNG,
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

    /// Write 4 bytes to flash at the given address
    pub fn write_flash_bytes(&self, addr: u32, data: &[u8; 4]) {
        let result = flc::write_32(&self.flc, addr, bytes_to_u32(data));
        match result {
            flc::FlashStatus::Success => (),
            flc::FlashStatus::NeedsErase => {
                // Erase the flash page
                let result = flc::erase_page(&self.flc, addr & 0xFFFF_E000);
                self.send_host_debug(b"The contents below should be erased:");
                for i in 0..4 {
                    let addr_ptr = addr as *const u8;
                    let byte = unsafe { addr_ptr.add(i).read() };
                    if byte != 0xff {
                        self.send_host_debug(b"The byte below does not match expected output!");
                    }
                    self.send_host_debug(&u8_to_hex_string(byte));
                }
                match result {
                    flc::FlashStatus::Success => {
                        // Retry the write
                        let result = flc::write_32(&self.flc, addr, bytes_to_u32(data));
                        match result {
                            flc::FlashStatus::Success => (),
                            _ => {
                                self.send_host_debug(b"Failed to write to flash after erasing page");
                                panic!();
                            },
                        }
                    },
                    _ => {
                        self.send_host_debug(b"Failed to erase flash page");
                        panic!();
                    },
                }
            },
            _ => {
                self.send_host_debug(b"Failed to write to flash");
                panic!();
            }
        }
        // Verify the write
        let addr_ptr = addr as *const u8;
        for i in 0..4 {
            let byte = unsafe { addr_ptr.add(i).read() };
            if byte != data[i] {
                self.send_host_debug(b"Flash write verification failed");
                panic!();
            }
        }
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

pub fn u8_to_hex_string(value: u8) -> [u8; 2] {
    let mut result: [u8; 2] = [0; 2];
    for i in 0..2 {
        let nibble = (value >> (4 * i)) & 0xF;
        result[1 - i] = match nibble {
            0..=9 => b'0' + nibble as u8,
            10..=15 => b'a' + (nibble - 10) as u8,
            _ => b'?',
        };
    }
    result
}

pub fn u32_to_hex_string(value: u32) -> [u8; 8] {
    let mut result: [u8; 8] = [0; 8];
    for i in 0..8 {
        let nibble = (value >> (4 * i)) & 0xF;
        result[7 - i] = match nibble {
            0..=9 => b'0' + nibble as u8,
            10..=15 => b'a' + (nibble - 10) as u8,
            _ => b'?',
        };
    }
    result
}

/// Convert a [u8; 4] to a u32 (little-endian)
pub fn bytes_to_u32(data: &[u8; 4]) -> u32 {
    (data[0] as u32) |
    ((data[1] as u32) << 8) |
    ((data[2] as u32) << 16) |
    ((data[3] as u32) << 24)
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