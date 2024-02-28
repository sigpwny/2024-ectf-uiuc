#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::tmr0;
use board::{Board, Led, u8_to_hex_string, u32_to_hex_string};
use board::secure_comms as hide;

mod ectf_global_secrets;
use ectf_global_secrets::{
    ASCON_SECRET_KEY_AP_TO_C,
    ASCON_SECRET_KEY_C_TO_AP,
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

//argon2 salts
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};



fn send_attest_data(board: &Board) {
    //set the target duration of the operation
    board.timer_reset();
    //Receive comp ID and PIN
    let mut attestation_request: [u8; 64] = [0u8; 64];
    let mut len = board.read_host_line(&mut attestation_request);  
    let mut correctlen : bool = True;
    let mut flag : bool = false;
    if(len != 7){
        correctlen = False;
    }
    len = board.read_host_line(&mut attestation_request);
    if(len != 11){
        correctlen = False;
    }

    //Wait a minimum of 0.8s TTT elapsed
    board.delay_total_us(800_000);

    //Argon 2i Compare PIN hash with AP_PIN_HASH
    if(correctlen){
        //configure Argon2i
        let config = Config {
            variant: Variant::Argon2i,
        };

        //checking the PIN
        let salt = SaltString::new(&AP_PIN_HASH);
        let argon2 = Argon2::new(config);
        let pin_hash = argon2.hash_password(attestation_request, &salt)?.to_string();
        let matches = argon2.verify_password(&pin_hash, &AP_PIN_HASH);

        //compare hash
        if Ok(matches) {
            flag = true;
        }
    }

    //Wait a minimum of 2.8s TTT elapsed
    board.delay_total_us(2_800_000);
    if(!flag){
       board.delay_us(5_000_000); 
       let message: [u8] = "%error: Attestation failed%".as_bytes();
       send_host_error(&self, message: &[u8]); /// Write error to the host
    } 
    else { 
        //AP requests attestation data
        get_provisioned_component_ids(&self) -> [u32; ATTEST_DATA]; // Get provisioned component IDs
        //AP sends attestation data
        send_host_info(&self, ATTEST_DATA); /// Write info to the host
        let success_message: [u8; LEN_MAX_SECURE] = "%success: Replacement success%".as_bytes();
        send_host_success(success_message);
    }

}

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
        board.send_host_debug(b"Hello, world!");
        count += 1;
    }

    loop {
        test_uart(&board);
        continue;
    }
}