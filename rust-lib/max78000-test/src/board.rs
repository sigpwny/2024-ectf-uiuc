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
        // Initialize clocks
        gcr::system_clock_ipo_init(&p.GCR);
        gcr::iso_init(&p.GCR);
        // Disable ICC
        p.ICC0.ctrl().modify(|_, w| w.en().clear_bit());
        // Initialize GPIO
        gcr::mxc_gpio0_init(&p.GCR);
        // Initialize UART0 for host communication
        gcr::mxc_uart0_shutdown(&p.GCR);
        gcr::mxc_uart0_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_UART0);
        uart0::config(&p.UART);
        // Initialize TMR0
        gcr::mxc_tmr0_shutdown(&p.GCR);
        gcr::mxc_tmr0_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_TMR0);
        // Configure TMR0 as continuous 32-bit system tick timer
        tmr0::config_as_systick(&p.TMR);
        // Initialize TMR1
        gcr::mxc_tmr1_shutdown(&p.GCR);
        gcr::mxc_tmr1_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_TMR1);
        // Initialize TRNG
        gcr::mxc_trng_shutdown(&p.GCR);
        gcr::mxc_trng_enable_clock(&p.GCR);
        // Initialize FLC
        flc::config(&p.FLC);
        // Write lock flash pages
        lock_pages(&p.FLC);
        // Initialize LEDs
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_LED0);
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_LED1);
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_LED2);
        // Initialize I2C1
        gcr::mxc_i2c1_shutdown(&p.GCR);
        gcr::mxc_i2c1_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_I2C1);
        // i2c1::master_config(&p.I2C1);
        // i2c1::slave_config(&p.I2C1, 0x25); // TODO: Use component ID for address

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

    /// Reset the one-shot timer to 0 at the start of a transaction
    pub fn timer_reset(&self) {
        tmr1::config_as_oneshot(&self.tmr1);
        // Verify that the timer has just started
        let start = tmr1::get_time_us(&self.tmr1);
        if start > 100 {
            self.send_host_debug(b"Timer did not reset properly!");
            panic!();
        }
    }

    /// Get the current time in microseconds (us)
    pub fn timer_get_us(&self) -> u32 {
        return tmr1::get_time_us(&self.tmr1);
    }

    /// Block for the specified number of microseconds
    pub fn delay_us(&self, us: u32) {
        let start = tmr1::get_time_us(&self.tmr1);
        // Ensure there is no integer overflow
        if start + us < start {
            self.send_host_debug(b"Timer overflow");
            panic!();
        }
        while tmr1::get_time_us(&self.tmr1) < start + us { }
    }

    /// Block until the specified number of microseconds has elapsed since the 
    /// last reset (total transaction time)
    pub fn delay_total_us(&self, us: u32) {
        while tmr1::get_time_us(&self.tmr1) < us { }
    }

    /// Host debugging is only enabled in debug builds
    /// Output sent via UART0
    #[cfg(all(debug_assertions, not(feature = "semihosting")))]
    pub fn send_host_debug(&self, message: &[u8]) {
        uart0::write_bytes(&self.uart0, b"%debug ");
        uart0::write_bytes(&self.uart0, message);
        uart0::write_bytes(&self.uart0, b"%\r\n");
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

    /// Read a command from the host (terminated by '\r')
    pub fn read_host_line(&self, buffer: &mut [u8]) -> Option<usize> {
        let mut index = 0;
        for byte in buffer.iter_mut() {
            let result = uart0::read_byte(&self.uart0);
            *byte = result;
            // Echo the received byte
            uart0::write_byte(&self.uart0, result);
            if result == b'\r' {
                return Some(index);
            }
            index += 1;
        }
        return None;
    }

    /// Write 4 bytes to flash at the given address (erases the flash page if necessary)
    pub fn write_flash_bytes(&self, addr: u32, data: &[u8; 4]) {
        let result = flc::write_32(&self.flc, addr, bytes_to_u32(data));
        match result {
            flc::FlashStatus::Success => (),
            flc::FlashStatus::NeedsErase => {
                // Erase the flash page
                let result = flc::erase_page(&self.flc, addr & 0xFFFF_E000);
                // Verify the erase
                for i in 0..4 {
                    let addr_ptr = addr as *const u8;
                    let byte = unsafe { addr_ptr.add(i).read() };
                    if byte != 0xff {
                        self.send_host_debug(b"Flash was not erased!");
                        panic!();
                    }
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
                    flc::FlashStatus::AccessViolation => {
                        self.send_host_debug(b"Access violation during flash erase");
                        self.send_host_debug(b"Failed to erase flash page");
                        panic!();
                    },
                    flc::FlashStatus::InvalidAddress => {
                        self.send_host_debug(b"Invalid address during flash erase");
                        self.send_host_debug(b"Failed to erase flash page");
                        panic!();
                    },
                    _ => {
                        self.send_host_debug(b"Unknown error");
                        self.send_host_debug(b"Failed to erase flash page");
                        panic!();
                    }
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

/// Lock all flash pages except for pages where we store data,
/// only lock flash pages in release builds
#[cfg(not(debug_assertions))]
pub fn lock_pages(flc: &pac::FLC) {
    for i in 0..64 {
        let exclusions = [0x1004_2000, 0x1004_4000];
        let addr = flc::FLASH_BASE + (i * flc::FLASH_PAGE_SIZE);
        if exclusions.contains(&addr) {
            continue;
        }
        let result = flc::block_page_write(flc, addr);
        match result {
            flc::FlashStatus::Success => (),
            _ => {
                panic!();
            }
        }
    }
}

/// Do not lock flash pages in debug builds
#[cfg(debug_assertions)]
pub fn lock_pages(_flc: &pac::FLC) {
    cortex_m::asm::nop();
}

/// Convert a u8 to a hex byte string array
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

/// Convert a u32 to a hex byte string array
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
