use ascon::{crypto_aead_encrypt, crypto_aead_decrypt};
use max78000_hal::i2c1;
use max78000_hal::trng;

mod ectf_global_secrets;
use ectf_global_secrets::{
    ASCON_SECRET_KEY_AP_TO_C,
    ASCON_SECRET_KEY_C_TO_AP,
};

pub const LEN_ASCON_128_KEY:    usize = 16;
pub const LEN_ASCON_128_TAG:    usize = 16;
pub const LEN_ASCON_128_NONCE:  usize = 16;
pub const LEN_ASCON_128_AD:     usize = 8;
pub const LEN_ASCON_128_PTXT:   usize = 64;
pub const LEN_ASCON_128_CTXT:   usize = LEN_ASCON_128_PTXT + LEN_ASCON_128_TAG;

// global variables 
let key: &[u8; LEN_ASCON_128_KEY] = [0; LEN_ASCON_128_KEY];
pub const MSG_REQ: &[u8; 1] = [40u8]; //magic byte
pub const ap_id: [u8; 4] = [0u8, 0u8, 0u8, 0u8]; // AP doesn't have a unique id


fn increment_nonce(nonce: &[u8]) {
    u8 index = LEN_ASCON_128_NONCE - 1;
    while nonce[index] == 255 && index >= 0 {
        nonce[index] = 0;
        index -= 1;
    }
    if index == -1 {
        nonce = [0; LEN_ASCON_128_NONCE];
    } else {
        nonce[index] += 1;
    }
}

// APPLICATION PROCESSOR FUNCTIONS
fn m_send(address: u8, message: &[u8], len: u8) {
    let mut buffer = [0u8; 112];
    if len > 112 { len = 112; }
    for i in 0..len {
        buffer[i] = message[i];
    }
    i2c1::master_write_bytes(i2c1, address, &buffer);
}

fn m_recv(address: u8) -> &[u8] {
    let mut msg_flag = false;
    let base: &[u8; LEN_ASCON_128_CTXT] = [0u8; LEN_ASCON_128_CTXT];
    let mut msg: &[u8; LEN_ASCON_128_CTXT] = base;
    while (!msg_flag) {
        i2c1::master_read_bytes(i2c1, address, &mut msg);
        if msg != base { msg_flag = true; }
    }
    return msg;
}

