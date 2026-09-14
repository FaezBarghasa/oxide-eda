//! External Parallel Asynchronous SRAM & Synchronous SDRAM Emulation (FMC/FSMC bus).

use serde::{Deserialize, Serialize};

/// Asynchronous Parallel SRAM (e.g. ISSI IS62WV51216, Cypress CY62167EV30 16-bit SRAM).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelSram {
    pub name: String,
    pub base_address: u32,
    pub size_bytes: usize,
    pub bus_width_bits: u8,
    pub memory: Vec<u8>,
}

impl ParallelSram {
    pub fn new_16bit(name: &str, base_address: u32, size_bytes: usize) -> Self {
        Self {
            name: name.to_string(),
            base_address,
            size_bytes,
            bus_width_bits: 16,
            memory: vec![0x00; size_bytes],
        }
    }

    /// Read 8-bit byte.
    pub fn read_u8(&self, addr: u32) -> Option<u8> {
        if addr >= self.base_address && (addr - self.base_address) < self.size_bytes as u32 {
            let offset = (addr - self.base_address) as usize;
            Some(self.memory[offset])
        } else {
            None
        }
    }

    /// Read 16-bit half-word.
    pub fn read_u16(&self, addr: u32) -> Option<u16> {
        if addr >= self.base_address && (addr - self.base_address + 1) < self.size_bytes as u32 {
            let offset = (addr - self.base_address) as usize;
            Some(u16::from_le_bytes([self.memory[offset], self.memory[offset + 1]]))
        } else {
            None
        }
    }

    /// Read 32-bit word.
    pub fn read_u32(&self, addr: u32) -> Option<u32> {
        if addr >= self.base_address && (addr - self.base_address + 3) < self.size_bytes as u32 {
            let offset = (addr - self.base_address) as usize;
            Some(u32::from_le_bytes([
                self.memory[offset],
                self.memory[offset + 1],
                self.memory[offset + 2],
                self.memory[offset + 3],
            ]))
        } else {
            None
        }
    }

    /// Write 8-bit byte.
    pub fn write_u8(&mut self, addr: u32, val: u8) -> bool {
        if addr >= self.base_address && (addr - self.base_address) < self.size_bytes as u32 {
            let offset = (addr - self.base_address) as usize;
            self.memory[offset] = val;
            true
        } else {
            false
        }
    }

    /// Write 16-bit half-word with Byte High/Low Enable (`BHE`/`BLE`).
    pub fn write_u16(&mut self, addr: u32, val: u16, ble: bool, bhe: bool) -> bool {
        if addr >= self.base_address && (addr - self.base_address + 1) < self.size_bytes as u32 {
            let offset = (addr - self.base_address) as usize;
            let bytes = val.to_le_bytes();
            if ble {
                self.memory[offset] = bytes[0];
            }
            if bhe {
                self.memory[offset + 1] = bytes[1];
            }
            true
        } else {
            false
        }
    }

    /// Write 32-bit word.
    pub fn write_u32(&mut self, addr: u32, val: u32) -> bool {
        if addr >= self.base_address && (addr - self.base_address + 3) < self.size_bytes as u32 {
            let offset = (addr - self.base_address) as usize;
            let bytes = val.to_le_bytes();
            self.memory[offset..offset + 4].copy_from_slice(&bytes);
            true
        } else {
            false
        }
    }
}
