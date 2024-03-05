use ectf_board::{
    post_boot_shared::{*},
    Board,
    pac,
    secure_comms as hide,
    ectf_constants::LEN_MISC_MESSAGE
};
use crate::ectf_comp_params::COMPONENT_ID;


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