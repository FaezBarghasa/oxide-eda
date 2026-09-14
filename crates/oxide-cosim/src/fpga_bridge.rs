//! FPGA Digital Logic & HDL Co-Simulation Bridge (Verilator / VPI / GHDL IPC).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Digital Logic State for FPGA signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalLogicState {
    Zero,
    One,
    HighZ,
    Undefined,
}

/// FPGA Pin Definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpgaPin {
    pub name: String,
    pub state: DigitalLogicState,
    pub is_output: bool,
    pub tied_net: Option<String>,
}

/// Cycle-accurate FPGA Simulation Bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpgaBridge {
    pub name: String,
    pub target_device: String, // e.g. "Xilinx Artix-7", "Lattice iCE40", "Intel Cyclone V"
    pub clock_freq_hz: u64,
    pub current_cycle: u64,
    pub pins: HashMap<String, FpgaPin>,
    pub memory_mapped_regs: HashMap<u32, u32>,
}

impl FpgaBridge {
    pub fn new(name: &str, target_device: &str, clock_freq_hz: u64) -> Self {
        Self {
            name: name.to_string(),
            target_device: target_device.to_string(),
            clock_freq_hz,
            current_cycle: 0,
            pins: HashMap::new(),
            memory_mapped_regs: HashMap::new(),
        }
    }

    /// Register a named FPGA pin.
    pub fn register_pin(&mut self, name: &str, is_output: bool, tied_net: Option<String>) {
        self.pins.insert(
            name.to_string(),
            FpgaPin {
                name: name.to_string(),
                state: DigitalLogicState::Zero,
                is_output,
                tied_net,
            },
        );
    }

    /// Step simulation clock cycles.
    pub fn step_cycles(&mut self, cycles: u64) {
        self.current_cycle += cycles;
    }

    /// Read AXI/Wishbone register bus from MCU side.
    pub fn read_register(&self, offset: u32) -> u32 {
        self.memory_mapped_regs.get(&offset).copied().unwrap_or(0x0000_0000)
    }

    /// Write AXI/Wishbone register bus from MCU side.
    pub fn write_register(&mut self, offset: u32, val: u32) {
        self.memory_mapped_regs.insert(offset, val);
    }
}
