//! Physical stackup modeling, IPC-2141 impedance calculator, and HDI via verification for Oxide EDA.

pub mod hdi;
pub mod impedance;
pub mod material;
pub mod stackup;
pub mod units;

pub use hdi::{check_hdi_rules, HdiError, ViaDefinition, ViaType};
pub use impedance::ImpedanceCalculator;
pub use material::MaterialProperties;
pub use stackup::{CopperWeight, LayerDefinition, LayerStackup, LayerType};
pub use units::{
    microns_to_mils, microns_to_mm, mils_to_microns, mm_to_microns, Microns, Nanometers,
};
