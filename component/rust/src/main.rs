#![no_std]
#![no_main]

use crate::*;
use cortex_m_rt::entry;

//secure send and receive values
let REQUEST_LOCATION: u32 = 0x60;
let SEND_LOCATION: u32 = 0x61;
let REQUEST_DATE: u32 = 0x62;
let SEND_DATE: u32 = 0x63;
let REQUEST_CUSTOMER: u32 = 0x64;
let SEND_CUSTOMER: u32 = 0x65;

//params to send.
use ectf_params::{
    COMPONENT_ID,
    COMPONENT_BOOT_MSG,
    ATTESTATION_LOCATION,
    ATTESTATION_DATE,
    ATTESTATION_CUSTOMER,
};

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
    let mut APLOCBYTE: u8 = \x00;
    let APLOCREQ: u8 = hide::comp_secure_receive(&APLOCBYTE);
    


    
}

fn boot_verify() {

}

fn post_boot() {

}