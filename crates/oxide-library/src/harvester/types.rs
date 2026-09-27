use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

use crate::param::ParamMap;

#[derive(Error, Debug)]
pub enum HarvestError {
    #[error("Part number '{0}' was not found in distributor databases or crawled web sources")]
    PartNotFound(String),
    #[error("Failed to parse mechanical package parameters from datasheet: {0}")]
    DatasheetExtractionFailure(String),
    #[error("Simulation model syntax unsupported or corrupted: {0}")]
    ModelSynthesisError(String),
    #[error("Network error during download: {0}")]
    Network(String),
    #[error("Rate limited by provider '{provider}', retry after {retry_after_seconds}s")]
    RateLimited {
        provider: String,
        retry_after_seconds: u64,
    },
    #[error("Verification gate failure: {0}")]
    VerificationFailed(String),
    #[error("IO error: {0}")]
    Io(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarvestQuery {
    pub mpn: String,
    pub manufacturer: Option<String>,
    pub preferred_package: Option<String>,
    pub require_simulation_model: bool,
    pub require_3d_step: bool,
}

impl HarvestQuery {
    pub fn new(mpn: impl Into<String>) -> Self {
        Self {
            mpn: mpn.into(),
            manufacturer: None,
            preferred_package: None,
            require_simulation_model: false,
            require_3d_step: false,
        }
    }

    pub fn with_manufacturer(mut self, mfr: impl Into<String>) -> Self {
        self.manufacturer = Some(mfr.into());
        self
    }

    pub fn with_package(mut self, pkg: impl Into<String>) -> Self {
        self.preferred_package = Some(pkg.into());
        self
    }
}

/// Electrical role of a pin discovered from datasheets or distributor APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ElectricalPinType {
    #[default]
    Input,
    Output,
    Bidirectional,
    Power,
    Ground,
    Passive,
    TriState,
    OpenCollector,
    OpenEmitter,
    NoConnect,
}

impl ElectricalPinType {
    /// Infers electrical pin type from pin name / token heuristics.
    pub fn from_pin_name(name: &str) -> Self {
        let n = name.trim().to_uppercase();
        if n.is_empty() || n == "NC" || n == "N.C." || n == "NIC" || n == "DNC" {
            Self::NoConnect
        } else if n == "GND"
            || n == "VSS"
            || n == "AGND"
            || n == "PGND"
            || n == "DGND"
            || n == "THERMAL"
            || n == "EP"
            || n == "PAD"
            || n.starts_with("GND_")
            || n.starts_with("VSS_")
        {
            Self::Ground
        } else if n == "VCC"
            || n == "VDD"
            || n == "3V3"
            || n == "5V"
            || n == "1V8"
            || n == "VBAT"
            || n == "AVDD"
            || n == "PVDD"
            || n == "VIN"
            || n == "V+"
            || n.starts_with("VCC_")
            || n.starts_with("VDD_")
        {
            Self::Power
        } else if n.starts_with("IN")
            || n.contains("_IN")
            || n.ends_with("_IN")
            || n.contains("RX")
            || n.contains("CLK_IN")
            || n == "EN"
            || n == "ENABLE"
            || n == "CS"
            || n == "CS#"
            || n == "NSS"
        {
            Self::Input
        } else if n.starts_with("OUT")
            || n.contains("_OUT")
            || n.ends_with("_OUT")
            || n.contains("TX")
            || n.contains("CLK_OUT")
        {
            Self::Output
        } else if n.starts_with("IO")
            || n.starts_with("PA")
            || n.starts_with("PB")
            || n.starts_with("PC")
            || n.starts_with("PD")
            || n.starts_with("PE")
            || n.starts_with("PF")
            || n.starts_with("PG")
            || n.starts_with("PH")
            || n.starts_with("SDA")
            || n.starts_with("SCL")
            || n.starts_with("MOSI")
            || n.starts_with("MISO")
            || n.starts_with("SWDIO")
            || n.starts_with("D+")
            || n.starts_with("D-")
            || n.starts_with("DP")
            || n.starts_with("DM")
        {
            Self::Bidirectional
        } else {
            Self::Passive
        }
    }
}

/// Discovered pin metadata extracted from unstructured datasheets or structured APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveredPin {
    pub number: String,
    pub name: String,
    pub electrical_type: ElectricalPinType,
    #[serde(default)]
    pub alternate_functions: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub io_bank: Option<u32>,
}

impl DiscoveredPin {
    pub fn new(number: impl Into<String>, name: impl Into<String>) -> Self {
        let name_str = name.into();
        let electrical = ElectricalPinType::from_pin_name(&name_str);
        Self {
            number: number.into(),
            name: name_str,
            electrical_type: electrical,
            alternate_functions: Vec::new(),
            description: None,
            io_bank: None,
        }
    }

