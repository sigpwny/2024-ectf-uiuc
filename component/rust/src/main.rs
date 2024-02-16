#![no_std]
#![no_main]

use crate::*;
use cortex_m_rt::entry;


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

fn post_boot() {

}