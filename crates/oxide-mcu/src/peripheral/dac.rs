//! Dual 12-bit Digital-to-Analog Converter (DAC) Emulation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DacWaveform {
    #[default]
    FlatDc,
    Triangle,
    Noise,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct DacChannelMode {
    pub enabled: bool,
    pub buffer_enabled: bool,
    pub waveform: DacWaveform,
    pub raw_data: u16,
    pub output_voltage: f64,
    pub output_impedance_ohms: f64,
}

/// Dual 12-bit DAC Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DacPeripheral {
    pub name: String,
    pub vref_voltage: f64,
    pub channels: [DacChannelMode; 2],
}

impl DacPeripheral {
    pub fn new(name: impl Into<String>, vref: f64) -> Self {
        Self {
            name: name.into(),
            vref_voltage: vref,
            channels: [
                DacChannelMode {
                    output_impedance_ohms: 15_000.0,
                    ..Default::default()
                },
                DacChannelMode {
                    output_impedance_ohms: 15_000.0,
                    ..Default::default()
                },
            ],
        }
    }

    pub fn write_channel_12bit(&mut self, channel_idx: usize, data_12bit: u16) {
        if channel_idx < 2 {
            let ch = &mut self.channels[channel_idx];
            let clamped = data_12bit & 0x0FFF;
            ch.raw_data = clamped;
            let frac = clamped as f64 / 4095.0;
            ch.output_voltage = frac * self.vref_voltage;
        }
    }

    pub fn set_buffer_enabled(&mut self, channel_idx: usize, enabled: bool) {
        if channel_idx < 2 {
            let ch = &mut self.channels[channel_idx];
            ch.buffer_enabled = enabled;
            ch.output_impedance_ohms = if enabled { 5.0 } else { 15_000.0 };
        }
    }

    pub fn get_output_voltage(&self, channel_idx: usize) -> f64 {
        if channel_idx < 2 && self.channels[channel_idx].enabled {
            self.channels[channel_idx].output_voltage
        } else {
            0.0
        }
    }
}
