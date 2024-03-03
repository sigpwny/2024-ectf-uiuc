use ascon::{crypto_aead_encrypt, crypto_aead_decrypt};
use max78000_hal::{i2c1, uart0};
use max78000_hal::trng;
use crate::Board;
use crate::{u8_to_hex_string, u32_to_hex_string};
use crate::ectf_global_secrets::ASCON_SECRET_KEYS;

// Length of component ID
// TODO: move to different constants file
pub const LEN_COMPONENT_ID:     usize = 4;

/// Magic bytes for HIDE communication
pub const MAGIC_PKT_REQ:        u8 = 0x40;
pub const MAGIC_PKT_CHAL_SEND:  u8 = 0x41;
pub const MAGIC_PKT_CHAL_RESP:  u8 = 0x42;

/// HIDE packet component lengths
pub const LEN_HIDE_CHAL_NONCE:  usize = 16;
pub const LEN_MISC_MESSAGE:     usize = 80;
pub const LEN_HIDE_MESSAGE:     usize = LEN_HIDE_CHAL_NONCE + LEN_MISC_MESSAGE;

// HIDE packet lengths
pub const LEN_HIDE_PKT_REQ:     usize = 1;
pub const LEN_HIDE_PKT_CHAL_SEND:   usize = LEN_ASCON_128_NONCE + LEN_HIDE_CHAL_NONCE;
pub const LEN_HIDE_PKT_CHAL_RESP:   usize = LEN_ASCON_128_NONCE + LEN_HIDE_MESSAGE;
pub const LEN_HIDE_PKT_MAX:     usize = LEN_HIDE_PKT_CHAL_RESP;

// Ascon-128 lengths
pub const LEN_ASCON_128_KEY:    usize = 16;
pub const LEN_ASCON_128_TAG:    usize = 16;
pub const LEN_ASCON_128_NONCE:  usize = 16;
pub const LEN_ASCON_128_AD:     usize = 8;
pub const LEN_ASCON_128_PTXT:   usize = LEN_HIDE_MESSAGE;
pub const LEN_ASCON_128_CTXT:   usize = LEN_ASCON_128_PTXT;

#[repr(C, align(16))]
pub struct Ascon128Keys {
    pub ap_to_c:    [u8; 16],
    pub c_to_ap:    [u8; 16],
}

#[repr(C, align(16))]
pub struct Ascon128Data {
    pub ciphertext: [u8; LEN_ASCON_128_CTXT],
    pub message:    [u8; LEN_ASCON_128_PTXT],
    pub ad:         [u8; LEN_ASCON_128_AD],
    pub nonce:      [u8; LEN_ASCON_128_NONCE],
}

impl Ascon128Data {
    pub fn new(
        board: &Board,
        comp_id: &[u8; LEN_COMPONENT_ID],
        pkt_magic: u8,
    ) -> Ascon128Data {
        let mut ascon_data = Ascon128Data {
            ciphertext: [0u8; LEN_ASCON_128_CTXT],
            message:    [0u8; LEN_ASCON_128_PTXT],
            ad:         [0u8; LEN_ASCON_128_AD],
            nonce:      [0u8; LEN_ASCON_128_NONCE],
        };
        trng::random_bytes(&board.trng, &mut ascon_data.ciphertext);
        trng::random_bytes(&board.trng, &mut ascon_data.message);
        gen_associated_data(&mut ascon_data.ad, comp_id, pkt_magic);
        trng::random_bytes(&board.trng, &mut ascon_data.nonce);
        return ascon_data;
    }
    /// Helper function to set the HIDE challenge nonce
    pub fn set_challenge_nonce(&mut self, challenge_nonce: &[u8; LEN_HIDE_CHAL_NONCE]) {
        self.message[0..LEN_HIDE_CHAL_NONCE].copy_from_slice(challenge_nonce);
    }
    /// Helper function to set the MISC message in the HIDE packet
    pub fn set_misc_message(&mut self, misc_message: &[u8]) {
        self.message[LEN_HIDE_CHAL_NONCE..].copy_from_slice(misc_message);
    }
}


