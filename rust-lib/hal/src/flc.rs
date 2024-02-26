pub use max78000_pac::FLC;

pub const FLASH_BASE: u32 = 0x1000_0000;
pub const FLASH_SIZE: u32 = 0x0008_0000;
pub const FLASH_END: u32 = FLASH_BASE + FLASH_SIZE;

pub enum FlashStatus {
    Success = 0,
    InvalidAddress = -1,
    AccessViolation = -2,
    NeedsErase = -3,
}

/// Check if the FLC is busy with a write or erase operation
pub fn is_busy(flc: &FLC) -> bool {
    flc.ctrl().read().pge().bit_is_set() ||
    flc.ctrl().read().me().bit_is_set() ||
    flc.ctrl().read().wr().bit_is_set()
}

/// Configure the FLC peripheral
pub fn config(flc: &FLC) {
    // Wait for the FLC to be ready
    while flc.ctrl().read().pend().bit_is_set() { }
    while is_busy(flc) { }
    // Set the FLC clock divisor to 100 (0x64) assuming a 100MHz system clock
    flc.clkdiv().modify(|_, w| unsafe {
        w.clkdiv().bits(0x64)
    });
    // Clear stale interrupt flags
    if flc.intr().read().af().bit_is_set() {
        flc.intr().modify(|_, w| w.af().clear_bit());
    }
}

/// Get the physical address of a flash address
fn get_phys_addr(addr: u32) -> u32 {
    return addr & (FLASH_SIZE - 1) | 0x8000_0000;
}

/// Write a 128-bit word to flash
/// Safety: Caller must check that the flash contents can be writable
unsafe fn write_128(flc: &FLC, addr: u32, data: &[u32; 4]) -> FlashStatus {
    // Check that the provided flash address is valid
    if addr < FLASH_BASE || addr > FLASH_END || addr & 0xF != 0 {
        return FlashStatus::InvalidAddress;
    }
    // Ensure FLC is configured
    config(flc);
    // Unlock flash
    flc.ctrl().modify(|_, w| w.unlock().unlocked());
    // Get the physical address of the 128-bit word
    let phys_addr = get_phys_addr(addr);
    // Safety: FLC address is valid
    flc.addr().write(|w| unsafe {
        w.addr().bits(phys_addr)
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
    flc.ctrl().modify(|_, w| w.wr().set_bit());
    // Wait until the write is complete
    while flc.ctrl().read().pend().bit_is_set() { }
    while is_busy(flc) { }
    // Lock flash
    flc.ctrl().modify(|_, w| w.unlock().locked());
    // Check access violations
    if flc.intr().read().af().bit_is_set() {
        flc.intr().modify(|_, w| w.af().clear_bit());
        return FlashStatus::AccessViolation;
    }
    return FlashStatus::Success;
}

/// Write a 32-bit word to flash via a 128-bit write
pub fn write_32(flc: &FLC, addr: u32, data: u32) -> FlashStatus {
    let mut current_data: [u32; 4] = [0xFFFF_FFFFu32; 4];
    // Check if the provided flash address is valid
    if addr < FLASH_BASE || addr > FLASH_END || addr & 0x3 != 0 {
        return FlashStatus::InvalidAddress;
    }
    // Cast input address as a pointer
    let addr_32: *mut u32 = addr as *mut u32;
    // Check that the only bits that need to be flipped are 1 -> 0
    // Safety: FLC address is valid
    unsafe {
        if *addr_32 & data != data {
            return FlashStatus::NeedsErase;
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
    // Check if the 32-bit word is already correct
    if current_data[idx] == data {
        return FlashStatus::Success;
    }
    // Update the 32-bit word
    current_data[idx] = data;
    // Write the 128-bit word to flash
    // Safety: We check that the only bits that need to be flipped are 1 -> 0
    return unsafe { write_128(flc, addr_128 as u32, &current_data) };
}

/// Erase a page of flash (8192 bytes)
pub fn erase_page(flc: &FLC, addr: u32) -> FlashStatus {
    // Check if the provided flash address is valid
    if addr < FLASH_BASE || addr > FLASH_END || addr & 0x1FFF != 0 {
        return FlashStatus::InvalidAddress;
    }
    // Ensure FLC is configured
    config(flc);
    // Create flash physical address
    let phys_addr = get_phys_addr(addr);
    // Safety: FLC address is valid
    flc.addr().write(|w| unsafe {
        w.addr().bits(phys_addr)
    });
    // Unlock flash
    flc.ctrl().modify(|_, w| w.unlock().unlocked());
    // Set FLC erase code
    flc.ctrl().modify(|_, w| w.erase_code().erase_page());
    // Commit the erase
    flc.ctrl().modify(|_, w| w.pge().set_bit());
    // Wait until the erase is complete
    while flc.ctrl().read().pend().bit_is_set() { }
    while is_busy(flc) { }
    // Lock flash
    flc.ctrl().modify(|_, w| w.unlock().locked());
    // Check access violations
    if flc.intr().read().af().bit_is_set() {
        flc.intr().modify(|_, w| w.af().clear_bit());
        return FlashStatus::AccessViolation;
    }
    return FlashStatus::Success;
}