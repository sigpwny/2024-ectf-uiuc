use max78000_hal::{tmr2, tmr4, trng};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha3::{Digest, Sha3_256};

use crate::Board;

pub struct CustomRng(ChaCha20Rng);

impl CustomRng {
    pub fn new(board: &Board, seed: [u8; 64]) -> CustomRng {
        let mut hasher = Sha3_256::new();
        hasher.update(seed);
        for _ in 0..0x100 {
            hasher.update(trng::random_u32(&board.trng).to_ne_bytes());
        }
        for _ in 0..0x800 {
            hasher.update(tmr2::get_tick_count(&board.tmr2).to_ne_bytes());
            hasher.update(tmr4::get_tick_count(&board.tmr4).to_ne_bytes());
        }
        CustomRng(ChaCha20Rng::from_seed(hasher.finalize().into()))
    }
}

impl RngCore for CustomRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.0.try_fill_bytes(dest)
    }
}