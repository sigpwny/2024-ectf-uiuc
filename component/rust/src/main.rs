#![no_std]
#![no_main]

use crate::*;
use cortex_m_rt::entry;


// Argon2 hashing docs: https://docs.rs/argon2/latest/argon2/
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
const LEN_MAX_AP_BOOT_NOW: usize = 64;
const LEN_MAX_BOOT_PINGPONG: usize = 64;
const LEN_MAX_COMP_BOOT_MSG: usize = 64;
const LEN_MAX_AP_BOOT_MSG: usize = 64;
const LEN_MAX_AP_ID: usize = 4;


/**
 * Magic bytes
 */
 // TODO: Add more here
 const MAGIC_BOOT_PING: u8 = 0x80;
 const MAGIC_BOOT_PONG: u8 = 0x81;
 const MAGIC_BOOT_NOW:  u8 = 0x82;


#[entry]
fn main() -> ! {
    // TODO: Initialization
    loop {
        // TODO: I2C loop to listen for messages from the AP
    }
}

fn list_component() {

}

fn send_attest_data() {

}

fn boot_verify() {

}

// Called if recived message equals boot.ping
fn validate_components() {
    let ap_id: [u8; LEN_MAX_AP_ID] = [0;LEN_MAX_AP_ID];
    let send_ap_pong: [u8; LEN_MAX_BOOT_PINGPONG] = [MAGIC_BOOT_PONG; LEN_MAX_BOOT_PINGPONG];
    secure_send(ap_id, &send_ap_pong);
}       

// Called if recived message equals Boot Now
fn boot_components() {
    let ap_id: [u8; LEN_MAX_AP_ID] = [0;LEN_MAX_AP_ID];
    let send_boot_msq: [u8; LEN_MAX_COMP_BOOT_MSG] = "comp_boot_msg".as_bytes();
    secure_send(ap_id, &send_boot_msq)
}

fn post_boot() {

}