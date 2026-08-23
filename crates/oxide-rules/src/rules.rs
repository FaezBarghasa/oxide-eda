//! Design rule constraint definitions (Clearance, Routing Width, High-Speed, Via Styles).

use serde::{Deserialize, Serialize};

use oxide_physics::Microns;

use crate::scope::RuleScope;

/// Physical PCB object primitives subject to clearance rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    Track,
    Via,
    Pad,
    Polygon,
    BoardOutline,
    Hole,
}

/// Minimum electrical clearance distance between objects or nets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearanceRule {
    pub scope: RuleScope,
    pub min_distance: Microns,
    #[serde(default)]
    pub object_types: Vec<ObjectType>,
}

impl ClearanceRule {
    pub fn new(scope: RuleScope, min_distance: Microns) -> Self {
        Self {
            scope,
            min_distance,
            object_types: vec![
                ObjectType::Track,
                ObjectType::Via,
                ObjectType::Pad,
                ObjectType::Polygon,
            ],
        }
    }
}

/// Copper trace routing width constraints (Min, Preferred, Max).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidthRule {
    pub scope: RuleScope,
    pub min_width: Microns,
    pub preferred_width: Microns,
    pub max_width: Microns,
}

impl WidthRule {
    pub fn new(
        scope: RuleScope,
        min_width: Microns,
        preferred_width: Microns,
        max_width: Microns,
    ) -> Self {
        Self {
            scope,
            min_width,
            preferred_width,
            max_width,
        }
    }
}

/// High-speed differential pair, impedance, and length matching constraints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighSpeedRule {
    pub net_class: String,
    /// Target characteristic or differential impedance in Ohms (e.g. 50.0Ω or 90.0Ω).
    pub impedance_target: f64,
    /// Length matching skew tolerance in micrometers (e.g. ±50 µm / ±2 mil).
    pub length_tolerance: Microns,
    /// Maximum allowed uncoupled length at transitions/connectors in micrometers.
    pub max_uncoupled_length: Microns,
}

impl HighSpeedRule {
    pub fn new_diff_pair(net_class: impl Into<String>, impedance_target: f64) -> Self {
        Self {
            net_class: net_class.into(),
            impedance_target,
            length_tolerance: 50,     // 50 µm (~2 mil)
            max_uncoupled_length: 500, // 500 µm (~20 mil)
        }
    }
}

/// Allowed via drill and pad dimensions per scope or via technology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViaStyleRule {
    pub scope: RuleScope,
    pub min_drill: Microns,
    pub min_diameter: Microns,
    pub preferred_drill: Microns,
    pub preferred_diameter: Microns,
}

/// Copper pour polygon connection mode and thermal spoke parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolygonConnectRule {
    pub scope: RuleScope,
    pub direct_connect: bool,
    pub spoke_count: u8,
    pub min_spoke_width: Microns,
    pub air_gap: Microns,
}

/// Top-level unified design rule variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "rule_type", rename_all = "snake_case")]
pub enum DesignRule {
    Clearance(ClearanceRule),
    Width(WidthRule),
    HighSpeed(HighSpeedRule),
    ViaStyle(ViaStyleRule),
    PolygonConnect(PolygonConnectRule),
}
