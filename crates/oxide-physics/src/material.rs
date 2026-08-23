//! Dielectric and conductive material catalog for PCB layer stackup modeling.

use serde::{Deserialize, Serialize};

/// Physical and electrical characteristics of a PCB substrate or conductor material.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialProperties {
    /// Commercial or generic material name (e.g., "FR-4 Standard", "Rogers RO4350B").
    pub name: String,
    /// Relative dielectric constant (Dk / εr) at 1 GHz.
    pub dielectric_constant: f64,
    /// Dielectric loss tangent (Df / tan δ) at 1 GHz.
    pub loss_tangent: f64,
    /// Thermal conductivity in W/(m·K).
    pub thermal_conductivity: f64,
}

impl MaterialProperties {
    pub const fn new(
        name: String,
        dielectric_constant: f64,
        loss_tangent: f64,
        thermal_conductivity: f64,
    ) -> Self {
        Self {
            name,
            dielectric_constant,
            loss_tangent,
            thermal_conductivity,
        }
    }

    /// Standard FR-4 epoxy glass substrate (Dk ≈ 4.4, Df ≈ 0.020).
    pub fn fr4_standard() -> Self {
        Self {
            name: "FR-4 Standard".into(),
            dielectric_constant: 4.4,
            loss_tangent: 0.020,
            thermal_conductivity: 0.3,
        }
    }

    /// High-Tg FR-4 (e.g. Isola 370HR / Shengyi S1000-2, Dk ≈ 4.0, Df ≈ 0.015).
    pub fn fr4_high_tg() -> Self {
        Self {
            name: "FR-4 High-Tg (370HR)".into(),
            dielectric_constant: 4.0,
            loss_tangent: 0.015,
            thermal_conductivity: 0.4,
        }
    }

    /// High-frequency hydrocarbon ceramic laminate: Rogers RO4350B (Dk ≈ 3.66, Df ≈ 0.0037).
    pub fn rogers_ro4350b() -> Self {
        Self {
            name: "Rogers RO4350B".into(),
            dielectric_constant: 3.66,
            loss_tangent: 0.0037,
            thermal_conductivity: 0.69,
        }
    }

    /// PTFE composite high-frequency laminate: Rogers RT/duroid 5880 (Dk ≈ 2.20, Df ≈ 0.0009).
    pub fn rogers_rt5880() -> Self {
        Self {
            name: "Rogers RT/duroid 5880".into(),
            dielectric_constant: 2.20,
            loss_tangent: 0.0009,
            thermal_conductivity: 0.20,
        }
    }

    /// Flexible polyimide film (e.g. DuPont Pyralux / Kapton, Dk ≈ 3.4, Df ≈ 0.002).
    pub fn polyimide_flex() -> Self {
        Self {
            name: "Polyimide (Flex)".into(),
            dielectric_constant: 3.4,
            loss_tangent: 0.002,
            thermal_conductivity: 0.12,
        }
    }

    /// Liquid Photoimageable (LPI) Solder Mask (Dk ≈ 3.8, Df ≈ 0.025).
    pub fn solder_mask_lpi() -> Self {
        Self {
            name: "LPI Solder Mask".into(),
            dielectric_constant: 3.8,
            loss_tangent: 0.025,
            thermal_conductivity: 0.2,
        }
    }

    /// Pure electrodeposited (ED) copper foil (for conductors and planes).
    pub fn copper_conductor() -> Self {
        Self {
            name: "Copper Foil (ED)".into(),
            dielectric_constant: 1.0,
            loss_tangent: 0.0,
            thermal_conductivity: 398.0,
        }
    }
}
