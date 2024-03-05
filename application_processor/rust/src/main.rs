#![no_std]
#![no_main]

use cortex_m_rt::entry;
use sha3::{Digest, Sha3_512};
use ectf_board::{
    Board,
    hal::i2c1,
    secure_comms as hide,
    ectf_constants::{*},
    debug,
    ret_error,
};

mod ectf_ap_params;
use ectf_ap_params::{*};

mod post_boot;
mod flash;


#[entry]
fn main() -> ! {
    let board = Board::new();
    i2c1::master_config(&board.i2c1);
    debug!(board, b"AP initialized!");

    loop {
        let mut host_command: [u8; LEN_MAX_INPUT] = [0u8; LEN_MAX_INPUT];
        match board.gets(&mut host_command) {
            Some(len) => {
                match core::str::from_utf8(&host_command[0..len]) {
                    Ok("list") => {
                        debug!(board, b"Listing components...");
                        list_components(&board);
                    }
                    Ok("attest") => {
                        debug!(board, b"Attesting component...");
                        attest_component(&board);
                    }
                    Ok("replace") => {
                        debug!(board, b"Replacing component...");
                        replace_component(&board);
                    }
                    Ok("boot") => {
                        debug!(board, b"Booting...");
                        boot_verify(&board);
                    }
                    // TODO: Remove
                    // Ok("test") => {
                    //     use tests::{*};
                    //     test_hide(&board, false);
                    //     test_ascon(&board);
                    //     test_random(&board);
                    //     test_flash(&board);
                    //     test_timer(&board);
                    //     loop {
                    //         board.delay_timer_wait_random_us(1_000_000, 5_000_000);
                    //         debug!(board, b"Hello, world!");
                    //         board.led_toggle(Led::Green);
                    //     }
                    // }
                    _ => {
                        debug!(board, b"Unknown command");
                    }
                }
            }
            None => {
                debug!(board, b"No buffer overflow for you!");
            }
        }
        continue;
    }
}

/// Compare a SHA3-512 hash with a salt and password
macro_rules! compare_sha3_512_hash {
    ($correct_hash:expr, $salt:expr, $password:expr) => {{
        let mut hasher = Sha3_512::new();
        hasher.update($salt);
        hasher.update($password);
        let result = hasher.finalize();
        &result.as_slice() == &$correct_hash
    }};
}

