//! Structured Signal Harness Architecture.
//!
//! Conforms to Master Technical Directive §5.2:
//! Heterogeneous Signal Packaging: Signal harnesses combine differential pairs, buses,
//! and single-ended control nets into a unified, strongly-typed logical connection bundle.

use serde::{Deserialize, Serialize};

/// Signal element contained within a structured signal harness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HarnessElement {
    /// Single-ended net (e.g. `CLK`, `RESET_N`)
    Net(String),
    /// Multi-bit digital bus (e.g. `ADDR[15..0]`, `DATA[7..0]`)
    Bus {
        name: String,
        msb: u32,
        lsb: u32,
    },
    /// Tightly coupled differential pair (e.g. `D_P`, `D_N` or `TX_P`, `TX_N`)
    DiffPair {
        pair_name: String,
        pos_net: String,
        neg_net: String,
    },
}

/// A structured signal harness definition (`.Harness`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalHarness {
    pub name: String,
    pub elements: Vec<HarnessElement>,
}

impl SignalHarness {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            elements: Vec::new(),
        }
    }

    /// Adds a single-ended net to the harness.
    pub fn add_net(&mut self, net_name: &str) {
        self.elements.push(HarnessElement::Net(net_name.to_string()));
    }

    /// Adds a multi-bit bus to the harness.
    pub fn add_bus(&mut self, bus_name: &str, msb: u32, lsb: u32) {
        self.elements.push(HarnessElement::Bus {
            name: bus_name.to_string(),
            msb,
            lsb,
        });
    }

    /// Adds a differential pair to the harness.
    pub fn add_diff_pair(&mut self, pair_name: &str, pos_net: &str, neg_net: &str) {
        self.elements.push(HarnessElement::DiffPair {
            pair_name: pair_name.to_string(),
            pos_net: pos_net.to_string(),
            neg_net: neg_net.to_string(),
        });
    }

    /// Expands the entire harness into a flat list of constituent net names.
    pub fn expand_nets(&self) -> Vec<String> {
        let mut nets = Vec::new();
        for elem in &self.elements {
            match elem {
                HarnessElement::Net(n) => nets.push(n.clone()),
                HarnessElement::Bus { name, msb, lsb } => {
                    let start = (*msb).min(*lsb);
                    let end = (*msb).max(*lsb);
                    for i in start..=end {
                        nets.push(format!("{}[{}]", name, i));
                    }
                }
                HarnessElement::DiffPair { pos_net, neg_net, .. } => {
                    nets.push(pos_net.clone());
                    nets.push(neg_net.clone());
                }
            }
        }
        nets
    }

    /// Verifies interface compatibility between two connected harness ports.
    pub fn validate_compatibility(&self, other: &Self) -> Result<(), String> {
        let self_nets = self.expand_nets();
        let other_nets = other.expand_nets();

        if self_nets.len() != other_nets.len() {
            return Err(format!(
                "Harness size mismatch: '{}' has {} nets, but '{}' has {} nets",
                self.name,
                self_nets.len(),
                other.name,
                other_nets.len()
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_harness_expansion_and_validation() {
        let mut h1 = SignalHarness::new("USB3_LINK");
        h1.add_diff_pair("TX", "USB_TX_P", "USB_TX_N");
        h1.add_diff_pair("RX", "USB_RX_P", "USB_RX_N");
        h1.add_net("VBUS");
        h1.add_net("GND");

        let flat = h1.expand_nets();
        assert_eq!(flat.len(), 6);
        assert_eq!(flat, vec!["USB_TX_P", "USB_TX_N", "USB_RX_P", "USB_RX_N", "VBUS", "GND"]);

        let mut h2 = SignalHarness::new("USB3_LINK_MATCH");
        h2.add_diff_pair("TX", "TX_P", "TX_N");
        h2.add_diff_pair("RX", "RX_P", "RX_N");
        h2.add_net("POWER");
        h2.add_net("GROUND");

        assert!(h1.validate_compatibility(&h2).is_ok());
    }
}
