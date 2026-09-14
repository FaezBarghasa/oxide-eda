//! Firmware Image Ingestion & Multi-Architecture Profiling (ESP32, AVR, PIC, RISC-V, ARM, NXP, TI).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::arch::{ArchClass, CoreProfile, McuVendor, MemoryModel};

#[derive(Debug, Error)]
pub enum FirmwareError {
    #[error("I/O error reading firmware: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid ELF magic header")]
    InvalidElfHeader,
    #[error("Unsupported target architecture: {0}")]
    UnsupportedArch(String),
    #[error("Firmware section parse failed: {0}")]
    ParseError(String),
}

/// ARM Architecture Specification Profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmArch {
    ArmV6M,
    ArmV7M,
    ArmV7EM,
    ArmV8MBaseline,
    ArmV8MMainline,
    ArmV81MMainline,
    ArmV7R,
    ArmV8R,
    ArmV8A,
}

/// Specific ARM Processor Core.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmCore {
    CortexM0,
    CortexM0Plus,
    CortexM1,
    CortexM3,
    CortexM4,
    CortexM4F,
    CortexM7,
    CortexM23,
    CortexM33,
    CortexM35P,
    CortexM55,
    CortexM85,
    CortexR4,
    CortexR5,
    CortexR52,
    CortexA7,
    CortexA53,
}

impl ArmCore {
    pub fn arch(&self) -> ArmArch {
        match self {
            ArmCore::CortexM0 | ArmCore::CortexM0Plus | ArmCore::CortexM1 => ArmArch::ArmV6M,
            ArmCore::CortexM3 => ArmArch::ArmV7M,
            ArmCore::CortexM4 | ArmCore::CortexM4F | ArmCore::CortexM7 => ArmArch::ArmV7EM,
            ArmCore::CortexM23 => ArmArch::ArmV8MBaseline,
            ArmCore::CortexM33 | ArmCore::CortexM35P => ArmArch::ArmV8MMainline,
            ArmCore::CortexM55 | ArmCore::CortexM85 => ArmArch::ArmV81MMainline,
            ArmCore::CortexR4 | ArmCore::CortexR5 => ArmArch::ArmV7R,
            ArmCore::CortexR52 => ArmArch::ArmV8R,
            ArmCore::CortexA7 | ArmCore::CortexA53 => ArmArch::ArmV8A,
        }
    }

    pub fn qemu_cpu(&self) -> &'static str {
        match self {
            ArmCore::CortexM0 => "cortex-m0",
            ArmCore::CortexM0Plus => "cortex-m0plus",
            ArmCore::CortexM1 => "cortex-m1",
            ArmCore::CortexM3 => "cortex-m3",
            ArmCore::CortexM4 => "cortex-m4",
            ArmCore::CortexM4F => "cortex-m4",
            ArmCore::CortexM7 => "cortex-m7",
            ArmCore::CortexM23 => "cortex-m23",
            ArmCore::CortexM33 => "cortex-m33",
            ArmCore::CortexM35P => "cortex-m35p",
            ArmCore::CortexM55 => "cortex-m55",
            ArmCore::CortexM85 => "cortex-m85",
            ArmCore::CortexR4 => "cortex-r4",
            ArmCore::CortexR5 => "cortex-r5",
            ArmCore::CortexR52 => "cortex-r52",
            ArmCore::CortexA7 => "cortex-a7",
            ArmCore::CortexA53 => "cortex-a53",
        }
    }
}

/// Universal Target Microcontroller Identifier across all vendors & architectures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum McuTarget {
    // Espressif (Xtensa & RISC-V)
    Esp32,
    Esp32S2,
    Esp32S3,
    Esp32C2,
    Esp32C3,
    Esp32C6,
    Esp32H2,
    Esp32P4,

    // Microchip / Atmel AVR 8-bit
    ATmega328P,
    ATmega2560,
    ATmega32U4,
    ATtiny85,
    ATtiny1614,
    AvrDx,

    // Microchip PIC 8-bit, 16-bit, 32-bit
    Pic16F877A,
    Pic18F4550,
    DsPic33E,
    Pic32MX,
    Pic32MZ,

    // RISC-V General & WCH
    Ch32V003,
    Ch32V203,
    Ch32V307,
    SiFiveFe310,
    Rp2350RiscV,

    // Texas Instruments
    Msp430G2553,
    Msp430F5529,
    C2000Tms320F28379D,

    // STMicroelectronics STM32
    Stm32F0,
    Stm32G0,
    Stm32C0,
    Stm32F1,
    Stm32F2,
    Stm32L1,
    Stm32F3,
    Stm32F4,
    Stm32G4,
    Stm32L4,
    Stm32F7,
    Stm32H7,
    Stm32L5,
    Stm32U5,

    // Raspberry Pi
    Rp2040,
    Rp2350Arm,

    // Nordic Semiconductor
    Nrf51,
    Nrf52,
    Nrf53,

    // NXP Semiconductors
    NxpLpc800,
    NxpLpc1700,
    NxpLpc5500,
    NxpImxRt1060,
    NxpS32K144,
    NxpKinetisK64,

    // Microchip SAM (ARM)
    SamD21,
    SamE54,

    // Texas Instruments (ARM)
    TiMspm0,
    TiTm4c123,

    // Generic Fallback
    GenericArm(ArmCore),
    GenericRiscV,
    GenericAvr8,
}