/// For AP: Send a message to the component.
/// Everything in `app_message` will be sent.
/// Returns the number of bytes sent as an option (None if HIDE is invalidated 
/// in any way)
#[must_use]
pub fn ap_secure_send(
    board: &Board,
    comp_id: &[u8; LEN_COMPONENT_ID],
    app_message: &[u8]
) -> Option<usize> {
    return secure_send(board, true, comp_id, app_message);
}

/// For Component: Send a message to the AP.
/// Everything in `app_message` will be sent.
#[must_use]
pub fn comp_secure_send(
    board: &Board,
    comp_id: &[u8; LEN_COMPONENT_ID],
    app_message: &[u8]
) -> Option<usize> {
    return secure_send(board, false, comp_id, app_message);
}

/// For AP: Receive a message from the component.
/// `output` should be the length of the message that needs to be received.
#[must_use]
pub fn ap_secure_receive(
    board: &Board,
    comp_id: &[u8; LEN_COMPONENT_ID],
    output: &mut [u8]
) -> Option<usize> {
    return secure_receive(board, true, comp_id, output);
}

/// For Component: Receive a message from the AP.
/// `output` should be the length of the message that needs to be received.
#[must_use]
pub fn comp_secure_receive(
    board: &Board,
    comp_id: &[u8; LEN_COMPONENT_ID],
    output: &mut [u8]
) -> Option<usize> {
    return secure_receive(board, false, comp_id, output);
}

/// Implements the HIDE protocol to send a message
#[must_use]
pub fn secure_send(
    board: &Board,
    is_master: bool,
    comp_id: &[u8; LEN_COMPONENT_ID],
    misc_message: &[u8]
) -> Option<usize> {
    if misc_message.len() > LEN_MISC_MESSAGE {
        return None;
    }
    let i2c_addr = comp_id[LEN_COMPONENT_ID - 1];
    // Step 1: Send PKT_REQ
    let packet_msg_req: [u8; LEN_HIDE_PKT_REQ] = [MAGIC_PKT_REQ; LEN_HIDE_PKT_REQ];
    if is_master {
        i2c1::master_write_bytes(&board.i2c1, i2c_addr, &packet_msg_req);
    } else {
        i2c1::slave_write_bytes(&board.i2c1, &packet_msg_req);
    }
    // Step 2: Receive PKT_CHAL_SEND, construct PKT_CHAL_RESP with solved challenge
    // let mut recv_buffer: [u8; LEN_HIDE_MESSAGE] = [0u8; LEN_HIDE_MESSAGE];
    let mut hide_pkt_chal_send = Ascon128Data::new(board, comp_id, MAGIC_PKT_CHAL_SEND);
    // if ascon_receive(board, is_master, comp_id, MAGIC_PKT_CHAL_SEND, &mut recv_buffer) != 0 {
    //     return None;
    // }
    if ascon_receive(board, is_master, i2c_addr, &mut hide_pkt_chal_send) != 0 {
        return None;
    }
    let mut hide_pkt_chal_resp = Ascon128Data::new(board, comp_id, MAGIC_PKT_CHAL_RESP);
    let mut solved_nonce: [u8; LEN_HIDE_CHAL_NONCE] = [0u8; LEN_HIDE_CHAL_NONCE];
    solve_hide_challenge(&mut solved_nonce, &hide_pkt_chal_send.message);
    // Step 3: Send PKT_CHAL_RESP with our solve challenge and app message
    // let mut send_buffer: [u8; LEN_HIDE_MESSAGE] = [0; LEN_HIDE_MESSAGE];
    // trng::random_bytes(&board.trng, &mut send_buffer);
    // send_buffer[0..LEN_HIDE_CHAL_NONCE].copy_from_slice(&solved_nonce);
    // TODO: Is this fine????
    // send_buffer[LEN_HIDE_CHAL_NONCE..].copy_from_slice(&misc_message);
    hide_pkt_chal_resp.set_challenge_nonce(&solved_nonce);
    hide_pkt_chal_resp.set_misc_message(misc_message);
    // if ascon_send(board, is_master, comp_id, MAGIC_PKT_CHAL_RESP, &send_buffer) != 0 {
    //     return None;
    // }
    if ascon_send(board, is_master, i2c_addr, &mut hide_pkt_chal_resp) != 0 {
        return None;
    }
    return Some(misc_message.len());
}

