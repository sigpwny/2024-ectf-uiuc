use ectf_board::{
    Led,
    hal,
    secure_comms as hide,
    ectf_constants::{LEN_COMPONENT_ID, LEN_MISC_MESSAGE, LEN_MAX_POST_BOOT_MSG}
};
use crate::{
    BOARD,
    ectf_ap_params::COMPONENT_CNT,
    flash
};


// External post_boot() function from C code for use in Rust
extern "C" {
    pub fn post_boot() -> !;
}

/// int secure_send(uint8_t address, uint8_t* buffer, uint8_t len);
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn secure_send(address: u8, buffer: *mut u8, len: u8) -> i32 {
    if len > LEN_MAX_POST_BOOT_MSG {
        return -1;
    }
    // Delay to allow the I2C slave to get ready
    MXC_Delay(10_000);
    // Find the component ID with the provided I2C address
    let mut comp_id: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
    let mut comp_id_found: bool = false;
    for i in 0..COMPONENT_CNT {
        if flash::get_provisioned_component_id(&mut comp_id, i) {
            if comp_id[3] == address {
                comp_id_found = true;
                break;
            }
        }
    }
    if !comp_id_found {
        return -1;
    }
    let mut message: [u8; LEN_MISC_MESSAGE] = [0u8; LEN_MISC_MESSAGE];
    // Encode length as first byte of message
    message[0] = len;
    for i in 0..len {
        let idx = i as usize;
        message[idx+1] = unsafe { buffer.add(idx).read() };
    }
    match hide::ap_secure_send(&BOARD, &comp_id, &message) {
        Some(LEN_MISC_MESSAGE) => 0,
        _ => -1,
    }
}

/// int secure_receive(i2c_addr_t address, uint8_t* buffer);
/// Returns number of bytes received on success, -1 on failure
#[no_mangle]
pub extern "C" fn secure_receive(address: u8, buffer: *mut u8) -> i32 {
    // Delay to allow the I2C slave to get ready
    MXC_Delay(10_000);
    // Find the component ID with the provided I2C address
    let mut comp_id: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
    let mut comp_id_found: bool = false;
    for i in 0..COMPONENT_CNT {
        if flash::get_provisioned_component_id(&mut comp_id, i) {
            if comp_id[3] == address {
                comp_id_found = true;
                break;
            }
        }
    }
    if !comp_id_found {
        return -1;
    }
    let mut message: [u8; LEN_MISC_MESSAGE] = [0u8; LEN_MISC_MESSAGE];
    match hide::ap_secure_receive(&BOARD, &comp_id, &mut message) {
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

/// int get_provisioned_ids(uint32_t* buffer);
/// Returns the number of provisioned IDs, stores the IDs in buffer
#[no_mangle]
pub extern "C" fn get_provisioned_ids(buffer: *mut u32) -> i32 {
    let mut comp_count: i32 = 0;
    for i in 0..COMPONENT_CNT {
        let mut comp_id: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
        if flash::get_provisioned_component_id(&mut comp_id, i) {
            // Copy the component ID into the buffer
            // let id: u32 = (comp_id[0] as u32) | ((comp_id[1] as u32) << 8) | ((comp_id[2] as u32) << 16) | ((comp_id[3] as u32) << 24);
            let id: u32 = (comp_id[3] as u32) | ((comp_id[2] as u32) << 8) | ((comp_id[1] as u32) << 16) | ((comp_id[0] as u32) << 24);
            unsafe { buffer.add(comp_count as usize).write(id) };
            comp_count += 1;
        }
        // Hard coded length protection
        if comp_count == 2 {
            break;
        }
    }
    return comp_count;
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