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

fn enc_send(address: u8, u32 id, message: &[u8; 64]) {
    let enc_message = encrypt(message); //Apply Ascon to message
    send(address, enc_message);
}

fn enc_recv(address: u8) -> [u8; 64] {
    let enc_message = recv(address);
    return decrypt(enc_message); //Decrypt message
}

fn secure_recv(u32 id, message: &mut [u8; 64]) {
    // TODO
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