//! Wire Harness & Multi-Signal Harness Connector Modeling.
//!
//! Provides Altium Designer parity for Harness Connectors, Harness Entries, and Harness Signal Bundles.
//! Enables complex mixed-signal buses (e.g. `SPI_WITH_INTERRUPT`, `DDR5_CTRL`, `AUDIO_I2S`) to be bundled
//! into a single high-level connection across hierarchical schematic sheets.

use oxide_types::schematic::Point;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// An individual signal pin/entry within a harness connector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessEntry {
    pub name: String,
    pub net_name: String,
    pub position_offset: Point,
}

/// A Harness Connector placed on a schematic sheet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessConnector {
    pub uuid: Uuid,
    pub name: String,
    pub harness_type: String,
    pub position: Point,
    pub entries: Vec<HarnessEntry>,
    pub is_primary: bool,
}

impl HarnessConnector {
    pub fn new(name: impl Into<String>, harness_type: impl Into<String>, position: Point) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name: name.into(),
            harness_type: harness_type.into(),
            position,
            entries: Vec::new(),
            is_primary: true,
        }
    }

    pub fn add_entry(&mut self, entry_name: impl Into<String>, net_name: impl Into<String>) {
        let count = self.entries.len();
        self.entries.push(HarnessEntry {
            name: entry_name.into(),
            net_name: net_name.into(),
            position_offset: Point::new(0.0, count as f64 * 2.54), // 100mil standard pitch
        });
    }

    /// Resolves net names mapped to each harness signal.
    pub fn resolved_nets(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for entry in &self.entries {
            map.insert(entry.name.clone(), entry.net_name.clone());
        }
        map
    }
}

/// Schema definition defining which signals belong to a specific Harness Type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarnessDefinition {
    pub harness_type: String,
    pub member_signals: Vec<String>,
}

/// Global repository of harness types and multi-sheet harness connection validator.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HarnessManager {
    pub definitions: HashMap<String, HarnessDefinition>,
    pub connectors: Vec<HarnessConnector>,
}

impl HarnessManager {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            connectors: Vec::new(),
        }
    }

    pub fn register_definition(&mut self, def: HarnessDefinition) {
        self.definitions.insert(def.harness_type.clone(), def);
    }

    pub fn add_connector(&mut self, connector: HarnessConnector) {
        self.connectors.push(connector);
    }

    /// Validate that all connectors matching a harness type contain the required signals.
    pub fn validate_connectors(&self) -> Vec<String> {
        let mut errors = Vec::new();

        for conn in &self.connectors {
            if let Some(def) = self.definitions.get(&conn.harness_type) {
                let actual_signals: HashSet<&str> =
                    conn.entries.iter().map(|e| e.name.as_str()).collect();
                for req in &def.member_signals {
                    if !actual_signals.contains(req.as_str()) {
                        errors.push(format!(
                            "Harness connector '{}' of type '{}' is missing required signal '{}'",
                            conn.name, conn.harness_type, req
                        ));
                    }
                }
            } else {
                errors.push(format!(
                    "Harness connector '{}' references undefined harness type '{}'",
                    conn.name, conn.harness_type
                ));
            }
        }

        errors
    }
}
