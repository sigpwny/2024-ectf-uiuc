#![no_std]

pub mod secure_comms;
pub mod ectf_constants;
pub mod ectf_global_secrets;
pub mod ectf_params;

use core::panic::PanicInfo;
use max78000_pac as pac;
use max78000_hal::{*};
use ectf_constants::{*};
use ectf_params::{*};

pub enum Led {
    Red = 0,
    Green = 1,
    Blue = 2,
}

pub struct Board {
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
        // Initialize TMR0 (total transaction timer)
        gcr::mxc_tmr0_shutdown(&p.GCR);
        gcr::mxc_tmr0_enable_clock(&p.GCR);
        gpio0::config(&p.GPIO0, gpio0::GPIO0_CFG_TMR0);
        // Initialize TMR1 (delay timer)
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
        i2c1::master_config(&p.I2C1);
        // Return the Board instance
        Board {
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

    /// Block for the specified number of microseconds
    pub fn delay_timer_wait_us(&self, us: u32) {
        tmr1::config(&self.tmr1);
        let start = tmr1::get_time_us(&self.tmr1);
        while tmr1::get_time_us(&self.tmr1) < start + us { }
    }

    /// Block for a random number of microseconds between min and max
    pub fn delay_timer_wait_random_us(&self, min: u32, max: u32) {
        let random = trng::random_u32(&self.trng);
        let delay = random % (max - min) + min;
        self.send_host_debug(b"Random delay (us): ");
        self.send_host_debug(&u32_to_hex_string(delay));
        self.delay_timer_wait_us(delay);
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

    /// Checks if a given u8 is a valid I2C address
    pub fn is_i2c_addr_blacklisted(&self, addr: u8) -> bool {
        let addr = addr & 0x7F;
        match addr {
            0x00..=0x07 => return true,
            0x18 => return true,
            0x28 => return true,
            0x36 => return true,
            0x78..=0x7F => return true,
            _ => return false,
        }
    }

    // Get provisioned component ID stored in flash
    pub fn get_provisioned_component_id(&self, cid: &mut [u8; LEN_COMPONENT_ID], idx: u8) -> bool {
        if idx >= COMPONENT_CNT {
            self.send_host_debug(b"Invalid component ID index");
            return false;
        }
        // If the component ID is written in flash, use that
        if self.is_comp_id_in_flash(idx) {
            let addr_ptr: *const u8 = match idx {
                0 => FLASH_ADDR_CID_0 as *const u8,
                1 => FLASH_ADDR_CID_1 as *const u8,
                _ => {
                    self.send_host_debug(b"Invalid component ID index");
                    return false;
                }
            };
            // Safety: We're reading from a valid flash address
            let byte0 = unsafe { core::ptr::read_volatile(addr_ptr) };
            let byte1 = unsafe { core::ptr::read_volatile(addr_ptr.add(1)) };
            let byte2 = unsafe { core::ptr::read_volatile(addr_ptr.add(2)) };
            let byte3 = unsafe { core::ptr::read_volatile(addr_ptr.add(3)) };
            cid[0] = byte0;
            cid[1] = byte1;
            cid[2] = byte2;
            cid[3] = byte3;
        }
        // Otherwise, use the original component ID
        else {
            match idx {
                0 => {
                    cid[0] = COMPONENT_ID_0[0];
                    cid[1] = COMPONENT_ID_0[1];
                    cid[2] = COMPONENT_ID_0[2];
                    cid[3] = COMPONENT_ID_0[3];
                }
                1 => {
                    cid[0] = COMPONENT_ID_1[0];
                    cid[1] = COMPONENT_ID_1[1];
                    cid[2] = COMPONENT_ID_1[2];
                    cid[3] = COMPONENT_ID_1[3];
                }
                _ => {
                    self.send_host_debug(b"Invalid component ID index");
                    return false;
                }
            }
        }
        // Ensure that the component ID is not blacklisted
        if self.is_i2c_addr_blacklisted(cid[3]) {
            self.send_host_debug(b"Invalid component ID");
            return false;
        }
        return true;
    }

    // Check if component IDs are initialized in flash
    pub fn is_comp_id_in_flash(&self, idx: u8) -> bool {
        let addr_ptr = match idx {
            0 => FLASH_ADDR_CID_0 as *const u8,
            1 => FLASH_ADDR_CID_1 as *const u8,
            _ => {
                self.send_host_debug(b"Invalid component ID index");
                return false;
            }
        };
        // Safety: We're reading from a valid flash address
        let byte = unsafe {
            core::ptr::read_volatile(addr_ptr.add(3))
        };
        // Check if the last byte is 0xFF
        if byte == 0xFF {
            return false;
        }
        return true;
    }

    // Set provisioned component ID in flash
    pub fn set_provisioned_component_id(&self, cid: &[u8; LEN_COMPONENT_ID], idx: u8) -> bool {
        if idx >= COMPONENT_CNT {
            self.send_host_debug(b"Invalid component ID index");
            return false;
        }
        // Check if the component ID is blacklisted
        if self.is_i2c_addr_blacklisted(cid[3]) {
            self.send_host_debug(b"Invalid component ID");
            return false;
        }
        let flash_addr = match idx {
            0 => FLASH_ADDR_CID_0,
            1 => FLASH_ADDR_CID_1,
            _ => {
                self.send_host_debug(b"Invalid component ID index");
                return false;
            }
        };
        self.write_flash_bytes(flash_addr, cid);
        return true;
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
    for i in 0..60 {
        let exclusions = [FLASH_ADDR_CID_0, FLASH_ADDR_CID_1];
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