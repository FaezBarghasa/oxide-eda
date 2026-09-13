pub mod backend;
pub mod cpu_backend;
pub mod drc;
pub mod thermal;
pub mod congestion;
pub mod signal;
pub mod wgpu_backend;

pub use backend::{
    BackendPreference, BackendType, BufferId, ComputeBackend, ComputeError, PipelineId,
};
pub use cpu_backend::CpuBackend;
pub use drc::{DrcParams, GpuBBox, GpuDrcChecker, GpuViolation};
pub use thermal::{ThermalParams, ThermalResult, ThermalSimulator};
pub use congestion::{CongestionMapGenerator, CongestionParams, CongestionResult, GpuTrack};
pub use signal::{FdtdParams, FdtdSimulator, FieldCell};
pub use wgpu_backend::WgpuBackend;

/// Factory function to select and initialize the best available compute backend
pub fn create_backend(preference: BackendPreference) -> Box<dyn ComputeBackend> {
    match preference {
        BackendPreference::Auto => {
            if let Ok(wgpu_backend) = WgpuBackend::new() {
                log::info!("Initialized WGPU GPU compute backend: {}", wgpu_backend.name());
                return Box::new(wgpu_backend);
            }
            log::info!("Falling back to Rayon CPU compute backend");
            Box::new(CpuBackend::new())
        }
        BackendPreference::Wgpu => match WgpuBackend::new() {
            Ok(wgpu_backend) => Box::new(wgpu_backend),
            Err(e) => {
                log::warn!("Failed to initialize WGPU backend ({e}), falling back to CPU");
                Box::new(CpuBackend::new())
            }
        },
        BackendPreference::Cuda => {
            log::warn!("CUDA backend not configured on this target, falling back to CPU");
            Box::new(CpuBackend::new())
        }
        BackendPreference::Cpu => Box::new(CpuBackend::new()),
    }
}
