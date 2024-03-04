#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::{i2c1, tmr0};
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

    // let mut count: i32 = 0;
    // for _ in 0..10 {
    //     let tick_count = tmr0::get_tick_count(&board.tmr0);
    //     while tmr0::get_tick_count(&board.tmr0) < tick_count + 50_000_000 { }
    //     if (count % 2) == 0 {
    //         board.led_on(Led::Green);
    //     } else {
    //         board.led_off(Led::Green);
    //     }
    //     board.send_host_debug(b"Hello, world! This is AP!");
    //     count += 1;
    // }
        
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

/// List all provisioned components and scan for connected components
fn list_components(board: &Board) {
    board.timer_reset();
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
    // Scan for components on the I2C bus
    for addr in 0x8..0x78 {
        if addr == 0x18 || addr == 0x28 || addr == 0x36 {
            continue;
        }
        // Small delay in between requests
        board.delay_us(10_000);
        let hide_pkt_req_list: [u8; hide::LEN_HIDE_PKT_REQ] = [hide::MAGIC_PKT_REQ_LIST; hide::LEN_HIDE_PKT_REQ];
        let mut component_id: [u8; LEN_COMPONENT_ID] = [0u8; LEN_COMPONENT_ID];
        let result = i2c1::master_write_bytes(&board.i2c1, addr, &hide_pkt_req_list);
        match result {
            i2c1::MasterI2CStatus::Success => {
                // Small delay to allow the component to respond
                board.delay_us(10_000);
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

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/replace_tool.py
fn replace_component(Board::&board) {
    board.timer_reset();
    // delay for 3 seconds
    board.delay_us(3000000);
    let mut flag : bool = false;
    let mut buffer = [0u8; 256];
    board.read_host_line(&mut buffer);
    let replacement_token = &buffer[0..16];
    let old_component_id = &buffer[16..20];
    let new_component_id = &buffer[20..24];
        
    Argon2::new(Variant::Argon2i, Version::V0x13, params);
        
    // Compare Token Attempt hash to stored Correct Token hash
    // Argon2::new(Variant::Argon2i, Version::V0x13, params);

    //password = replacement_token;
    
    if(Argon2::default().verify_password(replacement_token, AP_TOKEN_HASH).is_ok()) {
            flag = true;
    }

    // Wait until 4.8 seconds total time elapsed since beginning of transaction
    board.delay_total_us(4800000);
        
    // Update Component ID list with new Component ID if flag is true, and send success/error message
    if (flag) {
        for i in 0..COMPONENT_CNT {
            if (board.get_provisioned_component_id(i) == old_component_id){
                //TODO: Overwrite component i id with new_component_id (board.write_flash_bytes())
                let success_message: [u8; LEN_MAX_SECURE] = "%success: Replacement success%".as_bytes();
                send_host_success(success_message);
            }
            else {
                let error_message: [u8; LEN_MAX_SECURE] = "%error: Replacement failed, no matching componentID%".as_bytes();
                send_host_error(error_message);
            }
        }
    } else {
        let error_message: [u8; LEN_MAX_SECURE] = "%error: Replacement failed%".as_bytes();
        send_host_error(error_message);
    }
}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/boot_tool.py
fn boot_verify() {

}
