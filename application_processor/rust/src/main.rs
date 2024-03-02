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

fn list_components(board: &Board) {
    // Example of secure send/receive to component


    // let first_send: [u8; LEN_MAX_SECURE] = [0; LEN_MAX_SECURE]; // initialize with null bytes
    // let mut first_response: [u8; LEN_MAX_SECURE] = [0; LEN_MAX_SECURE];
    // first_send[u8; LEN_MAX_SECURE] = "PINGCOMPONENT".as_bytes(); // Ping the component. Then it should respond with it's ID.
    // secure_send(&first_send);
    // secure_recv(&first_response);
    const CID_STRING_LENGTH = 8;
    let mut provision_response: [u8; 13] = b"P>0x00000000\n";    
    let CID0_str: [u8;  CID_STRING_LENGTH] = u32_to_hex_string(get_provisioned_component_id(0)); // We pass in 0 for the 0th component
    for i in 0 .. 8{
        provision_response[i+4] = CID0_str[i];
    }
    
    board.send_host_info(&provision_response);

    
    let CID1_str: [u8;  CID_STRING_LENGTH] = u32_to_hex_string(get_provisioned_component_id(1)); // We pass in 1 for the 1st component
    for i in 0 .. 8{
        provision_response[i+4] = CID1_str[i];
    }
    board.send_host_info(&provision_response);    

    // now process the response, etc.
    let mut ping_byte: u8 = MAGIC_LIST_PING;
    let mut response: [u8;CID_STRING_LENGTH];
    let mut found_response:[u8;13] = b"F>0x00000000\n";
    let success_response: [u8;5 ] = b"List\n";
    for I2Caddr in 0x8..0x79{
        if addr == 0x18 || addr == 0x28 || addr == 0x36 {
            continue;
        }
        let mut fake_id:[u8;4];
        fake_id[0] = 0;
        fake_id[1] = 0;
        fake_id[2] = 0;
        fake_id[3] = I2Caddr as u8;
        if hide::ap_secure_send(&fake_id,&ping_byte) == 0 {
            hide::ap_secure_receive(&fake_id,&response);
            for i in 0 .. 8{
                found_response[i+4] = response[i];
            }
            board.send_host_info(&found_response);
        } 
       
    }
    board.send_host_success(&success_response);

}
