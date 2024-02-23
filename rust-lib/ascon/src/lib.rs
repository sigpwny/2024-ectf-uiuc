#![no_std]

extern {
    pub fn crypto_aead_encrypt(
        c: *mut u8,
        clen: *mut u64,
        m: *const u8,
        mlen: u64,
        ad: *const u8,
        adlen: u64,
        nsec: *const u8,
        npub: *const u8,
        k: *const u8,
    ) -> i32;
    pub fn crypto_aead_decrypt(
        m: *mut u8,
        mlen: *mut u64,
        nsec: *mut u8,
        c: *const u8,
        clen: u64,
        ad: *const u8,
        adlen: u64,
        npub: *const u8,
        k: *const u8,
    ) -> i32;
}