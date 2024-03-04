#![no_std]
#![no_main]

use cortex_m_rt::entry;
use sha3::{Digest, Sha3_512};
use max78000_hal::i2c1;
use board::{Board, Led, hex_string_to_u8};
use board::secure_comms as hide;
use board::ectf_constants::{*};
use board::ectf_params::{*};

mod post_boot;
use post_boot::post_boot;

mod tests;

#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"AP initialized!");

    loop {
        let mut host_command: [u8; LEN_MAX_INPUT] = [0u8; LEN_MAX_INPUT];
        match board.read_host_line(&mut host_command) {
            Some(len) => {
                match core::str::from_utf8(&host_command[0..len]) {
                    Ok("list") => {
                        board.send_host_debug(b"Listing components...");
                        list_components(&board);
                    }
                    Ok("attest") => {
                        board.send_host_debug(b"Attesting component...");
                        attest_component(&board);
                    }
                    Ok("replace") => {
                        board.send_host_debug(b"Replacing component...");
                        replace_component(&board);
                    }
                    Ok("boot") => {
                        board.send_host_debug(b"Booting...");
                        // boot_verify();
                    }
                    // TODO: Remove
                    Ok("test") => {
                        // use tests::{*};
                        // test_hide(&board, false);
                        // test_ascon(&board);
                        // test_random(&board);
                        // test_flash(&board);
                        // test_timer(&board);
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
                board.send_host_debug(b"No buffer overflow for you!");
                board.send_host_debug(b"Here's a free null dereference instead :)");
                // Safety: This is completely safe.
                unsafe { core::ptr::read_volatile(0x0000_0000 as *const u8); }
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
        match i2c1::master_write_bytes(&board.i2c1, addr, &hide_pkt_req_list) {
            i2c1::MasterI2CStatus::Success => {
                // Small delay to allow the component to respond
                board.delay_timer_wait_us(10_000);
                match i2c1::master_read_bytes(&board.i2c1, addr, &mut component_id) {
                    i2c1::MasterI2CStatus::Success => board.send_host_cid(b'F', &component_id),
                    _ => ()
                }
            }
            _ => ()
        }
    }
    board.send_host_success(b"List");
}

/// Attest a component
fn attest_component(board: &Board) {
    board.transaction_timer_reset();
    let mut input: [u8; LEN_MAX_INPUT] = [0u8; LEN_MAX_INPUT];
    // Get the attestation PIN
    let mut pin: [u8; LEN_ATTEST_PIN] = [0u8; LEN_ATTEST_PIN];
    board.send_host_ack();
    match board.read_host_line(&mut input) {
        Some(LEN_ATTEST_PIN) => {
            pin.copy_from_slice(&input[0..LEN_ATTEST_PIN]);
        },
        _ => {
            board.send_host_error(b"Invalid attestation PIN length");
            return;
        }
    }
    board.send_host_debug(b"Got PIN: ");
    board.send_host_debug(&pin);
    // Get the component ID
    let mut in_cid: [u8; LEN_CID_HEX_STRING] = [0u8; LEN_CID_HEX_STRING];
    board.send_host_ack();
    match board.read_host_line(&mut input) {
        Some(LEN_INPUT_CID_HEX_STRING) => {
            // Truncate "0x" from the input component ID
            in_cid.copy_from_slice(&input[2..LEN_INPUT_CID_HEX_STRING]);
        },
        _ => {
            board.send_host_error(b"Invalid component ID length");
            return;
        }
    }
    board.send_host_debug(b"Got new CID: ");
    board.send_host_debug(&in_cid);
    // Convert from ASCII hex string [u8; 8] to byte string [u8; 4]
    let mut cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
    for i in 0..LEN_COMPONENT_ID {
        cid[i] = hex_string_to_u8(&in_cid[2*i..2*i+2]);
    }

    // Validate first SHA3-512 hash of the attestation PIN
    board.delay_timer_wait_random_us(1_000, 300_000);
    let mut hasher = Sha3_512::new();
    hasher.update(AP_PIN_SALT_1);
    hasher.update(&pin);
    let result = hasher.finalize();
    board.delay_timer_wait_random_us(1_000, 300_000);
    let is_correct1 = &result.as_slice() == &AP_PIN_HASH_1;
    // Validate second SHA3-512 hash of the attestation PIN
    board.delay_timer_wait_random_us(1_000, 300_000);
    let mut hasher = Sha3_512::new();
    hasher.update(AP_PIN_SALT_2);
    hasher.update(&pin);
    let result = hasher.finalize();
    board.delay_timer_wait_random_us(1_000, 300_000);
    let is_correct2 = &result.as_slice() == &AP_PIN_HASH_2;
    // Validate third SHA3-512 hash of the attestation PIN
    board.delay_timer_wait_random_us(1_000, 300_000);
    let mut hasher = Sha3_512::new();
    hasher.update(AP_PIN_SALT_3);
    hasher.update(&pin);
    let result = hasher.finalize();
    board.delay_timer_wait_random_us(1_000, 300_000);
    let is_correct3 = &result.as_slice() == &AP_PIN_HASH_3;

    // Wait until 1.8 seconds total
    board.transaction_timer_wait_until_us(1_800_000);

    // If all three hashes are correct, send the attestation data
    board.delay_timer_wait_random_us(1_000, 100_000);
    if is_correct1 {
        board.delay_timer_wait_random_us(1_000, 100_000);
        if is_correct2 {
            board.delay_timer_wait_random_us(1_000, 100_000);
            if is_correct3 {
                retrieve_attest_data(&board, &cid);
                return;
            }
        }
    }

    // Wait until 8 seconds total (3 second requirement + 5 second attack delay)
    board.transaction_timer_wait_until_us(8_000_000);
    board.send_host_error(b"Attest failed");
}

/// Retrieve attestation data from a component and send it to the host.
fn retrieve_attest_data(board: &Board, cid: &[u8; LEN_COMPONENT_ID]) {
    // Validate that the requested component ID is provisioned
    for i in 0..COMPONENT_CNT {
        let mut prov_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
        board.get_provisioned_component_id(&mut prov_cid, i);
        if *cid == prov_cid {
            board.send_host_cid(b'C', &cid);
            // AP requests LOCATION
            let message: [u8; LEN_MISC_MESSAGE] = [MAGIC_MISC_REQ_LOCATION; LEN_MISC_MESSAGE];
            match hide::ap_secure_send(board, cid, &message) {
                Some(LEN_MISC_MESSAGE) => (),
                _ => {
                    board.send_host_debug(b"Failed to send LOCATION request");
                    board.send_host_error(b"Attest failed");
                    return;
                }
            }
            board.delay_timer_wait_us(100_000);
            // AP receives LOCATION
            let mut attest_location: [u8; LEN_ATTEST_LOCATION] = [0u8; LEN_ATTEST_LOCATION];
            match hide::ap_secure_receive(board, cid, &mut attest_location) {
                Some(LEN_ATTEST_LOCATION) => (),
                _ => {
                    board.send_host_debug(b"Failed to receive LOCATION");
                    board.send_host_error(b"Attest failed");
                    return;
                }
            }
            board.delay_timer_wait_us(100_000);
            // AP requests DATE
            let message: [u8; LEN_MISC_MESSAGE] = [MAGIC_MISC_REQ_DATE; LEN_MISC_MESSAGE];
            match hide::ap_secure_send(board, cid, &message) {
                Some(LEN_MISC_MESSAGE) => (),
                _ => {
                    board.send_host_debug(b"Failed to send DATE request");
                    board.send_host_error(b"Attest failed");
                    return;
                }
            }
            board.delay_timer_wait_us(100_000);
            // AP receives DATE
            let mut attest_date: [u8; LEN_ATTEST_DATE] = [0u8; LEN_ATTEST_DATE];
            match hide::ap_secure_receive(board, cid, &mut attest_date) {
                Some(LEN_ATTEST_DATE) => (),
                _ => {
                    board.send_host_debug(b"Failed to receive DATE"); // rip :(
                    board.send_host_error(b"Attest failed");
                    return;
                }
            }
            board.delay_timer_wait_us(100_000);
            // AP requests CUSTOMER
            let message: [u8; LEN_MISC_MESSAGE] = [MAGIC_MISC_REQ_CUSTOMER; LEN_MISC_MESSAGE];
            match hide::ap_secure_send(board, cid, &message) {
                Some(LEN_MISC_MESSAGE) => (),
                _ => {
                    board.send_host_debug(b"Failed to send CUSTOMER request");
                    board.send_host_error(b"Attest failed");
                    return;
                }
            }
            board.delay_timer_wait_us(100_000);
            // AP receives CUSTOMER
            let mut attest_customer: [u8; LEN_ATTEST_CUSTOMER] = [0u8; LEN_ATTEST_CUSTOMER];
            match hide::ap_secure_receive(board, cid, &mut attest_customer) {
                Some(LEN_ATTEST_CUSTOMER) => (),
                _ => {
                    board.send_host_debug(b"Failed to receive CUSTOMER");
                    board.send_host_error(b"Attest failed");
                    return;
                }
            }
            board.delay_timer_wait_us(100_000);

            // Send the attestation data to the host
            board.send_host_attest_data(&attest_location, &attest_date, &attest_customer);
            board.send_host_success(b"Attest");
            return;
        }
    }
}

/// Replace a provisioned component ID with a new component ID
fn replace_component(board: &Board) {
    board.transaction_timer_reset();
    let mut input: [u8; LEN_MAX_INPUT] = [0u8; LEN_MAX_INPUT];
    // Get the replacement token
    let mut token: [u8; LEN_REPLACEMENT_TOKEN] = [0u8; LEN_REPLACEMENT_TOKEN];
    board.send_host_ack();
    match board.read_host_line(&mut input) {
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
    match board.read_host_line(&mut input) {
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
    match board.read_host_line(&mut input) {
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

    // Validate first SHA3-512 hash of the replacement token
    board.delay_timer_wait_random_us(1_000, 1_000_000);
    let mut hasher = Sha3_512::new();
    hasher.update(AP_TOKEN_SALT_1);
    hasher.update(&token);
    let result = hasher.finalize();
    board.delay_timer_wait_random_us(1_000, 500_000);
    let is_correct1 = &result.as_slice() == &AP_TOKEN_HASH_1;
    // Validate second SHA3-512 hash of the replacement token
    board.delay_timer_wait_random_us(1_000, 1_000_000);
    let mut hasher = Sha3_512::new();
    hasher.update(AP_TOKEN_SALT_2);
    hasher.update(&token);
    let result = hasher.finalize();
    board.delay_timer_wait_random_us(1_000, 500_000);
    let is_correct2 = &result.as_slice() == &AP_TOKEN_HASH_2;
    // Validate third SHA3-512 hash of the replacement token
    board.delay_timer_wait_random_us(1_000, 1_000_000);
    let mut hasher = Sha3_512::new();
    hasher.update(AP_TOKEN_SALT_3);
    hasher.update(&token);
    let result = hasher.finalize();
    board.delay_timer_wait_random_us(1_000, 500_000);
    let is_correct3 = &result.as_slice() == &AP_TOKEN_HASH_3;

    // Wait until 4.5 seconds total
    board.transaction_timer_wait_until_us(4_500_000);

    // If all three hashes are correct, replace the old component ID with the new component ID
    board.delay_timer_wait_random_us(1_000, 100_000);
    if is_correct1 {
        board.delay_timer_wait_random_us(1_000, 100_000);
        if is_correct2 {
            board.delay_timer_wait_random_us(1_000, 100_000);
            if is_correct3 {
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
        }
    }
    // Wait until 10 seconds total (5 second requirement + 5 second attack delay)
    board.transaction_timer_wait_until_us(10_000_000);
    board.send_host_error(b"Replace failed");
}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/boot_tool.py
fn boot_verify() {

}
