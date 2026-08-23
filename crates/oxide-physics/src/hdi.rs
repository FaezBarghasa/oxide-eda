//! High-Density Interconnect (HDI) structures and blind/buried/microvia verification.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::stackup::LayerStackup;
use crate::units::Microns;

/// Category and physical layer span of a via structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViaType {
    /// Plated through-hole connecting all layers (top to bottom).
    ThroughHole,
    /// Blind via connecting an outer surface layer (L1 or L_bottom) to an inner copper layer.
    Blind {
        start_layer: usize,
        end_layer: usize,
    },
    /// Buried via connecting two inner copper layers (never touches outer surfaces).
    Buried {
        start_layer: usize,
        end_layer: usize,
    },
    /// Laser-drilled microvia for HDI routing (spans 1 or 2 dielectric layers).
    Microvia {
        start_layer: usize,
        end_layer: usize,
        diameter: Microns,
        laser_drilled: bool,
    },
}

/// Via geometry and technological classification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViaDefinition {
    pub name: String,
    pub via_type: ViaType,
    pub drill_diameter: Microns,
    pub pad_diameter: Microns,
}

impl ViaDefinition {
    pub fn standard_through_hole() -> Self {
        Self {
            name: "Standard TH Via (0.3/0.6mm)".into(),
            via_type: ViaType::ThroughHole,
            drill_diameter: 300,
            pad_diameter: 600,
        }
    }

    pub fn hdi_microvia(start_layer: usize, end_layer: usize) -> Self {
        Self {
            name: format!("HDI Microvia (L{start_layer}-L{end_layer})"),
            via_type: ViaType::Microvia {
                start_layer,
                end_layer,
                diameter: 100,
                laser_drilled: true,
            },
            drill_diameter: 100,
            pad_diameter: 250,
        }
    }
}

/// Violations resulting from invalid via geometry or impossible layer spans.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum HdiError {
    #[error("Via layer index out of bounds: start layer {start} or end layer {end} exceeds stackup size {total_layers}")]
    LayerOutOfBounds {
        start: usize,
        end: usize,
        total_layers: usize,
    },

    #[error("Via must start and end on conductive copper layers: start {start} (type {start_type:?}), end {end} (type {end_type:?})")]
    NonCopperEndpoint {
        start: usize,
        end: usize,
        start_type: String,
        end_type: String,
    },

    #[error("Via start layer {start} and end layer {end} must be different")]
    ZeroLengthSpan { start: usize, end: usize },

    #[error("Blind via must start or end on an outer copper layer (L{outer_first} or L{outer_last}), but spans L{start}-L{end}")]
    BlindViaNotOnOuterSurface {
        start: usize,
        end: usize,
        outer_first: usize,
        outer_last: usize,
    },

    #[error("Buried via must NOT touch outer surface layers (L{outer_first} or L{outer_last}), but spans L{start}-L{end}")]
    BuriedViaTouchesOuterSurface {
        start: usize,
        end: usize,
        outer_first: usize,
        outer_last: usize,
    },

    #[error("Microvia aspect ratio (depth/diameter = {depth_microns}µm/{diameter_microns}µm = {aspect_ratio:.2}) exceeds maximum manufacturable limit (1.0)")]
    MicroviaAspectRatioExceeded {
        depth_microns: Microns,
        diameter_microns: Microns,
        aspect_ratio: f64,
    },

    #[error("Laser microvia spans {span} copper layers; standard microvias can only span 1 layer (adjacent layers) or 2 (skip-via)")]
    MicroviaSpanTooLarge { span: usize },
}

/// Verify that a via structure obeys physical manufacturing and stackup constraints.
pub fn check_hdi_rules(via: &ViaDefinition, stackup: &LayerStackup) -> Result<(), HdiError> {
    let copper_indices = stackup.copper_layer_indices();
    if copper_indices.is_empty() {
        return Ok(());
    }

    let outer_first = *copper_indices.first().unwrap();
    let outer_last = *copper_indices.last().unwrap();
    let total_layers = stackup.layers.len();

    match via.via_type {
        ViaType::ThroughHole => {
            // Through holes connect all copper layers, always valid as long as stackup exists
            Ok(())
        }
        ViaType::Blind {
            start_layer,
            end_layer,
        } => {
            validate_endpoints(start_layer, end_layer, stackup, total_layers)?;

            let touches_outer = start_layer == outer_first
                || start_layer == outer_last
                || end_layer == outer_first
                || end_layer == outer_last;

            if !touches_outer {
                return Err(HdiError::BlindViaNotOnOuterSurface {
                    start: start_layer,
                    end: end_layer,
                    outer_first,
                    outer_last,
                });
            }

            Ok(())
        }
        ViaType::Buried {
            start_layer,
            end_layer,
        } => {
            validate_endpoints(start_layer, end_layer, stackup, total_layers)?;

            if start_layer == outer_first
                || start_layer == outer_last
                || end_layer == outer_first
                || end_layer == outer_last
            {
                return Err(HdiError::BuriedViaTouchesOuterSurface {
                    start: start_layer,
                    end: end_layer,
                    outer_first,
                    outer_last,
                });
            }

            Ok(())
        }
        ViaType::Microvia {
            start_layer,
            end_layer,
            diameter,
            laser_drilled: _,
        } => {
            validate_endpoints(start_layer, end_layer, stackup, total_layers)?;

            let (min_l, max_l) = if start_layer < end_layer {
                (start_layer, end_layer)
            } else {
                (end_layer, start_layer)
            };

            // Count copper layers spanned
            let copper_spanned = copper_indices
                .iter()
                .filter(|&&idx| idx >= min_l && idx <= max_l)
                .count();

            if copper_spanned > 3 {
                return Err(HdiError::MicroviaSpanTooLarge {
                    span: copper_spanned - 1,
                });
            }

            // Calculate drilled depth
            let depth: Microns = stackup.layers[min_l..=max_l]
                .iter()
                .map(|l| l.thickness)
                .sum();

            if diameter > 0 {
                let ratio = (depth as f64) / (diameter as f64);
                // Standard PCB fab aspect ratio limit for laser microvias is 1.0 (or 0.8-1.25)
                if ratio > 1.25 {
                    return Err(HdiError::MicroviaAspectRatioExceeded {
                        depth_microns: depth,
                        diameter_microns: diameter,
                        aspect_ratio: ratio,
                    });
                }
            }

            Ok(())
        }
    }
}

fn validate_endpoints(
    start: usize,
    end: usize,
    stackup: &LayerStackup,
    total_layers: usize,
) -> Result<(), HdiError> {
    if start >= total_layers || end >= total_layers {
        return Err(HdiError::LayerOutOfBounds {
            start,
            end,
            total_layers,
        });
    }

    if start == end {
        return Err(HdiError::ZeroLengthSpan { start, end });
    }

    let start_layer = &stackup.layers[start];
    let end_layer = &stackup.layers[end];

    if !start_layer.layer_type.is_copper() || !end_layer.layer_type.is_copper() {
        return Err(HdiError::NonCopperEndpoint {
            start,
            end,
            start_type: format!("{:?}", start_layer.layer_type),
            end_type: format!("{:?}", end_layer.layer_type),
        });
    }

    Ok(())
}
