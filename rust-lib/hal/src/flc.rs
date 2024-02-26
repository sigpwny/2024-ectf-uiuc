pub use max78000_pac::FLC;
use cortex_m::asm;

pub const FLASH_BASE: u32 = 0x1000_0000;
pub const FLASH_SIZE: u32 = 0x0008_0000;
pub const FLASH_END: u32 = FLASH_BASE + FLASH_SIZE;

pub enum FlashStatus {
    Success = 0,
    InvalidAddress = -1,
    AccessViolation = -2,
    NeedsErase = -3,
}


/// Configure the FLC peripheral
#[link_section = ".flashprog.config"]
pub fn config(flc: &FLC) {
    // Wait for the FLC to be ready
    // while flc.ctrl().read().pend().bit_is_set() { }
    unsafe { while is_busy(flc) { } }
    // Set the FLC clock divisor to 100 (0x64) assuming a 100MHz system clock
    flc.clkdiv().modify(|_, w| unsafe {
        w.clkdiv().bits(0x64)
    });
    // Clear stale interrupt flags
    if flc.intr().read().af().bit_is_set() {
        flc.intr().modify(|_, w| w.af().clear_bit());
    }
}

/// Check if the FLC is busy with a write or erase (page or mass) operation.
/// This function does not use the PAC to avoid jumping out of SRAM and
/// into flash while a write or erase operation is in progress.
/// Safety: This is a simple register read, so it should be safe.
#[link_section = ".flashprog.is_busy"]
#[inline(always)]
unsafe fn is_busy(_flc: &FLC) -> bool {
    let flc_ctrl_reg_addr = 0x4002_9008 as *const u32;
    return *flc_ctrl_reg_addr & 0b111 != 0;
}

/// Get the physical address of a flash address
#[link_section = ".flashprog.get_phys_addr"]
#[inline(always)]
fn get_phys_addr(addr: u32) -> u32 {
    return addr & (FLASH_SIZE - 1) /*| 0x8000_0000*/;
}

/// Clear the line fill buffer
#[link_section = ".flashprog.flush_flash"]
#[inline(always)]
fn flush_flash() {
    let start_addr: *mut u32 = FLASH_BASE as *mut u32;
    let end_addr: *mut u32 = FLASH_END as *mut u32;
    unsafe {
        let _line1 = *start_addr;
        let _line2 = *end_addr;
    }
}

/// Write a 128-bit word to flash
#[link_section = ".flashprog.write_128"]
#[inline(always)]
fn write_128(flc: &FLC, addr: u32, data: &[u32; 4]) -> FlashStatus {
    // Check that the provided flash address is valid
    if addr < FLASH_BASE || addr > FLASH_END || addr & 0xF != 0 {
        return FlashStatus::InvalidAddress;
    }
    // Ensure FLC is configured
    config(flc);
    // Unlock flash
    flc.ctrl().modify(|_, w| w.unlock().unlocked());
    while !flc.ctrl().read().unlock().is_unlocked() {}
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
    for _ in 0..1_000_000 {
        asm::nop();
    }
    // while flc.ctrl().read().pend().bit_is_set() { }
    // unsafe { while is_busy(flc) { } }
    // Lock flash
    flc.ctrl().modify(|_, w| w.unlock().locked());
    while flc.ctrl().read().unlock().is_unlocked() {}
    // Clear the line fill buffer
    flush_flash();
    // Check access violations
    if flc.intr().read().af().bit_is_set() {
        flc.intr().modify(|_, w| w.af().clear_bit());
        return FlashStatus::AccessViolation;
    }
    return FlashStatus::Success;
}

/// Write a 32-bit word to flash via a 128-bit write
#[link_section = ".flashprog.write_32"]
#[inline(never)]
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
    let result = write_128(flc, addr_128 as u32, &current_data);
    return result;
}

#[link_section = ".flashprog.erase_page_hot"]
#[inline(always)]
fn erase_page_hot(flc: &FLC, addr: u32) {
    // Create flash physical address
    let phys_addr = get_phys_addr(addr);
    // Safety: FLC address is valid
    flc.addr().write(|w| unsafe {
        w.addr().bits(phys_addr)
    });
    // Unlock flash
    flc.ctrl().modify(|_, w| w.unlock().unlocked());
    while !flc.ctrl().read().unlock().is_unlocked() {}
    // Set FLC erase code
    flc.ctrl().modify(|_, w| w.erase_code().erase_page());
    // Commit the erase
    flc.ctrl().modify(|_, w| w.pge().set_bit());
    // Wait until the erase is complete
    for _ in 0..1_000_000 {
        asm::nop();
    }
    // while flc.ctrl().read().pend().bit_is_set() { }
    // unsafe { while is_busy(flc) { } }
    // Lock flash
    flc.ctrl().modify(|_, w| w.unlock().locked());
    while flc.ctrl().read().unlock().is_unlocked() {}
}

/// Erase a page of flash (8192 bytes)
#[link_section = ".flashprog.erase_page"]
#[inline(never)]
pub fn erase_page(flc: &FLC, addr: u32) -> FlashStatus {
    // Check if the provided flash address is valid
    if addr < FLASH_BASE || addr > FLASH_END || addr & 0x1FFF != 0 {
        return FlashStatus::InvalidAddress;
    }
    // Ensure FLC is configured
    config(flc);
    // Execute inline erase
    erase_page_hot(flc, addr);
    // Clear the line fill buffer
    flush_flash();
    // Check access violations
    if flc.intr().read().af().bit_is_set() {
        flc.intr().modify(|_, w| w.af().clear_bit());
        return FlashStatus::AccessViolation;
    }
    return FlashStatus::Success;
}