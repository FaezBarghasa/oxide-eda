//! Universal Microcontroller Architecture Abstraction & Multi-Architecture Core Profiles.

use serde::{Deserialize, Serialize};

/// Target MCU Architecture Class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArchClass {
    /// ARM (Cortex-M0, M0+, M1, M3, M4, M7, M23, M33, M55, M85, Cortex-R, Cortex-A)
    Arm,
    /// Espressif Xtensa (32-bit LX6, LX7 Windowed Register Core)
    Xtensa,
    /// RISC-V (RV32I, RV32E, RV32EC, RV32IMC, RV32IMAC, RV32IMAFDC)
    RiscV,
    /// Microchip / Atmel AVR 8-bit (ATmega, ATtiny, AVR-Dx)
    Avr8,
    /// Microchip PIC 8-bit (PIC12, PIC16, PIC18)
    Pic8,
    /// Microchip PIC 16-bit & dsPIC (PIC24F, dsPIC33 DSC)
    Pic16,
    /// Microchip PIC 32-bit (PIC32MX, PIC32MZ MIPS microAptiv)
    Pic32,
    /// Texas Instruments MSP430 (16-bit Ultra-Low Power RISC)
    Msp430,
    /// Texas Instruments C2000 (32-bit Real-Time DSP/MCU TMS320F28x)
    C2000,
}

/// Target Microcontroller Silicon Vendor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum McuVendor {
    Espressif,
    StMicroelectronics,
    MicrochipAtmel,
    NxpSemiconductors,
    RaspberryPi,
    Wch,
    TexasInstruments,
    NordicSemiconductor,
    Generic,
}

/// Memory Architecture Model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryModel {
    /// Unified 32/64-bit Von Neumann address space (ARM, RISC-V, Xtensa, PIC32)
    VonNeumann32,
    /// Split Harvard Architecture with independent Program Flash and Data SRAM (AVR 8-bit, PIC 8/16-bit)
    HarvardSplit {
        flash_word_size_bytes: usize,
        sram_size_bytes: usize,
        eeprom_size_bytes: usize,
    },
}

/// Universal MCU Core Profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreProfile {
    pub name: String,
    pub arch: ArchClass,
    pub vendor: McuVendor,
    pub bit_width: u8,
    pub memory_model: MemoryModel,
    pub num_cores: u8,
    pub max_frequency_hz: u64,
    pub qemu_executable: &'static str,
    pub qemu_cpu: &'static str,
    pub qemu_machine: &'static str,
}
