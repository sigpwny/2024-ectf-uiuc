use ectf_board::{
    post_boot_shared::{*},
    Board,
    pac,
    secure_comms as hide,
    ectf_constants::LEN_MISC_MESSAGE
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