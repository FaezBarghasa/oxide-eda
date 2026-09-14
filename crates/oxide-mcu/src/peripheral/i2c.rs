//! Inter-Integrated Circuit (I2C) Emulation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum I2cSpeedMode {
    #[default]
    Standard100k,
    Fast400k,
    FastPlus1M,
    HighSpeed3_4M,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum I2cAddressingMode {
    #[default]
    Bits7,
    Bits10,
}

/// I2C Hardware peripheral model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I2cPeripheral {
    pub name: String,
    pub enabled: bool,
    pub own_address: u16,
    pub speed_mode: I2cSpeedMode,
    pub addressing_mode: I2cAddressingMode,
    pub clock_stretching: bool,
    pub bus_busy: bool,
    pub ack_received: bool,
}

impl I2cPeripheral {
    pub fn new(name: impl Into<String>, own_address: u16) -> Self {
        Self {
            name: name.into(),
            enabled: true,
            own_address,
            speed_mode: I2cSpeedMode::Standard100k,
            addressing_mode: I2cAddressingMode::Bits7,
            clock_stretching: false,
            bus_busy: false,
            ack_received: true,
        }
    }

    pub fn start_condition(&mut self) {
        self.bus_busy = true;
    }

    pub fn stop_condition(&mut self) {
        self.bus_busy = false;
    }

    pub fn send_address(&mut self, addr: u16, is_read: bool) -> bool {
        let _ = is_read;
        // In simple simulation, ACK if address is valid non-zero
        self.ack_received = addr > 0;
        self.ack_received
    }
}
