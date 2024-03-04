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
        let mut magic: [u8; LEN_MISC_MESSAGE] = [0u8; LEN_MISC_MESSAGE];
        match hide::comp_secure_receive(&board, &COMPONENT_ID, &mut magic) {
            Some(LEN_MISC_MESSAGE) => (),
            _ => {
                board.send_host_debug(b"Failed to receive message");
                continue;
            }
        }
        match resolve_command(&board, &magic) {
            Some(ComponentCommand::AttestReqLocation) => {
                send_attest_location(&board);
            }
            Some(ComponentCommand::AttestReqDate) => {
                send_attest_date(&board);
            }
            Some(ComponentCommand::AttestReqCustomer) => {
                send_attest_customer(&board);
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

/// Send the attestation location data to the host
fn send_attest_location(board: &Board) {
    let mut buffer: [u8; LEN_ATTEST_LOCATION] = [0u8; LEN_ATTEST_LOCATION];
    for (i, byte) in ATTESTATION_LOCATION.iter().enumerate() {
        buffer[i] = *byte;
    }
    match hide::comp_secure_send(&board, &COMPONENT_ID, &buffer) {
        Some(LEN_ATTEST_LOCATION) => board.send_host_debug(b"Location sent"),
        _ => board.send_host_debug(b"Failed to send location"),
    }
}

/// Send the attestation date data to the host
fn send_attest_date(board: &Board) {
    let mut buffer: [u8; LEN_ATTEST_DATE] = [0u8; LEN_ATTEST_DATE];
    for (i, byte) in ATTESTATION_DATE.iter().enumerate() {
        buffer[i] = *byte;
    }
    match hide::comp_secure_send(&board, &COMPONENT_ID, &buffer) {
        Some(LEN_ATTEST_DATE) => board.send_host_debug(b"Date sent"),
        _ => board.send_host_debug(b"Failed to send date"),
    }
}

/// Send the attestation customer data to the host
fn send_attest_customer(board: &Board) {
    let mut buffer: [u8; LEN_ATTEST_CUSTOMER] = [0u8; LEN_ATTEST_CUSTOMER];
    for (i, byte) in ATTESTATION_CUSTOMER.iter().enumerate() {
        buffer[i] = *byte;
    }
    match hide::comp_secure_send(&board, &COMPONENT_ID, &buffer) {
        Some(LEN_ATTEST_CUSTOMER) => board.send_host_debug(b"Customer sent"),
        _ => board.send_host_debug(b"Failed to send customer"),
    }
}

// fn boot_verify() {

// }

// Called if recived message equals boot.ping
fn validate_components() {
    let send_ap_pong: [u8; LEN_MAX_BOOT_PINGPONG] = [MAGIC_BOOT_PONG; LEN_MAX_BOOT_PINGPONG];
    hide::comp_secure_send(&send_ap_pong);
}       

// Called if recived message equals Boot Now
fn boot_components() {
    hide::comp_secure_send(COMPONENT_BOOT_MSG.as_bytes())

    // Start post boot
    post_boot();
}