// MISC state lengths
pub const LEN_COMPONENT_ID:         usize = 4;
pub const LEN_MISC_MESSAGE:         usize = 80;
pub const LEN_CID_HEX_STRING:       usize = 8;
pub const LEN_INPUT_CID_HEX_STRING: usize = 2 + LEN_CID_HEX_STRING;
pub const LEN_MAX_INPUT:            usize = 64;
pub const LEN_MAX_POST_BOOT_MSG:    u8 = 64;

// MISC secrets lengths
pub const LEN_ATTEST_PIN:           usize = 6;
pub const LEN_REPLACEMENT_TOKEN:    usize = 16;
pub const LEN_ATTEST_LOCATION:      usize = 64;
pub const LEN_ATTEST_DATE:          usize = 64;
pub const LEN_ATTEST_CUSTOMER:      usize = 64;
pub const LEN_AP_BOOT_MSG:          usize = 64;
pub const LEN_COMPONENT_BOOT_MSG:   usize = 64;
pub const LEN_SHA3_512_HASH:        usize = 64;

// MISC magic bytes
pub const MAGIC_MISC_REQ_LOCATION:  u8 = 0x60;
pub const MAGIC_MISC_REQ_DATE:      u8 = 0x62;
pub const MAGIC_MISC_REQ_CUSTOMER:  u8 = 0x64;
pub const MAGIC_MISC_BOOT_PING:     u8 = 0x80;
pub const MAGIC_MISC_BOOT_PONG:     u8 = 0x81;
pub const MAGIC_MISC_BOOT_NOW:      u8 = 0x82;

// Replace Components flash addresses
pub const FLASH_ADDR_CID_0:         u32 = 0x1007_A000;
pub const FLASH_ADDR_CID_1:         u32 = 0x1007_8000;