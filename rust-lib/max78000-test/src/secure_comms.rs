use ascon::{crypto_aead_encrypt, crypto_aead_decrypt};

pub const LEN_ASCON_128_KEY:    usize = 16;
pub const LEN_ASCON_128_TAG:    usize = 16;
pub const LEN_ASCON_128_NONCE:  usize = 16;
pub const LEN_ASCON_128_AD:     usize = 8;
pub const LEN_ASCON_128_PTXT:   usize = 64;
pub const LEN_ASCON_128_CTXT:   usize = LEN_ASCON_128_PTXT + LEN_ASCON_128_TAG;


/// TODO: Secure send
// pub fn hide_secure_send(
//     message: &[u8],
//     associated_data: &[u8],
//     nonce: &[u8],
//     key: &[u8],
// ) -> [u8; LEN_ASCON_128_CTXT] {
//     let mut ciphertext = [0u8; LEN_ASCON_128_CTXT];
//     let result = ascon_encrypt(
//         &mut ciphertext,
//         message,
//         associated_data,
//         nonce,
//         key,
//     );
//     if result != 0 {
//         panic!("Failed to encrypt message");
//     }
//     ciphertext
// }


/// Encrypts a message using Ascon-128
pub fn ascon_encrypt(
    ciphertext: &mut [u8; LEN_ASCON_128_CTXT],
    message: &[u8; LEN_ASCON_128_PTXT],
    associated_data: &[u8; LEN_ASCON_128_AD],
    nonce: &[u8; LEN_ASCON_128_NONCE],
    key: &[u8; LEN_ASCON_128_KEY],
) -> i32 {
    let mut _result: i32 = -1;
    let mut clen: u64 = 0;
    // Safety: We're calling the C impl, so raw pointers are required
    unsafe {
        _result = crypto_aead_encrypt(
            ciphertext.as_mut_ptr(),
            &mut clen,
            message.as_ptr(),
            LEN_ASCON_128_PTXT as u64,
            associated_data.as_ptr(),
            LEN_ASCON_128_AD as u64,
            core::ptr::null(),
            nonce.as_ptr(),
            key.as_ptr(),
        )
    }
    if clen != LEN_ASCON_128_CTXT as u64 {
        panic!("clen != LEN_ASCON_128_CTXT");
    }
    _result
}

/// Decrypts a message using Ascon-128
pub fn ascon_decrypt(
    message: &mut [u8; LEN_ASCON_128_PTXT],
    ciphertext: &[u8; LEN_ASCON_128_CTXT],
    associated_data: &[u8; LEN_ASCON_128_AD],
    nonce: &[u8; LEN_ASCON_128_NONCE],
    key: &[u8; LEN_ASCON_128_KEY],
) -> i32 {
    let mut _result: i32 = -1;
    let mut mlen: u64 = 0;
    // Safety: We're calling the C impl, so raw pointers are required
    unsafe {
        _result = crypto_aead_decrypt(
            message.as_mut_ptr(),
            &mut mlen,
            core::ptr::null_mut(),
            ciphertext.as_ptr(),
            LEN_ASCON_128_CTXT as u64,
            associated_data.as_ptr(),
            LEN_ASCON_128_AD as u64,
            nonce.as_ptr(),
            key.as_ptr(),
        )
    }
    if mlen != LEN_ASCON_128_PTXT as u64 {
        panic!("mlen != LEN_ASCON_128_PTXT");
    }
    _result
}