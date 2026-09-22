//! Serial Peripheral Interface (SPI) Emulation.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SpiRole {
    #[default]
    Master,
    Slave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SpiMode {
    /// CPOL = 0, CPHA = 0
    #[default]
    Mode0,
    /// CPOL = 0, CPHA = 1
    Mode1,
    /// CPOL = 1, CPHA = 0
    Mode2,
    /// CPOL = 1, CPHA = 1
    Mode3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SpiDataSize {
    #[default]
    Bits8,
    Bits16,
    Bits32,
}

/// SPI Hardware peripheral model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiPeripheral {
    pub name: String,
    pub enabled: bool,
    pub role: SpiRole,
    pub mode: SpiMode,
    pub data_size: SpiDataSize,
    pub baud_prescaler: u16,
    pub tx_fifo: VecDeque<u16>,
    pub rx_fifo: VecDeque<u16>,
}

impl SpiPeripheral {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled: true,
            role: SpiRole::Master,
            mode: SpiMode::Mode0,
            data_size: SpiDataSize::Bits8,
            baud_prescaler: 4,
            tx_fifo: VecDeque::with_capacity(32),
            rx_fifo: VecDeque::with_capacity(32),
        }
    }

    /// Full-duplex byte transfer on Master.
    pub fn transfer_byte(&mut self, tx_byte: u8) -> u8 {
        if !self.enabled {
            return 0xFF;
        }
        self.tx_fifo.push_back(tx_byte as u16);
        self.rx_fifo.pop_front().map(|v| v as u8).unwrap_or(0xFF)
    }

    pub fn inject_rx(&mut self, val: u16) {
        self.rx_fifo.push_back(val);
    }
}
