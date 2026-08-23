//! PCB layer definitions and physical stackup management.

use serde::{Deserialize, Serialize};

use crate::material::MaterialProperties;
use crate::units::{
    COPPER_1OZ_MICRONS, COPPER_2OZ_MICRONS, COPPER_HALF_OZ_MICRONS, Microns,
};

/// Type and functional role of a layer in the physical PCB stackup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerType {
    /// External or internal signal routing layer (copper).
    Signal,
    /// Solid internal plane (GND, PWR, copper pour).
    InternalPlane,
    /// Rigid core dielectric substrate.
    DielectricCore,
    /// Pre-impregnated resin bonding sheet (Prepreg).
    DielectricPrepreg,
    /// Solder resist coating.
    SolderMask,
    /// Component legend / silkscreen overlay.
    Silkscreen,
    /// Solder paste stencil mask.
    Paste,
    /// Mechanical or documentation layer.
    Mechanical,
}

impl LayerType {
    /// Returns true if this is an electrically conductive copper layer.
    pub fn is_copper(self) -> bool {
        matches!(self, Self::Signal | Self::InternalPlane)
    }

    /// Returns true if this is an insulating dielectric layer.
    pub fn is_dielectric(self) -> bool {
        matches!(
            self,
            Self::DielectricCore | Self::DielectricPrepreg | Self::SolderMask
        )
    }
}

/// Standard copper foil weight designation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopperWeight {
    /// 0.5 oz (~18 µm).
    HalfOz,
    /// 1.0 oz (~35 µm).
    OneOz,
    /// 2.0 oz (~70 µm).
    TwoOz,
    /// Custom thickness in integer micrometers.
    Custom(Microns),
}

impl CopperWeight {
    pub fn to_microns(self) -> Microns {
        match self {
            Self::HalfOz => COPPER_HALF_OZ_MICRONS,
            Self::OneOz => COPPER_1OZ_MICRONS,
            Self::TwoOz => COPPER_2OZ_MICRONS,
            Self::Custom(m) => m,
        }
    }
}

/// Single layer specification within a PCB stackup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerDefinition {
    /// User or CAD layer name (e.g. "Top Layer", "GND Plane", "Core 1").
    pub name: String,
    /// Layer type.
    pub layer_type: LayerType,
    /// Substrate or conductor material.
    pub material: MaterialProperties,
    /// Thickness in micrometers.
    pub thickness: Microns,
    /// Copper weight if applicable.
    #[serde(default)]
    pub copper_weight: Option<CopperWeight>,
}

impl LayerDefinition {
    pub fn new_signal(name: impl Into<String>, thickness: Microns) -> Self {
        Self {
            name: name.into(),
            layer_type: LayerType::Signal,
            material: MaterialProperties::copper_conductor(),
            thickness,
            copper_weight: Some(CopperWeight::Custom(thickness)),
        }
    }

    pub fn new_plane(name: impl Into<String>, thickness: Microns) -> Self {
        Self {
            name: name.into(),
            layer_type: LayerType::InternalPlane,
            material: MaterialProperties::copper_conductor(),
            thickness,
            copper_weight: Some(CopperWeight::Custom(thickness)),
        }
    }

    pub fn new_core(
        name: impl Into<String>,
        thickness: Microns,
        material: MaterialProperties,
    ) -> Self {
        Self {
            name: name.into(),
            layer_type: LayerType::DielectricCore,
            material,
            thickness,
            copper_weight: None,
        }
    }

    pub fn new_prepreg(
        name: impl Into<String>,
        thickness: Microns,
        material: MaterialProperties,
    ) -> Self {
        Self {
            name: name.into(),
            layer_type: LayerType::DielectricPrepreg,
            material,
            thickness,
            copper_weight: None,
        }
    }

    pub fn new_solder_mask(name: impl Into<String>, thickness: Microns) -> Self {
        Self {
            name: name.into(),
            layer_type: LayerType::SolderMask,
            material: MaterialProperties::solder_mask_lpi(),
            thickness,
            copper_weight: None,
        }
    }
}

