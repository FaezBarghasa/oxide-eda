//! Firmware Image Ingestion & Architecture Profiling for Virtual ARM MCUs.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

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

/// ARM Instruction Set Architecture (ISA) Class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmArch {
    /// ARMv6-M (Cortex-M0, Cortex-M0+, Cortex-M1)
    ArmV6M,
    /// ARMv7-M (Cortex-M3)
    ArmV7M,
    /// ARMv7E-M (Cortex-M4, Cortex-M4F, Cortex-M7)
    ArmV7EM,
    /// ARMv8-M Baseline (Cortex-M23)
    ArmV8MBaseline,
    /// ARMv8-M Mainline (Cortex-M33, Cortex-M35P)
    ArmV8MMainline,
    /// ARMv8.1-M Mainline with Helium MVE (Cortex-M55, Cortex-M85)
    ArmV81MMainline,
    /// ARMv7-R (Cortex-R4, Cortex-R5, Cortex-R7, Cortex-R8)
    ArmV7R,
    /// ARMv8-R (Cortex-R52)
    ArmV8R,
    /// ARMv7-A (Cortex-A7, Cortex-A9)
    ArmV7A,
    /// ARMv8-A 64-bit (Cortex-A53, Cortex-A72)
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
            ArmCore::CortexA7 => ArmArch::ArmV7A,
            ArmCore::CortexA53 => ArmArch::ArmV8A,
        }
    }

    pub fn qemu_cpu(&self) -> &'static str {
        match self {
            ArmCore::CortexM0 => "cortex-m0",
            ArmCore::CortexM0Plus => "cortex-m0plus",
            ArmCore::CortexM1 => "cortex-m0",
            ArmCore::CortexM3 => "cortex-m3",
            ArmCore::CortexM4 => "cortex-m4",
            ArmCore::CortexM4F => "cortex-m4",
            ArmCore::CortexM7 => "cortex-m7",
            ArmCore::CortexM23 => "cortex-m23",
            ArmCore::CortexM33 | ArmCore::CortexM35P => "cortex-m33",
            ArmCore::CortexM55 => "cortex-m55",
            ArmCore::CortexM85 => "cortex-m85",
            ArmCore::CortexR4 => "cortex-r4",
            ArmCore::CortexR5 | ArmCore::CortexR52 => "cortex-r5",
            ArmCore::CortexA7 => "cortex-a7",
            ArmCore::CortexA53 => "cortex-a53",
        }
    }
}

/// Target Microcontroller Family & SoC Preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum McuFamily {
    // STMicroelectronics
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
    Rp2350,
    // Nordic Semiconductor
    Nrf51,
    Nrf52,
    Nrf53,
    // NXP Semiconductors
    NxpLpc800,
    NxpLpc1700,
    NxpLpc5500,
    NxpImxRt1060,
    // Microchip / Atmel
    SamD21,
    SamE54,
    // Texas Instruments
    TiMspm0,
    TiTm4c123,
    // Generic
    GenericArm(ArmCore),
}

impl McuFamily {
    pub fn core(&self) -> ArmCore {
        match self {
            McuFamily::Stm32F0 => ArmCore::CortexM0,
            McuFamily::Stm32G0 | McuFamily::Stm32C0 | McuFamily::Rp2040 | McuFamily::NxpLpc800 | McuFamily::SamD21 | McuFamily::TiMspm0 => ArmCore::CortexM0Plus,
            McuFamily::Nrf51 => ArmCore::CortexM0,
            McuFamily::Stm32F1 | McuFamily::Stm32F2 | McuFamily::Stm32L1 | McuFamily::NxpLpc1700 => ArmCore::CortexM3,
            McuFamily::Stm32F3 | McuFamily::Stm32F4 | McuFamily::Stm32G4 | McuFamily::Stm32L4 | McuFamily::Nrf52 | McuFamily::SamE54 | McuFamily::TiTm4c123 => ArmCore::CortexM4F,
            McuFamily::Stm32F7 | McuFamily::Stm32H7 | McuFamily::NxpImxRt1060 => ArmCore::CortexM7,
            McuFamily::Stm32L5 | McuFamily::Stm32U5 | McuFamily::Rp2350 | McuFamily::Nrf53 | McuFamily::NxpLpc5500 => ArmCore::CortexM33,
            McuFamily::GenericArm(core) => *core,
        }
    }

