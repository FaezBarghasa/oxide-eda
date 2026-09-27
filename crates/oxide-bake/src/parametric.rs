use oxide_library::harvester::PackageDimensions;
use oxide_library::primitive::footprint::Footprint;
use thiserror::Error;

use crate::ipc7351::{DensityLevel, Ipc7351Generator, Package3DExtruder};

#[derive(Error, Debug)]
pub enum BakeError {
    #[error("Failed to synthesize footprint: {0}")]
    Synthesis(String),
    #[error("Unsupported package class: {0}")]
    UnsupportedPackage(String),
}

/// Parametric Footprint Synthesizer trait.
pub trait FootprintSynthesizer: Send + Sync {
    /// Mathematically compiles package dimensions into an IPC-7351C footprint.
    fn synthesize_footprint(
        &self,
        dimensions: &PackageDimensions,
        density: DensityLevel,
    ) -> Result<Footprint, BakeError>;

    /// Procedurally extrudes a 3D boundary representation when no STEP file is supplied.
    fn synthesize_3d_body(
        &self,
        dimensions: &PackageDimensions,
    ) -> Result<Vec<u8>, BakeError>;
}

/// Default IPC-7351C synthesizer engine.
#[derive(Debug, Default, Clone, Copy)]
pub struct Ipc7351Synthesizer;

impl FootprintSynthesizer for Ipc7351Synthesizer {
    fn synthesize_footprint(
        &self,
        dimensions: &PackageDimensions,
        density: DensityLevel,
    ) -> Result<Footprint, BakeError> {
        Ok(Ipc7351Generator::generate_footprint(
            &dimensions.package_class,
            dimensions,
            density,
        ))
    }

    fn synthesize_3d_body(
        &self,
        dimensions: &PackageDimensions,
    ) -> Result<Vec<u8>, BakeError> {
        Ok(Package3DExtruder::synthesize_mesh_obj(dimensions))
    }
}
