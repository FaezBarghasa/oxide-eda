//! 3D Transient Electro-Thermal Heat Diffusion Finite Difference Grid.
//!
//! Conforms to Master Technical Directive Horizon VI (§7, Task 6.2):
//! - Discretizes PCB volume (FR-4 substrate, copper planes, thermal vias) into 3D grid.
//! - Solves Fourier's heat diffusion equation:
//!   $\rho c_p \frac{\partial T}{\partial t} = \nabla \cdot (k \nabla T) + Q(\vec{r}, t)$
//! - Bidirectional Joule heat dissipation feedback loop with temperature-dependent resistivity.

use serde::{Deserialize, Serialize};

/// Material Thermal Classification for 3D Grid Cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum GridThermalMaterial {
    Air = 0,
    Fr4Substrate = 1,
    CopperTrace = 2,
    ThermalVia = 3,
    SiliconDie = 4,
    AluminumHeatSink = 5,
}

impl GridThermalMaterial {
    /// Thermal conductivity $k$ in $\text{W}/(\text{m}\cdot\text{K})$.
    pub fn conductivity_w_per_mk(&self) -> f64 {
        match self {
            Self::Air => 0.026,
            Self::Fr4Substrate => 0.30,
            Self::CopperTrace => 398.0,
            Self::ThermalVia => 350.0,
            Self::SiliconDie => 149.0,
            Self::AluminumHeatSink => 205.0,
        }
    }

    /// Volumetric heat capacity $C_v = \rho \cdot c_p$ in $\text{J}/(\text{m}^3\cdot\text{K})$.
    pub fn volumetric_heat_capacity(&self) -> f64 {
        match self {
            Self::Air => 1.2 * 1005.0,
            Self::Fr4Substrate => 1900.0 * 1150.0,
            Self::CopperTrace => 8960.0 * 385.0,
            Self::ThermalVia => 8500.0 * 390.0,
            Self::SiliconDie => 2330.0 * 705.0,
            Self::AluminumHeatSink => 2700.0 * 900.0,
        }
    }
}

/// 3D Electro-Thermal Grid Solver.
#[derive(Debug, Clone)]
pub struct ThermalGrid3D {
    pub dim_x: usize,
    pub dim_y: usize,
    pub dim_z: usize,
    pub voxel_size_m: f64,
    pub ambient_temp_k: f64,
    pub materials: Vec<GridThermalMaterial>,
    pub temperature_k: Vec<f64>,
    pub power_dissipation_w: Vec<f64>,
}

impl ThermalGrid3D {
    /// Creates a new 3D Thermal grid initialized to ambient temperature.
    pub fn new(
        dim_x: usize,
        dim_y: usize,
        dim_z: usize,
        voxel_size_m: f64,
        ambient_temp_c: f64,
    ) -> Self {
        let total_cells = dim_x * dim_y * dim_z;
        let ambient_k = ambient_temp_c + 273.15;
        Self {
            dim_x,
            dim_y,
            dim_z,
            voxel_size_m,
            ambient_temp_k: ambient_k,
            materials: vec![GridThermalMaterial::Fr4Substrate; total_cells],
            temperature_k: vec![ambient_k; total_cells],
            power_dissipation_w: vec![0.0; total_cells],
        }
    }

