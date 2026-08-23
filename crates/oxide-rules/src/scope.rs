//! Rule scoping and hierarchical specificity evaluation for constraint matching.

use serde::{Deserialize, Serialize};

/// Hierarchical scope where a design constraint applies.
///
/// Order of evaluation and specificity:
/// `Net` (highest, 4) > `NetClass` (3) > `Room` (2) > `Global` (lowest, 1).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleScope {
    /// Applies to every object across the entire PCB project.
    Global,
    /// Applies to objects physically positioned inside a designated geometric room / area.
    Room(String),
    /// Applies to all nets belonging to a specific net class (e.g., "Power", "HighSpeed_Diff").
    NetClass(String),
    /// Applies specifically to an individual named net (e.g., "VBUS", "+3V3", "USB_D+").
    Net(String),
}

impl RuleScope {
    /// Specificity priority rank. Higher numbers override lower numbers.
    #[inline]
    pub fn specificity(&self) -> u8 {
        match self {
            Self::Global => 1,
            Self::Room(_) => 2,
            Self::NetClass(_) => 3,
            Self::Net(_) => 4,
        }
    }

    /// Check if this scope matches the query parameters.
    pub fn matches(&self, net: &str, net_class: Option<&str>, room: Option<&str>) -> bool {
        match self {
            Self::Global => true,
            Self::Room(r) => room == Some(r.as_str()),
            Self::NetClass(nc) => net_class == Some(nc.as_str()),
            Self::Net(n) => net == n,
        }
    }
}