/// Implements the HIDE protocol to receive a message
#[must_use]
pub fn secure_receive(
    board: &Board,
    is_master: bool,
    comp_id: &[u8; LEN_COMPONENT_ID],
    output: &mut [u8]
) -> Option<usize> {
    if output.len() > LEN_MISC_MESSAGE {
        return None;
    }
    let i2c_addr = comp_id[LEN_COMPONENT_ID - 1];
    // Step 1: Wait until we receive a PKT_REQ
    let mut packet_msg_req: [u8; LEN_HIDE_PKT_REQ] = [0; LEN_HIDE_PKT_REQ];
    while packet_msg_req[0] != MAGIC_PKT_REQ {
        if is_master {
            i2c1::master_read_bytes(&board.i2c1, i2c_addr, &mut packet_msg_req);
        } else {
            i2c1::slave_read_bytes(&board.i2c1, &mut packet_msg_req);
        }
    }
    // Step 2: Generate a random challenge nonce and send PKT_CHAL_SEND
    let mut hide_pkt_chal_send = Ascon128Data::new(board, comp_id, MAGIC_PKT_CHAL_SEND);
    let mut chal_nonce: [u8; LEN_HIDE_CHAL_NONCE] = [0u8; LEN_HIDE_CHAL_NONCE];
    trng::random_bytes(&board.trng, &mut chal_nonce);
    hide_pkt_chal_send.set_challenge_nonce(&chal_nonce);
    // Solve the nonce to check if it's correct later
    let mut solved_nonce: [u8; LEN_HIDE_CHAL_NONCE] = [0u8; LEN_HIDE_CHAL_NONCE];
    solve_hide_challenge(&mut solved_nonce, &chal_nonce);
    // let mut send_buffer: [u8; LEN_HIDE_MESSAGE] = [0u8; LEN_HIDE_MESSAGE];
    // trng::random_bytes(&board.trng, &mut send_buffer);
    // send_buffer[0..LEN_HIDE_CHAL_NONCE].copy_from_slice(&chal_nonce);
    // if ascon_send(board, is_master, comp_id, MAGIC_PKT_CHAL_SEND, &send_buffer) != 0 {
    //     return None;
    // };
    if ascon_send(board, is_master, i2c_addr, &mut hide_pkt_chal_send) != 0 {
        return None;
    }
    // Step 3: Wait until we receive PKT_CHAL_RESP, then check the nonce and extract the message
    // let mut recv_buffer: [u8; LEN_HIDE_MESSAGE] = [0u8; LEN_HIDE_MESSAGE];
    // if ascon_receive(board, is_master, comp_id, MAGIC_PKT_CHAL_RESP, &mut recv_buffer) != 0 {
    //     return None;
    // }
    let mut hide_pkt_chal_resp = Ascon128Data::new(board, comp_id, MAGIC_PKT_CHAL_RESP);
    if ascon_receive(board, is_master, i2c_addr, &mut hide_pkt_chal_resp) != 0 {
        return None;
    }
    let mut given_nonce: [u8; LEN_HIDE_CHAL_NONCE] = [0u8; LEN_HIDE_CHAL_NONCE];
    given_nonce.copy_from_slice(&hide_pkt_chal_resp.message[0..LEN_HIDE_CHAL_NONCE]);
    // SECURITY CRITICAL: Check if the nonce was solved correctly
    for i in 0..LEN_HIDE_CHAL_NONCE {
        if given_nonce[i] != solved_nonce[i] {
            return None;
        }
    }
    // Copy the message into the output
    for i in 0..output.len() {
        output[i] = hide_pkt_chal_resp.message[LEN_HIDE_CHAL_NONCE + i];
    }
    return Some(output.len());
}

/// Helper function to solve the challenge nonce
fn solve_hide_challenge(solved_nonce: &mut [u8], nonce: &[u8]) {
    for i in 0..LEN_HIDE_CHAL_NONCE {
        solved_nonce[i] = nonce[i] ^ 0x55;
    }
}

