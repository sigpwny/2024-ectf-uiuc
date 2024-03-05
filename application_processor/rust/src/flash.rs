use ectf_board::{
    Board,
    ectf_constants::{*}
};
use crate::ectf_ap_params::{*};

/// Checks if a given u8 is a valid I2C address
pub fn is_i2c_addr_blacklisted(addr: u8) -> bool {
    let addr = addr & 0x7F;
    match addr {
        0x00..=0x07 => return true,
        0x18 => return true,
        0x28 => return true,
        0x36 => return true,
        0x78..=0x7F => return true,
        _ => return false,
    }
}

/// Get provisioned component ID stored in flash
pub fn get_provisioned_component_id(cid: &mut [u8; LEN_COMPONENT_ID], idx: u8) -> bool {
    if idx >= COMPONENT_CNT {
        return false;
    }
    // If the component ID is written in flash, use the flash value
    if is_comp_id_in_flash(idx) {
        let addr_ptr: *const u8 = match idx {
            0 => FLASH_ADDR_CID_0 as *const u8,
            1 => FLASH_ADDR_CID_1 as *const u8,
            _ => return false,
        };
        // Safety: We're reading from a valid flash address
        let byte0 = unsafe { core::ptr::read_volatile(addr_ptr) };
        let byte1 = unsafe { core::ptr::read_volatile(addr_ptr.add(1)) };
        let byte2 = unsafe { core::ptr::read_volatile(addr_ptr.add(2)) };
        let byte3 = unsafe { core::ptr::read_volatile(addr_ptr.add(3)) };
        cid[0] = byte0;
        cid[1] = byte1;
        cid[2] = byte2;
        cid[3] = byte3;
    }
    // Otherwise, use the original component ID
    else {
        match idx {
            0 => {
                cid[0] = COMPONENT_ID_0[0];
                cid[1] = COMPONENT_ID_0[1];
                cid[2] = COMPONENT_ID_0[2];
                cid[3] = COMPONENT_ID_0[3];
            }
            1 => {
                cid[0] = COMPONENT_ID_1[0];
                cid[1] = COMPONENT_ID_1[1];
                cid[2] = COMPONENT_ID_1[2];
                cid[3] = COMPONENT_ID_1[3];
            }
            _ => return false,
        }
    }
    // Ensure that the component ID is not blacklisted
    if is_i2c_addr_blacklisted(cid[3]) {
        return false;
    }
    return true;
}

/// Check if component IDs are initialized in flash
pub fn is_comp_id_in_flash(idx: u8) -> bool {
    let addr_ptr = match idx {
        0 => FLASH_ADDR_CID_0 as *const u8,
        1 => FLASH_ADDR_CID_1 as *const u8,
        _ => return false,
    };
    // Safety: We're reading from a valid flash address
    let byte = unsafe {
        core::ptr::read_volatile(addr_ptr.add(3))
    };
    // Check if the last byte is 0xFF
    if byte == 0xFF {
        return false;
    }
    return true;
}

/// Set provisioned component ID in flash
pub fn set_provisioned_component_id(board: &Board, cid: &[u8; LEN_COMPONENT_ID], idx: u8) -> bool {
    if idx >= COMPONENT_CNT {
        return false;
    }
    // Check if the component ID is blacklisted
    if is_i2c_addr_blacklisted(cid[3]) {
        return false;
    }
    let flash_addr = match idx {
        0 => FLASH_ADDR_CID_0,
        1 => FLASH_ADDR_CID_1,
        _ => return false,
    };
    board.write_flash_bytes(flash_addr, cid);
    return true;
}