#![no_std]

pub mod secure_comms;
pub mod rng;
pub mod ectf_constants;
pub mod ectf_global_secrets;

pub use max78000_pac as pac;
pub use max78000_hal as hal;
use max78000_hal::{*};
use ectf_constants::{*};
use rand::RngCore;
use rng::CustomRng;
use core::{cell::RefCell, panic::PanicInfo};
use core::arch::asm;

pub enum Led {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl Led {
    pub fn from_u32(value: u32) -> Option<Led> {
        match value {
            0 => Some(Led::Red),
            1 => Some(Led::Green),
            2 => Some(Led::Blue),
            _ => None,
        }
    }
}

pub struct Board {
    pub flc: pac::FLC,
    pub gcr: pac::GCR,
    pub gpio0: pac::GPIO0,
    pub gpio2: pac::GPIO2,
    pub i2c1: pac::I2C1,
    pub tmr0: pac::TMR,
    pub tmr1: pac::TMR1,
    pub tmr2: pac::TMR2,
    pub tmr4: pac::TMR4,
    pub trng: pac::TRNG,
    pub uart0: pac::UART,
    rng: RefCell<CustomRng>,
}

// Safety: Board is a singleton and we only use one core.
unsafe impl Sync for Board {}

impl Board {
    /// Create a new Board instance
    pub fn new(seed: [u8; 64]) -> Self {
        let p: pac::Peripherals = pac::Peripherals::take().unwrap();
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
        // Initialize TMR0 (total transaction timer)
        gcr::mxc_tmr0_shutdown(&p.GCR);
        gcr::mxc_tmr0_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_TMR0);
        // Initialize TMR1 (delay timer)
        gcr::mxc_tmr1_shutdown(&p.GCR);
        gcr::mxc_tmr1_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_TMR1);
        // Initialize TMR2 (entropy source)
        gcr::mxc_tmr2_shutdown(&p.GCR);
        gcr::mxc_tmr2_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_TMR2);
        tmr2::config(&p.TMR2);
        // Initialize TMR4 (entropy source)
        gcr::mxc_tmr4_shutdown(&p.LPGCR);
        gcr::mxc_tmr4_enable_clock(&p.LPGCR);
        gpio2::config(&p.GPIO2, gpio2::GPIO2_CFG_TMR4);
        tmr4::config(&p.TMR4);
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
        // Create custom RNG instance
        let rng = CustomRng::new(&p.TMR2, &p.TMR4, &p.TRNG, seed);
        // Return the Board instance
        Board {
            flc: p.FLC,
            gcr: p.GCR,
            gpio0: p.GPIO0,
            gpio2: p.GPIO2,
            i2c1: p.I2C1,
            tmr0: p.TMR,
            tmr1: p.TMR1,
            tmr2: p.TMR2,
            tmr4: p.TMR4,
            trng: p.TRNG,
            uart0: p.UART,
            rng: RefCell::new(rng),
        }
    }

    /// Reset the transaction timer to 0
    pub fn transaction_timer_reset(&self) {
        tmr0::config(&self.tmr0);
        // Verify that the timer has just started
        let start = tmr0::get_time_us(&self.tmr0);
        if start > 100 {
            self.send_host_debug(b"Timer did not reset properly!");
            panic!();
        }
    }

    /// Block until the specified number of microseconds has elapsed since the 
    /// last transaction timer reset (total transaction time)
    pub fn transaction_timer_wait_until_us(&self, us: u32) {
        while tmr0::get_time_us(&self.tmr0) < us { }
    }

    /// Block for the specified number of ticks
    pub fn delay_timer_wait_ticks(&self, ticks: u32) {
        tmr1::config(&self.tmr1);
        let start = tmr1::get_tick_count(&self.tmr1);
        while tmr1::get_tick_count(&self.tmr1) < start + ticks { }
    }

    /// Block for the specified number of microseconds
    pub fn delay_timer_wait_us(&self, us: u32) {
        let ticks = tmr1::us_to_ticks(us);
        self.delay_timer_wait_ticks(ticks);
    }

    /// Block for a random number of microseconds between min and max
    pub fn delay_timer_wait_random_us(&self, min: u32, max: u32) {
        let random = self.next_u32();
        let max_ticks = tmr1::us_to_ticks(max);
        let min_ticks = tmr1::us_to_ticks(min);
        let ticks = min_ticks + (random % (max_ticks - min_ticks));
        self.delay_timer_wait_ticks(ticks);
    }

    /// Host debugging is only enabled in debug builds
    /// Output sent via UART0
    #[cfg(all(debug_assertions, not(feature = "semihosting")))]
    pub fn send_host_debug(&self, message: &[u8]) {
        uart0::write_bytes(&self.uart0, b"%debug: ");
        uart0::write_bytes(&self.uart0, message);
        uart0::write_bytes(&self.uart0, b"\r\n%");
    }
    #[cfg(all(debug_assertions, not(feature = "semihosting")))]
    pub fn send_host_debug_no_fmt(&self, message: &[u8]) {
        uart0::write_bytes(&self.uart0, message);
    }
    
    /// Host debugging is only enabled in debug builds
    /// Output sent via semihosting (requires a debugger to be attached)
    #[cfg(all(debug_assertions, feature = "semihosting"))]
    pub fn send_host_debug(&self, message: &[u8]) {
        use cortex_m_semihosting::{heprint, syscall};
        heprint!("%debug: ");
        // Safety: Required to print type &[u8] to the host
        unsafe { syscall!(WRITE, 1, message.as_ptr(), message.len()) };
        heprint!("\r\n%");
    }
    #[cfg(all(debug_assertions, feature = "semihosting"))]
    pub fn send_host_debug_no_fmt(&self, message: &[u8]) {
        use cortex_m_semihosting::{heprint, syscall};
        // Safety: Required to print type &[u8] to the host
        unsafe { syscall!(WRITE, 1, message.as_ptr(), message.len()) };
    }

    /// Host debugging is disabled in release builds, so do nothing
    #[cfg(not(debug_assertions))]
    pub fn send_host_debug(&self, _message: &[u8]) {
        cortex_m::asm::nop();
    }
    #[cfg(not(debug_assertions))]
    pub fn send_host_debug_no_fmt(&self, _message: &[u8]) {
        cortex_m::asm::nop();
    }

    /// Write info to the host
    pub fn send_host_info(&self, message: &[u8]) {
        uart0::write_bytes(&self.uart0, b"%info: ");
        uart0::write_bytes(&self.uart0, message);
        uart0::write_bytes(&self.uart0, b"\r\n%");
    }

    /// Write error to the host
    pub fn send_host_error(&self, message: &[u8]) {
        uart0::write_bytes(&self.uart0, b"%error: ");
        uart0::write_bytes(&self.uart0, message);
        uart0::write_bytes(&self.uart0, b"\r\n%");
    }

    /// Write success to the host
    pub fn send_host_success(&self, message: &[u8]) {
        uart0::write_bytes(&self.uart0, b"%success: ");
        uart0::write_bytes(&self.uart0, message);
        uart0::write_bytes(&self.uart0, b"\r\n%");
    }

    /// Write ack to the host
    pub fn send_host_ack(&self) {
        uart0::write_bytes(&self.uart0, b"%ack%");
    }

    /// Send a formatted component ID to the host
    pub fn send_host_cid(&self, prefix: u8, cid: &[u8; LEN_COMPONENT_ID]) {
        uart0::write_bytes(&self.uart0, b"%info: ");
        uart0::write_bytes(&self.uart0, &[prefix, b'>', b'0', b'x']);
        uart0::write_bytes(&self.uart0, &u8_to_hex_string(cid[0]));
        uart0::write_bytes(&self.uart0, &u8_to_hex_string(cid[1]));
        uart0::write_bytes(&self.uart0, &u8_to_hex_string(cid[2]));
        uart0::write_bytes(&self.uart0, &u8_to_hex_string(cid[3]));
        uart0::write_bytes(&self.uart0, b"\r\n%");
    }

    /// Send formatted attestation data to the host
    pub fn send_host_attest_data(
        &self,
        location: &[u8; LEN_ATTEST_LOCATION],
        date: &[u8; LEN_ATTEST_DATE],
        customer: &[u8; LEN_ATTEST_CUSTOMER]
    ) {
        uart0::write_bytes(&self.uart0, b"%info: ");
        uart0::write_bytes(&self.uart0, b"LOC>");
        uart0::write_bytes(&self.uart0, location);
        uart0::write_bytes(&self.uart0, b"\r\n%");
        uart0::write_bytes(&self.uart0, b"%info: ");
        uart0::write_bytes(&self.uart0, b"DATE>");
        uart0::write_bytes(&self.uart0, date);
        uart0::write_bytes(&self.uart0, b"\r\n%");
        uart0::write_bytes(&self.uart0, b"%info: ");
        uart0::write_bytes(&self.uart0, b"CUST>");
        uart0::write_bytes(&self.uart0, customer);
        uart0::write_bytes(&self.uart0, b"\r\n%");
    }

    /// Send formatted AP boot message to the host
    pub fn send_host_ap_boot_msg(&self, ap_boot_msg: &[u8]) {
        uart0::write_bytes(&self.uart0, b"%info: AP>");
        uart0::write_bytes(&self.uart0, ap_boot_msg);
        uart0::write_bytes(&self.uart0, b"\r\n%");
    }

    /// Send formatted component boot message to the host
    pub fn send_host_comp_boot_msg(&self, comp_id: &[u8; LEN_COMPONENT_ID], comp_boot_msg: &[u8]) {
        uart0::write_bytes(&self.uart0, b"%info: 0x");
        uart0::write_bytes(&self.uart0, &u8_to_hex_string(comp_id[0]));
        uart0::write_bytes(&self.uart0, &u8_to_hex_string(comp_id[1]));
        uart0::write_bytes(&self.uart0, &u8_to_hex_string(comp_id[2]));
        uart0::write_bytes(&self.uart0, &u8_to_hex_string(comp_id[3]));
        uart0::write_bytes(&self.uart0, b">");
        uart0::write_bytes(&self.uart0, comp_boot_msg);
        uart0::write_bytes(&self.uart0, b"\r\n%");
    }

    /// Read a command from the host (terminated by '\r')
    /// Definitely safe :)
    pub fn gets(&self, buffer: &mut [u8]) -> Option<usize> {
        let mut num_read = 0;
        for out_byte in buffer.iter_mut() {
            let in_byte = uart0::read_byte(&self.uart0);
            uart0::write_byte(&self.uart0, in_byte);
            *out_byte = in_byte;
            if in_byte == b'\r' {
                return Some(num_read);
            }
            num_read += 1;
        }
        return None;
    }

    /// Read from UART, safe for C by rewriting input '\r' as '\n'
    /// Intended for POST_BOOT C code, based on MSDK stdio.c implementation
    pub fn libc_read_uart(&self, buffer: &mut [u8]) {
        for out_byte in buffer.iter_mut() {
            let in_byte = uart0::read_byte(&self.uart0);
            uart0::write_byte(&self.uart0, in_byte);
            if in_byte == b'\r' {
                *out_byte = b'\n';
                break;
            }
            *out_byte = in_byte;
        }
    }

    /// Write to UART, rewrites output byte '\n' as '\r\n'
    /// Intended for POST_BOOT C code, based on MSDK stdio.c implementation
    pub fn libc_write_uart(&self, buffer: &[u8]) {
        for byte in buffer.iter() {
            if *byte == b'\n' {
                uart0::write_byte(&self.uart0, b'\r');
            }
            uart0::write_byte(&self.uart0, *byte);
        }
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

    /// Turn on the specified LED
    pub fn led_on(&self, led: Led) {
        match led {
            Led::Red => gpio2::set_out(&self.gpio2, gpio2::GPIO2_CFG_LED0.pins),
            Led::Green => gpio2::set_out(&self.gpio2, gpio2::GPIO2_CFG_LED1.pins),
            Led::Blue => gpio2::set_out(&self.gpio2, gpio2::GPIO2_CFG_LED2.pins),
        }
    }

    /// Turn off the specified LED
    pub fn led_off(&self, led: Led) {
        match led {
            Led::Red => gpio2::clr_out(&self.gpio2, gpio2::GPIO2_CFG_LED0.pins),
            Led::Green => gpio2::clr_out(&self.gpio2, gpio2::GPIO2_CFG_LED1.pins),
            Led::Blue => gpio2::clr_out(&self.gpio2, gpio2::GPIO2_CFG_LED2.pins),
        }
    }

    /// Toggle the specified LED
    pub fn led_toggle(&self, led: Led) {
        match led {
            Led::Red => gpio2::toggle_out(&self.gpio2, gpio2::GPIO2_CFG_LED0.pins),
            Led::Green => gpio2::toggle_out(&self.gpio2, gpio2::GPIO2_CFG_LED1.pins),
            Led::Blue => gpio2::toggle_out(&self.gpio2, gpio2::GPIO2_CFG_LED2.pins),
        }
    }

    // Not RngCore because we are doing the interior mutability stuff :((
    /// Generate random bytes
    pub fn fill_bytes(&self, dest: &mut [u8]) {
        critical_section::with(|_| {
            self.rng.borrow_mut().fill_bytes(dest)
        })
    }

    /// Generate a random u32
    pub fn next_u32(&self) -> u32 {
        critical_section::with(|_| {
            self.rng.borrow_mut().next_u32()
        })
    }

    /// Generate a random u64
    pub fn next_u64(&self) -> u64 {
        critical_section::with(|_| {
            self.rng.borrow_mut().next_u64()
        })
    }

    /// Try to generate random bytes
    pub fn try_fill_bytes(&self, dest: &mut [u8]) -> Result<(), rand::Error> {
        critical_section::with(|_| {
            self.rng.borrow_mut().try_fill_bytes(dest)
        })
    }
}

/// Lock all flash pages except:
/// - 0x1000_0000 - 0x1000_DFFF (Bootloader)
/// - 0x1007_8000 - 0x1007_BFFF (Our flash for component IDs)
/// - 0x1007_C000 - 0x1007_FFFF (Bootloader Data and ROM)
/// We only lock flash pages in release builds
#[cfg(not(debug_assertions))]
pub fn lock_pages(flc: &pac::FLC) {
    for i in 7..60 {
        let addr = flc::FLASH_BASE + (i * flc::FLASH_PAGE_SIZE);
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

// Converts a byte hex string [u8; 2] to a u8
pub fn hex_string_to_u8(value: &[u8]) -> u8 {
    let mut result: u8 = 0;
    for i in 0..2 {
        let nibble = match value[i] {
            b'0'..=b'9' => value[i] - b'0',
            b'a'..=b'f' => value[i] - b'a' + 10,
            _ => 0,
        };
        result |= nibble << (4 * (1 - i));
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

///🙄
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "1337:",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b",
            "b 1337b"
        );
    }
    loop { }
}
