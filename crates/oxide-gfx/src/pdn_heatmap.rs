//! Power Delivery Network (PDN) DC IR Drop & Current Density Heatmap Generator.
//!
//! Models copper plane resistive meshes and solves per-node potential drops
//! to synthesize false-colour voltage gradient textures (Green < 5mV -> Yellow -> Red > 50mV drop).

/// Node in the 2D planar resistive mesh.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PdnMeshNode {
    pub x_mm: f64,
    pub y_mm: f64,
    pub voltage_v: f64,
    pub current_injection_a: f64,
}

/// 2D Resistive Grid for PDN DC Drop Analysis.
#[derive(Debug, Clone)]
pub struct PdnMeshGrid {
    pub cols: usize,
    pub rows: usize,
    pub step_mm: f64,
    pub nominal_voltage_v: f64,
    pub copper_thickness_um: f64,
    pub sheet_resistance_mohm_per_sq: f64,
    pub nodes: Vec<PdnMeshNode>,
}

impl PdnMeshGrid {
    /// Creates a uniform resistive mesh for a power plane rectangular area.
    pub fn new(
        width_mm: f64,
        height_mm: f64,
        step_mm: f64,
        nominal_voltage_v: f64,
        copper_thickness_um: f64,
    ) -> Self {
        let cols = (width_mm / step_mm).ceil() as usize + 1;
        let rows = (height_mm / step_mm).ceil() as usize + 1;

        // Copper resistivity ρ ≈ 1.72e-8 Ω·m
        // R_sq = ρ / thickness
        let sheet_resistance = (1.72e-8 / (copper_thickness_um * 1e-6)) * 1e3; // mΩ / sq

        let mut nodes = Vec::with_capacity(cols * rows);
        for r in 0..rows {
            for c in 0..cols {
                nodes.push(PdnMeshNode {
                    x_mm: (c as f64) * step_mm,
                    y_mm: (r as f64) * step_mm,
                    voltage_v: nominal_voltage_v,
                    current_injection_a: 0.0,
                });
            }
        }

        Self {
            cols,
            rows,
            step_mm,
            nominal_voltage_v,
            copper_thickness_um,
            sheet_resistance_mohm_per_sq: sheet_resistance,
            nodes,
        }
    }

    /// Sets VRM voltage source location.
    pub fn set_voltage_source(&mut self, col: usize, row: usize, voltage: f64) {
        if col < self.cols && row < self.rows {
            let idx = row * self.cols + col;
            self.nodes[idx].voltage_v = voltage;
        }
    }

    /// Sets load current sink (negative injection).
    pub fn add_current_sink(&mut self, col: usize, row: usize, current_a: f64) {
        if col < self.cols && row < self.rows {
            let idx = row * self.cols + col;
            self.nodes[idx].current_injection_a -= current_a;
        }
    }

    /// Iterative Gauss-Seidel solver for 2D Poisson potential distribution: $\nabla^2 V = R_{sq} \cdot J$
    pub fn solve_dc_drop(&mut self, iterations: usize) {
        let r_inter = (self.sheet_resistance_mohm_per_sq * 1e-3).max(1e-6);

        for _ in 0..iterations {
            for r in 1..(self.rows - 1) {
                for c in 1..(self.cols - 1) {
                    let idx = r * self.cols + c;
                    let i_inj = self.nodes[idx].current_injection_a;

                    let v_left = self.nodes[idx - 1].voltage_v;
                    let v_right = self.nodes[idx + 1].voltage_v;
                    let v_up = self.nodes[idx - self.cols].voltage_v;
                    let v_down = self.nodes[idx + self.cols].voltage_v;

                    // Standard 5-point finite difference stencil
                    self.nodes[idx].voltage_v =
                        0.25 * (v_left + v_right + v_up + v_down) + 0.25 * (i_inj * r_inter);
                }
            }
        }
    }

    /// Generates RGBA8 heatmap texture:
    /// Green: < 5mV drop (Acceptable)
    /// Yellow: 5mV - 25mV drop (Marginal)
    /// Red: > 50mV drop (Critical)
    pub fn to_rgba8_heatmap(&self) -> Vec<u8> {
        let mut rgba = Vec::with_capacity(self.cols * self.rows * 4);

        for node in &self.nodes {
            let drop_mv = ((self.nominal_voltage_v - node.voltage_v) * 1e3).max(0.0);
            let (r, g, b) = Self::voltage_drop_colormap(drop_mv);
            rgba.extend_from_slice(&[r, g, b, 255]);
        }

        rgba
    }

    fn voltage_drop_colormap(drop_mv: f64) -> (u8, u8, u8) {
        if drop_mv < 5.0 {
            // Green (#2ECC71)
            (46, 204, 113)
        } else if drop_mv < 25.0 {
            // Yellow (#F1C40F)
            let factor = (drop_mv - 5.0) / 20.0;
            let r = (46.0 + factor * (241.0 - 46.0)) as u8;
            let g = (204.0 + factor * (196.0 - 204.0)) as u8;
            let b = (113.0 + factor * (15.0 - 113.0)) as u8;
            (r, g, b)
        } else {
            // Red (#E74C3C)
            let factor = ((drop_mv - 25.0) / 25.0).clamp(0.0, 1.0);
            let r = (241.0 + factor * (231.0 - 241.0)) as u8;
            let g = (196.0 - factor * (196.0 - 76.0)) as u8;
            let b = (15.0 + factor * (60.0 - 15.0)) as u8;
            (r, g, b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdn_dc_ir_drop_solver() {
        let mut grid = PdnMeshGrid::new(20.0, 20.0, 2.0, 3.3, 35.0);
        grid.set_voltage_source(0, 0, 3.3);
        grid.add_current_sink(10, 10, 2.0); // 2A sink at corner

        grid.solve_dc_drop(50);

        let rgba = grid.to_rgba8_heatmap();
        assert_eq!(rgba.len(), grid.cols * grid.rows * 4);
    }
}
