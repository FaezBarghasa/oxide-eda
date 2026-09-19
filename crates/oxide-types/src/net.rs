use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Net identity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetClassId(pub String);

// ---------------------------------------------------------------------------
// Net class
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetClass {
    pub name: String,
    #[serde(default)]
    pub clearance: f64,
    #[serde(default)]
    pub trace_width: f64,
    #[serde(default)]
    pub via_diameter: f64,
    #[serde(default)]
    pub via_drill: f64,
    #[serde(default)]
    pub diff_pair_gap: f64,
    #[serde(default)]
    pub diff_pair_width: f64,
}

// ---------------------------------------------------------------------------
// Differential pair
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffPair {
    pub positive_net: String,
    pub negative_net: String,
    pub class: String,
}

// ---------------------------------------------------------------------------
// Netlist — the authoritative schematic-derived connectivity contract
// ---------------------------------------------------------------------------

/// One pin instance connected to a net: the placed symbol's `uuid`, its
/// reference designator (`R1`, `U3`), and the pin identifier (number or name).
///
/// `symbol` disambiguates terminals a bare reference string collapses —
/// unannotated `R?` and duplicate designators (the same refdes on two sheet
/// occurrences) — and links the terminal back to the placed symbol. `reference`
/// and `pin` stay for exporters and display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Terminal {
    #[serde(default)]
    pub symbol: Uuid,
    pub reference: String,
    pub pin: String,
    /// Internal IC package bond-wire / leadframe delay in picoseconds (ps).
    #[serde(default)]
    pub internal_delay_ps: f64,
}

// Implement Eq manually for Terminal since internal_delay_ps is f64
impl Eq for Terminal {}

/// A node in an extended signal (xSignal) flight path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XSignalNode {
    pub reference: String,
    pub pin: String,
    #[serde(default)]
    pub internal_delay_ps: f64,
}

/// An xSignal represents a complete physical/logical signal path spanning across
/// discrete passives (e.g. Driver pin -> Series damping resistor -> DDR4 receiver pin).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XSignal {
    pub name: String,
    pub source: XSignalNode,
    pub destination: XSignalNode,
    #[serde(default)]
    pub intermediate_nets: Vec<String>,
    #[serde(default)]
    pub target_length_microns: Option<i64>,
    #[serde(default)]
    pub tolerance_microns: i64,
}

impl XSignal {
    pub fn new(name: impl Into<String>, source: XSignalNode, destination: XSignalNode) -> Self {
        Self {
            name: name.into(),
            source,
            destination,
            intermediate_nets: Vec::new(),
            target_length_microns: None,
            tolerance_microns: 50, // 50µm default (~2 mil)
        }
    }

    /// Total internal package delay in picoseconds (ps) from source and destination pins.
    pub fn total_package_delay_ps(&self) -> f64 {
        self.source.internal_delay_ps + self.destination.internal_delay_ps
    }

    /// Convert internal package delay into equivalent copper track length in micrometers
    /// (assumes standard ~150 ps/inch = ~5.9 ps/mm => ~0.169 mm/ps = ~169.5 µm/ps propagation speed).
    pub fn package_delay_to_length_microns(&self, ps_per_mm: f64) -> i64 {
        let ps_rate = if ps_per_mm > 0.0 { ps_per_mm } else { 5.9 };
        let mm = self.total_package_delay_ps() / ps_rate;
        (mm * 1000.0).round() as i64
    }
}

/// A logical net: a set of electrically-connected terminals derived from the
/// schematic (wires + junctions + labels + pins). `id` is a build-time stable
/// number (also usable as the PCB net number); `name` comes from the
/// highest-priority label on the net, or is auto-assigned when unlabelled.
///
/// `wires` and `junctions` are the schematic elements the net occupies — the
/// membership the net-flood highlights and the ratsnest reads. `class` is a
/// *project-rules* concern layered on top of connectivity: the connectivity
/// builder leaves it `None`, and a later pass assigns a [`NetClassId`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Net {
    pub id: NetId,
    pub name: String,
    #[serde(default)]
    pub class: Option<NetClassId>,
    #[serde(default)]
    pub wires: Vec<Uuid>,
    #[serde(default)]
    pub junctions: Vec<Uuid>,
    pub terminals: Vec<Terminal>,
}

impl Eq for Net {}

/// The authoritative netlist: every net derived from a schematic. This is the
/// single connectivity source the net-flood UI, the ratsnest, PCB net
/// assignment, and the netlist exporter are meant to read — replacing the
/// ad-hoc union-find copies scattered across the app (ADR-0001 A3.1).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Netlist {
    pub nets: Vec<Net>,
    #[serde(default)]
    pub xsignals: Vec<XSignal>,
}

impl Eq for Netlist {}
