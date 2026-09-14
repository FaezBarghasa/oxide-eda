//! Virtual Pin & Peripheral Synchronization Bridge.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Digital Pin Logic Level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicLevel {
    Low,
    High,
    HighImpedance,
}

impl LogicLevel {
    pub fn to_voltage(&self, vdd: f64) -> f64 {
        match self {
            LogicLevel::Low => 0.0,
            LogicLevel::High => vdd,
            LogicLevel::HighImpedance => 0.0,
        }
    }

    pub fn from_voltage(voltage: f64, v_threshold: f64) -> Self {
        if voltage >= v_threshold {
            LogicLevel::High
        } else {
            LogicLevel::Low
        }
    }
}

/// Direction and function of a virtual MCU pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinFunction {
    GpioInput,
    GpioOutput,
    AdcInput { channel: u8 },
    DacOutput { channel: u8 },
    PwmOutput { channel: u8 },
    UartTx,
    UartRx,
    SpiMosi,
    SpiMiso,
    SpiSck,
    SpiCs,
    I2cSda,
    I2cScl,
}

/// State of a single virtual MCU pin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualPinState {
    pub pin_name: String,
    pub function: PinFunction,
    pub level: LogicLevel,
    pub analog_voltage: f64,
    pub pwm_duty_cycle: f64,
    pub connected_net: Option<String>,
}

impl VirtualPinState {
    pub fn new(pin_name: impl Into<String>, function: PinFunction) -> Self {
        Self {
            pin_name: pin_name.into(),
            function,
            level: LogicLevel::Low,
            analog_voltage: 0.0,
            pwm_duty_cycle: 0.0,
            connected_net: None,
        }
    }
}

/// Pin Bridge managing synchronization between QEMU virtual peripherals and SPICE schematic nets.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PinBridge {
    pub vdd_voltage: f64,
    pub logic_threshold_v: f64,
    pub pins: HashMap<String, VirtualPinState>,
}

impl PinBridge {
    pub fn new(vdd_voltage: f64) -> Self {
        Self {
            vdd_voltage,
            logic_threshold_v: vdd_voltage * 0.5,
            pins: HashMap::new(),
        }
    }

    pub fn register_pin(&mut self, pin_name: impl Into<String>, function: PinFunction, connected_net: Option<String>) {
        let name = pin_name.into();
        let mut pin = VirtualPinState::new(&name, function);
        pin.connected_net = connected_net;
        self.pins.insert(name, pin);
    }

    /// Sets virtual GPIO level from MCU firmware.
    pub fn set_gpio_output(&mut self, pin_name: &str, level: LogicLevel) {
        if let Some(pin) = self.pins.get_mut(pin_name) {
            pin.level = level;
            pin.analog_voltage = level.to_voltage(self.vdd_voltage);
        }
    }

    /// Updates pin voltage sampled from the SPICE analog engine.
    pub fn update_from_spice_voltage(&mut self, pin_name: &str, voltage: f64) {
        if let Some(pin) = self.pins.get_mut(pin_name) {
            pin.analog_voltage = voltage;
            pin.level = LogicLevel::from_voltage(voltage, self.logic_threshold_v);
        }
    }

    /// Converts analog voltage into 12-bit ADC raw integer count (0..4095).
    pub fn read_adc_raw_12bit(&self, pin_name: &str) -> Option<u16> {
        let pin = self.pins.get(pin_name)?;
        let frac = (pin.analog_voltage / self.vdd_voltage).clamp(0.0, 1.0);
        Some((frac * 4095.0).round() as u16)
    }
}
