use ascon::{crypto_aead_encrypt, crypto_aead_decrypt};
use max78000_hal::i2c1;
use max78000_hal::trng;
use crate::Board;

use crate::ectf_global_secrets::{
    ASCON_SECRET_KEY_AP_TO_C,
    ASCON_SECRET_KEY_C_TO_AP,
};

pub const LEN_ASCON_128_KEY:    usize = 16;
pub const LEN_ASCON_128_TAG:    usize = 16;
pub const LEN_ASCON_128_NONCE:  usize = 16;
pub const LEN_ASCON_128_AD:     usize = 8;
// pub const LEN_ASCON_128_PTXT:   usize = 64;
// pub const LEN_ASCON_128_CTXT:   usize = LEN_ASCON_128_PTXT;


// Length of component ID
pub const LEN_COMPONENT_ID:     usize = 4;

/// Magic bytes for HIDE communication
pub const MAGIC_PKT_REQ:        u8 = 0x40;
pub const MAGIC_PKT_CHAL_SEND:  u8 = 0x41;
pub const MAGIC_PKT_CHAL_RESP:  u8 = 0x42;

/// Message component lengths
pub const LEN_HIDE_CHALLENGE_NONCE: usize = 16;
pub const LEN_APP_MESSAGE:      usize = 80;
pub const LEN_HIDE_MESSAGE:     usize = LEN_HIDE_CHALLENGE_NONCE + LEN_APP_MESSAGE;

// Message lengths
pub const LEN_HIDE_PKT_REQ:     usize = 1;
pub const LEN_HIDE_PKT_CHAL_SEND:   usize = LEN_ASCON_128_NONCE + LEN_HIDE_CHALLENGE_NONCE;
pub const LEN_HIDE_PKT_CHAL_RESP:   usize = LEN_ASCON_128_NONCE + LEN_HIDE_MESSAGE;
pub const LEN_HIDE_PKT_MAX:     usize = LEN_HIDE_PKT_CHAL_RESP;

/// For AP: Send a message to the component.
/// Everything in `app_message` will be sent.
/// Returns the number of bytes sent as an option (None if HIDE is invalidated 
/// in any way)
#[must_use]
pub fn ap_secure_send(
    board: &Board,
    comp_id: [u8; LEN_COMPONENT_ID],
    app_message: &[u8]
) -> Option<usize> {
    return secure_send(board, true, comp_id, app_message);
}

/// For Component: Send a message to the AP.
/// Everything in `app_message` will be sent.
#[must_use]
pub fn comp_secure_send(
    board: &Board,
    comp_id: [u8; LEN_COMPONENT_ID],
    app_message: &[u8]
) -> Option<usize> {
    return secure_send(board, false, comp_id, app_message);
}

/// For AP: Receive a message from the component.
/// `output` should be the length of the message that needs to be received.
#[must_use]
pub fn ap_secure_receive(
    board: &Board,
    comp_id: [u8; LEN_COMPONENT_ID],
    output: &mut [u8]
) -> Option<usize> {
    return secure_receive(board, true, comp_id, output);
}

/// For Component: Receive a message from the AP.
/// `output` should be the length of the message that needs to be received.
#[must_use]
pub fn comp_secure_receive(
    board: &Board,
    comp_id: [u8; LEN_COMPONENT_ID],
    output: &mut [u8]
) -> Option<usize> {
    return secure_receive(board, false, comp_id, output);
}

