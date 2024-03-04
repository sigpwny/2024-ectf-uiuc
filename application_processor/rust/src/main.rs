#![no_std]
#![no_main]

use cortex_m_rt::entry;
use sha3::{Digest, Sha3_512};
use max78000_hal::i2c1;
use board::{Board, Led, u8_to_hex_string, u32_to_hex_string, hex_string_to_u8};
use board::secure_comms as hide;
use board::ectf_constants::{*};
use board::ectf_params::{*};

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

    loop {
        let mut host_command: [u8; LEN_MAX_INPUT] = [0u8; LEN_MAX_INPUT];
        let len = board.read_host_line(&mut host_command);
        match len {
            Some(len) => {
                match core::str::from_utf8(&host_command[0..len]) {
                    Ok("list") => {
                        board.send_host_debug(b"Listing components...");
                        list_components(&board);
                    }
                    Ok("attest") => {
                        board.send_host_debug(b"Attesting component...");
                        // attest_component();
                    }
                    Ok("replace") => {
                        board.send_host_debug(b"Replacing component...");
                        replace_component(&board);
                    }
                    Ok("boot") => {
                        board.send_host_debug(b"Booting component...");
                        // boot_verify();
                    }
                    // TODO: Remove
                    Ok("test") => {
                        loop {
                            board.delay_timer_wait_random_us(1_000_000, 5_000_000);
                            board.send_host_debug(b"Hello, world!");
                            board.led_toggle(Led::Green);
                        }
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

/// List all provisioned components and scan for connected components
fn list_components(board: &Board) {
    board.transaction_timer_reset();
    // Print each provisioned component first
    for comp_idx in 0..COMPONENT_CNT {
        let mut prov_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
        match board.get_provisioned_component_id(&mut prov_cid, comp_idx) {
            true => board.send_host_cid(b'P', &prov_cid),
            false => board.send_host_debug(b"Failed to get provisioned component ID"),
        }
    }
    // Scan for components on the I2C bus
    for addr in 0x8..0x78 {
        if board.is_i2c_addr_blacklisted(addr) {
            continue;
        }
        // Small delay in between requests
        board.delay_timer_wait_us(10_000);
        let hide_pkt_req_list: [u8; hide::LEN_HIDE_PKT_REQ] = [hide::MAGIC_PKT_REQ_LIST; hide::LEN_HIDE_PKT_REQ];
        let mut component_id: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
        let result = i2c1::master_write_bytes(&board.i2c1, addr, &hide_pkt_req_list);
        match result {
            i2c1::MasterI2CStatus::Success => {
                // Small delay to allow the component to respond
                board.delay_timer_wait_us(10_000);
                let comp_result = i2c1::master_read_bytes(&board.i2c1, addr, &mut component_id);
                match comp_result {
                    i2c1::MasterI2CStatus::Success => {
                        board.send_host_cid(b'F', &component_id);
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }
    board.send_host_success(b"List");
}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/attestation_tool.py
fn attest_component() {

}

/// Replace a provisioned component ID with a new component ID
fn replace_component(board: &Board) {
    board.transaction_timer_reset();
    let mut input: [u8; LEN_MAX_INPUT] = [0u8; LEN_MAX_INPUT];
    // Get the replacement token
    let mut token: [u8; LEN_REPLACEMENT_TOKEN] = [0u8; LEN_REPLACEMENT_TOKEN];
    board.send_host_ack();
    let len = board.read_host_line(&mut input);
    match len {
        Some(LEN_REPLACEMENT_TOKEN) => {
            token.copy_from_slice(&input[0..LEN_REPLACEMENT_TOKEN]);
        },
        _ => {
            board.send_host_error(b"Invalid replacement token length");
            return;
        }
    }
    board.send_host_debug(b"Got token: ");
    board.send_host_debug(&token);
    // Get the new component ID
    let mut in_new_cid: [u8; LEN_CID_HEX_STRING] = [0u8; LEN_CID_HEX_STRING];
    board.send_host_ack();
    let len = board.read_host_line(&mut input);
    match len {
        Some(LEN_INPUT_CID_HEX_STRING) => {
            // Truncate "0x" from the input component ID
            in_new_cid.copy_from_slice(&input[2..LEN_INPUT_CID_HEX_STRING]);
        },
        _ => {
            board.send_host_error(b"Invalid new component ID length");
            return;
        }
    }
    board.send_host_debug(b"Got new CID: ");
    board.send_host_debug(&in_new_cid);
    // Convert from ASCII hex string [u8; 8] to byte string [u8; 4]
    let mut new_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
    for i in 0..LEN_COMPONENT_ID {
        new_cid[i] = hex_string_to_u8(&in_new_cid[2*i..2*i+2]);
    }
    // Get the old component ID to replace
    let mut in_old_cid: [u8; LEN_CID_HEX_STRING] = [0u8; LEN_CID_HEX_STRING];
    board.send_host_ack();
    let len = board.read_host_line(&mut input);
    match len {
        Some(LEN_INPUT_CID_HEX_STRING) => {
            // Truncate "0x" from the input component ID
            in_old_cid.copy_from_slice(&input[2..LEN_INPUT_CID_HEX_STRING]);
        },
        _ => {
            board.send_host_error(b"Invalid old component ID length");
            return;
        }
    }
    board.send_host_debug(b"Got old CID: ");
    board.send_host_debug(&in_old_cid);
    // Convert from ASCII hex string [u8; 8] to byte string [u8; 4]
    let mut old_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
    for i in 0..LEN_COMPONENT_ID {
        old_cid[i] = hex_string_to_u8(&in_old_cid[2*i..2*i+2]);
    }

    // Validate SHA3-512 hash of the replacement token
    board.delay_timer_wait_random_us(1_000, 3_000_000);
    let mut hasher = Sha3_512::new();
    hasher.update(AP_TOKEN_SALT_1);
    hasher.update(&token);
    let result = hasher.finalize();
    // Compare the hash with the expected hash
    board.delay_timer_wait_random_us(1_000, 1_000_000);
    let is_correct = &result.as_slice() == &AP_TOKEN_HASH_1;

    // Wait until 4.5 seconds total
    board.transaction_timer_wait_until_us(4_500_000);

    board.delay_timer_wait_random_us(1_000, 100_000);
    if is_correct {
        board.delay_timer_wait_random_us(1_000, 100_000);
        for i in 0..COMPONENT_CNT {
            let mut prov_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
            board.get_provisioned_component_id(&mut prov_cid, i);
            if old_cid == prov_cid {
                if board.set_provisioned_component_id(&new_cid, i) {
                    board.send_host_success(b"Replace");
                    return;
                }
                board.send_host_error(b"Flash failure");
                return;
            }
        }
    }
    // Wait until 10 seconds total (5 second requirement + 5 second attack delay)
    board.transaction_timer_wait_until_us(10_000_000);
    board.send_host_error(b"Replace failed");
}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/boot_tool.py
fn boot_verify() {

}
