//! Hardware True / Pseudo-Random Number Generator (RNG) Emulation.

use serde::{Deserialize, Serialize};

/// Hardware RNG Peripheral Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngPeripheral {
    pub enabled: bool,
    pub state: u64,
    pub seed_error: bool,
    pub clock_error: bool,
}

impl Default for RngPeripheral {
    fn default() -> Self {
        Self::new(0x1234_5678_9ABC_DEF0)
    }
}

impl RngPeripheral {
    pub fn new(seed: u64) -> Self {
        Self {
            enabled: true,
            state: if seed == 0 { 0xFEED_FACE_CAFE_BEEF } else { seed },
            seed_error: false,
            clock_error: false,
        }
    }

    /// Generates next 32-bit random integer using xorshift64* pseudo-entropy.
    pub fn next_u32(&mut self) -> Option<u32> {
        if !self.enabled || self.seed_error || self.clock_error {
            return None;
        }

        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;

        Some((x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32)
    }
}
