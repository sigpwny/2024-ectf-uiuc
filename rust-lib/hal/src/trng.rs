pub use max78000_pac::TRNG;

/// Generate a random 32-bit number
pub fn random_u32(trng: &TRNG) -> u32 {
    while !trng.status().read().rdy().is_ready() { }
    return trng.data().read().bits();
}

/// Generate a random 8-bit number
pub fn random_u8(trng: &TRNG) -> u8 {
    while !trng.status().read().rdy().is_ready() { }
    return trng.data().read().bits() as u8;
}

/// Generate an array of random bytes
pub fn random_bytes(trng: &TRNG, bytes: &mut [u8]) {
    for byte in bytes {
        *byte = random_u8(trng);
    }
}