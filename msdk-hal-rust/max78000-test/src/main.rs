#![no_std]
#![no_main]

use panic_halt as _;
// use panic_semihosting as _;
use cortex_m_rt::entry;
use max78000_pac as _;

// pub union Vector {
//     handler: unsafe extern "C" fn(),
//     reserved: usize,
// }

#[entry]
fn main() -> ! {
    let mut int32: i32 = 0;
    loop {
        int32 += 1;
        continue;
    }
}

// #[link_section = ".vector_table.reset_vector"]
// #[no_mangle]
// static RESET_VECTOR: unsafe extern "C" fn() -> ! = main;

// #[allow(unsafe_code)]
// #[link_section = ".vector_table.exceptions"]
// #[no_mangle]
// static __EXCEPTIONS: [Option<unsafe extern "C" fn()>; 14] = [None; 14];

// #[allow(unsafe_code)]
// #[link_section = ".vector_table.interrupts"]
// #[no_mangle]
// static __INTERRUPTS: [Option<unsafe extern "C" fn()>; 104] = [None; 104];