/// Helper function to compute associated data for Ascon
fn gen_associated_data(
    output: &mut [u8; LEN_ASCON_128_AD],
    comp_id: &[u8; LEN_COMPONENT_ID],
    pkt_magic: u8
) {
    output[0..LEN_COMPONENT_ID].copy_from_slice(comp_id);
    output[LEN_COMPONENT_ID] = pkt_magic;
}

/// Use Ascon to encrypt and send I2C message
#[must_use]
fn ascon_send(
    board: &Board,
    is_master: bool,
    i2c_addr: u8,
    // comp_id: &[u8; LEN_COMPONENT_ID],
    // pkt_magic: u8,
    // message: &[u8; LEN_HIDE_MESSAGE]
    ascon_data: &mut Ascon128Data
) -> i32 {
    // Set up send buffer and associated data
    // let mut enc_message: [u8; LEN_HIDE_MESSAGE] = [0u8; LEN_HIDE_MESSAGE];
    // let mut assoc_data: [u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    // compute_associated_data(&mut assoc_data, comp_id, pkt_magic);
    // Generate random nonce for Ascon
    // let mut ascon_nonce: [u8; LEN_ASCON_128_NONCE] = [0u8; LEN_ASCON_128_NONCE];
    // trng::random_bytes(&board.trng, &mut ascon_nonce);
    uart0::write_bytes(&board.uart0, b"Plaintext message: ");
    for byte in ascon_data.message.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    uart0::write_bytes(&board.uart0, b"Associated data (BEFORE): ");
    for byte in ascon_data.ad.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    // Encrypt message with Ascon
    let key = if is_master {
        &ASCON_SECRET_KEYS.ap_to_c
    } else {
        &ASCON_SECRET_KEYS.c_to_ap
    };
    let result = ascon_encrypt(
        &mut ascon_data.ciphertext,
        &ascon_data.message,
        &ascon_data.ad,
        &ascon_data.nonce,
        key
    );
    if result != 0 {
        return result;
    }
    // Send encrypted output
    let mut send_buffer: [u8; LEN_HIDE_PKT_MAX] = [0u8; LEN_HIDE_PKT_MAX];
    send_buffer[0..LEN_ASCON_128_NONCE].copy_from_slice(&ascon_data.nonce);
    send_buffer[LEN_ASCON_128_NONCE..LEN_HIDE_PKT_MAX].copy_from_slice(&ascon_data.ciphertext);
    // DEBUG
    uart0::write_bytes(&board.uart0, b"Attempting to encrypt PKT_CHAL_SEND...\r\n");
    uart0::write_bytes(&board.uart0, b"Sending buffer: ");
    for byte in send_buffer.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    uart0::write_bytes(&board.uart0, b"Parsed ascon nonce: ");
    for byte in ascon_data.nonce.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    uart0::write_bytes(&board.uart0, b"Encrypted message: ");
    for byte in ascon_data.ciphertext.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    uart0::write_bytes(&board.uart0, b"Associated data (AFTER): ");
    for byte in ascon_data.ad.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    // END DEBUG
    // MORE DEBUG
    uart0::write_bytes(&board.uart0, b"Key: ");
    for byte in key.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    // END MORE DEBUG
    if is_master {
        i2c1::master_write_bytes(&board.i2c1, i2c_addr, &send_buffer);
    } else {
        i2c1::slave_write_bytes(&board.i2c1, &send_buffer);
    }
    return 0;
}

