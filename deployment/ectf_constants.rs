// General lengths
pub const LEN_COMPONENT_ID:         usize = 4;
pub const LEN_CID_HEX_STRING:       usize = 8;
pub const LEN_MAX_INPUT:            usize = 64;
pub const LEN_SHA3_512_HASH:        usize = 64;

// MISC magic bytes
// pub const MAGIC_MISC_LIST_PING:     u8 = 0x50;
// pub const MAGIC_MISC_LIST_PONG:     u8 = 0x51;
pub const MAGIC_MISC_REQ_LOCATION:  u8 = 0x60;
pub const MAGIC_MISC_SEND_LOCATION: u8 = 0x61;
pub const MAGIC_MISC_REQ_DATE:      u8 = 0x62;
pub const MAGIC_MISC_SEND_DATE:     u8 = 0x63;
pub const MAGIC_MISC_REQ_CUSTOMER:  u8 = 0x64;
pub const MAGIC_MISC_SEND_CUSTOMER: u8 = 0x65;
pub const MAGIC_MISC_BOOT_PING:     u8 = 0x80;
pub const MAGIC_MISC_BOOT_PONG:     u8 = 0x81;
pub const MAGIC_MISC_BOOT_NOW:      u8 = 0x82;

// List Component lengths
pub const LEN_LIST_PING:            usize = 1;

// Attestation lengths

// Replace Component lengths
pub const LEN_REPLACEMENT_TOKEN:    usize = 16;
pub const LEN_INPUT_CID_HEX_STRING: usize = 2 + LEN_CID_HEX_STRING;

// Replace Components flash addresses
pub const FLASH_ADDR_CID_0:         u32 = 0x1007_C000;
pub const FLASH_ADDR_CID_1:         u32 = 0x1007_A000;