impl McuTarget {
    /// Returns the comprehensive architectural profile for this target.
    pub fn profile(&self) -> CoreProfile {
        match self {
            // Espressif Xtensa
            McuTarget::Esp32 => CoreProfile {
                name: "ESP32 (Dual Xtensa LX6)".to_string(),
                arch: ArchClass::Xtensa,
                vendor: McuVendor::Espressif,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 2,
                max_frequency_hz: 240_000_000,
                qemu_executable: "qemu-system-xtensa",
                qemu_cpu: "esp32",
                qemu_machine: "esp32",
            },
            McuTarget::Esp32S2 => CoreProfile {
                name: "ESP32-S2 (Single Xtensa LX7)".to_string(),
                arch: ArchClass::Xtensa,
                vendor: McuVendor::Espressif,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 240_000_000,
                qemu_executable: "qemu-system-xtensa",
                qemu_cpu: "esp32s2",
                qemu_machine: "esp32s2",
            },
            McuTarget::Esp32S3 => CoreProfile {
                name: "ESP32-S3 (Dual Xtensa LX7)".to_string(),
                arch: ArchClass::Xtensa,
                vendor: McuVendor::Espressif,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 2,
                max_frequency_hz: 240_000_000,
                qemu_executable: "qemu-system-xtensa",
                qemu_cpu: "esp32s3",
                qemu_machine: "esp32s3",
            },
            // Espressif RISC-V
            McuTarget::Esp32C2 | McuTarget::Esp32C3 | McuTarget::Esp32C6 | McuTarget::Esp32H2 | McuTarget::Esp32P4 => CoreProfile {
                name: "ESP32-C Series (RISC-V RV32IMC)".to_string(),
                arch: ArchClass::RiscV,
                vendor: McuVendor::Espressif,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 160_000_000,
                qemu_executable: "qemu-system-riscv32",
                qemu_cpu: "rv32",
                qemu_machine: "virt",
            },
            // Microchip AVR
            McuTarget::ATmega328P => CoreProfile {
                name: "ATmega328P (Arduino Uno)".to_string(),
                arch: ArchClass::Avr8,
                vendor: McuVendor::MicrochipAtmel,
                bit_width: 8,
                memory_model: MemoryModel::HarvardSplit {
                    flash_word_size_bytes: 32768,
                    sram_size_bytes: 2048,
                    eeprom_size_bytes: 1024,
                },
                num_cores: 1,
                max_frequency_hz: 16_000_000,
                qemu_executable: "qemu-system-avr",
                qemu_cpu: "avr5",
                qemu_machine: "uno",
            },
            McuTarget::ATmega2560 => CoreProfile {
                name: "ATmega2560 (Arduino Mega)".to_string(),
                arch: ArchClass::Avr8,
                vendor: McuVendor::MicrochipAtmel,
                bit_width: 8,
                memory_model: MemoryModel::HarvardSplit {
                    flash_word_size_bytes: 262144,
                    sram_size_bytes: 8192,
                    eeprom_size_bytes: 4096,
                },
                num_cores: 1,
                max_frequency_hz: 16_000_000,
                qemu_executable: "qemu-system-avr",
                qemu_cpu: "avr6",
                qemu_machine: "mega2560",
            },
            McuTarget::ATtiny85 | McuTarget::ATtiny1614 | McuTarget::ATmega32U4 | McuTarget::AvrDx => CoreProfile {
                name: "AVR 8-bit Microcontroller".to_string(),
                arch: ArchClass::Avr8,
                vendor: McuVendor::MicrochipAtmel,
                bit_width: 8,
                memory_model: MemoryModel::HarvardSplit {
                    flash_word_size_bytes: 8192,
                    sram_size_bytes: 512,
                    eeprom_size_bytes: 512,
                },
                num_cores: 1,
                max_frequency_hz: 20_000_000,
                qemu_executable: "qemu-system-avr",
                qemu_cpu: "avr5",
                qemu_machine: "uno",
            },
            // Microchip PIC
            McuTarget::Pic16F877A | McuTarget::Pic18F4550 => CoreProfile {
                name: "PIC 8-bit Microcontroller".to_string(),
                arch: ArchClass::Pic8,
                vendor: McuVendor::MicrochipAtmel,
                bit_width: 8,
                memory_model: MemoryModel::HarvardSplit {
                    flash_word_size_bytes: 16384,
                    sram_size_bytes: 2048,
                    eeprom_size_bytes: 256,
                },
                num_cores: 1,
                max_frequency_hz: 48_000_000,
                qemu_executable: "qemu-system-misc",
                qemu_cpu: "pic18",
                qemu_machine: "pic-board",
            },
            McuTarget::DsPic33E => CoreProfile {
                name: "dsPIC33E 16-bit DSC".to_string(),
                arch: ArchClass::Pic16,
                vendor: McuVendor::MicrochipAtmel,
                bit_width: 16,
                memory_model: MemoryModel::HarvardSplit {
                    flash_word_size_bytes: 131072,
                    sram_size_bytes: 16384,
                    eeprom_size_bytes: 0,
                },
                num_cores: 1,
                max_frequency_hz: 70_000_000,
                qemu_executable: "qemu-system-misc",
                qemu_cpu: "dspic33",
                qemu_machine: "dspic-board",
            },
            McuTarget::Pic32MX | McuTarget::Pic32MZ => CoreProfile {
                name: "PIC32 32-bit MIPS Microcontroller".to_string(),
                arch: ArchClass::Pic32,
                vendor: McuVendor::MicrochipAtmel,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 200_000_000,
                qemu_executable: "qemu-system-mipsel",
                qemu_cpu: "mips32r2",
                qemu_machine: "pic32_board",
            },
            // RISC-V General & WCH
            McuTarget::Ch32V003 => CoreProfile {
                name: "WCH CH32V003 (RV32EC)".to_string(),
                arch: ArchClass::RiscV,
                vendor: McuVendor::Wch,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 48_000_000,
                qemu_executable: "qemu-system-riscv32",
                qemu_cpu: "rv32",
                qemu_machine: "virt",
            },
            McuTarget::Ch32V203 | McuTarget::Ch32V307 | McuTarget::SiFiveFe310 | McuTarget::Rp2350RiscV => CoreProfile {
                name: "RISC-V 32-bit Microcontroller".to_string(),
                arch: ArchClass::RiscV,
                vendor: McuVendor::Wch,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 144_000_000,
                qemu_executable: "qemu-system-riscv32",
                qemu_cpu: "rv32",
                qemu_machine: "sifive_e",
            },
            // Texas Instruments
            McuTarget::Msp430G2553 | McuTarget::Msp430F5529 => CoreProfile {
                name: "TI MSP430 16-bit RISC".to_string(),
                arch: ArchClass::Msp430,
                vendor: McuVendor::TexasInstruments,
                bit_width: 16,
                memory_model: MemoryModel::HarvardSplit {
                    flash_word_size_bytes: 32768,
                    sram_size_bytes: 2048,
                    eeprom_size_bytes: 0,
                },
                num_cores: 1,
                max_frequency_hz: 25_000_000,
                qemu_executable: "qemu-system-misc",
                qemu_cpu: "msp430",
                qemu_machine: "msp430_board",
            },
            McuTarget::C2000Tms320F28379D => CoreProfile {
                name: "TI C2000 TMS320F28379D Real-Time Dual Core".to_string(),
                arch: ArchClass::C2000,
                vendor: McuVendor::TexasInstruments,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 2,
                max_frequency_hz: 200_000_000,
                qemu_executable: "qemu-system-misc",
                qemu_cpu: "c28x",
                qemu_machine: "c2000_board",
            },
            // STMicroelectronics STM32 (ARM Cortex-M)
            McuTarget::Stm32F0 => CoreProfile {
                name: "STM32F0 (Cortex-M0)".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::StMicroelectronics,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 48_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m0",
                qemu_machine: "microbit",
            },
            McuTarget::Stm32F1 => CoreProfile {
                name: "STM32F1 (Cortex-M3)".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::StMicroelectronics,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 72_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m3",
                qemu_machine: "stm32vldiscovery",
            },
            McuTarget::Stm32F4 => CoreProfile {
                name: "STM32F4 (Cortex-M4F)".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::StMicroelectronics,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 168_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m4",
                qemu_machine: "netduinoplus2",
            },
            McuTarget::Stm32F7 | McuTarget::Stm32H7 => CoreProfile {
                name: "STM32F7/H7 (Cortex-M7)".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::StMicroelectronics,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 480_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m7",
                qemu_machine: "mps2-an500",
            },
            McuTarget::Stm32L5 | McuTarget::Stm32U5 => CoreProfile {
                name: "STM32L5/U5 (Cortex-M33 TrustZone)".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::StMicroelectronics,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 160_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m33",
                qemu_machine: "mps2-an505",
            },
            // Raspberry Pi RP2040 / RP2350
            McuTarget::Rp2040 => CoreProfile {
                name: "Raspberry Pi RP2040 (Dual Cortex-M0+)".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::RaspberryPi,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 2,
                max_frequency_hz: 133_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m0plus",
                qemu_machine: "microbit",
            },
            McuTarget::Rp2350Arm => CoreProfile {
                name: "Raspberry Pi RP2350 (Dual Cortex-M33)".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::RaspberryPi,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 2,
                max_frequency_hz: 150_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m33",
                qemu_machine: "mps2-an505",
            },
            // NXP
            McuTarget::NxpS32K144 => CoreProfile {
                name: "NXP S32K144 Automotive (Cortex-M4F)".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::NxpSemiconductors,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 112_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m4",
                qemu_machine: "netduinoplus2",
            },
            McuTarget::NxpImxRt1060 => CoreProfile {
                name: "NXP i.MX RT1060 Crossover (Cortex-M7 600MHz)".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::NxpSemiconductors,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 600_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m7",
                qemu_machine: "mps2-an500",
            },
            _ => CoreProfile {
                name: "Generic 32-bit Microcontroller".to_string(),
                arch: ArchClass::Arm,
                vendor: McuVendor::Generic,
                bit_width: 32,
                memory_model: MemoryModel::VonNeumann32,
                num_cores: 1,
                max_frequency_hz: 100_000_000,
                qemu_executable: "qemu-system-arm",
                qemu_cpu: "cortex-m4",
                qemu_machine: "netduinoplus2",
            },
        }
    }
}

