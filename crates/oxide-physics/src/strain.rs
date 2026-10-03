//! Dynamic Modulus-Weighted Neutral Axis & IPC-2223 Rigid-Flex Strain Analysis.
//!
//! Conforms to Master Technical Directive §4 & IPC-2223 Section 5.2:
//! - Exact multi-layer Young's modulus neutral axis calculation
//! - Copper tensile/compressive strain extraction under tight bend radii
//! - Transition zone design rule checks (coverlay overlap, stiffener keepout, via clearance)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error conditions in rigid-flex mechanical analysis.
#[derive(Error, Debug, PartialEq)]
pub enum StrainError {
    #[error("Zero total modulus-thickness product: stackup layers invalid")]
    ZeroModulusProduct,
    #[error(
        "Bend radius too small ({radius_um} µm); copper strain {strain_pct:.2}% exceeds fatigue limit"
    )]
    FatigueLimitExceeded { radius_um: f64, strain_pct: f64 },
    #[error("IPC-2223 coverlay overlap {found_um} µm is less than required {required_um} µm")]
    InsufficientCoverlayOverlap { found_um: f64, required_um: f64 },
    #[error("Via at distance {distance_um} µm breaches IPC-2223 transition keepout ({min_um} µm)")]
    ViaInTransitionKeepout { distance_um: f64, min_um: f64 },
}

/// A layer slice in a multi-material flexible stackup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlexLayerSlice {
    pub name: String,
    pub thickness_um: f64,
    /// Young's modulus in GPa (e.g. Copper ~110 GPa, Polyimide ~3.5 GPa, Adhesive ~1.5 GPa).
    pub youngs_modulus_gpa: f64,
    pub is_conductor: bool,
}

/// Dynamic neutral axis and strain evaluator for flexible circuit hinges.
#[derive(Debug, Clone, Default)]
pub struct NeutralAxisEvaluator {
    pub layers: Vec<FlexLayerSlice>,
}

impl NeutralAxisEvaluator {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(
        &mut self,
        name: &str,
        thickness_um: f64,
        youngs_modulus_gpa: f64,
        is_conductor: bool,
    ) {
        self.layers.push(FlexLayerSlice {
            name: name.to_string(),
            thickness_um: thickness_um.max(0.0),
            youngs_modulus_gpa: youngs_modulus_gpa.max(0.01),
            is_conductor,
        });
    }

    /// Computes the exact modulus-weighted neutral axis elevation $y_{\text{neutral}}$ from the bottom substrate surface.
    ///
    /// $$y_{\text{neutral}} = \frac{\sum E_i t_i \bar{y}_i}{\sum E_i t_i}$$
    pub fn compute_neutral_axis_um(&self) -> Result<f64, StrainError> {
        let mut sum_e_t_y = 0.0;
        let mut sum_e_t = 0.0;
        let mut curr_y = 0.0;

        for layer in &self.layers {
            let y_mid = curr_y + layer.thickness_um * 0.5;
            let e_t = layer.youngs_modulus_gpa * layer.thickness_um;
            sum_e_t_y += e_t * y_mid;
            sum_e_t += e_t;
            curr_y += layer.thickness_um;
        }

        if sum_e_t <= 1e-12 {
            return Err(StrainError::ZeroModulusProduct);
        }

        Ok(sum_e_t_y / sum_e_t)
    }

    /// Evaluates copper tensile strain $\varepsilon_{\text{copper}}$ under a given bend radius $R_{\text{bend}}$ in µm.
    /// Returns the maximum strain found across all conductor layers as a fraction (e.g., 0.003 = 0.3%).
    pub fn compute_max_copper_strain(&self, bend_radius_um: f64) -> Result<f64, StrainError> {
        let y_neutral = self.compute_neutral_axis_um()?;
        let mut max_strain = 0.0;
        let mut curr_y = 0.0;

        for layer in &self.layers {
            let y_mid = curr_y + layer.thickness_um * 0.5;
            if layer.is_conductor {
                let dist_from_neutral = (y_mid - y_neutral).abs();
                let strain = dist_from_neutral / (bend_radius_um + y_neutral);
                if strain > max_strain {
                    max_strain = strain;
                }
            }
            curr_y += layer.thickness_um;
        }

        Ok(max_strain)
    }

