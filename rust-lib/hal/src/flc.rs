pub use max78000_pac::FLC;

const FLASH_BASE: u32 = 0x1000_0000;
const FLASH_END: u32 = 0x1008_0000;

/// Check if the FLC is busy with a write or erase operation
pub fn is_busy(flc: &FLC) -> bool {
    flc.ctrl().read().pend().bit_is_set() ||
    flc.ctrl().read().pge().bit_is_set() ||
    flc.ctrl().read().me().bit_is_set() ||
    flc.ctrl().read().wr().bit_is_set()
}

/// Write a 128-bit word to flash
fn write_128(flc: &FLC, addr: u32, data: &[u32; 4]) -> i32 {
    // Check that the provided flash address is valid
    if addr < FLASH_BASE || addr > FLASH_END || addr & 0xF != 0 {
        return -1;
    }
    // Wait for the FLC to be ready
    while flc.ctrl().read().pend().bit_is_set() { }
    // FLC is already configured to 1MHz by default using the IPO
    // Unlock flash
    flc.ctrl().write(|w| w.unlock().unlocked());
    // Safety: FLC address is valid
    flc.addr().write(|w| unsafe {
        w.addr().bits(addr)
    });
    // Safety: FLC data is placed in the correct location
    flc.data0().write(|w| unsafe {
        w.data().bits(data[0])
    });
    flc.data1().write(|w| unsafe {
        w.data().bits(data[1])
    });
    flc.data2().write(|w| unsafe {
        w.data().bits(data[2])
    });
    flc.data3().write(|w| unsafe {
        w.data().bits(data[3])
    });
    // Commit the write
    flc.ctrl().write(|w| w.wr().set_bit());
    // Wait until the write is complete
    while is_busy(flc) { }
    // Lock flash
    flc.ctrl().write(|w| w.unlock().locked());
    // Check access violations
    if flc.intr().read().af().bit_is_set() {
        flc.intr().write(|w| w.af().clear_bit());
        return -2;
    }
    return 0;
}

/// Write a 32-bit word to flash via a 128-bit write
pub fn write_32(flc: &FLC, addr: u32, data: u32) -> i32 {
    let mut current_data: [u32; 4] = [0xFFFF_FFFFu32; 4];
    // Check if the provided flash address is valid
    if addr < FLASH_BASE || addr > FLASH_END || addr & 0x3 != 0 {
        return -1;
    }
    // Cast input address as a pointer
    let addr_32: *mut u32 = addr as *mut u32;
    // Check that the only bits that need to be flipped are 1 -> 0
    // Safety: FLC address is valid
    unsafe {
        if *addr_32 & data != data {
            return -2;
        }
    }
    // Determine index of the 32-bit word within the 128-bit word
    let addr_128: *mut u32 = (addr_32 as u32 & 0xFFFF_FFF0) as *mut u32;
    let idx: usize = ((addr >> 2) & 0x3).try_into().unwrap();
    // Construct the 128-bit word from current flash contents
    // Safety: FLC address is valid
    unsafe {
        current_data[0] = *addr_128;
        current_data[1] = *(addr_128.offset(1));
        current_data[2] = *(addr_128.offset(2));
        current_data[3] = *(addr_128.offset(3));
    }
    // Update the 32-bit word
    current_data[idx] = data;
    // Write the 128-bit word to flash
    return write_128(flc, addr_128 as u32, &current_data);
}

/// Erase a page of flash (8192 bytes)
pub fn page_erase(flc: &FLC, addr: u32) -> i32 {
    // Check if the provided flash address is valid
    if addr < FLASH_BASE || addr > FLASH_END || addr & 0x1FFF != 0 {
        return -1;
    }
    // Wait for the FLC to be ready
    while flc.ctrl().read().pend().bit_is_set() { }
    // FLC is already configured to 1MHz by default using the IPO
    // Unlock flash
    flc.ctrl().write(|w| w.unlock().unlocked());
    // Safety: FLC address is valid
    flc.addr().write(|w| unsafe {
        w.addr().bits(addr)
    });
    // Set FLC erase code
    flc.ctrl().write(|w| w.erase_code().erase_page());
    // Commit the erase
    flc.ctrl().write(|w| w.pge().set_bit());
    // Wait until the erase is complete
    while is_busy(flc) { }
    // Lock flash
    flc.ctrl().write(|w| w.unlock().locked());
    // Check access violations
    if flc.intr().read().af().bit_is_set() {
        flc.intr().write(|w| w.af().clear_bit());
        return -2;
    }
    return 0;
}