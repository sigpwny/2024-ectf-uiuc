#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::tmr0;
use board::{Board, Led, u8_to_hex_string, u32_to_hex_string};
use board::secure_comms as hide;
use board::ectf_constants::{*};

use argon2::{
    password_hash::PasswordHash,
    Argon2
};

mod ectf_params;
use ectf_params::{
    AP_PIN_HASH,
    AP_TOKEN_HASH,
    AP_BOOT_MSG,
    COMPONENT_CNT,
    // ORIGINAL_COMPONENT_IDS, // DO NOT USE THESE!
};

mod post_boot;
use post_boot::post_boot;

mod tests;
use tests::{
    test_hide,
    test_uart,
    test_ascon,
    test_random,
    test_flash,
    test_timer,
};

#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"AP initialized!");

    // test_hide(&board, false);
    // test_ascon(&board);
    // test_random(&board);
    // test_flash(&board);
    // test_timer(&board);

    let mut count: i32 = 0;
    for _ in 0..10 {
        let tick_count = tmr0::get_tick_count(&board.tmr0);
        while tmr0::get_tick_count(&board.tmr0) < tick_count + 50_000_000 { }
        if (count % 2) == 0 {
            board.led_on(Led::Green);
        } else {
            board.led_off(Led::Green);
        }
        board.send_host_debug(b"Hello, world! This is AP!");
        count += 1;
    }

    loop {
        let mut host_command: [u8; 64] = [0u8; 64];
        let len = board.read_host_line(&mut host_command);
        match len {
            Some(len) => {
                match core::str::from_utf8(&host_command[0..len]) {
                    Ok("list") => {
                        board.send_host_debug(b"Listing components...");
                        list_components(&board);
                    }
                    Ok("attest") => {
                        // attest_component();
                        board.send_host_debug(b"Attesting component...");
                    }
                    Ok("replace") => {
                        // replace_component();
                        board.send_host_debug(b"Replacing component...");
                    }
                    Ok("boot") => {
                        // boot_verify();
                        board.send_host_debug(b"Booting component...");
                    }
                    _ => {
                        board.send_host_debug(b"Unknown command");
                    }
                }
            }
            None => {
                board.send_host_debug(b"Failed to read message");
            }
        }
        continue;
    }
}

fn list_components(board: &Board) {
    // Print each provisioned component first
    for comp_idx in 0..COMPONENT_CNT {
        let mut prov_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
        match board.get_provisioned_component_id(&mut prov_cid, comp_idx) {
            Some(_) => {
                board.send_host_cid(b'P', &prov_cid);
            }
            None => {
                board.send_host_debug(b"Failed to get provisioned component ID");
            }
        }
    }  

    // now process the response, etc.
    let mut magic_bytes: [u8; hide::LEN_MISC_MESSAGE] = [0u8; hide::LEN_MISC_MESSAGE];
    for i in 0..hide::LEN_MISC_MESSAGE {
        magic_bytes[i] = MAGIC_MISC_LIST_PING;
    }
    for addr in 0x8..0x79{
        if addr == 0x18 || addr == 0x28 || addr == 0x36 {
            continue;
        }
        let test_id: [u8; LEN_COMPONENT_ID] = [0x00, 0x00, 0x00, addr];
        board.send_host_cid(b'T', &test_id);
        let result = hide::ap_secure_send(board, &test_id, &magic_bytes);
        match result {
            Some(len) => {
                if len != hide::LEN_MISC_MESSAGE {
                    board.send_host_debug(b"Length does not match");
                    break;
                }
                let mut buffer: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
                let result = hide::ap_secure_receive(board, &test_id, &mut buffer);
                match result {
                    Some(len) => {
                        if len != LEN_COMPONENT_ID {
                            board.send_host_debug(b"Length does not match");
                        } else {
                            board.send_host_cid(b'F', &buffer);
                        }
                    }
                    None => {
                        board.send_host_debug(b"Failed to receive message");
                        break;
                    }
                }
            }
            None => {
                // board.send_host_debug(b"Failed to send message");
                break;
            }
        }
    }
    board.send_host_success(b"List");
}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/attestation_tool.py
fn attest_component() {

}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/replace_tool.py
fn replace_component() {

}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/boot_tool.py
fn boot_verify() {

}
