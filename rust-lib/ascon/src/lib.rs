#![no_std]

extern {
    pub fn crypto_aead_encrypt(
        c: *mut cty::c_uchar,
        clen: *mut cty::c_ulonglong,
        m: *const cty::c_uchar,
        mlen: cty::c_ulonglong,
        ad: *const cty::c_uchar,
        adlen: cty::c_ulonglong,
        nsec: *const cty::c_uchar,
        npub: *const cty::c_uchar,
        k: *const cty::c_uchar,
    ) -> cty::c_int;
    pub fn crypto_aead_decrypt(
        m: *mut cty::c_uchar,
        mlen: *mut cty::c_ulonglong,
        nsec: *mut cty::c_uchar,
        c: *const cty::c_uchar,
        clen: cty::c_ulonglong,
        ad: *const cty::c_uchar,
        adlen: cty::c_ulonglong,
        npub: *const cty::c_uchar,
        k: *const cty::c_uchar,
    ) -> cty::c_int;
}