#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::tmr0;
use board::{Board, Led, u8_to_hex_string, u32_to_hex_string};
use board::secure_comms as hide;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

// Don't use magic numbers - always define as constants!

/**
 * Lengths
 */
const LEN_MAX_SECURE: usize = 64;
const LEN_MAX_HOST:   usize = 32;
const LEN_MAX_AP_BOOT_NOW: usize = 64;
const LEN_MAX_BOOT_PINGPONG: usize = 64;
const LEN_MAX_COMP_BOOT_MSG: usize = 64;
const LEN_MAX_AP_BOOT_MSG: usize = 64;
const LEN_MAX_HOST_SUCCESS_MSG: usize = 64;
const LEN_BOOT_SUCCESS_MSG: usize = 5;
const LEN_MAX_HOST_ERROR_MSG: usize = 64;
const LEN_BOOT_ERROR_MSG: usize = 20;
const LEN_COMP_ID: usize = 4;


/**
 * Magic bytes
 */
 // TODO: Add more here
const CID1: u8 = 0x10;
const CID2: u8 = 0x11;
const MAGIC_LIST_PING: u8 = 0x50;
const MAGIC_LIST_PONG: u8 = 0x51;


 const MAGIC_BOOT_PING: u8 = 0x80;
 const MAGIC_BOOT_PONG: u8 = 0x81;
 const MAGIC_BOOT_NOW:  u8 = 0x82;
 const MAGIC_NEW_LINE: u8 = 0x0a;


// Reference Rust code from last year: https://github.com/sigpwny/2023-ectf-sigpwny/blob/main/docker_env/src/bin/car.rs

mod ectf_params;
use ectf_params::{
    AP_PIN_HASH,
    AP_TOKEN_HASH,
    AP_BOOT_MSG,
    COMPONENT_CNT,
    // ORIGINAL_COMPONENT_IDS, // DO NOT USE THESE!
};

mod tests;
use tests::{
    test_uart,
    test_ascon,
    test_random,
    test_flash,
    test_timer,
};

#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"Board initialized!");

    test_ascon(&board);
    test_random(&board);
    test_flash(&board);
    test_timer(&board);

    let mut host_data: [u8; LEN_MAX_HOST];
    recv_host(&host_data); // Read data from the Host!

    //
    loop {
        // TODO: UART loop to listen for messages from the Host Computer
    }
}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/list_tool.py
fn list_components() {
    // Example of secure send/receive to component


    // let first_send: [u8; LEN_MAX_SECURE] = [0; LEN_MAX_SECURE]; // initialize with null bytes
    // let mut first_response: [u8; LEN_MAX_SECURE] = [0; LEN_MAX_SECURE];
    // first_send[u8; LEN_MAX_SECURE] = "PINGCOMPONENT".as_bytes(); // Ping the component. Then it should respond with it's ID.
    // secure_send(&first_send);
    // secure_recv(&first_response);

    // now process the response, etc.
    let ping_byte: u8 = MAGIC_LIST_PING;
    let mut first_response: u8  = 0;
    secure_send(&ping_byte); 
    secure_recv(&first_response); // Check i2c bus if there is data, then repeatedly receive a response.

    delay(1000000);

    if first_response == MAGIC_LIST_PONG{
        let success_message: [u8; LEN_MAX_SECURE] = "%success: Component found%".as_bytes();
        send_host_success(success_message);
    } else {
        let error_message: [u8; LEN_MAX_SECURE] = "%error: Component not found%".as_bytes();
        send_host_error(error_message);
    }
    /*
     let start_time = Instant::now();
        let timeout = Duration::from_secs(1);
        secure_recv(&first_response);
        /*Wait for a response from the component */
      if Instant::now() - start_time >= timeout {
            println!("One second has elapsed. Exiting...");
            break;
        }
     */


}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/attestation_tool.py
fn attest_component() {

}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/replace_tool.py
fn replace_component() {

}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/boot_tool.py
fn boot_verify(boasrd: &Board) {
    board.timer_reset();
    // Check whether components are present and valid
    for i in 0..COMPONENT_CNT {
        let comp_id: [u8; LEN_COMP_ID] = board.get_provisioned_component_id(i);

        if validate_component(comp_id) != 0 {
            let mut boot_fail_msg: [u8; LEN_MAX_HOST_ERROR_MSG] = [0; LEN_MAX_HOST_ERROR_MSG];
            boot_fail_msg[0..LEN_BOOT_ERROR_MSG].copy_from_slice(b"Component Boot Fail:");
            boot_fail_msg[LEN_BOOT_ERROR_MSG - 1] = comp_id[LEN_COMP_ID - 1];
            board.send_host_error(&boot_fail_msg);
            return;
        }
    }

    // Wait 2.8 Seconds minimum before continuing
    board.delay_total_us(2_800_000);

    // Boot Components and Send messages to Host
    for i in 0..COMPONENT_CNT {
        let comp_id: [u8; LEN_COMP_ID] = board.get_provisioned_component_id(i);
        boot_component(board, comp_id);
    }
    
    // Send AP boot message
    board.send_host_info(AP_BOOT_MSG.as_bytes());

    // Generate and send boot success message
    let mut boot_success_msg: [u8; LEN_MAX_HOST_SUCCESS_MSG] = [0; LEN_MAX_HOST_SUCCESS_MSG];
    boot_success_msg[0..(LEN_BOOT_SUCCESS_MSG - 1)].copy_from_slice(b"Boot");
    boot_success_msg[LEN_BOOT_SUCCESS_MSG - 1] = MAGIC_NEW_LINE;
    board.send_host_success(&boot_success_msg);

    // Start post boot
    post_boot();
}

fn validate_component(comp_id [u8, LEN_COMP_ID]) -> i32 {
    // Send boot.ping and get boot.pong to component 1
    let send_comp_ping: [u8; LEN_MAX_BOOT_PINGPONG] = [MAGIC_BOOT_PING; LEN_MAX_BOOT_PINGPONG];
    let mut recieve_comp_pong: [u8; LEN_MAX_BOOT_PINGPONG] = [0; LEN_MAX_BOOT_PINGPONG];
    hide::ap_secure_send(comp_id, &send_comp_ping);
    hide::ap_secure_recv(comp_id, &recieve_comp_pong);

    // Check if sent back value is boot.pong
    if recieve_comp_pong[0] != MAGIC_BOOT_PONG {
        return -1;
    }
    return 0;
}
 
fn boot_component(board: &Board, comp_id [u8, LEN_COMP_ID]) {
    // Boot first component
    let send_comp_boot: [u8; LEN_MAX_AP_BOOT_NOW] = [MAGIC_BOOT_NOW; LEN_MAX_AP_BOOT_NOW];
    let mut recieve_comp_boot: [u8; LEN_MAX_COMP_BOOT_MSG] = [0; LEN_MAX_COMP_BOOT_MSG];
    hide::ap_secure_send(comp_id, &send_comp_boot);
    hide::ap_secure_recv(comp_id, &recieve_comp_boot);

    // Send component boot message
    board.send_host_info(&recieve_comp_boot);
}

fn post_boot() {

    let mut count: i32 = 0;
    for _ in 0..20 {
        let tick_count = tmr0::get_tick_count(&board.tmr0);
        while tmr0::get_tick_count(&board.tmr0) < tick_count + 50_000_000 { }
        if (count % 2) == 0 {
            board.led_on(Led::Green);
        } else {
            board.led_off(Led::Green);
        }
        board.send_host_debug(b"Hello, world!");
        count += 1;
    }

    loop {
        test_uart(&board);
        continue;
    }
}