use max78000_hal::{trng, i2c1};
use board::{Board, u8_to_hex_string, u32_to_hex_string};
use board::secure_comms as hide;

pub fn test_hide(board: &Board, is_ap: bool) {
    board.timer_reset();
    let component_id: &[u8; 4] = b"\x11\x11\x11\x24";
    const MSG_LEN_1: usize = 19;
    const MSG_LEN_2: usize = 4;
    if is_ap {
        i2c1::master_config(&board.i2c1);
        let message: &[u8; MSG_LEN_1] = b"Hey there, go boot!";
        let result = hide::ap_secure_send(&board, &component_id, message);
        match result {
            Some(len) => {
                if len != MSG_LEN_1 {
                    board.send_host_debug(b"[Error] Length does not match");
                } else {
                    board.send_host_debug(b"[Info] Sent message:");
                    board.send_host_debug(message);
                }
            }
            None => {
                board.send_host_debug(b"[Error] Failed to send message");
            }
        }
        let mut buffer: [u8; MSG_LEN_2] = [0u8; MSG_LEN_2];
        board.delay_us(1_000_000);
        let result = hide::ap_secure_receive(&board, &component_id, &mut buffer);
        match result {
            Some(len) => {
                if len != MSG_LEN_2 {
                    board.send_host_debug(b"[Error] Length does not match");
                } else {
                    board.send_host_debug(b"[Info] Received message:");
                    board.send_host_debug(&buffer);
                }
            }
            None => {
                board.send_host_debug(b"[Error] Failed to receive message");
            }
        }
    } else {
        i2c1::slave_config(&board.i2c1, component_id[3]);
        let mut buffer: [u8; MSG_LEN_1] = [0u8; MSG_LEN_1];
        let result = hide::comp_secure_receive(&board, &component_id, &mut buffer);
        match result {
            Some(len) => {
                if len != MSG_LEN_1 {
                    board.send_host_debug(b"[Error] Length does not match");
                } else {
                    board.send_host_debug(b"[Info] Received message:");
                    board.send_host_debug(&buffer);
                }
            }
            None => {
                board.send_host_debug(b"[Error] Failed to receive message");
            }
        }
        let message: &[u8; MSG_LEN_2] = b"Hey!";
        let result = hide::comp_secure_send(&board, &component_id, message);
        match result {
            Some(len) => {
                if len != MSG_LEN_2 {
                    board.send_host_debug(b"[Error] Length does not match");
                } else {
                    board.send_host_debug(b"[Info] Sent message:");
                    board.send_host_debug(message);
                }
            }
            None => {
                board.send_host_debug(b"[Error] Failed to send message");
            }
        }
    }
}

pub fn test_uart(board: &Board) {
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

pub fn test_ascon(board: &Board) {
    // let message: [u8; 96] = [b'A'; 96];
    // let associated_data: [u8; hide::LEN_ASCON_128_AD] = [1, 2, 3, 4, 5, 6, 7, 8];
    // let nonce: [u8; hide::LEN_ASCON_128_NONCE] = [7u8; hide::LEN_ASCON_128_NONCE];
    // let key: &[u8] = b"\x12\x34\x56\x78\x90\xAB\xCD\xEF\x12\x34\x56\x78\x90\xAB\xCD\xEF";
    // let mut ciphertext: [u8; 96] = [0u8; 96];
    // let mut plaintext: [u8; 96] = [0u8; 96];

    // board.send_host_debug(b"Testing Ascon-128!");
    // board.send_host_debug(b"Original message:");
    // board.send_host_debug(&message);

    // let result = hide::ascon_encrypt(
    //     &mut ciphertext,
    //     &message,
    //     &associated_data,
    //     &nonce,
    //     &key,
    // );
    // if result != 0 {
    //     panic!("Failed to encrypt message");
    // }
    // board.send_host_debug(b"Encrypted message:");
    // board.send_host_debug(&ciphertext);

    // // let wrong_associated_data: [u8; hide::LEN_ASCON_128_AD] = [1, 2, 3, 4, 5, 6, 7, 9];
    // let result = hide::ascon_decrypt(
    //     &mut plaintext,
    //     &ciphertext,
    //     &associated_data,
    //     &nonce,
    //     &key,
    // );
    // if result != 0 {
    //     board.send_host_debug(b"Failed to decrypt message");
    // }

    // board.send_host_debug(b"Decrypted message:");
    // board.send_host_debug(&plaintext);

    // for i in 0..96 {
    //     if message[i] != plaintext[i] {
    //         board.send_host_debug(b"Plaintext does not match message");
    //         break;
    //     }
    // }
}

pub fn test_random(board: &Board) {
    let mut random: [u8; 16] = [0u8; 16];
    trng::random_bytes(&board.trng, &mut random);
    board.send_host_debug(b"Random bytes:");
    for byte in random.iter() {
        board.send_host_debug(&u8_to_hex_string(*byte));
    }
}

pub fn test_flash(board: &Board) {
    let addr: u32 = 0x1004_4100;
    let data: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    // Print the original flash contents
    let addr_ptr = addr as *const u8;
    board.send_host_debug(b"Original flash contents:");
    for i in 0..4 {
        let byte = unsafe { addr_ptr.add(i).read() };
        board.send_host_debug(&u8_to_hex_string(byte));
    }
    // Success: Write new data to flash
    board.write_flash_bytes(addr, &data);
    board.send_host_debug(b"Wrote 4 bytes to flash");
    board.send_host_debug(b"New flash contents:");
    for i in 0..4 {
        let byte = unsafe { addr_ptr.add(i).read() };
        if byte != data[i] {
            board.send_host_debug(b"The byte below does not match expected output!");
        }
        board.send_host_debug(&u8_to_hex_string(byte));
    }
    // Success: Should write after erase
    let data: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    board.write_flash_bytes(addr, &data);
    board.send_host_debug(b"Wrote 4 bytes to flash");
    board.send_host_debug(b"New flash contents:");
    for i in 0..4 {
        let byte = unsafe { addr_ptr.add(i).read() };
        if byte != data[i] {
            board.send_host_debug(b"The byte below does not match expected output!");
        }
        board.send_host_debug(&u8_to_hex_string(byte));
    }
    board.send_host_debug(b"Success: Wrote to flash!");
}

pub fn test_timer(board: &Board) {
    board.timer_reset();
    board.send_host_debug(b"Timer reset!");
    let current_us = board.timer_get_us();
    board.send_host_debug(b"Current time (us):");
    board.send_host_debug(&u32_to_hex_string(current_us));
    // Block for 1 second
    board.delay_us(1_000_000);
    board.send_host_debug(b"1 second has passed!");
    // Block for 2 seconds
    board.delay_us(2_000_000);
    board.send_host_debug(b"3 seconds have passed!");
    // Block until 5 seconds total (since reset) have passed
    board.delay_total_us(10_000_000);
    board.send_host_debug(b"10 seconds have passed!");
    board.timer_reset();
    board.send_host_debug(b"Timer reset!");
    let current_us = board.timer_get_us();
    board.send_host_debug(b"Current time (us):");
    board.send_host_debug(&u32_to_hex_string(current_us));
    // Block for 1 second
    board.delay_us(1_000_000);
    board.send_host_debug(b"1 second has passed!");
    // Block for 2 seconds
    board.delay_us(2_000_000);
    board.send_host_debug(b"3 seconds have passed!");
    // Block until 5 seconds total (since reset) have passed
    board.delay_total_us(10_000_000);
    board.send_host_debug(b"10 seconds have passed!");
    board.timer_reset();
    board.send_host_debug(b"Timer reset!");
}