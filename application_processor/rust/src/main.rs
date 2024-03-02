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

    let mut count: i32 = 0;
    for _ in 0..20 {
        let tick_count = tmr0::get_tick_count(&board.tmr0);
        while tmr0::get_tick_count(&board.tmr0) < tick_count + 50_000_000 { }
        if (count % 2) == 0 {
            board.led_on(Led::Green);
        } else {
            board.led_off(Led::Green);
        }
 

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
        
}       board.send_host_debug(b"Hello, world!");
        count += 1;
    }
        
    loop {
        test_uart(&board);
        continue;
        }
}