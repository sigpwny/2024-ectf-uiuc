#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::tmr0;
use board::{Board, Led, u8_to_hex_string, u32_to_hex_string};
use board::secure_comms as hide;
use board::ectf_params::{
    COMPONENT_ID,
    COMPONENT_BOOT_MSG,
    ATTESTATION_LOCATION,
    ATTESTATION_DATE,
    ATTESTATION_CUSTOMER,
};

mod post_boot;
use post_boot::post_boot;


#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"Component initialized!");

    let mut count: i32 = 0;
    for _ in 0..10 {
        let tick_count = tmr0::get_tick_count(&board.tmr0);
        while tmr0::get_tick_count(&board.tmr0) < tick_count + 50_000_000 { }
        if (count % 2) == 0 {
            board.led_on(Led::Green);
        } else {
            board.led_off(Led::Green);
        }
        board.send_host_debug(b"Hello, world! This is component!");
        count += 1;
    }

    loop {
        // TODO: I2C loop to listen for messages from the AP
        // Safety: This function is defined in our C code
        // Unsafety: DO NOT DO THIS IN FINAL DESIGN! DO BOOT VERIFICATION FIRST!
        unsafe { post_boot() };
        continue;
    }
}

fn list_component() {

}

fn send_attest_data() {

}

fn boot_verify() {

}