pub type McuFamily = McuTarget;

/// Memory segment extracted from firmware image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemorySegment {
    pub addr: u32,
    pub size: usize,
    pub name: String,
}

/// Parsed Firmware Metadata and verification summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirmwareImage {
    pub path: PathBuf,
    pub target: McuTarget,
    pub entry_point: u32,
    pub segments: Vec<MemorySegment>,
    pub total_size_bytes: usize,
}

impl FirmwareImage {
    /// Parses ELF header, Intel HEX, or raw binary.
    pub fn inspect(path: impl AsRef<Path>, target: McuTarget) -> Result<Self, FirmwareError> {
        let path_buf = path.as_ref().to_path_buf();
        let bytes = std::fs::read(&path_buf)?;

        if bytes.is_empty() {
            return Err(FirmwareError::ParseError("Firmware file is empty".to_string()));
        }

        // Check Intel HEX format (starts with ':')
        if bytes[0] == b':' {
            return Ok(Self {
                path: path_buf,
                target,
                entry_point: 0,
                segments: vec![MemorySegment {
                    addr: 0,
                    size: bytes.len(),
                    name: ".hex".to_string(),
                }],
                total_size_bytes: bytes.len(),
            });
        }

        // Check ELF header
        if bytes.len() >= 52 && &bytes[0..4] == b"\x7fELF" {
            let entry_point = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);
            return Ok(Self {
                path: path_buf,
                target,
                entry_point,
                segments: vec![MemorySegment {
                    addr: entry_point & !0x3FF,
                    size: bytes.len(),
                    name: ".text".to_string(),
                }],
                total_size_bytes: bytes.len(),
            });
        }

        // Raw Binary Fallback
        Ok(Self {
            path: path_buf,
            target,
            entry_point: 0x0800_0000,
            segments: vec![MemorySegment {
                addr: 0x0800_0000,
                size: bytes.len(),
                name: ".bin".to_string(),
            }],
            total_size_bytes: bytes.len(),
        })
    }
}