/// Implements the HIDE protocol to send a message
#[must_use]
pub fn secure_send(
    board: &Board,
    is_master: bool,
    comp_id: [u8; LEN_COMPONENT_ID],
    app_message: &[u8]
) -> Option<usize> {
    if app_message.len() > LEN_APP_MESSAGE {
        return None;
    }
    let addr = comp_id[LEN_COMPONENT_ID - 1];
    // Step 1: Send PKT_REQ
    let packet_msg_req: [u8; LEN_HIDE_PKT_REQ] = [MAGIC_PKT_REQ; LEN_HIDE_PKT_REQ];
    if is_master {
        i2c1::master_write_bytes(&board.i2c1, addr, &packet_msg_req);
    } else {
        i2c1::slave_write_bytes(&board.i2c1, &packet_msg_req);
    }
    // Step 2: Receive PKT_CHAL_SEND, extract the challenge nonce and solve the challenge
    let mut recv_buffer: [u8; LEN_HIDE_MESSAGE] = [0u8; LEN_HIDE_MESSAGE];
    if ascon_receive(board, is_master, comp_id, MAGIC_PKT_CHAL_SEND, &mut recv_buffer) != 0 {
        return None;
    }
    let mut challenge_nonce: [u8; LEN_HIDE_CHALLENGE_NONCE] = [0u8; LEN_HIDE_CHALLENGE_NONCE];
    challenge_nonce.copy_from_slice(&recv_buffer[0..LEN_HIDE_CHALLENGE_NONCE]);
    let mut solved_nonce: [u8; LEN_HIDE_CHALLENGE_NONCE] = [0u8; LEN_HIDE_CHALLENGE_NONCE];
    solve_nonce(&challenge_nonce, &mut solved_nonce);
    // Step 3: Send PKT_CHAL_RESP with our solve challenge and app message
    let mut send_buffer: [u8; LEN_HIDE_MESSAGE] = [0; LEN_HIDE_MESSAGE];
    trng::random_bytes(&board.trng, &mut send_buffer);
    send_buffer[0..LEN_HIDE_CHALLENGE_NONCE].copy_from_slice(&solved_nonce);
    // TODO: Is this fine????
    send_buffer[LEN_HIDE_CHALLENGE_NONCE..].copy_from_slice(&app_message);
    ascon_send(board, is_master, comp_id, MAGIC_PKT_CHAL_RESP, &send_buffer);
    return Some(app_message.len());
}

/// Implements the HIDE protocol to receive a message
#[must_use]
pub fn secure_receive(
    board: &Board,
    is_master: bool,
    comp_id: [u8; LEN_COMPONENT_ID],
    output: &mut [u8]
) -> Option<usize> {
    if output.len() > LEN_APP_MESSAGE {
        return None;
    }
    // Step 1: Wait until we receive a PKT_REQ
    let mut packet_msg_req: [u8; LEN_HIDE_PKT_REQ] = [0; LEN_HIDE_PKT_REQ];
    while packet_msg_req[0] != MAGIC_PKT_REQ {
        if is_master {
            i2c1::master_read_bytes(&board.i2c1, comp_id[LEN_COMPONENT_ID - 1], &mut packet_msg_req);
        } else {
            i2c1::slave_read_bytes(&board.i2c1, &mut packet_msg_req);
        }
    }
    // Step 2: Generate a random challenge nonce and send PKT_CHAL_SEND
    let mut chal_nonce: [u8; LEN_HIDE_CHALLENGE_NONCE] = [0u8; LEN_HIDE_CHALLENGE_NONCE];
    trng::random_bytes(&board.trng, &mut chal_nonce);
    let mut send_buffer: [u8; LEN_HIDE_MESSAGE] = [0u8; LEN_HIDE_MESSAGE];
    trng::random_bytes(&board.trng, &mut send_buffer);
    send_buffer[0..LEN_HIDE_CHALLENGE_NONCE].copy_from_slice(&chal_nonce);
    if ascon_send(board, is_master, comp_id, MAGIC_PKT_CHAL_SEND, &send_buffer) != 0 {
        return None;
    };
    // Step 3: Wait until we receive PKT_CHAL_RESP, then check the nonce and extract the message
    let mut recv_buffer: [u8; LEN_HIDE_MESSAGE] = [0u8; LEN_HIDE_MESSAGE];
    if ascon_receive(board, is_master, comp_id, MAGIC_PKT_CHAL_SEND, &mut recv_buffer) != 0 {
        return None;
    }
    // Check if the nonce was solved correctly
    let mut solved_nonce: [u8; LEN_HIDE_CHALLENGE_NONCE] = [0u8; LEN_HIDE_CHALLENGE_NONCE];
    solve_nonce(&chal_nonce, &mut solved_nonce);
    // SECURITY CRITICAL: Check if the nonce was solved correctly
    for i in 0..LEN_HIDE_CHALLENGE_NONCE {
        if recv_buffer[i] != solved_nonce[i] {
            return None;
        }
    }
    // Copy the message into the output
    for i in 0..output.len() {
        output[i] = recv_buffer[LEN_HIDE_CHALLENGE_NONCE + i];
    }
    return Some(output.len());
}

