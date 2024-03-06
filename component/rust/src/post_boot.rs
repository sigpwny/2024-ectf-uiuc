use ectf_board::{
    Led,
    hal,
    secure_comms as hide,
    ectf_constants::{LEN_MISC_MESSAGE, LEN_MAX_POST_BOOT_MSG}
};
use crate::{
    BOARD,
    ectf_comp_params::COMPONENT_ID
};


// External post_boot() function from C code for use in Rust
extern "C" {
    pub fn post_boot() -> !;
}

/// void secure_send(uint8_t* buffer, uint8_t len);
#[no_mangle]
pub extern "C" fn secure_send(buffer: *mut u8, len: u8) {
    if len > LEN_MAX_POST_BOOT_MSG {
        return;
    }
    let mut message: [u8; LEN_MISC_MESSAGE] = [0u8; LEN_MISC_MESSAGE];
    // Encode length as first byte of message
    message[0] = len;
    for i in 0..len {
        let idx = i as usize;
        message[idx+1] = unsafe { buffer.add(idx).read() };
    }
    let _ = hide::comp_secure_send(&BOARD, &COMPONENT_ID, &message);
}

/// int secure_receive(uint8_t* buffer);
/// Returns the length of the message on success, -1 on failure
#[no_mangle]
pub extern "C" fn secure_receive(buffer: *mut u8) -> i32 {
    let mut message: [u8; LEN_MISC_MESSAGE] = [0u8; LEN_MISC_MESSAGE];
    match hide::comp_secure_receive(&BOARD, &COMPONENT_ID, &mut message) {
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

////////////////////////////////////////
// Shared functions for POST_BOOT code
////////////////////////////////////////

/// led.h
/// int LED_Init(void);
#[no_mangle]
pub extern "C" fn LED_Init() -> i32 {
    hal::gpio2::config(&BOARD.gpio2, hal::gpio2::GPIO2_CFG_LED0);
    hal::gpio2::config(&BOARD.gpio2, hal::gpio2::GPIO2_CFG_LED1);
    hal::gpio2::config(&BOARD.gpio2, hal::gpio2::GPIO2_CFG_LED2);
    return 0;
}
/// led.h
/// void LED_On(unsigned int idx);
#[no_mangle]
pub extern "C" fn LED_Off(idx: u32) {
    match Led::from_u32(idx) {
        Some(led) => BOARD.led_off(led),
        None => {}
    }
}
/// led.h
/// void LED_Off(unsigned int idx);
#[no_mangle]
pub extern "C" fn LED_On(idx: u32) {
    match Led::from_u32(idx) {
        Some(led) => BOARD.led_on(led),
        None => {}
    }
}
/// led.h
/// void LED_Toggle(unsigned int idx);
#[no_mangle]
pub extern "C" fn LED_Toggle(idx: u32) {
    match Led::from_u32(idx) {
        Some(led) => BOARD.led_toggle(led),
        None => {}
    }
}
/// mxc_delay.h
/// int MXC_Delay(uint32_t us);
#[no_mangle]
pub extern "C" fn MXC_Delay(us: u32) {
    BOARD.delay_timer_wait_us(us);
}
/// stdio.h
/// Read handler for libc
#[no_mangle]
pub extern "C" fn _read(fd: i32, buf: *mut u8, count: usize) -> isize {
    // If STDIN
    if fd == 0 {
        // Create a &mut [u8] from the provided buffer pointer and count
        // Safety: We assume that the POST_BOOT code provides a valid buffer and count
        let buffer = unsafe { core::slice::from_raw_parts_mut(buf, count) };
        BOARD.libc_read_uart(buffer);
        return count as isize;
    }
    return -1;
}
/// stdio.h
/// Write handler for libc
#[no_mangle]
pub extern "C" fn _write(fd: i32, buf: *const u8, count: usize) -> isize {
    // If STDOUT or STDERR
    if fd == 1 || fd == 2 {
        // Create a &[u8] from the provided buffer pointer and count
        // Safety: We assume that the POST_BOOT code provides a valid buffer and count
        let buffer = unsafe { core::slice::from_raw_parts(buf, count) };
        BOARD.libc_write_uart(buffer);
        return count as isize;
    }
    return -1;
}
/// stdio.h
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