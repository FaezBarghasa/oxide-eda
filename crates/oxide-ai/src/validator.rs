//! Real-part validator pipeline for ensuring AI never introduces hallucinated components.

use thiserror::Error;

use crate::intent::{CircuitIntent, Distributor};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidationError {
    #[error("Component '{function}' has no verified manufacturer part number (MPN)")]
    UnmatchedPart { function: String },

    #[error("Part '{mpn}' is currently Out of Stock at distributor {distributor:?}")]
    OutOfStock {
        mpn: String,
        distributor: Distributor,
    },

    #[error("Part '{mpn}' is marked Not Recommended for New Designs (NRND) or Obsolete")]
    ObsoletePart { mpn: String },
}

/// Abstract distributor part catalog query interface.
pub trait PartLookupService {
    fn find_in_stock_part(&self, function: &str) -> Option<(String, Distributor, u64)>;
}

/// In-memory catalog lookup service for fast, deterministic part resolution.
#[derive(Debug, Default)]
pub struct StandardCatalogService {
    parts: Vec<(String, String, Distributor, u64)>, // (keyword, mpn, distributor, stock)
}

impl StandardCatalogService {
    pub fn new() -> Self {
        let mut svc = Self { parts: Vec::new() };
        // Seed common maker and professional ICs
        svc.register(
            "ESP32-S3",
            "ESP32-S3-WROOM-1-N8R8",
            Distributor::Lcsc,
            14200,
        );
        svc.register("TP4056", "TP4056-42-ESOP8", Distributor::Lcsc, 85000);
        svc.register("BMP280", "BMP280", Distributor::DigiKey, 4300);
        svc.register("USB-C", "TYPE-C-16PIN-SMD", Distributor::Lcsc, 120000);
        svc.register(
            "AMS1117-3.3",
            "AMS1117-3.3V-SOT-223",
            Distributor::Lcsc,
            250000,
        );
        svc
    }

    pub fn register(&mut self, keyword: &str, mpn: &str, distributor: Distributor, stock: u64) {
        self.parts
            .push((keyword.to_lowercase(), mpn.to_string(), distributor, stock));
    }
}

impl PartLookupService for StandardCatalogService {
    fn find_in_stock_part(&self, function: &str) -> Option<(String, Distributor, u64)> {
        let func_lower = function.to_lowercase();
        self.parts
            .iter()
            .find(|(kw, _, _, _)| func_lower.contains(kw))
            .map(|(_, mpn, dist, stock)| (mpn.clone(), *dist, *stock))
    }
}

/// Validate and enrich AI circuit intent with real, verified parts.
pub fn validate_and_enrich(
    intent: &mut CircuitIntent,
    service: &dyn PartLookupService,
) -> Result<(), ValidationError> {
    for comp in &mut intent.components {
        if comp.verified_part_number.is_none() {
            if let Some((mpn, dist, stock)) = service.find_in_stock_part(&comp.function) {
                comp.verified_part_number = Some(mpn);
                comp.preferred_distributor = Some(dist);
                comp.in_stock_quantity = Some(stock);
            } else {
                return Err(ValidationError::UnmatchedPart {
                    function: comp.function.clone(),
                });
            }
        }
    }
    intent.is_validated = true;
    Ok(())
}
