#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use max78000_pac as _;

#[entry]
fn main() -> ! {
    let mut int32: i32 = 0;
    loop {
        int32 += 1;
        continue;
    }
}
