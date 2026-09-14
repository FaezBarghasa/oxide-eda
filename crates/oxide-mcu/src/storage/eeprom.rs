//! External EEPROM Emulation: Microchip 24LCxx (I2C) & 25LCxx (SPI) series.

use serde::{Deserialize, Serialize};

/// I2C EEPROM Model (e.g. Microchip 24LC04, 24LC64, 24LC256, 24LC512, 24LC1025).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I2cEeprom {
    pub name: String,
    pub i2c_address: u8,
    pub size_bytes: usize,
    pub page_size_bytes: usize,
    pub memory: Vec<u8>,
    pub write_protected: bool,
    internal_address: u16,
    is_16bit_addr: bool,
}

impl I2cEeprom {
    pub fn new_24lc256(i2c_address: u8) -> Self {
        Self {
            name: "24LC256 (32KB I2C EEPROM)".to_string(),
            i2c_address,
            size_bytes: 32768,
            page_size_bytes: 64,
            memory: vec![0xFF; 32768],
            write_protected: false,
            internal_address: 0,
            is_16bit_addr: true,
        }
    }

    pub fn new_24lc04(i2c_address: u8) -> Self {
        Self {
            name: "24LC04 (512B I2C EEPROM)".to_string(),
            i2c_address,
            size_bytes: 512,
            page_size_bytes: 16,
            memory: vec![0xFF; 512],
            write_protected: false,
            internal_address: 0,
            is_16bit_addr: false,
        }
    }

    /// Handles incoming I2C write transaction (address + data).
    pub fn handle_i2c_write(&mut self, data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }

        if self.is_16bit_addr {
            if data.len() < 2 {
                return false;
            }
            self.internal_address = ((data[0] as u16) << 8) | (data[1] as u16);
            if self.internal_address as usize >= self.size_bytes {
                self.internal_address %= self.size_bytes as u16;
            }

            if data.len() > 2 && !self.write_protected {
                let payload = &data[2..];
                for (offset, &byte) in payload.iter().enumerate() {
                    let addr = (self.internal_address as usize + offset) % self.size_bytes;
                    self.memory[addr] = byte;
                }
            }
        } else {
            self.internal_address = data[0] as u16;
            if data.len() > 1 && !self.write_protected {
                let payload = &data[1..];
                for (offset, &byte) in payload.iter().enumerate() {
                    let addr = (self.internal_address as usize + offset) % self.size_bytes;
                    self.memory[addr] = byte;
                }
            }
        }
        true
    }

    /// Handles sequential I2C read from current internal address.
    pub fn handle_i2c_read(&mut self, len: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            let addr = self.internal_address as usize;
            out.push(self.memory[addr]);
            self.internal_address = ((self.internal_address as usize + 1) % self.size_bytes) as u16;
        }
        out
    }
}

/// SPI EEPROM Model (e.g. Microchip 25LC040, 25LC640, 25LC256, 25LC1024).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiEeprom {
    pub name: String,
    pub size_bytes: usize,
    pub page_size_bytes: usize,
    pub memory: Vec<u8>,
    pub write_enable_latch: bool,
    pub status_register: u8,
    state: SpiEepromState,
    cmd_buffer: Vec<u8>,
    read_address: usize,
    write_address: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SpiEepromState {
    Idle,
    ReadingData,
    WritingData,
    ReadingStatus,
}

impl SpiEeprom {
    pub fn new_25lc256() -> Self {
        Self {
            name: "25LC256 (32KB SPI EEPROM)".to_string(),
            size_bytes: 32768,
            page_size_bytes: 64,
            memory: vec![0xFF; 32768],
            write_enable_latch: false,
            status_register: 0x00,
            state: SpiEepromState::Idle,
            cmd_buffer: Vec::with_capacity(4),
            read_address: 0,
            write_address: 0,
        }
    }

    /// Pull CS High: Ends transaction and commits page writes.
    pub fn chip_deselect(&mut self) {
        self.state = SpiEepromState::Idle;
        self.cmd_buffer.clear();
    }

    /// Full-duplex SPI byte exchange.
    pub fn transfer_byte(&mut self, byte: u8) -> u8 {
        match self.state {
            SpiEepromState::Idle => {
                self.cmd_buffer.push(byte);
                match self.cmd_buffer[0] {
                    0x06 => { // WREN: Write Enable
                        self.write_enable_latch = true;
                        self.status_register |= 0x02; // WEL bit
                        self.cmd_buffer.clear();
                        0xFF
                    }
                    0x04 => { // WRDI: Write Disable
                        self.write_enable_latch = false;
                        self.status_register &= !0x02;
                        self.cmd_buffer.clear();
                        0xFF
                    }
                    0x05 => { // RDSR: Read Status Register
                        self.state = SpiEepromState::ReadingStatus;
                        0xFF
                    }
                    0x03 => { // READ: Read data from memory
                        if self.cmd_buffer.len() == 3 {
                            self.read_address = ((self.cmd_buffer[1] as usize) << 8) | (self.cmd_buffer[2] as usize);
                            self.read_address %= self.size_bytes;
                            self.state = SpiEepromState::ReadingData;
                        }
                        0xFF
                    }
                    0x02 => { // WRITE: Write data to memory
                        if self.cmd_buffer.len() == 3 {
                            self.write_address = ((self.cmd_buffer[1] as usize) << 8) | (self.cmd_buffer[2] as usize);
                            self.write_address %= self.size_bytes;
                            self.state = SpiEepromState::WritingData;
                        }
                        0xFF
                    }
                    _ => 0xFF,
                }
            }
            SpiEepromState::ReadingStatus => {
                self.status_register
            }
            SpiEepromState::ReadingData => {
                let val = self.memory[self.read_address];
                self.read_address = (self.read_address + 1) % self.size_bytes;
                val
            }
            SpiEepromState::WritingData => {
                if self.write_enable_latch {
                    self.memory[self.write_address] = byte;
                    self.write_address = (self.write_address + 1) % self.size_bytes;
                }
                0xFF
            }
        }
    }
}
