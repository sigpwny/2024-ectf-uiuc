#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::{tmr0, trng, i2c1};
use board::{Board, Led, u8_to_hex_string};
use board::secure_comms as hide;

#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"Board initialized!");

    let mut count = 0;

    //test_ascon(&board);
    //test_random(&board);

    test_i2c(&board);

    loop {
        // This timer logic doesn't make any sense, don't use it
        // let tmr_cnt = tmr0::get_tick_count(&board.tmr0);
        // while tmr0::get_tick_count(&board.tmr0) < tmr_cnt + 50_000_000 { }
        // board.send_host_debug(b"Hello, world!");
        // if count % 2 == 0 {
        //     board.led_on(Led::Green);
        // } else {
        //     board.led_off(Led::Green);
        // }
        // // Test panic
        // if count == 20 {
        //     panic!("Panicked after 20 seconds!");
        // }
        // count += 1;
        continue;
    }
}

fn test_i2c(board: &Board) {
    let master = true;
    const LEN_MSG_1: usize = 25;
    const LEN_MSG_2: usize = 17;
    const SLAVE_ADDR: u8 = 0x0b;
    // Master test
    if master {
        i2c1::master_config(&board.i2c1);
        let mut sent = false;
        // Send message to slave
        while !sent {
            let data: &[u8; LEN_MSG_1] = b"This is more than 4 bytes";
            let result = i2c1::master_write_bytes(&board.i2c1, SLAVE_ADDR, data);
            match result {
                i2c1::MasterI2CStatus::Success => {
                    board.send_host_debug(b"[Info] Data sent successfully!");
                    sent = true;
                },
                i2c1::MasterI2CStatus::Nack => {
                    board.send_host_debug(b"[Info] Nack received, retrying...");
                    for _ in 0..50_000_000 {
                        cortex_m::asm::nop();
                    }
                },
                _ => {
                    board.send_host_debug(b"[Info] Unknown error, retrying...");
                    for _ in 0..50_000_000 {
                        cortex_m::asm::nop();
                    }
                },
            }
        }
        // board.led_on(Led::Green);
        // Read message from slave
        let mut data: [u8; LEN_MSG_2] = [0; LEN_MSG_2];
        let mut received = false;
        while !received {
            let result = i2c1::master_read_bytes(&board.i2c1, SLAVE_ADDR, &mut data);
            match result {
                i2c1::MasterI2CStatus::Success => {
                    board.send_host_debug(b"[Info] Data received from slave:");
                    board.send_host_debug(&data);
                    received = true;
                },
                i2c1::MasterI2CStatus::Nack => {
                    board.send_host_debug(b"[Info] Nack received, retrying...");
                    for _ in 0..50_000_000 {
                        cortex_m::asm::nop();
                    }
                },
                _ => {
                    board.send_host_debug(b"[Info] Unknown error, retrying...");
                    for _ in 0..50_000_000 {
                        cortex_m::asm::nop();
                    }
                },
            }
        }
        // board.led_on(Led::Blue);
    // Slave test
    } else {
        i2c1::slave_config(&board.i2c1, SLAVE_ADDR);
        let mut data: [u8; LEN_MSG_1] = [0; LEN_MSG_1];
        i2c1::slave_read_bytes(&board.i2c1, &mut data);
        board.send_host_debug(b"[Info] Data received from master:");
        board.send_host_debug(&data);
        // board.led_on(Led::Green);
        let msg2: &[u8; LEN_MSG_2] = b"Hello from slave!";
        i2c1::slave_write_bytes(&board.i2c1, msg2);
        // board.led_on(Led::Blue);
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
