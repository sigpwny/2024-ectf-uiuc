use max78000_pac as pac;
use max78000_hal as hal;

// Use post_boot() function from C code
extern "C" {
    pub fn post_boot() -> !;
}

// TODO: Provide functions that the POST_BOOT code can call


/// Read handler for POST_BOOT libc
#[no_mangle]
pub extern "C" fn _read(fd: i32, buf: *mut u8, count: usize) -> isize {
    if fd == 0 {
        let uart0 = unsafe { pac::UART::steal() };
        let mut num_read = 0;
        for i in 0..count {
            let byte = hal::uart0::read_byte(&uart0);
            hal::uart0::write_byte(&uart0, byte);
            num_read += 1;
            if byte == '\r' as u8 {
                unsafe { buf.add(i).write(b'\n') };
                break;
            }
            unsafe { buf.add(i).write(byte) };
        }
        return num_read as isize;
    }
    return -1;
}
/// Write handler for POST_BOOT libc
#[no_mangle]
pub extern "C" fn _write(fd: i32, buf: *const u8, count: usize) -> isize {
    if fd == 1 || fd == 2 {
        let uart0 = unsafe { pac::UART::steal() };
        for i in 0..count {
            let byte = unsafe { buf.add(i).read() };
            if byte == b'\n' as u8 {
                hal::uart0::write_byte(&uart0, b'\r');
            }
            hal::uart0::write_byte(&uart0, byte);
        }
        return count as isize;
    }
    return -1;
}
/// The following functions are unused by the POST_BOOT code, but are 
/// required by the linker, so we provide stubs for them here.
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