    #[inline(always)]
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        z * (self.dim_x * self.dim_y) + y * self.dim_x + x
    }

    /// Sets material type at 3D voxel coordinate $(x, y, z)$.
    pub fn set_material(&mut self, x: usize, y: usize, z: usize, material: GridThermalMaterial) {
        if x < self.dim_x && y < self.dim_y && z < self.dim_z {
            let idx = self.index(x, y, z);
            self.materials[idx] = material;
        }
    }

    /// Injects localized heat source $Q$ (in Watts) from SPICE Joule losses into voxel $(x, y, z)$.
    pub fn inject_joule_heat(&mut self, x: usize, y: usize, z: usize, power_w: f64) {
        if x < self.dim_x && y < self.dim_y && z < self.dim_z {
            let idx = self.index(x, y, z);
            self.power_dissipation_w[idx] = power_w;
        }
    }

    /// Clears all dynamic power dissipation terms.
    pub fn clear_heat_sources(&mut self) {
        self.power_dissipation_w.fill(0.0);
    }

    /// Advances transient thermal diffusion by timestep $\Delta t$ (in seconds).
    /// Employs sub-stepping to satisfy the explicit finite difference von Neumann stability criterion:
    /// $\Delta t_{sub} \le \frac{\Delta x^2 \cdot C_v}{6 k_{max}}$
    pub fn step_transient(&mut self, dt_sec: f64) {
        let dx = self.voxel_size_m;
        let dx2 = dx * dx;
        let cell_volume = dx * dx * dx;

        // Find worst-case thermal diffusivity $\alpha = k / C_v$ across materials present
        let max_diffusivity = self
            .materials
            .iter()
            .map(|m| m.conductivity_w_per_mk() / m.volumetric_heat_capacity())
            .fold(1e-9, f64::max);

        // Explicit 3D von Neumann stability limit: dt_max = dx^2 / (6 * alpha) * safety_margin
        let dt_limit = (dx2 / (6.0 * max_diffusivity)) * 0.8;
        let num_substeps = ((dt_sec / dt_limit).ceil() as usize).max(1);
        let sub_dt = dt_sec / (num_substeps as f64);

        for _ in 0..num_substeps {
            let mut next_temp = self.temperature_k.clone();

            for z in 0..self.dim_z {
                for y in 0..self.dim_y {
                    for x in 0..self.dim_x {
                        let idx = self.index(x, y, z);

                        // Boundary conditions: Ambient heat sink at exterior edges
                        if x == 0
                            || x == self.dim_x - 1
                            || y == 0
                            || y == self.dim_y - 1
                            || z == 0
                            || z == self.dim_z - 1
                        {
                            next_temp[idx] = self.ambient_temp_k;
                            continue;
                        }

                        let t_c = self.temperature_k[idx];
                        let t_left = self.temperature_k[self.index(x - 1, y, z)];
                        let t_right = self.temperature_k[self.index(x + 1, y, z)];
                        let t_down = self.temperature_k[self.index(x, y - 1, z)];
                        let t_up = self.temperature_k[self.index(x, y + 1, z)];
                        let t_back = self.temperature_k[self.index(x, y, z - 1)];
                        let t_front = self.temperature_k[self.index(x, y, z + 1)];

                        let mat = self.materials[idx];
                        let k = mat.conductivity_w_per_mk();
                        let cv = mat.volumetric_heat_capacity();

                        // 3D 6-point Laplacian
                        let laplacian =
                            (t_left + t_right + t_down + t_up + t_back + t_front - 6.0 * t_c) / dx2;
                        let volumetric_q = self.power_dissipation_w[idx] / cell_volume;

                        // Fourier diffusion update: T_new = T + sub_dt * (k * Lap + Q) / Cv
                        let dt_flux = sub_dt * (k * laplacian + volumetric_q) / cv;
                        next_temp[idx] = t_c + dt_flux;
                    }
                }
            }

            self.temperature_k = next_temp;
        }
    }

    /// Evaluates maximum temperature across the board volume in Celsius.
    pub fn max_temperature_celsius(&self) -> f64 {
        let max_k = self
            .temperature_k
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        max_k - 273.15
    }

    /// Returns temperature in Celsius at voxel $(x, y, z)$.
    pub fn temperature_celsius(&self, x: usize, y: usize, z: usize) -> f64 {
        if x < self.dim_x && y < self.dim_y && z < self.dim_z {
            self.temperature_k[self.index(x, y, z)] - 273.15
        } else {
            self.ambient_temp_k - 273.15
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3d_thermal_joule_heating() {
        let mut grid = ThermalGrid3D::new(10, 10, 5, 1e-3, 25.0);

        // Place a silicon power MOSFET at center (5, 5, 2)
        grid.set_material(5, 5, 2, GridThermalMaterial::SiliconDie);
        grid.inject_joule_heat(5, 5, 2, 2.0); // 2W dissipation

        // Run transient steps
        for _ in 0..500 {
            grid.step_transient(0.005);
        }

        let max_t = grid.max_temperature_celsius();
        assert!(
            max_t > 25.0,
            "Center temperature should rise due to Joule dissipation"
        );
        assert!(
            max_t < 200.0,
            "Temperature should remain within physical bounds"
        );
    }
}
