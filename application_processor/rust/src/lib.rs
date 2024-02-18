#![no_std]
#![no_main]

let MSG_REQ = 0001; //magic byte
let CHAL_SEND = 0010; //magic byte
let CHAL_RESP = 0011; //magic byte
let device_id = get_id(); //assuming a device can access its own id

fn send(address: u8, message: &[u8; 64]) {
    // Send message to address
}

fn recv(address: u8) -> [u8; 64] {
    let mut msg_flag = false;
    while (!msg_flag) {
        if /* msg */ { msg_flag = true; }
    }
    return msg;
}

fn enc_send(address: u8, message: &[u8; 64]) {
    let enc_message = encrypt(message); //Apply Ascon to message
    send(address, enc_message);
}

fn enc_recv(address: u8) -> [u8; 64] {
    let enc_message = recv(address);
    return decrypt(enc_message); //Decrypt message
}

fn secure_send(address: u8, message: &[u8; 64]) {
    // Send MSQ_REQ with Device A ID and Device B ID
    enc_send(address, MSG_REQ + device_id + address);
    // Wait until message received and decrypt message
    let chal_send = enc_recv(address);
    // If message received starts with CHAL_SEND and Device A ID and Device B ID, 
    //  send CHAL_RESP with Device A ID and Device B ID and nonce + 1 and message
    if chal_send.contains(CHAL_SEND + address + device_id) {
        // idea is to extract nonce from CHAL_SEND if the message starts appropriately
        let nonce = &chal_send[chal_send.find(device_id) + device_id.len()..];
    }
    enc_send(address, CHAL_RESP + device_id + address + (nonce + 1) + message);
}

fn secure_recv(address: u8) -> [u8; 64] {
    // Wait until message received and decrypt message
    let msg_req = enc_recv(address);
    // If message received starts with MSG_REQ and Device A ID and Device B ID,
    //   send CHAL_SEND and Device A ID and Device B ID and nonce
    if msg_req.contains(MSG_REQ + address + device_id) {
        let nonce = /* randomly generate nonce */
    }
    enc_send(address, CHAL_SEND + device_id + address + nonce);
    // Wait until message received and decrypt message
    let chal_resp = enc_recv(address);
    // If message received starts with CHAL_RESP and Device A ID and Device B ID and nonce + 1, 
    //  extract message
    if chal_resp.contains(CHAL_RESP + address + device_id + (nonce + 1)) {
        // Extract message from chal_resp
        let message = &chal_resp[chal_send.find(nonce + 1) + (nonce + 1).len()..];
    }
    // idea is that there might be a gateway function that uses output of 
    //  secure_recv to match message to other function
    return message;
}

// Delays the MCU by the number of microseconds provided (blocking)
fn delay(us: u32) {
    // TODO
}

// Do not use this function in main.rs! Instead, use the formatted message functions
fn host_uart_write(message: &[u8; 128]) {
    // TODO
}

fn send_host_info(message: &[u8; 64]) {
    // TODO
}

fn send_host_ack(message: &[u8; 64]) {
    // TODO
}

fn send_host_success(message: &[u8; 64]) {
    // TODO
}

fn send_host_error(message: &[u8; 64]) {
    // TODO
}

fn send_host_debug(message: &[u8; 64]) {
    // TODO
}

fn recv_host(message: &mut [u8; 32]) {
    // TODO
}