//! External SPI / Quad-SPI PSRAM Emulation (APMemory APS6404L / ISSI IS66WVS series).

use serde::{Deserialize, Serialize};

/// High-Speed SPI / Quad-SPI Pseudo-Static RAM (PSRAM).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiRam {
    pub name: String,
    pub size_bytes: usize,
    pub memory: Vec<u8>,
    state: SpiRamState,
    cmd_buffer: Vec<u8>,
    current_addr: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SpiRamState {
    Idle,
    Reading { dummy_cycles: usize },
    Writing,
    ReadingId { byte_idx: usize },
}

impl SpiRam {
    /// 8MB (64Mbit) SPI/QSPI PSRAM (e.g. ESP-PSRAM64H / APS6404L).
    pub fn new_aps6404() -> Self {
        Self {
            name: "APS6404L (8MB QSPI PSRAM)".to_string(),
            size_bytes: 8 * 1024 * 1024,
            memory: vec![0x00; 8 * 1024 * 1024],
            state: SpiRamState::Idle,
            cmd_buffer: Vec::with_capacity(5),
            current_addr: 0,
        }
    }

    /// 4MB (32Mbit) SPI/QSPI PSRAM (e.g. APS3204L).
    pub fn new_aps3204() -> Self {
        Self {
            name: "APS3204L (4MB QSPI PSRAM)".to_string(),
            size_bytes: 4 * 1024 * 1024,
            memory: vec![0x00; 4 * 1024 * 1024],
            state: SpiRamState::Idle,
            cmd_buffer: Vec::with_capacity(5),
            current_addr: 0,
        }
    }

    pub fn chip_deselect(&mut self) {
        self.state = SpiRamState::Idle;
        self.cmd_buffer.clear();
    }

    /// Full-duplex SPI byte exchange.
    pub fn transfer_byte(&mut self, byte: u8) -> u8 {
        match self.state {
            SpiRamState::Idle => {
                self.cmd_buffer.push(byte);
                match self.cmd_buffer[0] {
                    0x03 => { // READ: Normal Read (3 address bytes, no dummy)
                        if self.cmd_buffer.len() == 4 {
                            self.current_addr = ((self.cmd_buffer[1] as usize) << 16)
                                | ((self.cmd_buffer[2] as usize) << 8)
                                | (self.cmd_buffer[3] as usize);
                            self.current_addr %= self.size_bytes;
                            self.state = SpiRamState::Reading { dummy_cycles: 0 };
                        }
                        0xFF
                    }
                    0x0B | 0xEB => { // Fast Read / Quad Fast Read (3 address bytes + 1 dummy byte)
                        if self.cmd_buffer.len() == 4 {
                            self.current_addr = ((self.cmd_buffer[1] as usize) << 16)
                                | ((self.cmd_buffer[2] as usize) << 8)
                                | (self.cmd_buffer[3] as usize);
                            self.current_addr %= self.size_bytes;
                            self.state = SpiRamState::Reading { dummy_cycles: 1 };
                        }
                        0xFF
                    }
                    0x02 | 0x38 => { // WRITE / Quad Write (3 address bytes)
                        if self.cmd_buffer.len() == 4 {
                            self.current_addr = ((self.cmd_buffer[1] as usize) << 16)
                                | ((self.cmd_buffer[2] as usize) << 8)
                                | (self.cmd_buffer[3] as usize);
                            self.current_addr %= self.size_bytes;
                            self.state = SpiRamState::Writing;
                        }
                        0xFF
                    }
                    0x9F => { // Read ID (KGD / Manufacturer ID)
                        self.state = SpiRamState::ReadingId { byte_idx: 0 };
                        0xFF
                    }
                    0x66 | 0x99 => { // Reset Enable / Reset
                        self.cmd_buffer.clear();
                        0xFF
                    }
                    _ => 0xFF,
                }
            }
            SpiRamState::ReadingId { ref mut byte_idx } => {
                let id = match *byte_idx {
                    0 => 0x0D, // APMemory Vendor ID
                    1 => 0x5D, // Device ID (PSRAM)
                    _ => 0x00,
                };
                *byte_idx += 1;
                id
            }
            SpiRamState::Reading { ref mut dummy_cycles } => {
                if *dummy_cycles > 0 {
                    *dummy_cycles -= 1;
                    0xFF
                } else {
                    let val = self.memory[self.current_addr];
                    self.current_addr = (self.current_addr + 1) % self.size_bytes;
                    val
                }
            }
            SpiRamState::Writing => {
                self.memory[self.current_addr] = byte;
                self.current_addr = (self.current_addr + 1) % self.size_bytes;
                0xFF
            }
        }
    }
}
