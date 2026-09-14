//! External SPI / Quad-SPI NOR Flash Model (Winbond W25Qxx / Macronix MX25xx series).

use serde::{Deserialize, Serialize};

/// SPI / QSPI NOR Flash Emulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiFlash {
    pub name: String,
    pub size_bytes: usize,
    pub page_size_bytes: usize,
    pub sector_size_bytes: usize,
    pub block_size_bytes: usize,
    pub memory: Vec<u8>,
    pub write_enable_latch: bool,
    pub status_register_1: u8,
    pub status_register_2: u8,
    state: SpiFlashState,
    cmd_buffer: Vec<u8>,
    current_addr: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SpiFlashState {
    Idle,
    ReadingData,
    FastReadingData { dummy_cycles: usize },
    WritingPage,
    ReadingStatus1,
    ReadingStatus2,
    ReadingJedecId { byte_idx: usize },
    ReadingSfdp { addr: usize, dummy_cycles: usize },
}

impl SpiFlash {
    /// Constructs a standard 8MB (64Mbit) Winbond W25Q64JV model.
    pub fn new_w25q64() -> Self {
        Self {
            name: "W25Q64JV (8MB Quad-SPI Flash)".to_string(),
            size_bytes: 8 * 1024 * 1024,
            page_size_bytes: 256,
            sector_size_bytes: 4096,
            block_size_bytes: 65536,
            memory: vec![0xFF; 8 * 1024 * 1024],
            write_enable_latch: false,
            status_register_1: 0x00,
            status_register_2: 0x02, // Quad-Enable (QE) bit set
            state: SpiFlashState::Idle,
            cmd_buffer: Vec::with_capacity(5),
            current_addr: 0,
        }
    }

    /// Constructs a standard 16MB (128Mbit) Winbond W25Q128JV model.
    pub fn new_w25q128() -> Self {
        Self {
            name: "W25Q128JV (16MB Quad-SPI Flash)".to_string(),
            size_bytes: 16 * 1024 * 1024,
            page_size_bytes: 256,
            sector_size_bytes: 4096,
            block_size_bytes: 65536,
            memory: vec![0xFF; 16 * 1024 * 1024],
            write_enable_latch: false,
            status_register_1: 0x00,
            status_register_2: 0x02,
            state: SpiFlashState::Idle,
            cmd_buffer: Vec::with_capacity(5),
            current_addr: 0,
        }
    }

    pub fn chip_deselect(&mut self) {
        self.state = SpiFlashState::Idle;
        self.cmd_buffer.clear();
    }

    /// Erases 4KB Sector.
    pub fn erase_sector(&mut self, sector_addr: usize) {
        if self.write_enable_latch {
            let base = (sector_addr & !(self.sector_size_bytes - 1)) % self.size_bytes;
            for i in 0..self.sector_size_bytes {
                self.memory[base + i] = 0xFF;
            }
        }
    }

    /// Erases 64KB Block.
    pub fn erase_block_64k(&mut self, block_addr: usize) {
        if self.write_enable_latch {
            let base = (block_addr & !(self.block_size_bytes - 1)) % self.size_bytes;
            for i in 0..self.block_size_bytes {
                self.memory[base + i] = 0xFF;
            }
        }
    }

    /// Full chip erase (0xC7 / 0x60).
    pub fn erase_chip(&mut self) {
        if self.write_enable_latch {
            self.memory.fill(0xFF);
        }
    }

