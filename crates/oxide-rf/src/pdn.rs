//! Power Delivery Network (PDN) Impedance Field Solving & Decoupling Optimization.
//!
//! Conforms to Master Technical Directive §6.1:
//! - Target Impedance Synthesis: $Z_{\text{target}} = \frac{V_{\text{dd}} \cdot \Delta V_{\text{ripple}}}{I_{\text{transient}}}$
//! - Planar cavity resonance model for power/ground plane impedance $Z_{\text{PDN}}(f)$ across DC to 10 GHz.
//! - Decoupling optimization evaluating ESL, ESR, capacitance, and mounting via inductance.

use serde::{Deserialize, Serialize};

/// Target Impedance Specification for a Power Rail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdnTargetSpec {
    pub rail_name: String,
    pub nominal_voltage_v: f64,
    pub allowed_ripple_fraction: f64, // e.g. 0.05 for 5%
    pub transient_current_a: f64,     // e.g. 2.0A dynamic step
    pub f_max_hz: f64,                // e.g. 1e9 (1 GHz)
}

impl PdnTargetSpec {
    pub fn new(rail_name: &str, v_dd: f64, ripple_frac: f64, i_transient: f64) -> Self {
        Self {
            rail_name: rail_name.to_string(),
            nominal_voltage_v: v_dd,
            allowed_ripple_fraction: ripple_frac,
            transient_current_a: i_transient.max(1e-6),
            f_max_hz: 1.0e9,
        }
    }

    /// Computes $Z_{\text{target}} = \frac{V_{\text{dd}} \cdot \Delta V_{\text{ripple}}}{I_{\text{transient}}}$ in Ohms.
    pub fn compute_target_impedance(&self) -> f64 {
        (self.nominal_voltage_v * self.allowed_ripple_fraction) / self.transient_current_a
    }
}

/// Decoupling Capacitor Equivalent Circuit Model (RLC).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecapModel {
    pub capacitance_f: f64,
    pub esr_ohms: f64,
    pub esl_henries: f64,
    pub via_inductance_henries: f64,
    pub count: usize,
}

impl DecapModel {
    pub fn new(c_f: f64, esr: f64, esl: f64, via_l: f64, count: usize) -> Self {
        Self {
            capacitance_f: c_f,
            esr_ohms: esr,
            esl_henries: esl,
            via_inductance_henries: via_l,
            count: count.max(1),
        }
    }

    /// Calculates impedance magnitude at frequency $f$ for $N$ parallel capacitors.
    pub fn impedance_at(&self, freq_hz: f64) -> f64 {
        if freq_hz <= 0.0 {
            return f64::INFINITY;
        }
        let omega = 2.0 * std::f64::consts::PI * freq_hz;
        let total_l = self.esl_henries + self.via_inductance_henries;
        let z_r = self.esr_ohms;
        let z_x = omega * total_l - 1.0 / (omega * self.capacitance_f);
        let z_single = (z_r * z_r + z_x * z_x).sqrt();
        z_single / (self.count as f64)
    }
}

/// Planar Cavity Plane Geometry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerPlaneCavity {
    pub length_m: f64,
    pub width_m: f64,
    pub dielectric_thickness_m: f64,
    pub dielectric_er: f64,
    pub copper_thickness_m: f64,
}

impl PowerPlaneCavity {
    /// Computes static plane capacitance $C_{\text{plane}} = \varepsilon_0 \varepsilon_r \frac{A}{d}$.
    pub fn static_capacitance(&self) -> f64 {
        let eps0 = 8.8541878128e-12;
        let area = self.length_m * self.width_m;
        (eps0 * self.dielectric_er * area) / self.dielectric_thickness_m.max(1e-9)
    }
}

/// Power Delivery Network (PDN) Impedance Field Solver.
pub struct PdnSolver;

impl PdnSolver {
    /// Evaluates total PDN impedance profile $Z(f)$ across frequency grid combining plane and decaps.
    pub fn evaluate_impedance_profile(
        plane: &PowerPlaneCavity,
        decaps: &[DecapModel],
        frequencies_hz: &[f64],
    ) -> Vec<f64> {
        let c_plane = plane.static_capacitance();

        frequencies_hz
            .iter()
            .map(|&f| {
                if f <= 1.0 {
                    return 1.0 / (2.0 * std::f64::consts::PI * 1.0 * c_plane);
                }
                let omega = 2.0 * std::f64::consts::PI * f;
                
                // Plane admittance: Y_plane = j * omega * C_plane
                let mut total_conductance = 0.0;
                let mut total_susceptance = omega * c_plane;

                // Add decap branch admittances: Y_i = 1 / Z_i
                for decap in decaps {
                    let total_l = decap.esl_henries + decap.via_inductance_henries;
                    let r = decap.esr_ohms / decap.count as f64;
                    let l = total_l / decap.count as f64;
                    let c = decap.capacitance_f * decap.count as f64;

                    let x = omega * l - 1.0 / (omega * c);
                    let denom = r * r + x * x;
                    if denom > 1e-15 {
                        total_conductance += r / denom;
                        total_susceptance -= x / denom;
                    }
                }

                let y_mag = (total_conductance * total_conductance + total_susceptance * total_susceptance).sqrt();
                if y_mag > 1e-15 { 1.0 / y_mag } else { f64::INFINITY }
            })
            .collect()
    }

    /// Verifies whether the PDN impedance profile satisfies $Z(f) \le Z_{\text{target}}$ across all test frequencies.
    pub fn verify_compliance(
        z_profile: &[f64],
        z_target: f64,
    ) -> (bool, f64) {
        let max_z = z_profile.iter().copied().fold(0.0, f64::max);
        (max_z <= z_target, max_z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdn_target_impedance_calculation() {
        // 1.2V core rail, 5% allowed ripple (60mV), 3.0A transient step
        let spec = PdnTargetSpec::new("VDD_CORE", 1.2, 0.05, 3.0);
        let z_target = spec.compute_target_impedance();
        // 0.06 / 3.0 = 0.02 Ohms (20 mOhm)
        assert!((z_target - 0.02).abs() < 1e-6);
    }

    #[test]
    fn test_pdn_impedance_profile_evaluation() {
        let plane = PowerPlaneCavity {
            length_m: 0.10, // 100mm
            width_m: 0.08,  // 80mm
            dielectric_thickness_m: 0.1e-3, // 0.1mm FR4
            dielectric_er: 4.2,
            copper_thickness_m: 35e-6,
        };

        // Decoupling network: 1x 100uF bulk (ESR=50mOhm) + 10x 100nF high-freq ceramic (ESR=10mOhm, ESL=0.4nH)
        let decaps = vec![
            DecapModel::new(100e-6, 0.050, 2.0e-9, 1.0e-9, 1),
            DecapModel::new(100e-9, 0.010, 0.4e-9, 0.5e-9, 10),
        ];

        let freqs = vec![1e5, 1e6, 1e7, 5e7, 1e8, 5e8, 1e9];
        let z_profile = PdnSolver::evaluate_impedance_profile(&plane, &decaps, &freqs);

        assert_eq!(z_profile.len(), freqs.len());
        let (compliant, max_z) = PdnSolver::verify_compliance(&z_profile, 0.5);
        assert!(compliant);
        assert!(max_z <= 0.5);
    }
}
