#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::tmr0;
use board::{Board, Led, u8_to_hex_string, u32_to_hex_string};
use board::secure_comms as hide;

mod ectf_params;
use ectf_params::{
    COMPONENT_ID,
    COMPONENT_BOOT_MSG,
    ATTESTATION_LOCATION,
    ATTESTATION_DATE,
    ATTESTATION_CUSTOMER,
};


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
const LEN_COMP_BOOT_MSG: usize = 22; //Temporary
const LEN_MAX_AP_BOOT_MSG: usize = 64;
const LEN_AP_ID: usize = 4;


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

// Called if recived message equals boot.ping
fn validate_components() {
    let send_ap_pong: [u8; LEN_MAX_BOOT_PINGPONG] = [MAGIC_BOOT_PONG; LEN_MAX_BOOT_PINGPONG];
    comp_secure_send(&send_ap_pong);
}       

// Called if recived message equals Boot Now
fn boot_components() {
    let mut send_boot_msq: [u8; LEN_MAX_COMP_BOOT_MSG] = COMPONENT_BOOT_MSG.as_bytes(); // temp assignment should use flash function
    send_boot_msg[0..LEN_COMP_BOOT_MSG].copy_from_slice(COMPONENT_BOOT_MSG.as_bytes());
    comp_secure_send(&send_boot_msq)
}

fn post_boot() {

}