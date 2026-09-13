use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::backend::{BackendType, BufferId, ComputeBackend, ComputeError, PipelineId};

static NEXT_BUFFER_ID: AtomicU64 = AtomicU64::new(1000);

pub struct CpuBackend {
    buffers: HashMap<BufferId, Vec<u8>>,
    pipelines: HashMap<PipelineId, String>,
}

impl Default for CpuBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuBackend {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            pipelines: HashMap::new(),
        }
    }
}

impl ComputeBackend for CpuBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Cpu
    }

    fn name(&self) -> &str {
        "CPU (Rayon multithreaded)"
    }

    fn supports_compute(&self) -> bool {
        true
    }

    fn allocate_buffer_raw(&mut self, size_bytes: usize, data: Option<&[u8]>) -> Result<BufferId, ComputeError> {
        let buf = match data {
            Some(d) => d.to_vec(),
            None => vec![0u8; size_bytes],
        };
        let id = BufferId(NEXT_BUFFER_ID.fetch_add(1, Ordering::Relaxed));
        self.buffers.insert(id, buf);
        Ok(id)
    }

    fn upload_raw(&mut self, buffer_id: BufferId, data: &[u8]) -> Result<(), ComputeError> {
        let entry = self.buffers.get_mut(&buffer_id).ok_or(ComputeError::BufferNotFound(buffer_id))?;
        if entry.len() < data.len() {
            entry.resize(data.len(), 0);
        }
        entry[..data.len()].copy_from_slice(data);
        Ok(())
    }

    fn download_raw(&mut self, buffer_id: BufferId, out_bytes: &mut [u8]) -> Result<(), ComputeError> {
        let entry = self.buffers.get(&buffer_id).ok_or(ComputeError::BufferNotFound(buffer_id))?;
        let len = out_bytes.len().min(entry.len());
        out_bytes[..len].copy_from_slice(&entry[..len]);
        Ok(())
    }

    fn create_compute_pipeline(
        &mut self,
        id: PipelineId,
        _shader_source: &str,
        entry_point: &str,
    ) -> Result<(), ComputeError> {
        self.pipelines.insert(id, entry_point.to_string());
        Ok(())
    }

    fn dispatch(
        &mut self,
        pipeline_id: PipelineId,
        _workgroups: [u32; 3],
        _bindings: &[BufferId],
    ) -> Result<(), ComputeError> {
        if !self.pipelines.contains_key(&pipeline_id) {
            return Err(ComputeError::PipelineNotFound(pipeline_id));
        }
        // CPU backend delegates domain computations to optimized Rayon parallel loops in domain structs.
        Ok(())
    }

    fn synchronize(&mut self) -> Result<(), ComputeError> {
        Ok(())
    }
}
