//! xSignals & High-Speed Matched Length Group Manager.
//!
//! Propagates electrical signal paths across series passives (termination resistors,
//! AC coupling capacitors) and computes aggregate delay and length matching statistics.

use crate::geometry::rtree::NetId;
use oxide_physics::Microns;

/// An xSignal represents a logical high-speed connection spanning across multiple sub-nets
/// through series passives (e.g. `U1.TX -> R1 -> C1 -> J1.RX`).
#[derive(Debug, Clone, PartialEq)]
pub struct XSignal {
    pub name: String,
    pub sub_nets: Vec<NetId>,
    pub total_length: Microns,
    pub total_delay_ps: f64,
    pub package_delay_ps: f64,
}

impl XSignal {
    pub fn new(name: &str, sub_nets: Vec<NetId>) -> Self {
        Self {
            name: name.to_string(),
            sub_nets,
            total_length: 0,
            total_delay_ps: 0.0,
            package_delay_ps: 0.0,
        }
    }

    pub fn total_electrical_delay_ps(&self) -> f64 {
        self.total_delay_ps + self.package_delay_ps
    }
}

/// Matched length group for high-speed buses (e.g. DDR5 Byte 0: DQ0-DQ7, DQS_P/N).
#[derive(Debug, Clone, PartialEq)]
pub struct MatchedGroup {
    pub name: String,
    pub signals: Vec<XSignal>,
    pub target_length: Option<Microns>,
    pub tolerance: Microns,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchedGroupStatus {
    pub min_length: Microns,
    pub max_length: Microns,
    pub delta: Microns,
    pub within_tolerance: bool,
    pub skew_ps: f64,
}

impl MatchedGroup {
    pub fn new(name: &str, tolerance: Microns) -> Self {
        Self {
            name: name.to_string(),
            signals: Vec::new(),
            target_length: None,
            tolerance,
        }
    }

    pub fn add_signal(&mut self, signal: XSignal) {
        self.signals.push(signal);
    }

    pub fn evaluate_status(&self, ps_per_mm: f64) -> MatchedGroupStatus {
        if self.signals.is_empty() {
            return MatchedGroupStatus {
                min_length: 0,
                max_length: 0,
                delta: 0,
                within_tolerance: true,
                skew_ps: 0.0,
            };
        }

        let mut min_l = self.signals[0].total_length;
        let mut max_l = self.signals[0].total_length;

        for s in &self.signals {
            if s.total_length < min_l {
                min_l = s.total_length;
            }
            if s.total_length > max_l {
                max_l = s.total_length;
            }
        }

        let delta = max_l - min_l;
        let within_tolerance = delta <= self.tolerance;
        let delta_mm = delta as f64 / 1000.0;
        let skew_ps = delta_mm * ps_per_mm;

        MatchedGroupStatus {
            min_length: min_l,
            max_length: max_l,
            delta,
            within_tolerance,
            skew_ps,
        }
    }
}
