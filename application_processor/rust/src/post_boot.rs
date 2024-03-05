use ectf_board::{
    post_boot_shared::{*},
    Board,
    pac,
    secure_comms as hide,
    ectf_constants::{LEN_COMPONENT_ID, LEN_MISC_MESSAGE}
};
use crate::{
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
    match hide::ap_secure_send(&board, &comp_id, &message) {
        Some(LEN_MISC_MESSAGE) => 0,
        _ => -1,
    }
}

/// int secure_receive(i2c_addr_t address, uint8_t* buffer);
/// Returns number of bytes received on success, -1 on failure
#[no_mangle]
pub extern "C" fn secure_receive(address: u8, buffer: *mut u8) -> i32 {
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
    match hide::ap_secure_receive(&board, &comp_id, &mut message) {
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