//! Multi-Channel Analog-to-Digital Converter (ADC) Emulation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AdcResolution {
    Bits8,
    Bits10,
    #[default]
    Bits12,
    Bits16,
}

impl AdcResolution {
    pub fn max_value(&self) -> u32 {
        match self {
            AdcResolution::Bits8 => 255,
            AdcResolution::Bits10 => 1023,
            AdcResolution::Bits12 => 4095,
            AdcResolution::Bits16 => 65535,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AdcConversionMode {
    #[default]
    Single,
    Continuous,
    Scan,
    Discontinuous,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct AdcChannelConfig {
    pub channel_number: u8,
    pub sampling_cycles: u32,
    pub input_voltage: f64,
}

/// Multi-Channel SAR ADC Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdcPeripheral {
    pub name: String,
    pub enabled: bool,
    pub resolution: AdcResolution,
    pub conversion_mode: AdcConversionMode,
    pub vref_voltage: f64,
    pub sequence: Vec<u8>,
    pub regular_data: u32,
    pub channels: Vec<AdcChannelConfig>,
    pub eoc_interrupt_enable: bool,
    pub watchdog_high_threshold: u32,
    pub watchdog_low_threshold: u32,
    pub watchdog_tripped: bool,
}

impl AdcPeripheral {
    pub fn new(name: impl Into<String>, vref: f64) -> Self {
        let mut channels = Vec::with_capacity(19);
        for ch in 0..=18 {
            channels.push(AdcChannelConfig {
                channel_number: ch,
                sampling_cycles: 15,
                input_voltage: 0.0,
            });
        }

        Self {
            name: name.into(),
            enabled: false,
            resolution: AdcResolution::Bits12,
            conversion_mode: AdcConversionMode::Single,
            vref_voltage: vref,
            sequence: vec![0],
            regular_data: 0,
            channels,
            eoc_interrupt_enable: false,
            watchdog_high_threshold: 4095,
            watchdog_low_threshold: 0,
            watchdog_tripped: false,
        }
    }

    pub fn set_channel_voltage(&mut self, channel: u8, voltage: f64) {
        if let Some(ch) = self.channels.get_mut(channel as usize) {
            ch.input_voltage = voltage;
        }
    }

    /// Performs ADC conversion for the selected channel in sequence.
    /// Returns converted integer count.
    pub fn convert_channel(&mut self, channel: u8) -> u32 {
        let voltage = self.channels.get(channel as usize).map(|c| c.input_voltage).unwrap_or(0.0);
        let max_val = self.resolution.max_value() as f64;
        let frac = (voltage / self.vref_voltage).clamp(0.0, 1.0);
        let raw = (frac * max_val).round() as u32;

        self.regular_data = raw;

        // Check Analog Watchdog
        if raw > self.watchdog_high_threshold || raw < self.watchdog_low_threshold {
            self.watchdog_tripped = true;
        }

        raw
    }

    /// Read raw ADC integer output.
    pub fn read_data(&mut self) -> u32 {
        self.regular_data
    }
}
