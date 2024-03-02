#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max78000_hal::tmr0;
use board::{Board, Led, u8_to_hex_string, u32_to_hex_string};
use board::secure_comms as hide;

use argon2::{
    password_hash::PasswordHash,
    Argon2
};

mod ectf_global_secrets;
use ectf_global_secrets::{
    ASCON_SECRET_KEY_AP_TO_C,
    ASCON_SECRET_KEY_C_TO_AP,
};

mod ectf_params;
use ectf_params::{
    AP_PIN_HASH,
    AP_TOKEN_HASH,
    AP_BOOT_MSG,
    COMPONENT_CNT,
    // ORIGINAL_COMPONENT_IDS, // DO NOT USE THESE!
};

mod post_boot;
use post_boot::post_boot;

mod tests;
use tests::{
    test_uart,
    test_ascon,
    test_random,
    test_flash,
    test_timer,
};

#[entry]
fn main() -> ! {
    let board = Board::new();
    board.send_host_debug(b"Board initialized!");

    test_ascon(&board);
    test_random(&board);
    // test_flash(&board);
    test_timer(&board);

    let mut count: i32 = 0;
    for _ in 0..20 {
        let tick_count = tmr0::get_tick_count(&board.tmr0);
        while tmr0::get_tick_count(&board.tmr0) < tick_count + 50_000_000 { }
        if (count % 2) == 0 {
            board.led_on(Led::Green);
        } else {
            board.led_off(Led::Green);
        }
        board.send_host_debug(b"Hello, world!");
        count += 1;
    }

    loop {
        test_uart(&board);
        // Safety: This function is defined in our C code
        // Unsafety: DO NOT DO THIS IN FINAL DESIGN! DO BOOT VERIFICATION FIRST!
        unsafe { post_boot() };
        continue;
    }
}

use max78000_pac as pac;
use max78000_hal as hal;

#[no_mangle]
pub extern "C" fn _read(fd: i32, buf: *mut u8, count: usize) -> isize {
    if fd == 0 {
        let uart0 = unsafe { pac::UART::steal() };
        let mut num_read = 0;
        for i in 0..count {
            let byte = hal::uart0::read_byte(&uart0);
            unsafe { buf.add(i).write(byte) };
            hal::uart0::write_byte(&uart0, byte);
            num_read += 1;
            // if byte == '\r' as u8 {
            //     hal::uart0::write_byte(&uart0, '\n' as u8);
            //     break;
            // }
        }
        return num_read as isize;
    }
    return -1;
}

#[no_mangle]
pub extern "C" fn _write(fd: i32, buf: *const u8, count: usize) -> isize {
    if fd == 1 || fd == 2 {
        let uart0 = unsafe { pac::UART::steal() };
        for i in 0..count {
            let byte = unsafe { buf.add(i).read() };
            // if byte == b'\n' as u8 {
            //     hal::uart0::write_byte(&uart0, b'\r');
            // }
            hal::uart0::write_byte(&uart0, byte);
        }
    }
    return -1;
}

#[no_mangle]
pub extern "C" fn _exit() {
    loop {}
}

#[no_mangle]
pub extern "C" fn _open() -> i32 {
    return -1;
}

#[no_mangle]
pub extern "C" fn _close() -> i32 {
    return -1;
}

#[no_mangle]
pub extern "C" fn _isatty() -> i32 {
    return -1;
}

#[no_mangle]
pub extern "C" fn _lseek() -> i32 {
    return -1;
}

#[no_mangle]
pub extern "C" fn _fstat() -> i32 {
    return -1;
}

#[no_mangle]
pub extern "C" fn _sbrk() -> i32 {
    return -1;
}

#[no_mangle]
pub extern "C" fn _kill() -> i32 {
    return -1;
}

#[no_mangle]
pub extern "C" fn _getpid() -> i32 {
    return -1;
}