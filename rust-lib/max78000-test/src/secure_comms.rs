use ascon::{crypto_aead_encrypt, crypto_aead_decrypt};

pub const LEN_ASCON_128_KEY:    usize = 16;
pub const LEN_ASCON_128_TAG:    usize = 16;
pub const LEN_ASCON_128_NONCE:  usize = 16;
pub const LEN_ASCON_128_AD:     usize = 8;
pub const LEN_ASCON_128_PTXT:   usize = 64;
pub const LEN_ASCON_128_CTXT:   usize = LEN_ASCON_128_PTXT + LEN_ASCON_128_TAG;

// global variables 
let key: &[u8; LEN_ASCON_128_KEY] = [0; LEN_ASCON_128_KEY];
let MSG_REQ: &[u8; 1] = [40u8]; //magic byte
let ap_id: [u8; 4] = [0u8, 0u8, 0u8, 0u8]; // AP doesn't have a unique id

/// TODO: Secure send
// pub fn hide_secure_send(
//     message: &[u8],
//     associated_data: &[u8],
//     nonce: &[u8],
//     key: &[u8],
// ) -> [u8; LEN_ASCON_128_CTXT] {
//     let mut ciphertext = [0u8; LEN_ASCON_128_CTXT];
//     let result = ascon_encrypt(
//         &mut ciphertext,
//         message,
//         associated_data,
//         nonce,
//         key,
//     );
//     if result != 0 {
//         panic!("Failed to encrypt message");
//     }
//     ciphertext
// }


fn send(address: u8, message: &[u8], len: u8) {
    let mut buffer = [0u8; 112];
    for i in 0..len {
        buffer[i] = message[i];
    }
    send_c(address, &buffer);
}

fn send_c(address: u8, message: &[u8]) {
    // Send message to address
    // WILL BE WRAPPED IN RUST, CALLS C FUNCTION IMPLEMENTED ELSEWHERE
}

fn recv(address: u8) -> &[u8] {
    let mut msg_flag = false;
    while (!msg_flag) {
        if /* msg */ { msg_flag = true; }
    }
    return msg;
}

fn enc_send(address: u8, message: &[u8]) {
    let mut enc_message: &[u8; LEN_ASCON_128_CTXT] = [0u8; LEN_ASCON_128_CTXT];
    let comp_id: [u8; 4] = //look up component id from address
    let mut assoc_data: &[u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    assoc_data[..4].copy_from_slice(comp_id);
    let nonce: &[u8; LEN_ASCON_128_NONCE] = //generate random nonce
    ascon_encrypt(enc_message, message, assoc_data, nonce, key); //Apply Ascon to message
    enc_message = //add nonce to front of enc_message
    send(address, enc_message, enc_message.len());
}

fn enc_recv(address: u8) -> &[u8] {
    let rec = recv(address);
    let nonce = &rec[0..16]; //first 16 bytes of rec
    let enc_message = &rec[16..];
    let mut message: &[u8; LEN_ASCON_128_PTXT] = [0u8; LEN_ASCON_128_PTXT];
    let comp_id: [u8; 4] = //look up component id from address
    let mut assoc_data: &[u8; LEN_ASCON_128_AD] = [0u8; LEN_ASCON_128_AD];
    assoc_data[4..].copy_from_slice(comp_id);
    ascon_decrypt(message, enc_message, assoc_data, nonce, key); //Decrypt message
    return message; 
}

fn secure_send(address: u8, message: &[u8]) {
    // Send MSQ_REQ with Device A ID and Device B ID
    send(address, MSG_REQ, 1);
    // Wait until message received and decrypt message, which is the chal_nonce
    let mut chal_nonce = enc_recv(address);
    //Add 1 to chal_nonce (may want to handle this in its own function)
    if chal_nonce[15] < 255 {
        chal_nonce[15] += 1;
    } else {
        //handle carrying over
    }
    //Send the challenge response
    enc_send(address, chal_nonce);
}

fn secure_recv(address: u8) -> &[u8] {
    // Wait until message received and decrypt message
    let msg_req = recv(address);
    // If message received is MSG_REQ, send challenge nonce
    if msg_req == MSG_REQ {
        let chal_nonce: &[u8: 16] = /* randomly generate nonce */
    } else {
        //reset to default state
    }
    enc_send(address, chal_nonce);
    // Wait until message received and decrypt message, which is challenge response
    let chal_resp = enc_recv(address);
    // If chal_resp starts with the challenge nonce + 1, extract message
    // chal_nonce + 1 is incorrect syntax but it's the correct response
    if chal_resp[..16].contains(chal_nonce + 1) {
        // Extract message from chal_resp
        let message = chal_resp[16..];
    } else {
        //reset to default state
    }
    // idea is that there might be a gateway function that uses output of 
    //  secure_recv to match message to other function
    return message;
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