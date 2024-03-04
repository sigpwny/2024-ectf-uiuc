#![no_std]
#![no_main]

use cortex_m_rt::entry;
use board::{Board, Led, u8_to_hex_string, u32_to_hex_string};
use board::secure_comms as hide;
use board::ectf_constants::{*};
use board::ectf_params::{
    COMPONENT_ID,
    COMPONENT_BOOT_MSG,
    ATTESTATION_LOCATION,
    ATTESTATION_DATE,
    ATTESTATION_CUSTOMER,
};

mod post_boot;
use post_boot::post_boot;

pub enum ComponentCommand {
    AttestReqLocation,
    AttestReqDate,
    AttestReqCustomer,
    BootPing,
    BootNow,
}

// Every byte in the message should match the magic byte string for the command
// Determine which command to check from the first byte
fn resolve_command(board: &Board, bytes: &[u8]) -> Option<ComponentCommand> {
    let initial = match bytes[0] {
        MAGIC_MISC_REQ_LOCATION => ComponentCommand::AttestReqLocation,
        MAGIC_MISC_REQ_DATE => ComponentCommand::AttestReqDate,
        MAGIC_MISC_REQ_CUSTOMER => ComponentCommand::AttestReqCustomer,
        MAGIC_MISC_BOOT_PING => ComponentCommand::BootPing,
        MAGIC_MISC_BOOT_NOW => ComponentCommand::BootNow,
        _ => {
            board.send_host_debug(b"Unknown message");
            return None;
        }
    };
    for byte in bytes {
        if *byte != bytes[0] {
            board.send_host_debug(b"Magic byte mismatch");
            return None;
        }
    }
    Some(initial)
}

#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"Component initialized!");

    loop {
        let mut magic: [u8; hide::LEN_MISC_MESSAGE] = [0u8; hide::LEN_MISC_MESSAGE];
        let result = hide::comp_secure_receive(&board, &COMPONENT_ID, &mut magic);
        match result {
            Some(len) => {
                if len != hide::LEN_MISC_MESSAGE {
                    board.send_host_debug(b"Length does not match");
                    continue;
                }
            }
            None => {
                board.send_host_debug(b"Failed to receive message");
                continue;
            }
        }
        match resolve_command(&board, &magic) {
            Some(ComponentCommand::AttestReqLocation) => {
                // send_attest_data();
            }
            Some(ComponentCommand::AttestReqDate) => {
                // send_attest_data();
            }
            Some(ComponentCommand::AttestReqCustomer) => {
                // send_attest_data();
            }
            Some(ComponentCommand::BootPing) => {
                // boot_verify();
            }
            Some(ComponentCommand::BootNow) => {
                // Safety: This function is defined in our C code
                // Unsafety: DO NOT DO THIS IN FINAL DESIGN! DO BOOT VERIFICATION FIRST!
                // unsafe { post_boot() };
                continue;
            }
            None => {
                continue;
            }
        }
        continue;
    }
}

fn send_attest_data() {
    let mut APLOCBYTE: u8 = \x00;
    let APLOCREQ: u8 = hide::comp_secure_receive(&APLOCBYTE);
}

// fn boot_verify() {

// }