    pub fn qemu_cpu(&self) -> &'static str {
        self.core().qemu_cpu()
    }

    pub fn qemu_machine(&self) -> &'static str {
        match self {
            McuFamily::Nrf51 => "microbit",
            McuFamily::Stm32F1 => "stm32vldiscovery",
            McuFamily::Stm32F4 | McuFamily::Stm32F3 | McuFamily::Stm32G4 | McuFamily::Stm32L4 => "netduinoplus2",
            McuFamily::Stm32F7 | McuFamily::Stm32H7 | McuFamily::NxpImxRt1060 => "mps2-an500",
            McuFamily::Stm32L5 | McuFamily::Stm32U5 | McuFamily::Rp2350 | McuFamily::NxpLpc5500 => "mps2-an505",
            McuFamily::Rp2040 | McuFamily::Stm32F0 | McuFamily::Stm32G0 | McuFamily::NxpLpc800 | McuFamily::SamD21 | McuFamily::TiMspm0 => "microbit",
            McuFamily::GenericArm(core) => match core.arch() {
                ArmArch::ArmV6M => "microbit",
                ArmArch::ArmV7M => "stm32vldiscovery",
                ArmArch::ArmV7EM => "netduinoplus2",
                ArmArch::ArmV8MBaseline => "mps2-an519",
                ArmArch::ArmV8MMainline => "mps2-an505",
                ArmArch::ArmV81MMainline => "mps3-an547",
                ArmArch::ArmV7R | ArmArch::ArmV8R => "virt",
                ArmArch::ArmV7A | ArmArch::ArmV8A => "virt",
            },
            _ => "netduinoplus2",
        }
    }

    pub fn default_flash_base(&self) -> u32 {
        match self {
            McuFamily::Rp2040 | McuFamily::Rp2350 => 0x1000_0000,
            McuFamily::SamD21 | McuFamily::SamE54 => 0x0000_0000,
            McuFamily::Nrf51 | McuFamily::Nrf52 | McuFamily::Nrf53 => 0x0000_0000,
            _ => 0x0800_0000, // Standard STM32 / ARM Flash base
        }
    }

    pub fn default_sram_base(&self) -> u32 {
        match self {
            McuFamily::Rp2040 | McuFamily::Rp2350 => 0x2000_0000,
            _ => 0x2000_0000, // Standard ARM SRAM base
        }
    }
}

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
    pub family: McuFamily,
    pub entry_point: u32,
    pub segments: Vec<MemorySegment>,
    pub total_size_bytes: usize,
}

impl FirmwareImage {
    /// Parses ELF header or raw binary to inspect entry point and architecture.
    pub fn inspect(path: impl AsRef<Path>, family: McuFamily) -> Result<Self, FirmwareError> {
        let path_buf = path.as_ref().to_path_buf();
        let bytes = std::fs::read(&path_buf)?;

        if bytes.len() < 52 {
            return Err(FirmwareError::InvalidElfHeader);
        }

        // Check ELF Magic (\x7fELF)
        if &bytes[0..4] != b"\x7fELF" {
            // Raw binary fallback
            let flash_base = family.default_flash_base();
            return Ok(Self {
                path: path_buf,
                family,
                entry_point: flash_base,
                segments: vec![MemorySegment {
                    addr: flash_base,
                    size: bytes.len(),
                    name: ".flash".to_string(),
                }],
                total_size_bytes: bytes.len(),
            });
        }

        // Parse 32-bit Little-Endian ELF header
        let is_32bit = bytes[4] == 1;
        let is_little_endian = bytes[5] == 1;

        if !is_32bit || !is_little_endian {
            return Err(FirmwareError::UnsupportedArch("Only 32-bit Little Endian ARM is supported".to_string()));
        }

        let entry_point = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);

        Ok(Self {
            path: path_buf,
            family,
            entry_point,
            segments: vec![MemorySegment {
                addr: entry_point & !0x3FF,
                size: bytes.len(),
                name: ".text".to_string(),
            }],
            total_size_bytes: bytes.len(),
        })
    }
}
