//! Firmware Image Ingestion & Parsing for Virtual MCU Targets.

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

/// Target Microcontroller Family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum McuFamily {
    CortexM0,
    CortexM3,
    CortexM4F,
    CortexM7,
    Stm32F4,
    Stm32F7,
    GenericArm,
}

impl McuFamily {
    pub fn qemu_cpu(&self) -> &'static str {
        match self {
            McuFamily::CortexM0 => "cortex-m0",
            McuFamily::CortexM3 => "cortex-m3",
            McuFamily::CortexM4F | McuFamily::Stm32F4 => "cortex-m4",
            McuFamily::CortexM7 | McuFamily::Stm32F7 => "cortex-m7",
            McuFamily::GenericArm => "cortex-m4",
        }
    }

    pub fn qemu_machine(&self) -> &'static str {
        match self {
            McuFamily::Stm32F4 => "netduinoplus2",
            _ => "netduinoplus2",
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
            return Ok(Self {
                path: path_buf,
                family,
                entry_point: 0x0800_0000, // Standard STM32 flash base
                segments: vec![MemorySegment {
                    addr: 0x0800_0000,
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