    pub fn with_type(mut self, el_type: ElectricalPinType) -> Self {
        self.electrical_type = el_type;
        self
    }
}

/// Mechanical package dimensions extracted or synthesized for IPC-7351C footprint generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageDimensions {
    pub package_class: String, // e.g. "SOIC", "QFP", "QFN", "DFN", "SOT23", "CHIP", "BGA"
    pub pin_count: usize,
    pub body_length_mm: [f64; 3], // min, nom, max (D)
    pub body_width_mm: [f64; 3],  // min, nom, max (E)
    pub seated_height_mm: [f64; 3], // min, nom, max (A)
    pub lead_pitch_mm: f64,       // e
    pub lead_width_mm: [f64; 3],  // min, nom, max (b)
    pub lead_length_mm: [f64; 3], // min, nom, max (L)
    #[serde(default)]
    pub thermal_pad_mm: Option<[f64; 2]>, // [width, length]
}

impl PackageDimensions {
    /// Creates default standard SOIC dimensions for standard pin count.
    pub fn standard_soic(pin_count: usize) -> Self {
        let pitch = 1.27;
        let half = (pin_count / 2) as f64;
        let length_nom = half * pitch + 0.5;
        Self {
            package_class: "SOIC".into(),
            pin_count,
            body_length_mm: [length_nom - 0.2, length_nom, length_nom + 0.2],
            body_width_mm: [3.8, 3.9, 4.0],
            seated_height_mm: [1.35, 1.50, 1.75],
            lead_pitch_mm: pitch,
            lead_width_mm: [0.33, 0.41, 0.51],
            lead_length_mm: [0.40, 0.72, 1.27],
            thermal_pad_mm: None,
        }
    }

    /// Creates standard 2-terminal SMD chip dimensions (e.g., 0402, 0603, 0805, 1206).
    pub fn standard_chip(length_mm: f64, width_mm: f64, height_mm: f64) -> Self {
        Self {
            package_class: "CHIP".into(),
            pin_count: 2,
            body_length_mm: [length_mm - 0.1, length_mm, length_mm + 0.1],
            body_width_mm: [width_mm - 0.1, width_mm, width_mm + 0.1],
            seated_height_mm: [height_mm - 0.1, height_mm, height_mm + 0.1],
            lead_pitch_mm: length_mm * 0.7,
            lead_width_mm: [width_mm - 0.05, width_mm, width_mm + 0.05],
            lead_length_mm: [0.25, 0.35, 0.50],
            thermal_pad_mm: None,
        }
    }

    /// Creates standard QFN dimensions.
    pub fn standard_qfn(pin_count: usize, body_size_mm: f64, pitch_mm: f64, thermal_pad_mm: Option<[f64; 2]>) -> Self {
        Self {
            package_class: "QFN".into(),
            pin_count,
            body_length_mm: [body_size_mm - 0.1, body_size_mm, body_size_mm + 0.1],
            body_width_mm: [body_size_mm - 0.1, body_size_mm, body_size_mm + 0.1],
            seated_height_mm: [0.70, 0.75, 0.80],
            lead_pitch_mm: pitch_mm,
            lead_width_mm: [0.18, 0.23, 0.28],
            lead_length_mm: [0.30, 0.40, 0.50],
            thermal_pad_mm,
        }
    }
}

/// Raw harvested data payload from distributor APIs, crawlers, or OCR ingestion.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HarvestedRawData {
    pub mpn: String,
    pub manufacturer: String,
    pub description: String,
    #[serde(default)]
    pub datasheet_pdf_bytes: Option<Vec<u8>>,
    #[serde(default)]
    pub datasheet_url: Option<String>,
    #[serde(default)]
    pub step_model_bytes: Option<Vec<u8>>,
    #[serde(default)]
    pub raw_spice_model: Option<String>,
    #[serde(default)]
    pub raw_ibis_model: Option<String>,
    #[serde(default)]
    pub direct_pins: Option<Vec<DiscoveredPin>>,
    #[serde(default)]
    pub direct_dimensions: Option<PackageDimensions>,
    #[serde(default)]
    pub parameters: ParamMap,
    #[serde(default)]
    pub extra_metadata: BTreeMap<String, String>,
}

/// Component harvesting contract.
pub trait ComponentHarvester: Send + Sync {
    /// Discovers all available distributor inventory, datasheets, and raw models.
    fn harvest_part(&self, query: &HarvestQuery) -> Result<HarvestedRawData, HarvestError>;
}
