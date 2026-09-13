use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use wgpu::util::DeviceExt;

use crate::backend::{BackendType, BufferId, ComputeBackend, ComputeError, PipelineId};

static NEXT_BUFFER_ID: AtomicU64 = AtomicU64::new(1);

struct BufferEntry {
    buffer: wgpu::Buffer,
    size: u64,
}

pub struct WgpuBackend {
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter_info: wgpu::AdapterInfo,
    buffers: HashMap<BufferId, BufferEntry>,
    pipelines: HashMap<PipelineId, wgpu::ComputePipeline>,
    bind_group_layouts: HashMap<PipelineId, wgpu::BindGroupLayout>,
}

impl WgpuBackend {
    pub fn new() -> Result<Self, ComputeError> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .map_err(|e| ComputeError::DeviceCreation(format!("Failed to acquire adapter: {e}")))?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("oxide-compute-device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            },
            None,
        ))
        .map_err(|e| ComputeError::DeviceCreation(e.to_string()))?;

        Ok(Self {
            adapter_info: adapter.get_info(),
            device,
            queue,
            buffers: HashMap::new(),
            pipelines: HashMap::new(),
            bind_group_layouts: HashMap::new(),
        })
    }

    pub fn from_device_and_queue(
        device: wgpu::Device,
        queue: wgpu::Queue,
        adapter_info: wgpu::AdapterInfo,
    ) -> Self {
        Self {
            device,
            queue,
            adapter_info,
            buffers: HashMap::new(),
            pipelines: HashMap::new(),
            bind_group_layouts: HashMap::new(),
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn adapter_info(&self) -> &wgpu::AdapterInfo {
        &self.adapter_info
    }
}

impl ComputeBackend for WgpuBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Wgpu
    }

    fn name(&self) -> &str {
        &self.adapter_info.name
    }

    fn supports_compute(&self) -> bool {
        true
    }

    fn allocate_buffer_raw(&mut self, size_bytes: usize, data: Option<&[u8]>) -> Result<BufferId, ComputeError> {
        let size = (size_bytes as u64).max(16);
        let buffer = match data {
            Some(d) => self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("compute-storage-buffer"),
                contents: d,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC
                    | wgpu::BufferUsages::UNIFORM,
            }),
            None => self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("compute-storage-buffer"),
                size,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC
                    | wgpu::BufferUsages::UNIFORM,
                mapped_at_creation: false,
            }),
        };

        let id = BufferId(NEXT_BUFFER_ID.fetch_add(1, Ordering::Relaxed));
        self.buffers.insert(id, BufferEntry { buffer, size });
        Ok(id)
    }

    fn upload_raw(&mut self, buffer_id: BufferId, data: &[u8]) -> Result<(), ComputeError> {
        let entry = self.buffers.get(&buffer_id).ok_or(ComputeError::BufferNotFound(buffer_id))?;
        self.queue.write_buffer(&entry.buffer, 0, data);
        Ok(())
    }

    fn download_raw(&mut self, buffer_id: BufferId, out_bytes: &mut [u8]) -> Result<(), ComputeError> {
        let entry = self.buffers.get(&buffer_id).ok_or(ComputeError::BufferNotFound(buffer_id))?;
        let read_size = out_bytes.len() as u64;

        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("compute-staging-download-buffer"),
            size: read_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("compute-download-encoder"),
        });

        encoder.copy_buffer_to_buffer(&entry.buffer, 0, &staging_buffer, 0, read_size);
        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });

        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());

        match receiver.recv() {
            Ok(Ok(())) => {
                let data = buffer_slice.get_mapped_range();
                out_bytes.copy_from_slice(&data[..out_bytes.len()]);
                drop(data);
                staging_buffer.unmap();
                Ok(())
            }
            Ok(Err(e)) => Err(ComputeError::Synchronization(format!("Map async failed: {e:?}"))),
            Err(e) => Err(ComputeError::Synchronization(format!("Channel receive error: {e}"))),
        }
    }

    fn create_compute_pipeline(
        &mut self,
        id: PipelineId,
        shader_source: &str,
        entry_point: &str,
    ) -> Result<(), ComputeError> {
        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("shader-{id}")),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(&format!("pipeline-{id}")),
            layout: None, // Auto layout based on shader reflection
            module: &shader,
            entry_point: Some(entry_point),
            compilation_options: Default::default(),
            cache: None,
        });

        let layout = pipeline.get_bind_group_layout(0);
        self.bind_group_layouts.insert(id, layout);
        self.pipelines.insert(id, pipeline);
        Ok(())
    }

    fn dispatch(
        &mut self,
        pipeline_id: PipelineId,
        workgroups: [u32; 3],
        bindings: &[BufferId],
    ) -> Result<(), ComputeError> {
        let pipeline = self
            .pipelines
            .get(&pipeline_id)
            .ok_or(ComputeError::PipelineNotFound(pipeline_id))?;
        let layout = self
            .bind_group_layouts
            .get(&pipeline_id)
            .ok_or(ComputeError::PipelineNotFound(pipeline_id))?;

        let mut entries = Vec::with_capacity(bindings.len());
        for (i, &buf_id) in bindings.iter().enumerate() {
            let entry = self
                .buffers
                .get(&buf_id)
                .ok_or(ComputeError::BufferNotFound(buf_id))?;
            entries.push(wgpu::BindGroupEntry {
                binding: i as u32,
                resource: entry.buffer.as_entire_binding(),
            });
        }

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compute-bind-group"),
            layout,
            entries: &entries,
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("compute-dispatch-encoder"),
        });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("compute-pass"),
                timestamp_writes: None,
            });
            cpass.set_pipeline(pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch_workgroups(workgroups[0], workgroups[1], workgroups[2]);
        }

        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    fn synchronize(&mut self) -> Result<(), ComputeError> {
        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());
        Ok(())
    }
}
