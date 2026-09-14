//! Fast Analog Comparator (COMP) Emulation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ComparatorHysteresis {
    #[default]
    None,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ComparatorOutputPolarity {
    #[default]
    NotInverted,
    Inverted,
}

/// Analog Comparator Model.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComparatorPeripheral {
    pub name: String,
    pub enabled: bool,
    pub hysteresis: ComparatorHysteresis,
    pub polarity: ComparatorOutputPolarity,
    pub v_in_pos: f64,
    pub v_in_neg: f64,
    pub output_state: bool,
}

impl ComparatorPeripheral {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled: false,
            hysteresis: ComparatorHysteresis::None,
            polarity: ComparatorOutputPolarity::NotInverted,
            v_in_pos: 0.0,
            v_in_neg: 0.0,
            output_state: false,
        }
    }

    /// Evaluates analog voltages and computes digital comparator output.
    pub fn evaluate(&mut self) -> bool {
        if !self.enabled {
            self.output_state = false;
            return false;
        }

        let hyst_v = match self.hysteresis {
            ComparatorHysteresis::None => 0.0,
            ComparatorHysteresis::Low => 0.010,    // 10 mV
            ComparatorHysteresis::Medium => 0.020, // 20 mV
            ComparatorHysteresis::High => 0.030,   // 30 mV
        };

        let raw_cmp = if self.output_state {
            self.v_in_pos >= (self.v_in_neg - hyst_v)
        } else {
            self.v_in_pos > (self.v_in_neg + hyst_v)
        };

        self.output_state = match self.polarity {
            ComparatorOutputPolarity::NotInverted => raw_cmp,
            ComparatorOutputPolarity::Inverted => !raw_cmp,
        };

        self.output_state
    }
}
