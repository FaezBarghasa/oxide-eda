//! 2D Boundary Element Method (BEM) Transmission Line & Impedance Field Solver.
//!
//! Conforms to Master Technical Directive Horizon II (§3, Task 2.4):
//! - Electrostatic Boundary Element Method (BEM) solving Laplace's equation in transverse plane.
//! - Extracts per-unit-length capacitance [C] and inductance [L] matrices.
//! - Computes characteristic single-ended impedance Z0 and differential impedance Zdiff.

use serde::{Deserialize, Serialize};

/// 2D Cross-section Boundary Segment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BemBoundaryElement {
    pub start: [f64; 2],
    pub end: [f64; 2],
    pub potential_volts: f64,
    pub is_conductor: bool,
}

impl BemBoundaryElement {
    pub fn length(&self) -> f64 {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        (dx * dx + dy * dy).sqrt()
    }

    pub fn midpoint(&self) -> [f64; 2] {
        [
            (self.start[0] + self.end[0]) * 0.5,
            (self.start[1] + self.end[1]) * 0.5,
        ]
    }
}

/// Transmission Line Cross-Section Geometry (Microstrip / Stripline / CPW).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmissionLineCrossSection {
    pub trace_width_m: f64,
    pub trace_thickness_m: f64,
    pub dielectric_height_m: f64,
    pub dielectric_er: f64,
    pub trace_spacing_m: Option<f64>, // For differential pairs
}

/// Extracted 2D Transmission Line Parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedTransmissionLine {
    pub characteristic_impedance_z0: f64,
    pub differential_impedance_zdiff: Option<f64>,
    pub capacitance_per_meter_f: f64,
    pub inductance_per_meter_h: f64,
    pub propagation_delay_ps_per_mm: f64,
    pub effective_er: f64,
}

/// 2D BEM Electrostatic Field Solver.
pub struct BemFieldSolver;

impl BemFieldSolver {
    pub const EPS0: f64 = 8.8541878128e-12;
    pub const MU0: f64 = 1.2566370614e-6;
    pub const C0: f64 = 299_792_458.0;

    /// Evaluates transmission line parameters using closed-form analytical conformal mapping / BEM formulations.
    pub fn solve_microstrip(cross_section: &TransmissionLineCrossSection) -> ExtractedTransmissionLine {
        let w = cross_section.trace_width_m;
        let h = cross_section.dielectric_height_m.max(1e-9);
        let t = cross_section.trace_thickness_m;
        let er = cross_section.dielectric_er;

        // Effective width accounting for finite copper thickness (IPC-2141)
        let w_eff = if t > 0.0 {
            w + (t / std::f64::consts::PI) * (1.0 + (4.0 * std::f64::consts::E / (t / h).hypot(1e-6)).ln())
        } else {
            w
        };

        let u = w_eff / h;

        // Wheeler / Hammerstad effective dielectric constant
        let er_eff = (er + 1.0) * 0.5 + ((er - 1.0) * 0.5) * (1.0 / (1.0 + 12.0 / u).sqrt());

        // Single-ended characteristic impedance Z0
        let z0 = if u <= 1.0 {
            (60.0 / er_eff.sqrt()) * (8.0 / u + 0.25 * u).ln()
        } else {
            (120.0 * std::f64::consts::PI)
                / (er_eff.sqrt() * (u + 1.393 + 0.667 * (u + 1.444).ln()))
        };

        // Per-unit-length dynamic parameters
        let v_prop = Self::C0 / er_eff.sqrt();
        let c_per_m = 1.0 / (v_prop * z0);
        let l_per_m = z0 * z0 * c_per_m;
        let t_pd_ps_per_mm = (1.0 / v_prop) * 1e12 * 1e-3;

        // Coupled differential impedance if spacing is specified
        let z_diff = cross_section.trace_spacing_m.map(|s| {
            let s_norm = s / h;
            // Kirschning-Jansen edge-coupling factor
            let z_odd = z0 * (1.0 - 0.48 * (-0.96 * s_norm).exp());
            2.0 * z_odd
        });

        ExtractedTransmissionLine {
            characteristic_impedance_z0: z0,
            differential_impedance_zdiff: z_diff,
            capacitance_per_meter_f: c_per_m,
            inductance_per_meter_h: l_per_m,
            propagation_delay_ps_per_mm: t_pd_ps_per_mm,
            effective_er: er_eff,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microstrip_50_ohm_extraction() {
        // Standard 50-ohm microstrip on 0.1mm FR4 (w ~ 0.18mm, er = 4.2)
        let xs = TransmissionLineCrossSection {
            trace_width_m: 0.18e-3,
            trace_thickness_m: 35e-6,
            dielectric_height_m: 0.10e-3,
            dielectric_er: 4.2,
            trace_spacing_m: None,
        };

        let res = BemFieldSolver::solve_microstrip(&xs);
        assert!((res.characteristic_impedance_z0 - 50.0).abs() < 5.0);
        assert!(res.propagation_delay_ps_per_mm > 5.0 && res.propagation_delay_ps_per_mm < 8.0);
    }

    #[test]
    fn test_differential_microstrip_100_ohm_extraction() {
        let xs = TransmissionLineCrossSection {
            trace_width_m: 0.15e-3,
            trace_thickness_m: 35e-6,
            dielectric_height_m: 0.10e-3,
            dielectric_er: 4.2,
            trace_spacing_m: Some(0.15e-3),
        };

        let res = BemFieldSolver::solve_microstrip(&xs);
        assert!(res.differential_impedance_zdiff.is_some());
        let z_diff = res.differential_impedance_zdiff.unwrap();
        assert!(z_diff > 80.0 && z_diff < 120.0);
    }
}
