use ascon::{crypto_aead_encrypt, crypto_aead_decrypt};

mod ectf_global_secrets;
use ectf_global_secrets::{
    ASCON_SECRET_KEY_AP_TO_C,
    ASCON_SECRET_KEY_C_TO_AP,
};

pub const LEN_ASCON_128_KEY:    usize = 16;
pub const LEN_ASCON_128_TAG:    usize = 16;
pub const LEN_ASCON_128_NONCE:  usize = 16;
pub const LEN_ASCON_128_AD:     usize = 8;
pub const LEN_ASCON_128_PTXT:   usize = 64;
pub const LEN_ASCON_128_CTXT:   usize = LEN_ASCON_128_PTXT + LEN_ASCON_128_TAG;


/// For AP: Send a message to the component.
/// Everything in `message` will be sent.
pub fn ap_secure_send(component_id: &[u8; 4], message: &[u8]) {
    unimplemented!();
}

/// For AP: Receive a message from the component.
/// `output` should be the length of the message that needs to be received.
pub fn ap_secure_receive(component_id: &[u8; 4], output: &mut [u8]) {
    unimplemented!();
}

/// For Component: Send a message to the AP.
/// Everything in `message` will be sent.
pub fn comp_secure_send(message: &[u8]) {
    unimplemented!();
}

/// For Component: Receive a message from the AP.
/// `output` should be the length of the message that needs to be received.
pub fn comp_secure_receive(output: &mut [u8]) {
    unimplemented!();
}


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