fn m_enc_send(comp_id: [u8; 4], message: &[u8], board: &Board) {
    let mut enc_message: &[u8; LEN_ASCON_128_CTXT] = [0u8; LEN_ASCON_128_CTXT];
    let mut assoc_data: &[u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    assoc_data[..LEN_ASCON_128_AD/2].copy_from_slice(comp_id);
    let nonce: &[u8; LEN_ASCON_128_NONCE] = trng::random_bytes(&board.trng, &mut nonce);
    ascon_encrypt(enc_message, message, assoc_data, nonce, key); //Apply Ascon to message
    let mut n_enc_message = &[u8; LEN_ASCON_128_CTXT] = [0u8; LEN_ASCON_128_CTXT];
    int i = 0;
    for byte in enc_message {
        n_enc_message[i + LEN_ASCON_128_NONCE] = byte;
        i += 1;
    }
    n_enc_message[..LEN_ASCON_128_NONCE].copy_from_slice(nonce);
    m_send(comp_id[3], n_enc_message, n_enc_message.len());
}

fn m_enc_recv(comp_id: [u8; 4]) -> &[u8] {
    let rec = m_recv(comp_id[3]);
    let nonce = &rec[0..LEN_ASCON_128_NONCE]; //first 16 bytes of rec
    let enc_message = &rec[LEN_ASCON_128_NONCE..];
    let mut message: &[u8; LEN_ASCON_128_PTXT] = [0u8; LEN_ASCON_128_PTXT];
    let mut assoc_data: &[u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    assoc_data[LEN_ASCON_128_AD/2..].copy_from_slice(comp_id);
    ascon_decrypt(message, enc_message, assoc_data, nonce, key); //Decrypt message
    return message; 
}

/// For AP: Send a message to the component.
/// Everything in `message` will be sent.
pub fn ap_secure_send(comp_id: [u8; 4], message: &[u8], board: &Board) {
    // Send MSQ_REQ 
    m_send(comp_id[3], MSG_REQ, 1);
    // Wait until message received and decrypt message, which is the chal_nonce
    let mut chal_nonce = m_enc_recv(comp_id[3]);
    //Add 1 to chal_nonce (may want to handle this in its own function)
    increment_nonce(chal_nonce);
    //Send the challenge response
    m_enc_send(comp_id[3], chal_nonce, board);
}

/// For AP: Receive a message from the component.
/// `output` should be the length of the message that needs to be received.
pub fn ap_secure_receive(comp_id: [u8; 4], output: &mut [u8], board: &Board) {
    // Wait until message received and decrypt message
    let msg_req = m_recv(comp_id[3]);
    // If message received is MSG_REQ, send challenge nonce
    if msg_req == MSG_REQ {
        let chal_nonce: &[u8: 16] = trng::random_bytes(&board.trng, &mut nonce);
    } else {
        *output = b"";
        return;
    }
    m_enc_send(comp_id[3], chal_nonce, board);
    // Wait until message received and decrypt message, which is challenge response
    let chal_resp = m_enc_recv(comp_id[3]);
    // If chal_resp starts with the challenge nonce + 1, extract message
    if chal_resp[..LEN_ASCON_128_NONCE].contains(increment_nonce(chal_nonce)) {
        // Extract message from chal_resp
        let message = chal_resp[LEN_ASCON_128_NONCE..];
    } else {
        *output = b"";
        return;
    }
    // idea is that there might be a gateway function that uses output of 
    //  secure_recv to match message to other function
    *output = message;
    return;
}

//COMPONENT FUNCTIONS
fn s_send(message: &[u8], len: u8) {
    let mut buffer = [0u8; 112];
    if len > 112 { len = 112; }
    for i in 0..len {
        buffer[i] = message[i];
    }
    i2c1::slave_write_bytes(i2c1, &buffer);
}

fn s_recv() -> &[u8] {
    let mut msg_flag = false;
    let base: &[u8; LEN_ASCON_128_CTXT] = [0u8; LEN_ASCON_128_CTXT];
    let mut msg: &[u8; LEN_ASCON_128_CTXT] = base;
    while (!msg_flag) {
        i2c1::slave_read_bytes(i2c1, &mut msg);
        if msg != base { msg_flag = true; }
    }
    return msg;
}

fn s_enc_send(message: &[u8], board: &Board) {
    let mut enc_message: &[u8; LEN_ASCON_128_CTXT] = [0u8; LEN_ASCON_128_CTXT];
    let mut assoc_data: &[u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    assoc_data[LEN_ASCON_128_AD/2..].copy_from_slice(COMPONENT_ID);
    let nonce: &[u8; LEN_ASCON_128_NONCE] = trng::random_bytes(&board.trng, &mut nonce);
    ascon_encrypt(enc_message, message, assoc_data, nonce, key); //Apply Ascon to message
    let mut n_enc_message = &[u8; LEN_ASCON_128_CTXT] = [0u8; LEN_ASCON_128_CTXT];
    int i = 0;
    for byte in enc_message {
        n_enc_message[i + LEN_ASCON_128_NONCE] = byte;
        i += 1;
    }
    n_enc_message[..LEN_ASCON_128_NONCE].copy_from_slice(nonce);
    s_send(n_enc_message, n_enc_message.len());
}

fn s_enc_recv() -> &[u8] {
    let rec = s_recv(comp_id[3]);
    let nonce = &rec[0..LEN_ASCON_128_NONCE]; //first 16 bytes of rec
    let enc_message = &rec[LEN_ASCON_128_NONCE..];
    let mut message: &[u8; LEN_ASCON_128_PTXT] = [0u8; LEN_ASCON_128_PTXT];
    let mut assoc_data: &[u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    assoc_data[..LEN_ASCON_128_AD/2].copy_from_slice(COMPONENT_ID);
    ascon_decrypt(message, enc_message, assoc_data, nonce, key); //Decrypt message
    return message; 
}

/// For Component: Send a message to the AP.
/// Everything in `message` will be sent.
pub fn comp_secure_send(message: &[u8], board: &Board) {
    // Send MSQ_REQ 
    s_send(MSG_REQ, 1);
    // Wait until message received and decrypt message, which is the chal_nonce
    let mut chal_nonce = s_enc_recv();
    //Add 1 to chal_nonce (may want to handle this in its own function)
    increment_nonce(chal_nonce);
    //Send the challenge response
    s_enc_send(chal_nonce, board);
}

/// For Component: Receive a message from the AP.
/// `output` should be the length of the message that needs to be received.
pub fn comp_secure_receive(output: &mut [u8], board: &Board) {
    // Wait until message received and decrypt message
    let msg_req = s_recv();
    // If message received is MSG_REQ, send challenge nonce
    if msg_req == MSG_REQ {
        let chal_nonce: &[u8: 16] = trng::random_bytes(&board.trng, &mut nonce);
    } else {
        *output = b"";
        return;
    }
    s_enc_send(chal_nonce, board);
    // Wait until message received and decrypt message, which is challenge response
    let chal_resp = s_enc_recv();
    // If chal_resp starts with the challenge nonce + 1, extract message
    if chal_resp[..LEN_ASCON_128_NONCE].contains(increment_nonce(chal_nonce)) {
        // Extract message from chal_resp
        let message = chal_resp[LEN_ASCON_128_NONCE..];
    } else {
        *output = b"";
        return;
    }
    // idea is that there might be a gateway function that uses output of 
    //  secure_recv to match message to other function
    *output = message;
    return;
}


/// Encrypts a message using Ascon-128
pub fn ascon_encrypt(
    ciphertext: &mut [u8; LEN_ASCON_128_CTXT],
    message: &[u8; LEN_ASCON_128_PTXT],
    associated_data: &[u8; LEN_ASCON_128_AD],
    nonce: &[u8; LEN_ASCON_128_NONCE],
    key: &[u8; LEN_ASCON_128_KEY],
) -> i32 {
    let mut _result: i32 = -1;
    let mut clen: u64 = 0;
    // Safety: We're calling the C impl, so raw pointers are required
    unsafe {
        _result = crypto_aead_encrypt(
            ciphertext.as_mut_ptr(),
            &mut clen,
            message.as_ptr(),
            LEN_ASCON_128_PTXT as u64,
            associated_data.as_ptr(),
            LEN_ASCON_128_AD as u64,
            core::ptr::null(),
            nonce.as_ptr(),
            key.as_ptr(),
        )
    }
    if clen != LEN_ASCON_128_CTXT as u64 {
        panic!("clen != LEN_ASCON_128_CTXT");
    }
    _result
}

/// Decrypts a message using Ascon-128
pub fn ascon_decrypt(
    message: &mut [u8; LEN_ASCON_128_PTXT],
    ciphertext: &[u8; LEN_ASCON_128_CTXT],
    associated_data: &[u8; LEN_ASCON_128_AD],
    nonce: &[u8; LEN_ASCON_128_NONCE],
    key: &[u8; LEN_ASCON_128_KEY],
) -> i32 {
    let mut _result: i32 = -1;
    let mut mlen: u64 = 0;
    // Safety: We're calling the C impl, so raw pointers are required
    unsafe {
        _result = crypto_aead_decrypt(
            message.as_mut_ptr(),
            &mut mlen,
            core::ptr::null_mut(),
            ciphertext.as_ptr(),
            LEN_ASCON_128_CTXT as u64,
            associated_data.as_ptr(),
            LEN_ASCON_128_AD as u64,
            nonce.as_ptr(),
            key.as_ptr(),
        )
    }
    if mlen != LEN_ASCON_128_PTXT as u64 {
        panic!("mlen != LEN_ASCON_128_PTXT");
    }
    _result
}