    /// Full-duplex SPI byte exchange.
    pub fn transfer_byte(&mut self, byte: u8) -> u8 {
        match self.state {
            SpiFlashState::Idle => {
                self.cmd_buffer.push(byte);
                let cmd = self.cmd_buffer[0];
                match cmd {
                    0x06 => { // WREN: Write Enable
                        self.write_enable_latch = true;
                        self.status_register_1 |= 0x02;
                        self.cmd_buffer.clear();
                        0xFF
                    }
                    0x04 => { // WRDI: Write Disable
                        self.write_enable_latch = false;
                        self.status_register_1 &= !0x02;
                        self.cmd_buffer.clear();
                        0xFF
                    }
                    0x05 => { // RDSR1: Read Status Register 1
                        self.state = SpiFlashState::ReadingStatus1;
                        0xFF
                    }
                    0x35 => { // RDSR2: Read Status Register 2
                        self.state = SpiFlashState::ReadingStatus2;
                        0xFF
                    }
                    0x9F => { // RDID: Read JEDEC ID
                        self.state = SpiFlashState::ReadingJedecId { byte_idx: 0 };
                        0xFF
                    }
                    0x03 => { // READ: Normal Read (3 address bytes)
                        if self.cmd_buffer.len() == 4 {
                            self.current_addr = ((self.cmd_buffer[1] as usize) << 16)
                                | ((self.cmd_buffer[2] as usize) << 8)
                                | (self.cmd_buffer[3] as usize);
                            self.current_addr %= self.size_bytes;
                            self.state = SpiFlashState::ReadingData;
                        }
                        0xFF
                    }
                    0x0B => { // Fast Read (3 address bytes + 1 dummy byte)
                        if self.cmd_buffer.len() == 4 {
                            self.current_addr = ((self.cmd_buffer[1] as usize) << 16)
                                | ((self.cmd_buffer[2] as usize) << 8)
                                | (self.cmd_buffer[3] as usize);
                            self.current_addr %= self.size_bytes;
                            self.state = SpiFlashState::FastReadingData { dummy_cycles: 1 };
                        }
                        0xFF
                    }
                    0x20 => { // Sector Erase 4KB
                        if self.cmd_buffer.len() == 4 {
                            let addr = ((self.cmd_buffer[1] as usize) << 16)
                                | ((self.cmd_buffer[2] as usize) << 8)
                                | (self.cmd_buffer[3] as usize);
                            self.erase_sector(addr);
                            self.cmd_buffer.clear();
                        }
                        0xFF
                    }
                    0xD8 => { // Block Erase 64KB
                        if self.cmd_buffer.len() == 4 {
                            let addr = ((self.cmd_buffer[1] as usize) << 16)
                                | ((self.cmd_buffer[2] as usize) << 8)
                                | (self.cmd_buffer[3] as usize);
                            self.erase_block_64k(addr);
                            self.cmd_buffer.clear();
                        }
                        0xFF
                    }
                    0x02 => { // Page Program
                        if self.cmd_buffer.len() == 4 {
                            self.current_addr = ((self.cmd_buffer[1] as usize) << 16)
                                | ((self.cmd_buffer[2] as usize) << 8)
                                | (self.cmd_buffer[3] as usize);
                            self.current_addr %= self.size_bytes;
                            self.state = SpiFlashState::WritingPage;
                        }
                        0xFF
                    }
                    0xC7 | 0x60 => { // Chip Erase
                        self.erase_chip();
                        self.cmd_buffer.clear();
                        0xFF
                    }
                    _ => 0xFF,
                }
            }
            SpiFlashState::ReadingStatus1 => self.status_register_1,
            SpiFlashState::ReadingStatus2 => self.status_register_2,
            SpiFlashState::ReadingJedecId { ref mut byte_idx } => {
                let id = match *byte_idx {
                    0 => 0xEF, // Manufacturer: Winbond
                    1 => 0x40, // Memory Type: SPI
                    2 => if self.size_bytes >= 16 * 1024 * 1024 { 0x18 } else { 0x17 }, // Capacity: 128Mbit or 64Mbit
                    _ => 0x00,
                };
                *byte_idx += 1;
                id
            }
            SpiFlashState::ReadingData => {
                let val = self.memory[self.current_addr];
                self.current_addr = (self.current_addr + 1) % self.size_bytes;
                val
            }
            SpiFlashState::FastReadingData { ref mut dummy_cycles } => {
                if *dummy_cycles > 0 {
                    *dummy_cycles -= 1;
                    0xFF
                } else {
                    let val = self.memory[self.current_addr];
                    self.current_addr = (self.current_addr + 1) % self.size_bytes;
                    val
                }
            }
            SpiFlashState::WritingPage => {
                if self.write_enable_latch {
                    // Flash bits can only transition 1 -> 0 without erase
                    self.memory[self.current_addr] &= byte;
                    let page_base = self.current_addr & !(self.page_size_bytes - 1);
                    let page_offset = (self.current_addr + 1) % self.page_size_bytes;
                    self.current_addr = page_base + page_offset;
                }
                0xFF
            }
            SpiFlashState::ReadingSfdp { ref mut addr, ref mut dummy_cycles } => {
                if *dummy_cycles > 0 {
                    *dummy_cycles -= 1;
                    0xFF
                } else {
                    let val = if *addr < 4 { b"SFDP"[*addr] } else { 0xFF };
                    *addr += 1;
                    val
                }
            }
        }
    }
}
