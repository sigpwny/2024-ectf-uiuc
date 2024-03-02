#![no_std]
#![no_main]

use crate::*;
use cortex_m_rt::entry;

const COMPONENT_ID:[u8;4];

#[entry]
fn main() -> ! {
    // Initialize peripherals
    let peripherals = cortex_m::Peripherals::take().unwrap();
    
    // Initialize board 
    let board = Board::init();

    // Initialize I2C peripheral, how we will communicate with AP 
    let i2c = I2c::new(peripherals.I2C, /* fill in your I2C configuration */);

    loop {
        // TODO: I2C loop to listen for messages from the AP 

        
        
    }

    
}

fn list_component(board: &Board) {
    
    const CID_STRING_LENGTH = 8;
    let mut COMPONENT_ID_32 : u32;
    COMPONENT_ID_32 = (COMPONENT_ID[3] as u32) | ((COMPONENT_ID[2] as u32) << 8) | ((COMPONENT_ID[1] as u32) << 16) | ((COMPONENT_ID[0] as u32) << 24)
    let response: [u8; CID_STRING_LENGTH] = u32_to_hex_string(COMPONENT_ID_32);
    hide::comp_secure_send(&response);         // Send the component ID to the AP

}

// fn send_attest_data() {

// }

// fn boot_verify() {

// }

// fn post_boot() {

// }