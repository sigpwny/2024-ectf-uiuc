#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::{tmr0, trng};
use board::{Board, Led, u8_to_hex_string};
use board::secure_comms as hide;

#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"Board initialized!");

    test_ascon(&board);
    test_random(&board);

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

fn test_uart(board: &Board) {
    let mut buffer: [u8; 64] = [0u8; 64];
    board.send_host_debug(b"Enter a message:");
    let len = board.read_host_line(&mut buffer);
    match len {
        Some(len) => {
            board.send_host_debug(b"Received message:");
            board.send_host_debug(&buffer[..len]);
        }
        None => {
            board.send_host_debug(b"Failed to read message");
        }
    }
}

fn test_ascon(board: &Board) {
    let message: [u8; hide::LEN_ASCON_128_PTXT] = [b'A'; hide::LEN_ASCON_128_PTXT];
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
            break;
        }
    }
}

fn test_random(board: &Board) {
    let mut random: [u8; 16] = [0u8; 16];
    trng::random_bytes(&board.trng, &mut random);
    board.send_host_debug(b"Random bytes:");
    for byte in random.iter() {
        board.send_host_debug(&u8_to_hex_string(*byte));
    }
}