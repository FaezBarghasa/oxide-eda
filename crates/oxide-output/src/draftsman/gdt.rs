//! Geometric Dimensioning and Tolerancing (GD&T) per ASME Y14.5 & ISO 128.
//!
//! Provides data structures for feature control frames, tolerance modifiers,
//! datum reference frames, and associative dimensioning annotations.

use serde::{Deserialize, Serialize};

/// Geometric Characteristic Symbols per ASME Y14.5-2018.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeometricCharacteristic {
    // Form
    Straightness,
    Flatness,
    Circularity,
    Cylindricity,
    // Profile
    ProfileOfLine,
    ProfileOfSurface,
    // Orientation
    Perpendicularity,
    Parallelism,
    Angularity,
    // Location
    Position,
    Concentricity,
    Symmetry,
    // Runout
    CircularRunout,
    TotalRunout,
}

/// Material Condition Modifiers per ASME Y14.5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialCondition {
    /// Maximum Material Condition (MMC - [M])
    MaximumMaterialCondition,
    /// Least Material Condition (LMC - [L])
    LeastMaterialCondition,
    /// Regardless of Feature Size (RFS) - default
    RegardlessOfFeatureSize,
}

/// Datum Reference with optional material boundary modifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatumReference {
    pub label: String,
    pub modifier: Option<MaterialCondition>,
}

/// Feature Control Frame (FCF) per ASME Y14.5.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureControlFrame {
    pub characteristic: GeometricCharacteristic,
    pub is_diameter_zone: bool,
    pub tolerance_value_mm: f64,
    pub material_condition: Option<MaterialCondition>,
    pub primary_datum: Option<DatumReference>,
    pub secondary_datum: Option<DatumReference>,
    pub tertiary_datum: Option<DatumReference>,
}

impl FeatureControlFrame {
    pub fn new(characteristic: GeometricCharacteristic, tolerance_value_mm: f64) -> Self {
        Self {
            characteristic,
            is_diameter_zone: false,
            tolerance_value_mm,
            material_condition: None,
            primary_datum: None,
            secondary_datum: None,
            tertiary_datum: None,
        }
    }

    /// Generates a standardized textual representation e.g. "| [Position] | Ø0.05 (M) | A | B (M) | C |"
    pub fn to_standard_string(&self) -> String {
        let diam = if self.is_diameter_zone { "Ø" } else { "" };
        let mat = match self.material_condition {
            Some(MaterialCondition::MaximumMaterialCondition) => " Ⓓ",
            Some(MaterialCondition::LeastMaterialCondition) => " Ⓛ",
            _ => "",
        };

        let mut parts = vec![
            format!("{:?}", self.characteristic),
            format!("{}{:.3}{}", diam, self.tolerance_value_mm, mat),
        ];

        if let Some(ref d) = self.primary_datum {
            let d_mod = match d.modifier {
                Some(MaterialCondition::MaximumMaterialCondition) => " Ⓓ",
                Some(MaterialCondition::LeastMaterialCondition) => " Ⓛ",
                _ => "",
            };
            parts.push(format!("{}{}", d.label, d_mod));
        }
        if let Some(ref d) = self.secondary_datum {
            let d_mod = match d.modifier {
                Some(MaterialCondition::MaximumMaterialCondition) => " Ⓓ",
                Some(MaterialCondition::LeastMaterialCondition) => " Ⓛ",
                _ => "",
            };
            parts.push(format!("{}{}", d.label, d_mod));
        }
        if let Some(ref d) = self.tertiary_datum {
            let d_mod = match d.modifier {
                Some(MaterialCondition::MaximumMaterialCondition) => " Ⓓ",
                Some(MaterialCondition::LeastMaterialCondition) => " Ⓛ",
                _ => "",
            };
            parts.push(format!("{}{}", d.label, d_mod));
        }

        format!("| {} |", parts.join(" | "))
    }
}

/// Associative Dimensioning Types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DimensionKind {
    Linear {
        p1: [f64; 2],
        p2: [f64; 2],
        text_pos: [f64; 2],
        measured_value_mm: f64,
        tolerance_plus_mm: f64,
        tolerance_minus_mm: f64,
    },
    Ordinate {
        datum_origin: [f64; 2],
        target_point: [f64; 2],
        is_horizontal: bool,
        measured_value_mm: f64,
    },
    Radial {
        center: [f64; 2],
        rim_point: [f64; 2],
        radius_mm: f64,
    },
    Diameter {
        center: [f64; 2],
        diameter_mm: f64,
    },
    GdtCallout {
        anchor_point: [f64; 2],
        frame: FeatureControlFrame,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_control_frame_formatting() {
        let mut fcf = FeatureControlFrame::new(GeometricCharacteristic::Position, 0.05);
        fcf.is_diameter_zone = true;
        fcf.material_condition = Some(MaterialCondition::MaximumMaterialCondition);
        fcf.primary_datum = Some(DatumReference {
            label: "A".to_string(),
            modifier: None,
        });
        fcf.secondary_datum = Some(DatumReference {
            label: "B".to_string(),
            modifier: Some(MaterialCondition::MaximumMaterialCondition),
        });

        let formatted = fcf.to_standard_string();
        assert!(formatted.contains("Position"));
        assert!(formatted.contains("Ø0.050"));
        assert!(formatted.contains("A"));
        assert!(formatted.contains("B"));
    }
}
