//! Physical stackup modeling, IPC-2141 impedance calculator, and HDI via verification for Oxide EDA.

pub mod hdi;
pub mod impedance;
pub mod material;
pub mod stackup;
pub mod units;

pub use hdi::{HdiError, ViaDefinition, ViaType, check_hdi_rules};
pub use impedance::ImpedanceCalculator;
pub use material::MaterialProperties;
pub use stackup::{CopperWeight, LayerDefinition, LayerStackup, LayerType};
pub use units::{
    Microns, Nanometers, microns_to_mils, microns_to_mm, mils_to_microns, mm_to_microns,
};
