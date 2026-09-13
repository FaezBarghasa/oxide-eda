use bytemuck::{Pod, Zeroable};

use crate::backend::{BackendType, ComputeBackend, PipelineId};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct FieldCell {
    pub ex: f32,
    pub ey: f32,
    pub ez: f32,
    pub hx: f32,
    pub hy: f32,
    pub hz: f32,
    pub _pad0: f32,
    pub _pad1: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct FdtdParams {
    pub grid_x: u32,
    pub grid_y: u32,
    pub grid_z: u32,
    pub dt: f32,
    pub dx: f32,
    pub epsilon: f32,
    pub mu: f32,
    pub _pad: u32,
}

pub struct FdtdSimulator {
    backend: Box<dyn ComputeBackend>,
    pipeline_id: PipelineId,
    grid_x: u32,
    grid_y: u32,
    grid_z: u32,
    dt: f32,
    dx: f32,
    epsilon: f32,
    mu: f32,
}

impl FdtdSimulator {
    pub fn new(
        mut backend: Box<dyn ComputeBackend>,
        grid_x: u32,
        grid_y: u32,
        grid_z: u32,
        dx: f32,
    ) -> Self {
        let pipeline_id = PipelineId(401);
        let shader = include_str!("shaders/fdtd.wgsl");
        let _ = backend.create_compute_pipeline(pipeline_id, shader, "main");

        // Courant stability condition dt <= dx / (c * sqrt(3))
        let c = 3.0e8; // speed of light m/s
        let dt = (dx / (c * 1.732)) * 0.9;
        let eps0 = 8.854e-12;
        let mu0 = 1.2566e-6;

        Self {
            backend,
            pipeline_id,
            grid_x,
            grid_y,
            grid_z,
            dt,
            dx,
            epsilon: eps0,
            mu: mu0,
        }
    }

    pub fn step(
        &mut self,
        fields: &mut [FieldCell],
        steps: u32,
    ) -> Result<(), crate::backend::ComputeError> {
        let total_cells = (self.grid_x * self.grid_y * self.grid_z) as usize;
        if fields.len() < total_cells {
            return Err(crate::backend::ComputeError::BufferAllocation(format!(
                "fields buffer size {} is smaller than required grid size {}",
                fields.len(),
                total_cells
            )));
        }

        if self.backend.backend_type() == BackendType::Cpu {
            self.step_cpu(fields, steps);
            return Ok(());
        }

        let fields_buf = self.backend.allocate_buffer(fields)?;
        let params = FdtdParams {
            grid_x: self.grid_x,
            grid_y: self.grid_y,
            grid_z: self.grid_z,
            dt: self.dt,
            dx: self.dx,
            epsilon: self.epsilon,
            mu: self.mu,
            _pad: 0,
        };
        let params_buf = self.backend.allocate_buffer(&[params])?;

        let wg_x = (self.grid_x + 3) / 4;
        let wg_y = (self.grid_y + 3) / 4;
        let wg_z = (self.grid_z + 3) / 4;

        for _ in 0..steps {
            self.backend.dispatch(
                self.pipeline_id,
                [wg_x, wg_y, wg_z],
                &[fields_buf, params_buf],
            )?;
        }

        self.backend.synchronize()?;

        let updated: Vec<FieldCell> = self.backend.download(fields_buf, total_cells)?;
        fields.copy_from_slice(&updated);
        Ok(())
    }

    pub fn step_cpu(&self, fields: &mut [FieldCell], steps: u32) {
        let gx = self.grid_x as usize;
        let gy = self.grid_y as usize;
        let gz = self.grid_z as usize;
        let inv_dx = 1.0 / self.dx;
        let inv_eps = 1.0 / self.epsilon;

        for _ in 0..steps {
            for z in 1..gz {
                for y in 1..gy {
                    for x in 1..gx {
                        let idx = z * gx * gy + y * gx + x;
                        let idx_xm = z * gx * gy + y * gx + (x - 1);
                        let idx_ym = z * gx * gy + (y - 1) * gx + x;
                        let idx_zm = (z - 1) * gx * gy + y * gx + x;

                        let cell = fields[idx];
                        let cell_xm = fields[idx_xm];
                        let cell_ym = fields[idx_ym];
                        let cell_zm = fields[idx_zm];

                        let dhz_dy = (cell.hz - cell_ym.hz) * inv_dx;
                        let dhy_dz = (cell.hy - cell_zm.hy) * inv_dx;
                        let dhx_dz = (cell.hx - cell_zm.hx) * inv_dx;
                        let dhz_dx = (cell.hz - cell_xm.hz) * inv_dx;
                        let dhy_dx = (cell.hy - cell_xm.hy) * inv_dx;
                        let dhx_dy = (cell.hx - cell_ym.hx) * inv_dx;

                        fields[idx].ex += self.dt * inv_eps * (dhz_dy - dhy_dz);
                        fields[idx].ey += self.dt * inv_eps * (dhx_dz - dhz_dx);
                        fields[idx].ez += self.dt * inv_eps * (dhy_dx - dhx_dy);
                    }
                }
            }
        }
    }
}