/// Helper function to solve the challenge nonce
fn solve_nonce(
    nonce: &[u8; LEN_HIDE_CHALLENGE_NONCE],
    solved_nonce: &mut [u8; LEN_HIDE_CHALLENGE_NONCE]
) {
    for i in 0..LEN_HIDE_CHALLENGE_NONCE {
        solved_nonce[i] = nonce[i] ^ 0x55;
    }
}

/// Helper function to compute associated data for Ascon
fn compute_associated_data(
    output: &mut [u8; LEN_ASCON_128_AD],
    comp_id: [u8; LEN_COMPONENT_ID],
    pkt_magic: u8
) {
    output[0..LEN_COMPONENT_ID].copy_from_slice(&comp_id);
    output[LEN_COMPONENT_ID] = pkt_magic;
}

/// Use Ascon to encrypt and send I2C message
#[must_use]
fn ascon_send(
    board: &Board,
    is_master: bool,
    comp_id: [u8; LEN_COMPONENT_ID],
    pkt_magic: u8,
    message: &[u8; LEN_HIDE_MESSAGE]
) -> i32 {
    // Set up send buffer and associated data
    let mut enc_message: [u8; LEN_HIDE_MESSAGE] = [0u8; LEN_HIDE_MESSAGE];
    let mut assoc_data: [u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    compute_associated_data(&mut assoc_data, comp_id, pkt_magic);
    // Generate random nonce for Ascon
    let mut ascon_nonce: [u8; LEN_ASCON_128_NONCE] = [0u8; LEN_ASCON_128_NONCE];
    trng::random_bytes(&board.trng, &mut ascon_nonce);
    // Encrypt message with Ascon
    let mut result: i32 = -1;
    result = ascon_encrypt(&mut enc_message, message, &assoc_data, &ascon_nonce, ASCON_SECRET_KEY_AP_TO_C);
    if result != 0 {
        return result;
    }
    // Send encrypted output
    let mut send_buffer: [u8; LEN_HIDE_PKT_MAX] = [0u8; LEN_HIDE_PKT_MAX];
    send_buffer[0..LEN_ASCON_128_NONCE].copy_from_slice(&ascon_nonce);
    send_buffer[LEN_ASCON_128_NONCE..LEN_HIDE_PKT_MAX].copy_from_slice(&enc_message);
    if is_master {
        i2c1::master_write_bytes(&board.i2c1, comp_id[LEN_COMPONENT_ID - 1], &send_buffer);
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
    comp_id: [u8; LEN_COMPONENT_ID],
    pkt_magic: u8,
    message: &mut [u8; LEN_HIDE_MESSAGE]
) -> i32 {
    // Receive the message
    let mut recv_buffer = [0u8; LEN_HIDE_PKT_MAX];
    if is_master {
        i2c1::master_read_bytes(&board.i2c1, comp_id[LEN_COMPONENT_ID - 1], &mut recv_buffer);
    } else {
        i2c1::slave_read_bytes(&board.i2c1, &mut recv_buffer);
    }
    let mut ascon_nonce: [u8; LEN_ASCON_128_NONCE] = [0u8; LEN_ASCON_128_NONCE];
    ascon_nonce.copy_from_slice(&recv_buffer[0..LEN_ASCON_128_NONCE]);
    let mut enc_message: [u8; LEN_HIDE_MESSAGE] = [0u8; LEN_HIDE_MESSAGE];
    enc_message.copy_from_slice(&recv_buffer[LEN_ASCON_128_NONCE..LEN_HIDE_PKT_MAX]);
    // Set up associated data
    let mut assoc_data: [u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    compute_associated_data(&mut assoc_data, comp_id, pkt_magic);
    // Decrypt message
    return ascon_decrypt(message, &enc_message, &assoc_data, &ascon_nonce, ASCON_SECRET_KEY_C_TO_AP);
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