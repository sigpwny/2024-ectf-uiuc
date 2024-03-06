use max78000_hal::{tmr2, tmr4, trng};
use max78000_pac::{TMR2, TMR4, TRNG};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha3::{Digest, Sha3_256};

/// Our custom RNG implementation based on ChaCha20
pub struct CustomRng(ChaCha20Rng);

impl CustomRng {
    /// Constructs our custom RNG combining timing drift from tmr2 and tmr4
    /// (on secondary oscilators) and the TRNG along with build-time seed
    pub fn new(tmr2: &TMR2, tmr4: &TMR4, trng: &TRNG, seed: [u8; 64]) -> CustomRng {
        let mut hasher = Sha3_256::new();
        hasher.update(seed);
        for _ in 0..0x100 {
            hasher.update(trng::random_u32(trng).to_ne_bytes());
        }
        for _ in 0..0x800 {
            hasher.update(tmr2::get_tick_count(tmr2).to_ne_bytes());
            hasher.update(tmr4::get_tick_count(tmr4).to_ne_bytes());
        }
        CustomRng(ChaCha20Rng::from_seed(hasher.finalize().into()))
    }
}

impl RngCore for CustomRng {
    /// Get u32 out of PRNG
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    /// Get u64 out of PRNG
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }
    
    /// Fill bytes out of PRNG
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }

    /// Fill bytes if possible from PRNG
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.0.try_fill_bytes(dest)
    }
}