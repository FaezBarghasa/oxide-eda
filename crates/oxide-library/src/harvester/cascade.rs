use std::sync::Arc;
use sha2::{Digest, Sha256};

use super::types::{
    ComponentHarvester, DiscoveredPin, ElectricalPinType, HarvestError, HarvestQuery,
    HarvestedRawData, PackageDimensions,
};
use crate::distributor::DistributorAdapter;

/// Harvester Orchestrator that manages the multi-tier cascade strategy.
pub struct HarvesterCascade {
    distributor_adapters: Vec<Arc<dyn DistributorAdapter>>,
}

impl Default for HarvesterCascade {
    fn default() -> Self {
        Self::new()
    }
}

impl HarvesterCascade {
    pub fn new() -> Self {
        Self {
            distributor_adapters: Vec::new(),
        }
    }

    pub fn with_distributor(mut self, adapter: Arc<dyn DistributorAdapter>) -> Self {
        self.distributor_adapters.push(adapter);
        self
    }

    /// Computes content hash (SHA-256) of raw artifact bytes for deduplication.
    pub fn hash_artifact(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        result.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// Synthesizes fallback component metadata if distributor APIs yield partial data.
    pub fn synthesize_fallback(&self, query: &HarvestQuery) -> HarvestedRawData {
        let mpn = &query.mpn;
        let mut raw = HarvestedRawData {
            mpn: mpn.clone(),
            manufacturer: query.manufacturer.clone().unwrap_or_else(|| "Generic".into()),
            description: format!("Synthesized component for {mpn}"),
            ..Default::default()
        };

        // Heuristic detection from MPN
        let upper = mpn.to_uppercase();
        if upper.contains("LM358") || upper.contains("NE5532") || upper.contains("TL072") {
            raw.description = "Dual Operational Amplifier".into();
            raw.direct_dimensions = Some(PackageDimensions::standard_soic(8));
            raw.direct_pins = Some(vec![
                DiscoveredPin::new("1", "OUT1").with_type(ElectricalPinType::Output),
                DiscoveredPin::new("2", "IN1-").with_type(ElectricalPinType::Input),
                DiscoveredPin::new("3", "IN1+").with_type(ElectricalPinType::Input),
                DiscoveredPin::new("4", "VEE").with_type(ElectricalPinType::Ground),
                DiscoveredPin::new("5", "IN2+").with_type(ElectricalPinType::Input),
                DiscoveredPin::new("6", "IN2-").with_type(ElectricalPinType::Input),
                DiscoveredPin::new("7", "OUT2").with_type(ElectricalPinType::Output),
                DiscoveredPin::new("8", "VCC").with_type(ElectricalPinType::Power),
            ]);
        } else if upper.starts_with('R') && (upper.contains("0805") || upper.contains("0603") || upper.contains("0402")) {
            raw.description = "Chip Resistor".into();
            let (l, w, h) = if upper.contains("0805") {
                (2.0, 1.25, 0.5)
            } else if upper.contains("0603") {
                (1.6, 0.8, 0.45)
            } else {
                (1.0, 0.5, 0.35)
            };
            raw.direct_dimensions = Some(PackageDimensions::standard_chip(l, w, h));
            raw.direct_pins = Some(vec![
                DiscoveredPin::new("1", "1").with_type(ElectricalPinType::Passive),
                DiscoveredPin::new("2", "2").with_type(ElectricalPinType::Passive),
            ]);
        } else if upper.starts_with('C') && (upper.contains("0805") || upper.contains("0603") || upper.contains("0402")) {
            raw.description = "Ceramic Capacitor".into();
            let (l, w, h) = if upper.contains("0805") {
                (2.0, 1.25, 0.5)
            } else if upper.contains("0603") {
                (1.6, 0.8, 0.45)
            } else {
                (1.0, 0.5, 0.35)
            };
            raw.direct_dimensions = Some(PackageDimensions::standard_chip(l, w, h));
            raw.direct_pins = Some(vec![
                DiscoveredPin::new("1", "1").with_type(ElectricalPinType::Passive),
                DiscoveredPin::new("2", "2").with_type(ElectricalPinType::Passive),
            ]);
        } else {
            // Default 8-pin dual inline / SOIC
            raw.direct_dimensions = Some(PackageDimensions::standard_soic(8));
            raw.direct_pins = Some((1..=8).map(|i| DiscoveredPin::new(i.to_string(), format!("P{i}"))).collect());
        }

        raw
    }
}

impl ComponentHarvester for HarvesterCascade {
    fn harvest_part(&self, query: &HarvestQuery) -> Result<HarvestedRawData, HarvestError> {
        // Tier 2: Authenticated Distributor REST APIs
        for adapter in &self.distributor_adapters {
            if let Ok(parts) = adapter.lookup_by_mpn(&query.mpn) {
                if let Some(best) = parts.into_iter().next() {
                    let mut data = HarvestedRawData {
                        mpn: best.mpn,
                        manufacturer: best.manufacturer,
                        description: best.description,
                        datasheet_url: best.datasheet_url.map(|u| u.to_string()),
                        parameters: best.parameters,
                        ..Default::default()
                    };
                    
                    // Populate default dimensions from footprint hint or synthesize
                    if let Some(hint) = best.footprint_hint {
                        if hint.contains("SOIC") || hint.contains("SOP") {
                            data.direct_dimensions = Some(PackageDimensions::standard_soic(8));
                        } else if hint.contains("0805") {
                            data.direct_dimensions = Some(PackageDimensions::standard_chip(2.0, 1.25, 0.5));
                        }
                    }

                    if data.direct_pins.is_none() || data.direct_dimensions.is_none() {
                        let fallback = self.synthesize_fallback(query);
                        if data.direct_pins.is_none() {
                            data.direct_pins = fallback.direct_pins;
                        }
                        if data.direct_dimensions.is_none() {
                            data.direct_dimensions = fallback.direct_dimensions;
                        }
                    }

                    return Ok(data);
                }
            }
        }

        // Tier 5: Generative Synthesis Fallback
        Ok(self.synthesize_fallback(query))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cascade_synthesizes_opamp_lm358() {
        let cascade = HarvesterCascade::new();
        let query = HarvestQuery::new("LM358DR");
        let result = cascade.harvest_part(&query).expect("should succeed");

        assert_eq!(result.mpn, "LM358DR");
        assert!(result.description.contains("Operational Amplifier"));
        let pins = result.direct_pins.expect("pins present");
        assert_eq!(pins.len(), 8);
        assert_eq!(pins[0].name, "OUT1");
        assert_eq!(pins[0].electrical_type, ElectricalPinType::Output);
        assert_eq!(pins[7].name, "VCC");
        assert_eq!(pins[7].electrical_type, ElectricalPinType::Power);

        let dims = result.direct_dimensions.expect("dimensions present");
        assert_eq!(dims.package_class, "SOIC");
        assert_eq!(dims.pin_count, 8);
    }

    #[test]
    fn cascade_synthesizes_resistor_0805() {
        let cascade = HarvesterCascade::new();
        let query = HarvestQuery::new("RC0805FR-0710KL");
        let result = cascade.harvest_part(&query).expect("should succeed");

        let pins = result.direct_pins.expect("pins present");
        assert_eq!(pins.len(), 2);
        let dims = result.direct_dimensions.expect("dimensions present");
        assert_eq!(dims.package_class, "CHIP");
    }
}
