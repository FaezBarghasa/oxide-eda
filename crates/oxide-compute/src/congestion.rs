use bytemuck::{Pod, Zeroable};
use rayon::prelude::*;

use crate::backend::{BackendType, ComputeBackend, PipelineId};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct CongestionParams {
    pub grid_width: u32,
    pub grid_height: u32,
    pub num_tracks: u32,
    pub cell_size: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct GpuTrack {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub layer: u32,
    pub width: f32,
    pub net_id: u32,
    pub _pad: u32,
}

#[derive(Debug, Clone)]
pub struct CongestionResult {
    pub grid: Vec<u32>,
    pub max_density: u32,
    pub grid_width: u32,
    pub grid_height: u32,
    pub cell_size: f32,
}

pub struct CongestionMapGenerator {
    backend: Box<dyn ComputeBackend>,
    pipeline_id: PipelineId,
    grid_width: u32,
    grid_height: u32,
    cell_size: f32,
}

impl CongestionMapGenerator {
    pub fn new(
        mut backend: Box<dyn ComputeBackend>,
        grid_width: u32,
        grid_height: u32,
        cell_size: f32,
    ) -> Self {
        let pipeline_id = PipelineId(301);
        let shader = include_str!("shaders/congestion.wgsl");
        let _ = backend.create_compute_pipeline(pipeline_id, shader, "main");

        Self {
            backend,
            pipeline_id,
            grid_width,
            grid_height,
            cell_size,
        }
    }

    pub fn compute_congestion(
        &mut self,
        tracks: &[GpuTrack],
    ) -> Result<CongestionResult, crate::backend::ComputeError> {
        let grid_size = (self.grid_width * self.grid_height) as usize;

        if self.backend.backend_type() == BackendType::Cpu {
            return Ok(self.compute_congestion_cpu(tracks));
        }

        let tracks_buf = self.backend.allocate_buffer(tracks)?;
        let init_grid = vec![0u32; grid_size];
        let grid_buf = self.backend.allocate_buffer(&init_grid)?;

        let params = CongestionParams {
            grid_width: self.grid_width,
            grid_height: self.grid_height,
            num_tracks: tracks.len() as u32,
            cell_size: self.cell_size,
        };
        let params_buf = self.backend.allocate_buffer(&[params])?;

        let workgroups = ((tracks.len() as u32) + 255) / 256;
        self.backend.dispatch(
            self.pipeline_id,
            [workgroups.max(1), 1, 1],
            &[tracks_buf, grid_buf, params_buf],
        )?;

        self.backend.synchronize()?;

        let result_grid: Vec<u32> = self.backend.download(grid_buf, grid_size)?;
        let max_density = result_grid.iter().copied().max().unwrap_or(0);

        Ok(CongestionResult {
            grid: result_grid,
            max_density,
            grid_width: self.grid_width,
            grid_height: self.grid_height,
            cell_size: self.cell_size,
        })
    }

    pub fn compute_congestion_cpu(&self, tracks: &[GpuTrack]) -> CongestionResult {
        let width = self.grid_width as usize;
        let height = self.grid_height as usize;
        let grid_size = width * height;
        let cell_size = self.cell_size;

        // Thread-local accumulation grids
        let aggregated_grid = tracks
            .par_chunks(256)
            .map(|chunk| {
                let mut local = vec![0u32; grid_size];
                for track in chunk {
                    let gx0 = ((track.x0 / cell_size).max(0.0) as usize).min(width.saturating_sub(1));
                    let gy0 = ((track.y0 / cell_size).max(0.0) as usize).min(height.saturating_sub(1));
                    let gx1 = ((track.x1 / cell_size).max(0.0) as usize).min(width.saturating_sub(1));
                    let gy1 = ((track.y1 / cell_size).max(0.0) as usize).min(height.saturating_sub(1));

                    let dx = (gx1 as isize) - (gx0 as isize);
                    let dy = (gy1 as isize) - (gy0 as isize);
                    let steps = dx.abs().max(dy.abs()) as usize;

                    if steps == 0 {
                        local[gy0 * width + gx0] += 1;
                        continue;
                    }

                    for s in 0..=steps {
                        let t = (s as f32) / (steps as f32);
                        let x = ((gx0 as f32) + t * (dx as f32)).round() as usize;
                        let y = ((gy0 as f32) + t * (dy as f32)).round() as usize;

                        if x < width && y < height {
                            local[y * width + x] += 1;
                        }
                    }
                }
                local
            })
            .reduce(
                || vec![0u32; grid_size],
                |mut a, b| {
                    for (va, vb) in a.iter_mut().zip(b.iter()) {
                        *va += *vb;
                    }
                    a
                },
            );

        let max_density = aggregated_grid.iter().copied().max().unwrap_or(0);

        CongestionResult {
            grid: aggregated_grid,
            max_density,
            grid_width: self.grid_width,
            grid_height: self.grid_height,
            cell_size: self.cell_size,
        }
    }
}
