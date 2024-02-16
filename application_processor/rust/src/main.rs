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
const LEN_MAX_HOST:   usize = 32;

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
    // NOTE: We are on an embedded system, so we *cannot* use the Rust standard library
    // That means we can't use String or str, so we'll have to use arrays of bytes (u8).
    // Do not use arrays of chars (char) because Rust can consider them to be up to 4 bytes,
    // and we're not doing unicode. Also, Rust strings are not null terminated.
    let hello_string: [u8; 13] = "Hello, world!".as_bytes(); // the type, [u8; 13] means 13 bytes
    // hello_string = "a new string!" // let is const by default, so you can't assign a new value!
    let mut cool_string: [u8; 10] = "Kinda cool".as_bytes(); // make the variable mutable instead!
    cool_string = "Super cool".as_bytes(); // note you HAVE to assign a new string of 10 bytes since that is the type of cool_string

    send_host_info(&hello_string); // you'll need these functions for communicating with the Host Computer
    send_host_ack(&hello_string);
    send_host_success(&hello_string);
    send_host_error(&hello_string);
    send_host_debug(&hello_string);

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
    let first_send: [u8; LEN_MAX_SECURE] = [0; LEN_MAX_SECURE]; // initialize with null bytes
    let mut first_response: [u8; LEN_MAX_SECURE] = [0; LEN_MAX_SECURE];
    secure_send(&first_send);
    secure_recv(&first_response);
    // now process the response, etc.
}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/attestation_tool.py
fn attest_component() {

}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/replace_tool.py
fn replace_component() {

}

// Host I/O should conform with https://github.com/sigpwny/2024-ectf-uiuc/blob/main/ectf_tools/boot_tool.py
fn boot_verify() {

}

fn post_boot() {

}