/// Receive and decrypt I2C message using Ascon
#[must_use]
fn ascon_receive(
    board: &Board,
    is_master: bool,
    // comp_id: &[u8; LEN_COMPONENT_ID],
    i2c_addr: u8,
    // pkt_magic: u8,
    // message: &mut [u8; LEN_HIDE_MESSAGE]
    ascon_data: &mut Ascon128Data
) -> i32 {
    // Receive the message
    let mut recv_buffer = [0u8; LEN_HIDE_PKT_MAX];
    if is_master {
        i2c1::master_read_bytes(&board.i2c1, i2c_addr, &mut recv_buffer);
    } else {
        i2c1::slave_read_bytes(&board.i2c1, &mut recv_buffer);
    }
    ascon_data.nonce.copy_from_slice(&recv_buffer[0..LEN_ASCON_128_NONCE]);
    ascon_data.ciphertext.copy_from_slice(&recv_buffer[LEN_ASCON_128_NONCE..LEN_HIDE_PKT_MAX]);
    // enc_message.copy_from_slice(&recv_buffer[LEN_ASCON_128_NONCE..LEN_HIDE_PKT_MAX]);
    // Set up associated data
    // let mut assoc_data: [u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    // compute_associated_data(&mut assoc_data, comp_id, pkt_magic);
    // DEBUG: Print decryption values
    uart0::write_bytes(&board.uart0, b"Attempting to decrypt PKT_CHAL_SEND...\r\n");
    uart0::write_bytes(&board.uart0, b"Received buffer: ");
    for byte in recv_buffer.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    uart0::write_bytes(&board.uart0, b"\r\nParsed ascon nonce: ");
    for byte in ascon_data.nonce.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    uart0::write_bytes(&board.uart0, b"Encrypted message: ");
    for byte in ascon_data.ciphertext.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    uart0::write_bytes(&board.uart0, b"Associated data: ");
    for byte in ascon_data.ad.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    // END DEBUG
    // Decrypt message
    let key = if is_master {
        &ASCON_SECRET_KEYS.c_to_ap
    } else {
        &ASCON_SECRET_KEYS.ap_to_c
    };
    // MORE DEBUG
    uart0::write_bytes(&board.uart0, b"Key: ");
    for byte in key.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    // END MORE DEBUG
    let result = ascon_decrypt(
        &mut ascon_data.message,
        &ascon_data.ciphertext,
        &ascon_data.ad,
        &ascon_data.nonce,
        key
    );
    uart0::write_bytes(&board.uart0, b"Decryption result: ");
    uart0::write_bytes(&board.uart0, &u32_to_hex_string(result as u32));
    uart0::write_bytes(&board.uart0, b"\r\n");
    uart0::write_bytes(&board.uart0, b"Decrypted message: ");
    for byte in ascon_data.message.iter() {
        uart0::write_bytes(&board.uart0, &u8_to_hex_string(*byte));
    }
    uart0::write_bytes(&board.uart0, b"\r\n");
    return result;
}

/// Encrypts a message using Ascon-128
#[must_use]
pub fn ascon_encrypt(
    ciphertext: &mut [u8],
    message: &[u8],
    associated_data: &[u8; LEN_ASCON_128_AD],
    nonce: &[u8; LEN_ASCON_128_NONCE],
    key: &[u8],
) -> i32 {
    let mut _result: i32 = -1;
    let mut clen: u64 = 0;
    // Check lengths of provided buffers
    if message.len() != ciphertext.len() {
        return -1;
    }
    if key.len() != LEN_ASCON_128_KEY {
        return -1;
    }
    // Safety: We're calling the C impl, so raw pointers are required
    unsafe {
        _result = crypto_aead_encrypt(
            ciphertext.as_mut_ptr(),
            &mut clen,
            message.as_ptr(),
            message.len() as u64,
            associated_data.as_ptr(),
            LEN_ASCON_128_AD as u64,
            core::ptr::null(),
            nonce.as_ptr(),
            key.as_ptr(),
        )
    }
    _result
}

/// Decrypts a message using Ascon-128
#[must_use]
pub fn ascon_decrypt(
    message: &mut [u8],
    ciphertext: &[u8],
    associated_data: &[u8; LEN_ASCON_128_AD],
    nonce: &[u8; LEN_ASCON_128_NONCE],
    key: &[u8],
) -> i32 {
    let mut _result: i32 = -1;
    let mut mlen: u64 = 0;
    // Check lengths of provided buffers
    if message.len() != ciphertext.len() {
        return -1;
    }
    if key.len() != LEN_ASCON_128_KEY {
        return -1;
    }
    // Safety: We're calling the C impl, so raw pointers are required
    unsafe {
        _result = crypto_aead_decrypt(
            message.as_mut_ptr(),
            &mut mlen,
            core::ptr::null_mut(),
            ciphertext.as_ptr(),
            ciphertext.len() as u64,
            associated_data.as_ptr(),
            LEN_ASCON_128_AD as u64,
            nonce.as_ptr(),
            key.as_ptr(),
        )
    }
    _result
}