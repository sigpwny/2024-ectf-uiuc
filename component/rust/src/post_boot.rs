use max78000_pac as pac;
use max78000_hal as hal;
use board::Board;
use board::secure_comms as hide;
use board::ectf_constants::LEN_MISC_MESSAGE;
use board::ectf_params::COMPONENT_ID;

pub const LEN_MAX_POST_BOOT_MSG: u8 = 64;

// Use post_boot() function from C code
extern "C" {
    pub fn post_boot() -> !;
}

/// void secure_send(uint8_t* buffer, uint8_t len);
#[no_mangle]
pub extern "C" fn secure_send(buffer: *mut u8, len: u8) {
    if len > LEN_MAX_POST_BOOT_MSG {
        return;
    }
    let p = unsafe { pac::Peripherals::steal() };
    let board = Board {
        flc: p.FLC,
        gcr: p.GCR,
        gpio0: p.GPIO0,
        gpio2: p.GPIO2,
        i2c1: p.I2C1,
        tmr0: p.TMR,
        tmr1: p.TMR1,
        trng: p.TRNG,
        uart0: p.UART,
    };
    let mut message: [u8; LEN_MISC_MESSAGE] = [0u8; LEN_MISC_MESSAGE];
    // Encode length as first byte of message
    message[0] = len;
    for i in 0..len {
        let idx = i as usize;
        message[idx+1] = unsafe { buffer.add(idx).read() };
    }
    let _ = hide::comp_secure_send(&board, &COMPONENT_ID, &message);
}

/// int secure_receive(uint8_t* buffer);
/// Returns the length of the message on success, -1 on failure
#[no_mangle]
pub extern "C" fn secure_receive(buffer: *mut u8) -> i32 {
    let p = unsafe { pac::Peripherals::steal() };
    let board = Board {
        flc: p.FLC,
        gcr: p.GCR,
        gpio0: p.GPIO0,
        gpio2: p.GPIO2,
        i2c1: p.I2C1,
        tmr0: p.TMR,
        tmr1: p.TMR1,
        trng: p.TRNG,
        uart0: p.UART,
    };
    let mut message: [u8; LEN_MISC_MESSAGE] = [0u8; LEN_MISC_MESSAGE];
    match hide::comp_secure_receive(&board, &COMPONENT_ID, &mut message) {
        Some(LEN_MISC_MESSAGE) => {
            // Decode the message length
            let len: u8 = message[0];
            if len > LEN_MAX_POST_BOOT_MSG {
                return -1;
            }
            for i in 0..len {
                let idx = i as usize;
                unsafe { buffer.add(idx).write(message[idx+1]) };
            }
            return len as i32;
        }
        _ => -1,
    }
}

/// led.h
/// int LED_Init(void);
#[no_mangle]
pub extern "C" fn LED_Init() -> i32 {
    let gpio2 = unsafe { pac::GPIO2::steal() };
    hal::gpio2::config(&gpio2, hal::gpio2::GPIO2_CFG_LED0);
    hal::gpio2::config(&gpio2, hal::gpio2::GPIO2_CFG_LED1);
    hal::gpio2::config(&gpio2, hal::gpio2::GPIO2_CFG_LED2);
    return 0;
}

/// led.h
/// void LED_On(unsigned int idx);
#[no_mangle]
pub extern "C" fn LED_Off(idx: u32) {
    let gpio2 = unsafe { pac::GPIO2::steal() };
    match idx {
        0 => hal::gpio2::clr_out(&gpio2, hal::gpio2::GPIO2_CFG_LED0.pins),
        1 => hal::gpio2::clr_out(&gpio2, hal::gpio2::GPIO2_CFG_LED1.pins),
        2 => hal::gpio2::clr_out(&gpio2, hal::gpio2::GPIO2_CFG_LED2.pins),
        _ => {}
    }
}

/// led.h
/// void LED_Off(unsigned int idx);
#[no_mangle]
pub extern "C" fn LED_On(idx: u32) {
    let gpio2 = unsafe { pac::GPIO2::steal() };
    match idx {
        0 => hal::gpio2::set_out(&gpio2, hal::gpio2::GPIO2_CFG_LED0.pins),
        1 => hal::gpio2::set_out(&gpio2, hal::gpio2::GPIO2_CFG_LED1.pins),
        2 => hal::gpio2::set_out(&gpio2, hal::gpio2::GPIO2_CFG_LED2.pins),
        _ => {}
    }
}

/// led.h
/// void LED_Toggle(unsigned int idx);
#[no_mangle]
pub extern "C" fn LED_Toggle(idx: u32) {
    let gpio2 = unsafe { pac::GPIO2::steal() };
    match idx {
        0 => hal::gpio2::toggle_out(&gpio2, hal::gpio2::GPIO2_CFG_LED0.pins),
        1 => hal::gpio2::toggle_out(&gpio2, hal::gpio2::GPIO2_CFG_LED1.pins),
        2 => hal::gpio2::toggle_out(&gpio2, hal::gpio2::GPIO2_CFG_LED2.pins),
        _ => {}
    }
}

/// mxc_delay.h
/// int MXC_Delay(uint32_t us);
#[no_mangle]
pub extern "C" fn MXC_Delay(us: u32) {
    let tmr0 = unsafe { pac::TMR::steal() };
    hal::tmr0::config(&tmr0);
    while hal::tmr0::get_time_us(&tmr0) < us { }
}

/// Read handler for POST_BOOT libc
#[no_mangle]
pub extern "C" fn _read(fd: i32, buf: *mut u8, count: usize) -> isize {
    if fd == 0 {
        let uart0 = unsafe { pac::UART::steal() };
        let mut num_read = 0;
        for i in 0..count {
            let byte = hal::uart0::read_byte(&uart0);
            hal::uart0::write_byte(&uart0, byte);
            num_read += 1;
            if byte == '\r' as u8 {
                unsafe { buf.add(i).write(b'\n') };
                break;
            }
            unsafe { buf.add(i).write(byte) };
        }
        return num_read as isize;
    }
    return -1;
}
/// Write handler for POST_BOOT libc
#[no_mangle]
pub extern "C" fn _write(fd: i32, buf: *const u8, count: usize) -> isize {
    if fd == 1 || fd == 2 {
        let uart0 = unsafe { pac::UART::steal() };
        for i in 0..count {
            let byte = unsafe { buf.add(i).read() };
            if byte == b'\n' as u8 {
                hal::uart0::write_byte(&uart0, b'\r');
            }
            hal::uart0::write_byte(&uart0, byte);
        }
        return count as isize;
    }
    return -1;
}
/// The following functions are unused by the POST_BOOT code, but are 
/// required by the linker, so we provide stubs for them here.
#[no_mangle]
pub extern "C" fn _exit() {
    loop {}
}
#[no_mangle]
pub extern "C" fn _open() -> i32 {
    return -1;
}
#[no_mangle]
pub extern "C" fn _close() -> i32 {
    return -1;
}
#[no_mangle]
pub extern "C" fn _isatty() -> i32 {
    return -1;
}
#[no_mangle]
pub extern "C" fn _lseek() -> i32 {
    return -1;
}
#[no_mangle]
pub extern "C" fn _fstat() -> i32 {
    return -1;
}
#[no_mangle]
pub extern "C" fn _sbrk() -> i32 {
    return -1;
}
#[no_mangle]
pub extern "C" fn _kill() -> i32 {
    return -1;
}
#[no_mangle]
pub extern "C" fn _getpid() -> i32 {
    return -1;
}