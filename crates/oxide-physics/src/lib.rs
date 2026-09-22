pub mod clearance_3d;
pub mod hdi;
pub mod impedance;
pub mod kinematics;
pub mod material;
pub mod stackup;
pub mod units;

pub use clearance_3d::{
    Aabb3d, Body3d, ClearanceEngine3d, ClearanceViolation3d, Vec3,
};
pub use hdi::{HdiError, ViaDefinition, ViaType, check_hdi_rules};
pub use impedance::ImpedanceCalculator;
pub use kinematics::{
    BendLine, ClearanceContact, KinematicSubstrate, Matrix4x4, RigidFlexKinematicEngine,
    SubstrateZone,
};
pub use material::MaterialProperties;
pub use stackup::{CopperWeight, LayerDefinition, LayerStackup, LayerType};
pub use units::{
    Microns, Nanometers, microns_to_mils, microns_to_mm, mils_to_microns, mm_to_microns,
};
