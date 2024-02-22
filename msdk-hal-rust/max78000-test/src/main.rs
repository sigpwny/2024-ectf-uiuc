#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::tmr0;
use board::{Board, Led};

#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"Board initialized!");

    let mut count = 0;
    loop {
        // This timer logic doesn't make any sense, don't use it
        let tmr_cnt = tmr0::get_tick_count(&board.tmr0);
        while tmr0::get_tick_count(&board.tmr0) < tmr_cnt + 50_000_000 { }
        board.send_host_debug(b"Hello, world!");
        if count % 2 == 0 {
            board.led_on(Led::Green);
        } else {
            board.led_off(Led::Green);
        }
        // Test panic
        if count == 20 {
            panic!("Panicked after 20 seconds!");
        }
        count += 1;
        continue;
    }
}