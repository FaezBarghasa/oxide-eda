//! External Storage Subsystem: EEPROM, SPI/QSPI Flash, SPI/QSPI RAM, and Parallel SRAM/SDRAM.

pub mod eeprom;
pub mod parallel_ram;
pub mod spi_flash;
pub mod spi_ram;

pub use eeprom::{I2cEeprom, SpiEeprom};
pub use parallel_ram::ParallelSram;
pub use spi_flash::SpiFlash;
pub use spi_ram::SpiRam;
