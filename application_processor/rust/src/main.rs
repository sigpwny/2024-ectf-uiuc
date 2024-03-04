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

//argon2 salts
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};


//secure send and receive values
let REQUEST_LOCATION: u32 = 0x60;
let SEND_LOCATION: u32 = 0x61;
let REQUEST_DATE: u32 = 0x62;
let SEND_DATE: u32 = 0x63;
let REQUEST_CUSTOMER: u32 = 0x64;
let SEND_CUSTOMER: u32 = 0x65;


fn send_attest_data(board: &Board) {
    //set the target duration of the operation
    board.timer_reset();
    //Receive comp ID and PIN
    let mut attestation_request: [u8; 64] = [0u8; 64];
    let mut component_request: [u8; 64] = [0u8; 64];
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
    //ADD COMPONENT ID RECEIVE
    let mut correct_complen : bool = True;
    let mut component_len = board.read_host_line(&mut component_request);
    if(component_len != 32){
        correct_complen = False;
    }
    let mut component_id: u32;
    if(correct_complen){
       component_id = board.read_host_line(&mut component_request);
    }else{
       board.delay_us(5_000_000); 
       let message: [u8] = "%error: ComponentID failed%".as_bytes();
       send_host_error(&self, message: &[u8]); /// Write error to the host
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
        //get_provisioned_component_ids(&self) -> [u32; ATTEST_DATA]; 
        //let component_id1: u32 = board.get_provisioned_component_id(0); // Get provisioned component IDs
        //let component_id: u32 = board.get_provisioned_component_id(1); // Get provisioned component IDs
        //AP sends attestation data
        send_host_info(&self, ATTEST_DATA); /// Write info to the host
        // this is in main.rs for AP, you should define a constant for LEN_MSG
        
        //AP requests LOCATION
        let mut message: u8 = REQUEST_LOCATION;
        hide::ap_secure_send(&component_id, &message);
        //AP receives LOCATION
        let LOCATION: [u8; 64] = hide::ap_secure_receive(&component_id);

        //AP requests DATE
        message = SEND_DATE;
        hide::ap_secure_send(&component_id, &message);
        //AP receives DATE
        let DATE: [u8; 64] = hide::ap_secure_receive(&component_id);

        //AP requests CUSTOMER
        message = SEND_CUSTOMER;
        hide::ap_secure_send(&component_id, &message);
        //AP receives CUSTOMER
        let CUSTOMER: [u8; 64] = hide::ap_secure_receive(&component_id);

        //send host info
        //needs work
        //send_host_info(&self, message: &[]);
        //send_host_info(&self, message: &[u8]);
        //send_host_info(&self, message: &[u8]);
        //send_host_info(&self, message: &[u8]);
    }   

}

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

    /// If all three hashes are correct, replace the old component ID with the new component ID
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
