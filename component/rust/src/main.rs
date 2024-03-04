#![no_std]
#![no_main]

use cortex_m_rt::entry;
use board::Board;
use board::secure_comms as hide;
use board::ectf_constants::{*};
use board::ectf_params::{*};

mod post_boot;
use post_boot::post_boot;

pub enum ComponentCommand {
    AttestReqLocation,
    AttestReqDate,
    AttestReqCustomer,
    BootPing,
    BootNow,
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
                send_boot_pong(&board);
            }
            Some(ComponentCommand::BootNow) => {
                boot_component(&board);
            }
            None => {
                continue;
            }
        }
        continue;
    }
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
    board.delay_timer_wait_random_us(1_000, 10_000);
    for byte in bytes {
        board.delay_timer_wait_random_us(100, 1_000);
        if *byte != bytes[0] {
            board.send_host_debug(b"Magic byte mismatch");
            return None;
        }
    }
    Some(initial)
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

/// Sends a boot pong message to the AP
fn send_boot_pong(board: &Board) {
    let boot_pong_msg: [u8; LEN_MISC_MESSAGE] = [MAGIC_MISC_BOOT_PONG; LEN_MISC_MESSAGE];
    match hide::comp_secure_send(board, &COMPONENT_ID, &boot_pong_msg) {
        Some(LEN_MISC_MESSAGE) => board.send_host_debug(b"Boot pong sent"),
        _ => board.send_host_debug(b"Failed to send boot pong"),
    }
}

/// Sends the boot message to the AP and boots the component
fn boot_component(board: &Board) {
    let mut boot_msg: [u8; LEN_COMPONENT_BOOT_MSG] = [0u8; LEN_COMPONENT_BOOT_MSG];
    for (i, byte) in COMPONENT_BOOT_MSG.iter().enumerate() {
        boot_msg[i] = *byte;
    }
    match hide::comp_secure_send(&board, &COMPONENT_ID, &boot_msg) {
        Some(LEN_COMPONENT_BOOT_MSG) => board.send_host_debug(b"Boot message sent"),
        _ => {
            board.send_host_debug(b"Failed to send boot message");
            return;
        }
    }

    // Start post boot
    // Safety: This jumps to the C POST_BOOT code. C is inherently unsafe so...
    unsafe { post_boot(); }
}