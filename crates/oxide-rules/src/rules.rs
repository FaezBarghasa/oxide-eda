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
            length_tolerance: 50,      // 50 µm (~2 mil)
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

/// Copper polygon pour / thermal relief connection style rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolygonConnectRule {
    pub scope: RuleScope,
    /// Whether connected directly (solid copper) or through thermal relief spokes.
    pub direct_connect: bool,
    /// Number of thermal relief spokes (e.g. 2, 4, 8).
    pub spoke_count: usize,
    /// Minimum width of each thermal relief spoke in micrometers.
    pub min_spoke_width: Microns,
    /// Thermal relief air gap distance in micrometers.
    pub air_gap: Microns,
}

/// Solder mask expansion and minimum sliver (green mask bridge between pads).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolderMaskRule {
    pub scope: RuleScope,
    /// Positive or negative mask expansion from copper edge in micrometers.
    pub expansion: Microns,
    /// Minimum solder mask bridge / sliver width (e.g. 100 µm = 0.1 mm) to prevent web breakage.
    pub min_sliver: Microns,
}

/// Silkscreen to solder mask and silkscreen to silkscreen clearance rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SilkscreenRule {
    pub scope: RuleScope,
    /// Minimum clearance from silkscreen text/graphics to exposed copper/solder mask opening.
    pub min_clearance_to_mask: Microns,
    /// Minimum clearance between adjacent silkscreen primitives.
    pub min_clearance_to_silk: Microns,
    /// Minimum line width for legible silkscreen printing.
    pub min_line_width: Microns,
}

/// Net Antenna / Dangling stub rule preventing dead-end open traces that radiate EMI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetAntennaRule {
    pub scope: RuleScope,
    /// Maximum allowed dangling stub length in micrometers (typically 0 or < 100 µm for testpoint taps).
    pub max_stub_length: Microns,
}

/// High-speed unbroken reference return path rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnPathRule {
    pub net_class: String,
    /// Maximum allowed distance from high-speed trace to unbroken reference ground/power plane.
    pub max_plane_distance_microns: Microns,
    /// Whether crossing a plane split/gap is strictly prohibited.
    pub forbid_split_crossing: bool,
}

/// 2D/3D Component clearance and spatial collision rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentClearanceRule {
    pub scope: RuleScope,
    /// Minimum horizontal (X/Y) clearance between component bodies in micrometers.
    pub min_horizontal_clearance: Microns,
    /// Minimum vertical (Z) clearance in micrometers.
    pub min_vertical_clearance: Microns,
    /// Minimum component height in micrometers.
    pub min_height: Option<Microns>,
    /// Maximum component height in micrometers.
    pub max_height: Option<Microns>,
}

/// Routing layer permitted stackup and routing topology constraints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingLayerRule {
    pub scope: RuleScope,
    /// Permitted physical layer IDs (e.g. `["TopLayer", "Inner1", "Inner2", "BottomLayer"]`).
    pub permitted_layers: Vec<String>,
    /// Routing topology (e.g. "shortest", "daisy_chain", "star", "balanced_tree").
    pub topology: String,
}

/// Differential Pair Phase Tuning & Skew Matching (Within-Pair and Bus Skew).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffPairPhaseRule {
    pub net_class: String,
    /// Maximum within-pair (intra-pair) phase skew tolerance in micrometers (e.g. 25 µm / 1 mil).
    pub max_intra_pair_skew: Microns,
    /// Maximum between-pair (inter-pair) bus delay/length matching skew tolerance in micrometers.
    pub max_inter_pair_skew: Microns,
}

/// Minimum Annular Ring constraint to prevent drill breakout on vias and PTH pads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnularRingRule {
    pub scope: RuleScope,
    /// Minimum annular ring width in micrometers: (pad_diameter - drill_diameter) / 2.
    pub min_annular_ring: Microns,
}

/// High-Voltage Creepage and Clearance rule (IEC 60950 / IEC 62368 / IPC-2221B).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreepageClearanceRule {
    pub scope: RuleScope,
    /// Working peak/RMS voltage between nets (e.g. 230V, 400V, 1000V).
    pub working_voltage_v: f64,
    /// Minimum electrical clearance through air in micrometers.
    pub min_clearance_microns: Microns,
    /// Minimum creepage distance along insulation surface in micrometers.
    pub min_creepage_microns: Microns,
}

/// Hole-to-Hole spacing constraint preventing mechanical drill web fracture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoleToHoleRule {
    pub scope: RuleScope,
    /// Minimum edge-to-edge spacing between drill holes in micrometers.
    pub min_hole_spacing: Microns,
}

/// Room Placement & Containment rule ensuring multi-channel components stay in designated rooms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomPlacementRule {
    pub room_name: String,
    pub component_designators: Vec<String>,
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
    SolderMask(SolderMaskRule),
    Silkscreen(SilkscreenRule),
    NetAntenna(NetAntennaRule),
    ReturnPath(ReturnPathRule),
    ComponentClearance(ComponentClearanceRule),
    RoutingLayer(RoutingLayerRule),
    DiffPairPhase(DiffPairPhaseRule),
    AnnularRing(AnnularRingRule),
    CreepageClearance(CreepageClearanceRule),
    HoleToHole(HoleToHoleRule),
    RoomPlacement(RoomPlacementRule),
}
