#![no_std]
#![no_main]

fn secure_send(id: u32, message: &[u8; 64]) {
    // TODO
}

fn secure_recv(id: u32, message: &mut [u8; 64]) {
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