use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComputeError {
    #[error("No compatible GPU adapter found")]
    NoGpuAdapter,

    #[error("GPU device creation failed: {0}")]
    DeviceCreation(String),

    #[error("Buffer allocation error: {0}")]
    BufferAllocation(String),

    #[error("Buffer not found: {0:?}")]
    BufferNotFound(BufferId),

    #[error("Pipeline not found: {0:?}")]
    PipelineNotFound(PipelineId),

    #[error("Pipeline creation error: {0}")]
    PipelineCreation(String),

    #[error("Dispatch failed: {0}")]
    DispatchFailed(String),

    #[error("Synchronization failed: {0}")]
    Synchronization(String),

    #[error("CUDA error: {0}")]
    CudaError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(pub u64);

impl fmt::Display for BufferId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BufferId({})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PipelineId(pub u64);

impl fmt::Display for PipelineId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PipelineId({})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    Wgpu,
    Cuda,
    Cpu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendPreference {
    Auto,
    Wgpu,
    Cuda,
    Cpu,
}

/// GPU & CPU compute backend abstraction
pub trait ComputeBackend: Send + Sync {
    fn backend_type(&self) -> BackendType;
    fn name(&self) -> &str;
    fn supports_compute(&self) -> bool;

    fn allocate_buffer_raw(&mut self, size_bytes: usize, data: Option<&[u8]>) -> Result<BufferId, ComputeError>;
    fn upload_raw(&mut self, buffer: BufferId, data: &[u8]) -> Result<(), ComputeError>;
    fn download_raw(&mut self, buffer: BufferId, out_bytes: &mut [u8]) -> Result<(), ComputeError>;

    fn create_compute_pipeline(&mut self, id: PipelineId, shader_source: &str, entry_point: &str) -> Result<(), ComputeError>;
    fn dispatch(&mut self, pipeline: PipelineId, workgroups: [u32; 3], bindings: &[BufferId]) -> Result<(), ComputeError>;
    fn synchronize(&mut self) -> Result<(), ComputeError>;
}

impl dyn ComputeBackend {
    pub fn allocate_buffer<T: bytemuck::Pod>(&mut self, data: &[T]) -> Result<BufferId, ComputeError> {
        let bytes = bytemuck::cast_slice(data);
        self.allocate_buffer_raw(bytes.len(), Some(bytes))
    }

    pub fn allocate_uninit_buffer<T: bytemuck::Pod>(&mut self, count: usize) -> Result<BufferId, ComputeError> {
        let size_bytes = count * std::mem::size_of::<T>();
        self.allocate_buffer_raw(size_bytes, None)
    }

    pub fn upload<T: bytemuck::Pod>(&mut self, buffer: BufferId, data: &[T]) -> Result<(), ComputeError> {
        let bytes = bytemuck::cast_slice(data);
        self.upload_raw(buffer, bytes)
    }

    pub fn download<T: bytemuck::Pod>(&mut self, buffer: BufferId, count: usize) -> Result<Vec<T>, ComputeError> {
        let mut result = vec![T::zeroed(); count];
        let bytes = bytemuck::cast_slice_mut(&mut result);
        self.download_raw(buffer, bytes)?;
        Ok(result)
    }
}