/// List all provisioned components and scan for connected components
fn list_components(board: &Board) {
    board.transaction_timer_reset();
    // Print each provisioned component first
    for comp_idx in 0..COMPONENT_CNT {
        let mut prov_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
        match flash::get_provisioned_component_id(&mut prov_cid, comp_idx) {
            true => board.send_host_cid(b'P', &prov_cid),
            false => debug!(board, b"Failed to get provisioned component ID"),
        }
    }
    // Scan for components on the I2C bus
    for addr in 0x8..0x78 {
        if flash::is_i2c_addr_blacklisted(addr) {
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
    match board.gets(&mut input) {
        Some(LEN_ATTEST_PIN) => pin.copy_from_slice(&input[0..LEN_ATTEST_PIN]),
        _ => ret_error!(board, b"Invalid attestation PIN length"),
    }
    debug!(board, b"Got PIN: ");
    debug!(board, &pin);
    // Get the component ID
    let mut in_cid: [u8; LEN_CID_HEX_STRING] = [0u8; LEN_CID_HEX_STRING];
    board.send_host_ack();
    match board.gets(&mut input) {
        Some(LEN_INPUT_CID_HEX_STRING) => in_cid.copy_from_slice(&input[2..LEN_INPUT_CID_HEX_STRING]),
        _ => ret_error!(board, b"Invalid component ID length"),
    }
    debug!(board, b"Got new CID: ");
    debug!(board, &in_cid);
    let mut cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
    for i in 0..LEN_COMPONENT_ID {
        cid[i] = ectf_board::hex_string_to_u8(&in_cid[2*i..2*i+2]);
    }

    // Validate hashes of the attestation PIN
    board.delay_timer_wait_random_us(100, 100_000);
    let is_correct1 = compare_sha3_512_hash!(AP_PIN_HASH_1, AP_PIN_SALT_1, &pin);
    board.delay_timer_wait_random_us(100, 100_000);
    let is_correct2 = compare_sha3_512_hash!(AP_PIN_HASH_2, AP_PIN_SALT_2, &pin);
    board.delay_timer_wait_random_us(100, 100_000);
    let is_correct3 = compare_sha3_512_hash!(AP_PIN_HASH_3, AP_PIN_SALT_3, &pin);

    // Wait until 1.5 seconds total
    board.transaction_timer_wait_until_us(1_500_000);

    // If all three hashes are correct, send the attestation data
    board.delay_timer_wait_random_us(100, 10_000);
    if is_correct1 {
        board.delay_timer_wait_random_us(100, 10_000);
        if is_correct2 {
            board.delay_timer_wait_random_us(100, 10_000);
            if is_correct3 {
                if retrieve_attest_data(&board, &cid) == 0 {
                    return;
                }
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    // Wait until 7.5 seconds total (3 second requirement + 5 second attack delay)
    board.transaction_timer_wait_until_us(7_500_000);
    ret_error!(board, b"Attest failed");
}

/// Retrieve attestation data from a component and send it to the host.
fn retrieve_attest_data(board: &Board, cid: &[u8; LEN_COMPONENT_ID]) -> i32 {
    // Validate that the requested component ID is provisioned
    for i in 0..COMPONENT_CNT {
        let mut prov_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
        flash::get_provisioned_component_id(&mut prov_cid, i);
        if *cid == prov_cid {
            // AP requests LOCATION
            let message: [u8; LEN_MISC_MESSAGE] = [MAGIC_MISC_REQ_LOCATION; LEN_MISC_MESSAGE];
            match hide::ap_secure_send(board, cid, &message) {
                Some(LEN_MISC_MESSAGE) => (),
                _ => {
                    debug!(board, b"Failed to send LOCATION request");
                    return -1;
                }
            }
            board.delay_timer_wait_us(10_000);
            // AP receives LOCATION
            let mut attest_location: [u8; LEN_ATTEST_LOCATION] = [0u8; LEN_ATTEST_LOCATION];
            match hide::ap_secure_receive(board, cid, &mut attest_location) {
                Some(LEN_ATTEST_LOCATION) => (),
                _ => {
                    debug!(board, b"Failed to receive LOCATION");
                    return -1;
                }
            }
            board.delay_timer_wait_us(10_000);
            // AP requests DATE
            let message: [u8; LEN_MISC_MESSAGE] = [MAGIC_MISC_REQ_DATE; LEN_MISC_MESSAGE];
            match hide::ap_secure_send(board, cid, &message) {
                Some(LEN_MISC_MESSAGE) => (),
                _ => {
                    debug!(board, b"Failed to send DATE request");
                    return -1;
                }
            }
            board.delay_timer_wait_us(10_000);
            // AP receives DATE
            let mut attest_date: [u8; LEN_ATTEST_DATE] = [0u8; LEN_ATTEST_DATE];
            match hide::ap_secure_receive(board, cid, &mut attest_date) {
                Some(LEN_ATTEST_DATE) => (),
                _ => {
                    debug!(board, b"Failed to receive DATE"); // rip :(
                    return -1;
                }
            }
            board.delay_timer_wait_us(10_000);
            // AP requests CUSTOMER
            let message: [u8; LEN_MISC_MESSAGE] = [MAGIC_MISC_REQ_CUSTOMER; LEN_MISC_MESSAGE];
            match hide::ap_secure_send(board, cid, &message) {
                Some(LEN_MISC_MESSAGE) => (),
                _ => {
                    debug!(board, b"Failed to send CUSTOMER request");
                    return -1;
                }
            }
            board.delay_timer_wait_us(10_000);
            // AP receives CUSTOMER
            let mut attest_customer: [u8; LEN_ATTEST_CUSTOMER] = [0u8; LEN_ATTEST_CUSTOMER];
            match hide::ap_secure_receive(board, cid, &mut attest_customer) {
                Some(LEN_ATTEST_CUSTOMER) => (),
                _ => {
                    debug!(board, b"Failed to receive CUSTOMER");
                    return -1;
                }
            }
            board.delay_timer_wait_us(10_000);

            // Send the attestation data to the host
            board.send_host_cid(b'C', &cid);
            board.send_host_attest_data(&attest_location, &attest_date, &attest_customer);
            board.send_host_success(b"Attest");
            return 0;
        }
    }
    return -1;
}

/// Replace a provisioned component ID with a new component ID
fn replace_component(board: &Board) {
    board.transaction_timer_reset();
    let mut input: [u8; LEN_MAX_INPUT] = [0u8; LEN_MAX_INPUT];
    // Get the replacement token
    let mut token: [u8; LEN_REPLACEMENT_TOKEN] = [0u8; LEN_REPLACEMENT_TOKEN];
    board.send_host_ack();
    match board.gets(&mut input) {
        Some(LEN_REPLACEMENT_TOKEN) => token.copy_from_slice(&input[0..LEN_REPLACEMENT_TOKEN]),
        _ => ret_error!(board, b"Invalid replacement token length")
    }
    debug!(board, b"Got token: ");
    debug!(board, &token);
    // Get the new component ID
    let mut in_new_cid: [u8; LEN_CID_HEX_STRING] = [0u8; LEN_CID_HEX_STRING];
    board.send_host_ack();
    match board.gets(&mut input) {
        Some(LEN_INPUT_CID_HEX_STRING) => in_new_cid.copy_from_slice(&input[2..LEN_INPUT_CID_HEX_STRING]),
        _ => ret_error!(board, b"Invalid new component ID length")
    }
    debug!(board, b"Got new CID: ");
    debug!(board, &in_new_cid);
    // Convert from ASCII hex string [u8; 8] to byte string [u8; 4]
    let mut new_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
    for i in 0..LEN_COMPONENT_ID {
        new_cid[i] = ectf_board::hex_string_to_u8(&in_new_cid[2*i..2*i+2]);
    }
    // Get the old component ID to replace
    let mut in_old_cid: [u8; LEN_CID_HEX_STRING] = [0u8; LEN_CID_HEX_STRING];
    board.send_host_ack();
    match board.gets(&mut input) {
        Some(LEN_INPUT_CID_HEX_STRING) => in_old_cid.copy_from_slice(&input[2..LEN_INPUT_CID_HEX_STRING]),
        _ => ret_error!(board, b"Invalid old component ID length")
    }
    debug!(board, b"Got old CID: ");
    debug!(board, &in_old_cid);
    // Convert from ASCII hex string [u8; 8] to byte string [u8; 4]
    let mut old_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
    for i in 0..LEN_COMPONENT_ID {
        old_cid[i] = ectf_board::hex_string_to_u8(&in_old_cid[2*i..2*i+2]);
    }

    // Validate hashes of the replacement token
    board.delay_timer_wait_random_us(100, 1_000_000);
    let is_correct1 = compare_sha3_512_hash!(AP_TOKEN_HASH_1, AP_TOKEN_SALT_1, &token);
    board.delay_timer_wait_random_us(100, 1_000_000);
    let is_correct2 = compare_sha3_512_hash!(AP_TOKEN_HASH_2, AP_TOKEN_SALT_2, &token);
    board.delay_timer_wait_random_us(100, 1_000_000);
    let is_correct3 = compare_sha3_512_hash!(AP_TOKEN_HASH_3, AP_TOKEN_SALT_3, &token);

    // Wait until 4.5 seconds total
    board.transaction_timer_wait_until_us(4_500_000);

    // If all three hashes are correct, replace the old component ID with the new component ID
    board.delay_timer_wait_random_us(100, 10_000);
    if is_correct1 {
        board.delay_timer_wait_random_us(100, 10_000);
        if is_correct2 {
            board.delay_timer_wait_random_us(100, 10_000);
            if is_correct3 {
                for i in 0..COMPONENT_CNT {
                    let mut prov_cid: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
                    flash::get_provisioned_component_id(&mut prov_cid, i);
                    if old_cid == prov_cid {
                        if flash::set_provisioned_component_id(board, &new_cid, i) {
                            board.send_host_success(b"Replace");
                            return;
                        }
                        ret_error!(board, b"Flash failure");
                    }
                }
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }
    // Wait until 9.5 seconds total (5 second requirement + 5 second attack delay)
    board.transaction_timer_wait_until_us(9_500_000);
    ret_error!(board, b"Replace failed");
}

/// Initiates the boot process by verifying attached components
fn boot_verify(board: &Board) {
    board.transaction_timer_reset();
    // Validate each provisioned component
    board.delay_timer_wait_random_us(100, 500_000);
    let mut comp_ids: [[u8; LEN_COMPONENT_ID]; COMPONENT_CNT as usize] = [[0u8; LEN_COMPONENT_ID]; COMPONENT_CNT as usize];
    for idx in 0..COMPONENT_CNT {
        let i = idx as usize;
        flash::get_provisioned_component_id(&mut comp_ids[i], idx);
        board.delay_timer_wait_random_us(100, 500_000);
        // AP sends BOOT_PING
        let message: [u8; LEN_MISC_MESSAGE] = [MAGIC_MISC_BOOT_PING; LEN_MISC_MESSAGE];
        match hide::ap_secure_send(board, &comp_ids[i], &message) {
            Some(LEN_MISC_MESSAGE) => (),
            _ => {
                debug!(board, b"Failed to send BOOT_PING command");
                ret_error!(board, b"Boot failed");
            }
        }
        board.delay_timer_wait_us(10_000);
        // AP receives BOOT_PONG
        let mut boot_pong_msg: [u8; LEN_MISC_MESSAGE] = [0u8; LEN_MISC_MESSAGE];
        match hide::ap_secure_receive(board, &comp_ids[i], &mut boot_pong_msg) {
            Some(LEN_MISC_MESSAGE) => (),
            _ => {
                debug!(board, b"Failed to receive BOOT_PONG message");
                ret_error!(board, b"Boot failed");
            }
        }
        // Verify the BOOT_PONG message
        board.delay_timer_wait_random_us(100, 10_000);
        for byte in 0..LEN_MISC_MESSAGE {
            board.delay_timer_wait_random_us(100, 10_000);
            if boot_pong_msg[byte] != MAGIC_MISC_BOOT_PONG {
                debug!(board, b"Invalid BOOT_PONG message");
                ret_error!(board, b"Boot failed");
            }
        }
        board.delay_timer_wait_us(10_000);
    }

    // Wait 0.5 second total
    board.transaction_timer_wait_until_us(500_000);

    // Boot each component and retrieve boot messages
    let mut comp_boot_msgs: [[u8; LEN_COMPONENT_BOOT_MSG]; COMPONENT_CNT as usize] = [[0u8; LEN_COMPONENT_BOOT_MSG]; COMPONENT_CNT as usize];
    for i in 0..COMPONENT_CNT as usize {
        // AP sends BOOT_NOW
        let message: [u8; LEN_MISC_MESSAGE] = [MAGIC_MISC_BOOT_NOW; LEN_MISC_MESSAGE];
        match hide::ap_secure_send(board, &comp_ids[i], &message) {
            Some(LEN_MISC_MESSAGE) => (),
            _ => {
                debug!(board, b"Failed to send BOOT_NOW command");
                ret_error!(board, b"Boot failed");
            }
        }
        board.delay_timer_wait_us(10_000);
        // AP receives component boot message
        match hide::ap_secure_receive(board, &comp_ids[i], &mut comp_boot_msgs[i]) {
            Some(LEN_COMPONENT_BOOT_MSG) => (),
            _ => {
                debug!(board, b"Failed to receive component boot message");
                ret_error!(board, b"Boot failed");
            }
        }
        board.delay_timer_wait_us(10_000);
    }

    // Send boot messages to host
    for i in 0..COMPONENT_CNT as usize {
        board.send_host_comp_boot_msg(&comp_ids[i], &comp_boot_msgs[i]);
    }
    board.send_host_ap_boot_msg(AP_BOOT_MSG);
    board.send_host_success(b"Boot");

    // Start post boot
    // Safety: This jumps to the C POST_BOOT code. C is inherently unsafe so...
    unsafe { post_boot::post_boot(); }
}