/// Ordered multilayer stackup representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerStackup {
    /// All layers from top-most (e.g. Top Solder Mask) to bottom-most (Bottom Solder Mask).
    pub layers: Vec<LayerDefinition>,
}

impl Default for LayerStackup {
    fn default() -> Self {
        Self::standard_4layer_1_6mm()
    }
}

impl LayerStackup {
    pub fn new(layers: Vec<LayerDefinition>) -> Self {
        Self { layers }
    }

    /// Calculate total thickness across all layers in micrometers.
    pub fn total_thickness(&self) -> Microns {
        self.layers.iter().map(|l| l.thickness).sum()
    }

    /// Count conductive copper layers (signals + planes).
    pub fn copper_layers_count(&self) -> usize {
        self.layers
            .iter()
            .filter(|l| l.layer_type.is_copper())
            .count()
    }

    /// Count dielectric insulating layers.
    pub fn dielectric_layers_count(&self) -> usize {
        self.layers
            .iter()
            .filter(|l| l.layer_type.is_dielectric())
            .count()
    }

    /// Returns the indices of all conductive copper layers in order.
    pub fn copper_layer_indices(&self) -> Vec<usize> {
        self.layers
            .iter()
            .enumerate()
            .filter_map(|(idx, l)| if l.layer_type.is_copper() { Some(idx) } else { None })
            .collect()
    }

    /// Find the nearest reference plane (GND or PWR plane) for a given signal layer index.
    ///
    /// Scans both upwards and downwards through dielectric layers and returns the index
    /// of the closest `InternalPlane`.
    pub fn get_reference_plane(&self, layer_idx: usize) -> Option<usize> {
        if layer_idx >= self.layers.len() {
            return None;
        }

        // Distance & index tracker for nearest plane
        let mut nearest: Option<(Microns, usize)> = None;

        // 1. Scan downwards (toward bottom of board)
        let mut dist_down: Microns = 0;
        for i in (layer_idx + 1)..self.layers.len() {
            if self.layers[i].layer_type == LayerType::InternalPlane {
                nearest = Some((dist_down, i));
                break;
            } else if self.layers[i].layer_type == LayerType::Signal {
                // Another signal layer encountered before plane - stop search in this direction
                break;
            } else {
                dist_down += self.layers[i].thickness;
            }
        }

        // 2. Scan upwards (toward top of board)
        let mut dist_up: Microns = 0;
        for i in (0..layer_idx).rev() {
            if self.layers[i].layer_type == LayerType::InternalPlane {
                match nearest {
                    Some((best_dist, _)) if dist_up < best_dist => {
                        nearest = Some((dist_up, i));
                    }
                    None => {
                        nearest = Some((dist_up, i));
                    }
                    _ => {}
                }
                break;
            } else if self.layers[i].layer_type == LayerType::Signal {
                break;
            } else {
                dist_up += self.layers[i].thickness;
            }
        }

        nearest.map(|(_, idx)| idx)
    }

    /// Get total dielectric height between a signal layer and its reference plane.
    pub fn get_dielectric_height_to_plane(
        &self,
        signal_layer_idx: usize,
        plane_layer_idx: usize,
    ) -> Option<Microns> {
        if signal_layer_idx >= self.layers.len() || plane_layer_idx >= self.layers.len() {
            return None;
        }

        let (start, end) = if signal_layer_idx < plane_layer_idx {
            (signal_layer_idx + 1, plane_layer_idx)
        } else {
            (plane_layer_idx + 1, signal_layer_idx)
        };

        let height: Microns = self.layers[start..end]
            .iter()
            .filter(|l| l.layer_type.is_dielectric())
            .map(|l| l.thickness)
            .sum();

        Some(height)
    }

