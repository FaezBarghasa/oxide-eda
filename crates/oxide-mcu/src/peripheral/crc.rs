//! Hardware Cyclic Redundancy Check (CRC) Calculation Unit.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CrcAlgorithm {
    /// Standard Ethernet / IEEE 802.3 32-bit polynomial (0x04C11DB7)
    #[default]
    Crc32Ethernet,
    /// CCITT 16-bit polynomial (0x1021)
    Crc16Ccitt,
    /// 8-bit polynomial (0x07)
    Crc8,
}

/// Hardware CRC Peripheral Model.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CrcPeripheral {
    pub algorithm: CrcAlgorithm,
    pub current_crc: u32,
    pub initial_value: u32,
}

impl CrcPeripheral {
    pub fn new() -> Self {
        Self {
            algorithm: CrcAlgorithm::Crc32Ethernet,
            current_crc: 0xFFFF_FFFF,
            initial_value: 0xFFFF_FFFF,
        }
    }

    pub fn reset(&mut self) {
        self.current_crc = self.initial_value;
    }

    /// Feeds 32-bit word into CRC calculation unit.
    pub fn feed_word(&mut self, mut data: u32) -> u32 {
        let poly = match self.algorithm {
            CrcAlgorithm::Crc32Ethernet => 0x04C1_1DB7,
            CrcAlgorithm::Crc16Ccitt => 0x1021,
            CrcAlgorithm::Crc8 => 0x07,
        };

        for _ in 0..32 {
            let msb = (self.current_crc ^ data) & 0x8000_0000;
            self.current_crc <<= 1;
            if msb != 0 {
                self.current_crc ^= poly;
            }
            data <<= 1;
        }

        self.current_crc
    }
}