    /// Validates dynamic flexing fatigue against the 0.3% IPC-2223 threshold.
    pub fn validate_dynamic_flex(&self, bend_radius_um: f64) -> Result<f64, StrainError> {
        let strain = self.compute_max_copper_strain(bend_radius_um)?;
        if strain > 0.003 {
            // > 0.3%
            return Err(StrainError::FatigueLimitExceeded {
                radius_um: bend_radius_um,
                strain_pct: strain * 100.0,
            });
        }
        Ok(strain)
    }
}

/// IPC-2223 Section 5.2 Transition Zone Design Rule Validator.
pub struct Ipc2223Validator;

impl Ipc2223Validator {
    /// Validates coverlay overlap into rigid zone (min 1.0 mm / 1000 µm or 20 * t_coverlay).
    pub fn check_coverlay_overlap(
        overlap_um: f64,
        coverlay_thickness_um: f64,
    ) -> Result<(), StrainError> {
        let min_required = 1000.0f64.max(20.0 * coverlay_thickness_um);
        if overlap_um < min_required {
            return Err(StrainError::InsufficientCoverlayOverlap {
                found_um: overlap_um,
                required_um: min_required,
            });
        }
        Ok(())
    }

    /// Validates that no via is placed within the 2.5 mm (2500 µm) rigid-to-flex transition keepout.
    pub fn check_via_transition_clearance(distance_from_seam_um: f64) -> Result<(), StrainError> {
        const MIN_VIA_CLEARANCE_UM: f64 = 2500.0;
        if distance_from_seam_um < MIN_VIA_CLEARANCE_UM {
            return Err(StrainError::ViaInTransitionKeepout {
                distance_um: distance_from_seam_um,
                min_um: MIN_VIA_CLEARANCE_UM,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_neutral_axis_shift() {
        let mut eval = NeutralAxisEvaluator::new();
        // Asymmetric stackup:
        // Bottom Coverlay: 25 µm (E = 3.5 GPa)
        // Adhesive: 15 µm (E = 1.5 GPa)
        // Copper: 18 µm (E = 110 GPa)
        // Top Polyimide: 25 µm (E = 3.5 GPa)
        eval.add_layer("Bottom Coverlay", 25.0, 3.5, false);
        eval.add_layer("Adhesive", 15.0, 1.5, false);
        eval.add_layer("Copper Foil", 18.0, 110.0, true);
        eval.add_layer("Top Coverlay", 25.0, 3.5, false);

        let y_neutral = eval.compute_neutral_axis_um().unwrap();
        // Geometric midpoint is (25+15+18+25)/2 = 41.5 µm.
        // Copper layer is located at y = 40..58 µm with center at 49 µm.
        // Because copper has E = 110 GPa (dominant), y_neutral should shift toward ~47-49 µm.
        assert!(
            y_neutral > 41.5,
            "Neutral axis should shift toward stiff copper layer, got {}",
            y_neutral
        );

        // Under 5 mm bend radius (5000 µm)
        let strain = eval.compute_max_copper_strain(5000.0).unwrap();
        assert!(strain < 0.003, "Strain should be within 0.3% fatigue limit");
    }

    #[test]
    fn test_ipc2223_rule_checks() {
        assert!(Ipc2223Validator::check_coverlay_overlap(1200.0, 25.0).is_ok());
        assert!(Ipc2223Validator::check_coverlay_overlap(800.0, 25.0).is_err());

        assert!(Ipc2223Validator::check_via_transition_clearance(3000.0).is_ok());
        assert!(Ipc2223Validator::check_via_transition_clearance(1500.0).is_err());
    }
}
