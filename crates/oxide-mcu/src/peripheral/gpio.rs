//! Multi-Port General Purpose I/O (GPIO) Emulation.

use serde::{Deserialize, Serialize};
use crate::pin_bridge::LogicLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GpioMode {
    #[default]
    Input,
    Output,
    AlternateFunction(u8),
    Analog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GpioType {
    #[default]
    PushPull,
    OpenDrain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GpioSpeed {
    Low,
    #[default]
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GpioPull {
    #[default]
    NoPull,
    PullUp,
    PullDown,
}

/// State of a single GPIO pin inside a port.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct GpioPinState {
    pub mode: GpioMode,
    pub otype: GpioType,
    pub speed: GpioSpeed,
    pub pull: GpioPull,
    pub output_data: LogicLevel,
    pub input_data: LogicLevel,
    pub voltage: f64,
}

/// 16-pin GPIO Port (e.g. GPIOA, GPIOB, GPIOC...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpioPort {
    pub name: String,
    pub pins: [GpioPinState; 16],
}

impl GpioPort {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pins: [GpioPinState::default(); 16],
        }
    }

    pub fn set_mode(&mut self, pin_idx: usize, mode: GpioMode) {
        if pin_idx < 16 {
            self.pins[pin_idx].mode = mode;
        }
    }

    pub fn set_pin_output(&mut self, pin_idx: usize, level: LogicLevel) {
        if pin_idx < 16 {
            self.pins[pin_idx].output_data = level;
            self.pins[pin_idx].voltage = match level {
                LogicLevel::Low => 0.0,
                LogicLevel::High => 3.3,
                LogicLevel::HighImpedance => 0.0,
            };
        }
    }

    /// Atomic Bit Set / Reset Register (BSRR) write emulation.
    /// Bits 0..15 set the pin High, bits 16..31 reset the pin Low.
    pub fn write_bsrr(&mut self, bsrr_val: u32) {
        // First handle Bit Reset (high 16 bits)
        let reset_mask = (bsrr_val >> 16) as u16;
        for i in 0..16 {
            if (reset_mask & (1 << i)) != 0 {
                self.set_pin_output(i, LogicLevel::Low);
            }
        }

        // Then handle Bit Set (low 16 bits)
        let set_mask = (bsrr_val & 0xFFFF) as u16;
        for i in 0..16 {
            if (set_mask & (1 << i)) != 0 {
                self.set_pin_output(i, LogicLevel::High);
            }
        }
    }

    /// Read Input Data Register (IDR) as 16-bit integer.
    pub fn read_idr(&self) -> u16 {
        let mut val = 0u16;
        for (i, pin) in self.pins.iter().enumerate() {
            if pin.input_data == LogicLevel::High {
                val |= 1 << i;
            }
        }
        val
    }

    /// Read Output Data Register (ODR) as 16-bit integer.
    pub fn read_odr(&self) -> u16 {
        let mut val = 0u16;
        for (i, pin) in self.pins.iter().enumerate() {
            if pin.output_data == LogicLevel::High {
                val |= 1 << i;
            }
        }
        val
    }
}
