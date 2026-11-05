//! Power Delivery Network (PDN) DC IR-Drop & Planar Resistor Mesh Solver (Phase 5.3).
//!
//! Models multi-layer copper power and ground planes as 2D finite-difference resistive meshes.
//! Solves nodal potentials under arbitrary DC load distributions and VRM sources using
//! iterative Poisson-Laplace relaxation.

use serde::{Deserialize, Serialize};

/// Node in the 2D planar resistive mesh.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PdnNode {
    pub x_mm: f64,
    pub y_mm: f64,
    pub voltage_v: f64,
    pub is_voltage_source: bool,
    pub current_sink_a: f64,
}

/// Summary statistics of PDN DC voltage drop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdnIrDropStats {
    pub nominal_voltage_v: f64,
    pub min_voltage_v: f64,
    pub max_drop_mv: f64,
    pub avg_drop_mv: f64,
    pub worst_location_mm: (f64, f64),
}

/// 2D Resistive Mesh PDN Solver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdnResistorMesh {
    pub width_mm: f64,
    pub height_mm: f64,
    pub grid_step_mm: f64,
    pub cols: usize,
    pub rows: usize,
    pub nominal_voltage_v: f64,
    pub copper_thickness_um: f64,
    pub sheet_resistance_ohms_per_sq: f64,
    pub nodes: Vec<PdnNode>,
}

impl PdnResistorMesh {
    /// Creates a uniform rectangular power plane resistive grid.
    ///
    /// Copper resistivity $\rho \approx 1.72 \times 10^{-8}\,\Omega\cdot\text{m}$.
    /// Sheet resistance $R_{\text{sq}} = \frac{\rho}{t}$.
    pub fn new(
        width_mm: f64,
        height_mm: f64,
        grid_step_mm: f64,
        nominal_voltage_v: f64,
        copper_thickness_um: f64,
    ) -> Self {
        let cols = (width_mm / grid_step_mm).ceil() as usize + 1;
        let rows = (height_mm / grid_step_mm).ceil() as usize + 1;

        let t_meters = (copper_thickness_um * 1e-6).max(1e-9);
        let sheet_r = 1.72e-8 / t_meters; // Ohms per square

        let mut nodes = Vec::with_capacity(cols * rows);
        for r in 0..rows {
            for c in 0..cols {
                nodes.push(PdnNode {
                    x_mm: (c as f64) * grid_step_mm,
                    y_mm: (r as f64) * grid_step_mm,
                    voltage_v: nominal_voltage_v,
                    is_voltage_source: false,
                    current_sink_a: 0.0,
                });
            }
        }

        Self {
            width_mm,
            height_mm,
            grid_step_mm,
            cols,
            rows,
            nominal_voltage_v,
            copper_thickness_um,
            sheet_resistance_ohms_per_sq: sheet_r,
            nodes,
        }
    }

    #[inline]
    fn idx(&self, col: usize, row: usize) -> usize {
        row * self.cols + col
    }

    /// Add a VRM voltage source terminal.
    pub fn set_voltage_source(&mut self, col: usize, row: usize, voltage_v: f64) {
        if col < self.cols && row < self.rows {
            let i = self.idx(col, row);
            self.nodes[i].voltage_v = voltage_v;
            self.nodes[i].is_voltage_source = true;
        }
    }

    /// Add a load current sink at a node.
    pub fn add_current_sink(&mut self, col: usize, row: usize, current_a: f64) {
        if col < self.cols && row < self.rows {
            let i = self.idx(col, row);
            self.nodes[i].current_sink_a += current_a;
        }
    }

    /// Iterative Gauss-Seidel solver for 2D Poisson voltage distribution:
    /// $\nabla^2 V = R_{\text{sq}} \cdot I_{\text{sink}}$
    pub fn solve_dc(&mut self, iterations: usize) {
        let r_cell = self.sheet_resistance_ohms_per_sq;

        for _ in 0..iterations {
            for r in 0..self.rows {
                for c in 0..self.cols {
                    let idx = self.idx(c, r);
                    if self.nodes[idx].is_voltage_source {
                        continue;
                    }

                    let mut neighbor_sum = 0.0;
                    let mut neighbor_count = 0.0;

                    if c > 0 {
                        neighbor_sum += self.nodes[self.idx(c - 1, r)].voltage_v;
                        neighbor_count += 1.0;
                    }
                    if c + 1 < self.cols {
                        neighbor_sum += self.nodes[self.idx(c + 1, r)].voltage_v;
                        neighbor_count += 1.0;
                    }
                    if r > 0 {
                        neighbor_sum += self.nodes[self.idx(c, r - 1)].voltage_v;
                        neighbor_count += 1.0;
                    }
                    if r + 1 < self.rows {
                        neighbor_sum += self.nodes[self.idx(c, r + 1)].voltage_v;
                        neighbor_count += 1.0;
                    }

                    if neighbor_count > 0.0 {
                        let i_sink = self.nodes[idx].current_sink_a;
                        // Nodal KCL relaxation
                        self.nodes[idx].voltage_v =
                            (neighbor_sum - (i_sink * r_cell)) / neighbor_count;
                    }
                }
            }
        }
    }

    /// Calculate aggregate IR-drop statistics.
    pub fn calculate_ir_drop_stats(&self) -> PdnIrDropStats {
        let mut min_v = self.nominal_voltage_v;
        let mut max_drop_mv = 0.0;
        let mut sum_drop_mv = 0.0;
        let mut worst_loc = (0.0, 0.0);

        for node in &self.nodes {
            let drop_mv = ((self.nominal_voltage_v - node.voltage_v) * 1e3).max(0.0);
            if node.voltage_v < min_v {
                min_v = node.voltage_v;
                worst_loc = (node.x_mm, node.y_mm);
            }
            if drop_mv > max_drop_mv {
                max_drop_mv = drop_mv;
            }
            sum_drop_mv += drop_mv;
        }

        let count = self.nodes.len().max(1) as f64;
        PdnIrDropStats {
            nominal_voltage_v: self.nominal_voltage_v,
            min_voltage_v: min_v,
            max_drop_mv,
            avg_drop_mv: sum_drop_mv / count,
            worst_location_mm: worst_loc,
        }
    }

    /// Generate false-color RGBA8 heatmap buffer.
    pub fn generate_heatmap_rgba(&self) -> Vec<u8> {
        let mut rgba = Vec::with_capacity(self.cols * self.rows * 4);

        for node in &self.nodes {
            let drop_mv = ((self.nominal_voltage_v - node.voltage_v) * 1e3).max(0.0);
            let (r, g, b) = Self::drop_color(drop_mv);
            rgba.extend_from_slice(&[r, g, b, 255]);
        }

        rgba
    }

    fn drop_color(drop_mv: f64) -> (u8, u8, u8) {
        if drop_mv < 10.0 {
            // Nominal green (#2ECC71)
            (46, 204, 113)
        } else if drop_mv < 30.0 {
            // Warning yellow
            (241, 196, 15)
        } else {
            // High drop red
            (231, 76, 60)
        }
    }
}