    /// Get the effective relative permittivity (Dk) of the dielectric between two layers.
    pub fn get_effective_er_between(
        &self,
        layer_a_idx: usize,
        layer_b_idx: usize,
    ) -> Option<f64> {
        if layer_a_idx >= self.layers.len() || layer_b_idx >= self.layers.len() {
            return None;
        }

        let (start, end) = if layer_a_idx < layer_b_idx {
            (layer_a_idx + 1, layer_b_idx)
        } else {
            (layer_b_idx + 1, layer_a_idx)
        };

        let dielectrics: Vec<&LayerDefinition> = self.layers[start..end]
            .iter()
            .filter(|l| l.layer_type.is_dielectric())
            .collect();

        if dielectrics.is_empty() {
            return None;
        }

        // Weighted harmonic average of Er across dielectric layers
        let total_h: Microns = dielectrics.iter().map(|d| d.thickness).sum();
        if total_h == 0 {
            return Some(1.0);
        }

        let weighted_inv_er: f64 = dielectrics
            .iter()
            .map(|d| (d.thickness as f64) / d.material.dielectric_constant)
            .sum();

        Some((total_h as f64) / weighted_inv_er)
    }

    /// Industry-standard 4-layer 1.6mm (63 mil) JLC/PCBWay stackup:
    /// L1 (Top Signal, 35µm) -> Prepreg 7628 (200µm) -> L2 (GND Plane, 35µm) ->
    /// Core FR-4 (1065µm) -> L3 (PWR Plane, 35µm) -> Prepreg 7628 (200µm) -> L4 (Bottom Signal, 35µm).
    pub fn standard_4layer_1_6mm() -> Self {
        let fr4 = MaterialProperties::fr4_standard();
        let layers = vec![
            LayerDefinition::new_solder_mask("Top Solder Mask", 20),
            LayerDefinition::new_signal("Top Layer (L1)", COPPER_1OZ_MICRONS),
            LayerDefinition::new_prepreg("Prepreg 7628", 200, fr4.clone()),
            LayerDefinition::new_plane("Inner GND (L2)", COPPER_1OZ_MICRONS),
            LayerDefinition::new_core("Core FR-4", 1065, fr4.clone()),
            LayerDefinition::new_plane("Inner PWR (L3)", COPPER_1OZ_MICRONS),
            LayerDefinition::new_prepreg("Prepreg 7628", 200, fr4),
            LayerDefinition::new_signal("Bottom Layer (L4)", COPPER_1OZ_MICRONS),
            LayerDefinition::new_solder_mask("Bottom Solder Mask", 20),
        ];
        Self { layers }
    }

    /// High-speed 6-layer 1.6mm stackup with RO4350B high-frequency top dielectric:
    /// L1 (High-Speed Signal) -> RO4350B (100µm) -> L2 (GND Plane) -> Core (400µm) ->
    /// L3 (Signal) -> Prepreg (400µm) -> L4 (PWR Plane) -> Core (400µm) -> L5 (GND) -> Prepreg (100µm) -> L6 (Signal).
    pub fn high_speed_6layer_rogers() -> Self {
        let rogers = MaterialProperties::rogers_ro4350b();
        let fr4 = MaterialProperties::fr4_high_tg();
        let layers = vec![
            LayerDefinition::new_solder_mask("Top Solder Mask", 15),
            LayerDefinition::new_signal("Top RF Signal (L1)", COPPER_1OZ_MICRONS),
            LayerDefinition::new_prepreg("Rogers RO4350B", 100, rogers),
            LayerDefinition::new_plane("GND Plane 1 (L2)", COPPER_1OZ_MICRONS),
            LayerDefinition::new_core("Core FR-4", 400, fr4.clone()),
            LayerDefinition::new_signal("Mid Signal (L3)", COPPER_HALF_OZ_MICRONS),
            LayerDefinition::new_prepreg("Prepreg FR-4", 400, fr4.clone()),
            LayerDefinition::new_plane("PWR Plane (L4)", COPPER_HALF_OZ_MICRONS),
            LayerDefinition::new_core("Core FR-4", 400, fr4.clone()),
            LayerDefinition::new_plane("GND Plane 2 (L5)", COPPER_1OZ_MICRONS),
            LayerDefinition::new_prepreg("Prepreg FR-4", 100, fr4),
            LayerDefinition::new_signal("Bottom Signal (L6)", COPPER_1OZ_MICRONS),
            LayerDefinition::new_solder_mask("Bottom Solder Mask", 15),
        ];
        Self { layers }
    }
}
