use bytemuck::{Pod, Zeroable};
use rayon::prelude::*;

use crate::backend::{BackendType, ComputeBackend, PipelineId};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct ThermalParams {
    pub grid_width: u32,
    pub grid_height: u32,
    pub ambient_temp: f32,
    pub dt: f32,
    pub copper_conductivity: f32,
    pub fr4_conductivity: f32,
    pub _pad0: u32,
    pub _pad1: u32,
}

#[derive(Debug, Clone)]
pub struct ThermalResult {
    pub temperature_grid: Vec<f32>,
    pub max_temp: f32,
    pub min_temp: f32,
    pub grid_width: u32,
    pub grid_height: u32,
}

pub struct ThermalSimulator {
    backend: Box<dyn ComputeBackend>,
    pipeline_id: PipelineId,
    grid_width: u32,
    grid_height: u32,
    ambient_temp: f32,
    dt: f32,
    copper_conductivity: f32,
    fr4_conductivity: f32,
}

impl ThermalSimulator {
    pub fn new(mut backend: Box<dyn ComputeBackend>, grid_width: u32, grid_height: u32) -> Self {
        let pipeline_id = PipelineId(201);
        let shader = include_str!("shaders/thermal.wgsl");
        let _ = backend.create_compute_pipeline(pipeline_id, shader, "main");

        Self {
            backend,
            pipeline_id,
            grid_width,
            grid_height,
            ambient_temp: 25.0,
            dt: 0.001,
            copper_conductivity: 400.0,
            fr4_conductivity: 0.3,
        }
    }

    pub fn set_properties(&mut self, ambient_temp: f32, dt: f32, copper_k: f32, fr4_k: f32) {
        self.ambient_temp = ambient_temp;
        self.dt = dt;
        self.copper_conductivity = copper_k;
        self.fr4_conductivity = fr4_k;
    }

    pub fn simulate(
        &mut self,
        power_map: &[f32],
        material_map: &[u32],
        iterations: u32,
    ) -> Result<ThermalResult, crate::backend::ComputeError> {
        let grid_size = (self.grid_width * self.grid_height) as usize;
        let temp_current = vec![self.ambient_temp; grid_size];

        if self.backend.backend_type() == BackendType::Cpu {
            return Ok(self.simulate_cpu(power_map, material_map, iterations));
        }

        let temp_a = self.backend.allocate_buffer(&temp_current)?;
        let temp_b = self.backend.allocate_buffer(&temp_current)?;
        let power_buf = self.backend.allocate_buffer(power_map)?;
        let material_buf = self.backend.allocate_buffer(material_map)?;

        let params = ThermalParams {
            grid_width: self.grid_width,
            grid_height: self.grid_height,
            ambient_temp: self.ambient_temp,
            dt: self.dt,
            copper_conductivity: self.copper_conductivity,
            fr4_conductivity: self.fr4_conductivity,
            _pad0: 0,
            _pad1: 0,
        };
        let params_buf = self.backend.allocate_buffer(&[params])?;

        let workgroups_x = self.grid_width.div_ceil(16);
        let workgroups_y = self.grid_height.div_ceil(16);

        for i in 0..iterations {
            let (src, dst) = if i.is_multiple_of(2) {
                (temp_a, temp_b)
            } else {
                (temp_b, temp_a)
            };

            self.backend.dispatch(
                self.pipeline_id,
                [workgroups_x, workgroups_y, 1],
                &[src, dst, power_buf, material_buf, params_buf],
            )?;
        }

        self.backend.synchronize()?;

        let final_buf = if iterations.is_multiple_of(2) {
            temp_a
        } else {
            temp_b
        };
        let final_temp = self.backend.download::<f32>(final_buf, grid_size)?;

        let mut max_temp = f32::MIN;
        let mut min_temp = f32::MAX;
        for &t in &final_temp {
            if t > max_temp {
                max_temp = t;
            }
            if t < min_temp {
                min_temp = t;
            }
        }

        Ok(ThermalResult {
            temperature_grid: final_temp,
            max_temp,
            min_temp,
            grid_width: self.grid_width,
            grid_height: self.grid_height,
        })
    }

    pub fn simulate_cpu(
        &self,
        power_map: &[f32],
        material_map: &[u32],
        iterations: u32,
    ) -> ThermalResult {
        let width = self.grid_width as usize;
        let height = self.grid_height as usize;
        let grid_size = width * height;

        let mut current = vec![self.ambient_temp; grid_size];
        let mut next = vec![self.ambient_temp; grid_size];

        for _ in 0..iterations {
            next.par_chunks_mut(width).enumerate().for_each(|(y, row)| {
                for (x, cell_out) in row.iter_mut().enumerate().take(width) {
                    let idx = y * width + x;
                    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                        *cell_out = self.ambient_temp;
                        continue;
                    }

                    let left = current[idx - 1];
                    let right = current[idx + 1];
                    let up = current[idx - width];
                    let down = current[idx + width];
                    let center = current[idx];

                    let k = if material_map[idx] == 1 {
                        self.copper_conductivity
                    } else {
                        self.fr4_conductivity
                    };

                    let laplacian = left + right + up + down - 4.0 * center;
                    let heat_source = power_map[idx];

                    *cell_out = center + self.dt * (k * laplacian + heat_source);
                }
            });

            std::mem::swap(&mut current, &mut next);
        }

        let mut max_temp = f32::MIN;
        let mut min_temp = f32::MAX;
        for &t in &current {
            if t > max_temp {
                max_temp = t;
            }
            if t < min_temp {
                min_temp = t;
            }
        }

        ThermalResult {
            temperature_grid: current,
            max_temp,
            min_temp,
            grid_width: self.grid_width,
            grid_height: self.grid_height,
        }
    }
}
