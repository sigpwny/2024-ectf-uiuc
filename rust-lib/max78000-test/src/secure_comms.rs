use ascon::{crypto_aead_encrypt, crypto_aead_decrypt};

pub const LEN_ASCON_128_KEY:    usize = 16;
pub const LEN_ASCON_128_TAG:    usize = 16;
pub const LEN_ASCON_128_NONCE:  usize = 16;
pub const LEN_ASCON_128_AD:     usize = 8;
pub const LEN_ASCON_128_PTXT:   usize = 64;
pub const LEN_ASCON_128_CTXT:   usize = LEN_ASCON_128_PTXT + LEN_ASCON_128_TAG;

// global variables 
let key: &[u8; LEN_ASCON_128_KEY] = [0; LEN_ASCON_128_KEY];
let MSG_REQ: u8 = 40u8; //magic byte
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

fn recv(address: u8) -> [u8] {
    let mut msg_flag = false;
    while (!msg_flag) {
        if /* msg */ { msg_flag = true; }
    }
    return msg;
}

fn enc_send(address: u8, id: u32, message: &[u8]) {
    let mut enc_message: [u8; LEN_ASCON_128_CTXT] = [0u8; LEN_ASCON_128_CTXT];
    let comp_id: [u8; 4] = //look up component id from address
    let assoc_data: [u8; 8] = //combine component and AP id
    let nonce: &[u8; LEN_ASCON_128_NONCE] = //generate random nonce
    ascon_encrypt(enc_message, message, assoc_data, nonce, key); //Apply Ascon to message
    enc_message = //add nonce to front of enc_message
    send(address, enc_message);
}

fn enc_recv(address: u8) -> [u8; 64] {
    let rec = recv(address);
    let nonce = &rec[0..16]; //first 16 bytes of rec
    let enc_message = &rec[16..];
    let mut message: [u8; LEN_ASCON_128_PTXT] = [0u8; LEN_ASCON_128_PTXT];
    let comp_id: [u8; 4] = //look up component id from address
    let assoc_data: [u8; 8] = //combine component and AP id
    ascon_decrypt(message, enc_message, assoc_data, nonce, key); //Decrypt message
    return message; 
}

fn secure_send(address: u8, message: &[u8; 64]) {
    // Send MSQ_REQ with Device A ID and Device B ID
    enc_send(address, MSG_REQ + device_id + address);
    // Wait until message received and decrypt message
    let chal_send = enc_recv(address);
    // If message received starts with CHAL_SEND and Device A ID and Device B ID, 
    //  send CHAL_RESP with Device A ID and Device B ID and nonce + 1 and message
    if chal_send.contains(address + device_id) {
        // idea is to extract nonce from CHAL_SEND if the message starts appropriately
        let nonce = &chal_send[chal_send.find(device_id) + device_id.len()..];
    }
    enc_send(address, device_id + address + (chal_nonce + 1) + message);
}

fn secure_recv(address: u8) -> [u8; 64] {
    // Wait until message received and decrypt message
    let msg_req = enc_recv(address);
    // If message received starts with MSG_REQ and Device A ID and Device B ID,
    //   send CHAL_SEND and Device A ID and Device B ID and nonce
    if msg_req.contains(MSG_REQ + address + device_id) {
        let chal_nonce = /* randomly generate nonce */
    }
    enc_send(address, device_id + address + nonce);
    // Wait until message received and decrypt message
    let chal_resp = enc_recv(address);
    // If message received starts with CHAL_RESP and Device A ID and Device B ID and nonce + 1, 
    //  extract message
    if chal_resp.contains(address + device_id + (chal_nonce + 1)) {
        // Extract message from chal_resp
        let message = &chal_resp[chal_send.find(chal_nonce + 1) + (chal_nonce + 1).len()..];
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