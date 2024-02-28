#![no_std]
#![no_main]

use crate::*;
use cortex_m_rt::entry;

const CID:u32 = 0x12345678;

#[entry]
fn main() -> ! {
    // TODO: Initialization
    loop {
        // TODO: I2C loop to listen for messages from the AP
        
    }
}

fn list_component(board: &Board) {
    let mut response: [u8; LEN_MAX_SECURE];
    let CID_str:[u8;8] = u32_to_hex_string(CID);
    for i in 0 .. 8{
        response[i] = CID_str[i];
    }
    board.securesend(&response);         // Send the component ID to the AP... Where are our CIDs located?



}

fn send_attest_data() {

}

fn boot_verify() {

}

fn post_boot() {

}