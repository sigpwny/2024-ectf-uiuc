#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::tmr0;
use board::{Board, Led};
use board::secure_comms as hide;

#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"Board initialized!");

    let mut count = 0;

    test_ascon(&board);

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

fn test_ascon(board: &Board) {
    let message: [u8; hide::LEN_ASCON_128_PTXT] = [65u8; hide::LEN_ASCON_128_PTXT];
    let associated_data: [u8; hide::LEN_ASCON_128_AD] = [1, 2, 3, 4, 5, 6, 7, 8];
    let nonce: [u8; hide::LEN_ASCON_128_NONCE] = [7u8; hide::LEN_ASCON_128_NONCE];
    let key: [u8; hide::LEN_ASCON_128_KEY] = [9u8; hide::LEN_ASCON_128_KEY];
    let mut ciphertext: [u8; hide::LEN_ASCON_128_CTXT] = [0u8; hide::LEN_ASCON_128_CTXT];
    let mut plaintext: [u8; hide::LEN_ASCON_128_PTXT] = [0u8; hide::LEN_ASCON_128_PTXT];

    board.send_host_debug(b"Testing Ascon-128!");
    board.send_host_debug(b"Original message:");
    board.send_host_debug(&message);

    let result = hide::ascon_encrypt(
        &mut ciphertext,
        &message,
        &associated_data,
        &nonce,
        &key,
    );
    if result != 0 {
        panic!("Failed to encrypt message");
    }
    board.send_host_debug(b"Encrypted message:");
    board.send_host_debug(&ciphertext);

    let wrong_associated_data: [u8; hide::LEN_ASCON_128_AD] = [1, 2, 3, 4, 5, 6, 7, 9];
    let result = hide::ascon_decrypt(
        &mut plaintext,
        &ciphertext,
        &wrong_associated_data,
        &nonce,
        &key,
    );
    if result != 0 {
        board.send_host_debug(b"Failed to decrypt message");
    }

    board.send_host_debug(b"Decrypted message:");
    board.send_host_debug(&plaintext);

    for i in 0..64 {
        if message[i] != plaintext[i] {
            board.send_host_debug(b"Plaintext does not match message");
        }
    }
}