//! Characteristic and differential impedance calculations based on IPC-2141 and Hammerstad models.

use crate::stackup::LayerStackup;
use crate::units::Microns;

/// Characteristic and differential impedance calculation engine.
#[derive(Debug, Clone, Copy, Default)]
pub struct ImpedanceCalculator;

impl ImpedanceCalculator {
    /// Calculate characteristic impedance (Z0 in Ohms) for a surface microstrip trace.
    ///
    /// Implements standard IPC-2141 equation:
    /// $$Z_0 = \frac{87}{\sqrt{\varepsilon_r + 1.41}} \ln\left( \frac{5.98 \cdot h}{0.8 \cdot w + t} \right)$$
    ///
    /// # Arguments
    /// * `trace_width` ($w$) - Width of the copper trace in micrometers.
    /// * `dielectric_height` ($h$) - Height of the dielectric to the reference plane in micrometers.
    /// * `copper_thickness` ($t$) - Thickness of the copper trace in micrometers (e.g. 35 µm for 1 oz).
    /// * `er` ($\varepsilon_r$) - Relative dielectric constant of the substrate (e.g. 4.4 for FR-4).
    pub fn calculate_microstrip(
        trace_width: Microns,
        dielectric_height: Microns,
        copper_thickness: Microns,
        er: f64,
    ) -> f64 {
        let w = trace_width as f64;
        let h = dielectric_height as f64;
        let t = copper_thickness as f64;

        if w <= 0.0 || h <= 0.0 || er <= 1.0 {
            return 0.0;
        }

        let denom = 0.8 * w + t;
        if denom <= 0.0 {
            return 0.0;
        }

        let ratio = (5.98 * h) / denom;
        if ratio <= 1.0 {
            return 0.0;
        }

        let numerator = 87.0;
        let sqrt_er = (er + 1.41).sqrt();

        (numerator / sqrt_er) * ratio.ln()
    }

    /// Calculate characteristic impedance (Z0 in Ohms) for an embedded / coated microstrip.
    ///
    /// Extends surface microstrip with a protective solder mask or dielectric coating of height $h_1$:
    /// $$\varepsilon_{r,\text{eff}} = \varepsilon_r \cdot \left(1 - e^{-1.55 \cdot h_1 / h}\right) + e^{-1.55 \cdot h_1 / h}$$
    pub fn calculate_embedded_microstrip(
        trace_width: Microns,
        dielectric_height: Microns,
        coating_height: Microns,
        copper_thickness: Microns,
        er: f64,
        coating_er: f64,
    ) -> f64 {
        let base_z0 =
            Self::calculate_microstrip(trace_width, dielectric_height, copper_thickness, er);
        let h = dielectric_height as f64;
        let h1 = coating_height as f64;

        if h <= 0.0 || h1 <= 0.0 {
            return base_z0;
        }

        // Coating reduction factor (IPC-2141 §5.2.2)
        let decay = (-1.55 * (h1 / h)).exp();
        let eff_factor = (coating_er / er).sqrt() * (1.0 - decay) + decay;

        base_z0 / eff_factor.max(1.0)
    }

    /// Calculate characteristic impedance (Z0 in Ohms) for a symmetrical stripline trace.
    ///
    /// Implements standard IPC-2141 equation:
    /// $$Z_0 = \frac{60}{\sqrt{\varepsilon_r}} \ln\left( \frac{1.9 \cdot b}{0.8 \cdot w + t} \right)$$
    ///
    /// # Arguments
    /// * `trace_width` ($w$) - Width of the copper trace in micrometers.
    /// * `total_plane_separation` ($b$) - Distance between ground/power reference planes in micrometers.
    /// * `copper_thickness` ($t$) - Thickness of the copper trace in micrometers.
    /// * `er` ($\varepsilon_r$) - Relative dielectric constant of the core/prepreg substrate.
    pub fn calculate_stripline(
        trace_width: Microns,
        total_plane_separation: Microns,
        copper_thickness: Microns,
        er: f64,
    ) -> f64 {
        let w = trace_width as f64;
        let b = total_plane_separation as f64;
        let t = copper_thickness as f64;

        if w <= 0.0 || b <= 0.0 || er <= 1.0 {
            return 0.0;
        }

        let denom = 0.8 * w + t;
        if denom <= 0.0 {
            return 0.0;
        }

        let ratio = (1.9 * b) / denom;
        if ratio <= 1.0 {
            return 0.0;
        }

        (60.0 / er.sqrt()) * ratio.ln()
    }

    /// Calculate edge-coupled differential microstrip impedance ($Z_{\text{diff}}$ in Ohms).
    ///
    /// $$Z_{\text{diff}} \approx 2 \cdot Z_0 \cdot \left(1 - 0.48 \cdot e^{-0.96 \cdot \frac{s}{h}}\right)$$
    ///
    /// # Arguments
    /// * `single_ended_z0` - Characteristic single-ended impedance of one trace in Ohms.
    /// * `trace_gap` ($s$) - Edge-to-edge separation between the pair traces in micrometers.
    /// * `dielectric_height` ($h$) - Height of the dielectric to the reference plane in micrometers.
    pub fn calculate_differential_microstrip(
        single_ended_z0: f64,
        trace_gap: Microns,
        dielectric_height: Microns,
    ) -> f64 {
        let s = trace_gap as f64;
        let h = dielectric_height as f64;

        if s <= 0.0 || h <= 0.0 {
            return single_ended_z0;
        }

        let coupling_factor = 1.0 - 0.48 * (-0.96 * (s / h)).exp();
        2.0 * single_ended_z0 * coupling_factor
    }

    /// Calculate edge-coupled differential stripline impedance ($Z_{\text{diff}}$ in Ohms).
    ///
    /// $$Z_{\text{diff}} \approx 2 \cdot Z_0 \cdot \left(1 - 0.347 \cdot e^{-2.9 \cdot \frac{s}{b}}\right)$$
    pub fn calculate_differential_stripline(
        single_ended_z0: f64,
        trace_gap: Microns,
        total_plane_separation: Microns,
    ) -> f64 {
        let s = trace_gap as f64;
        let b = total_plane_separation as f64;

        if s <= 0.0 || b <= 0.0 {
            return single_ended_z0;
        }

        let coupling_factor = 1.0 - 0.347 * (-2.9 * (s / b)).exp();
        2.0 * single_ended_z0 * coupling_factor
    }

    /// Invert the microstrip formula to solve for the required trace width (in micrometers)
    /// to achieve a target single-ended characteristic impedance $Z_0$.
    pub fn solve_microstrip_width(
        target_z0: f64,
        dielectric_height: Microns,
        copper_thickness: Microns,
        er: f64,
    ) -> Option<Microns> {
        if target_z0 <= 10.0 || target_z0 >= 200.0 {
            return None;
        }

        // Binary search between 10 µm (0.4 mil) and 2000 µm (80 mil)
        let mut low: Microns = 10;
        let mut high: Microns = 2000;

        for _ in 0..24 {
            let mid = (low + high) / 2;
            let current_z0 =
                Self::calculate_microstrip(mid, dielectric_height, copper_thickness, er);

            if (current_z0 - target_z0).abs() < 0.05 {
                return Some(mid);
            }

            // Microstrip impedance decreases as width increases
            if current_z0 > target_z0 {
                low = mid;
            } else {
                high = mid;
            }
        }

        Some((low + high) / 2)
    }

    /// Calculate single-ended impedance of a trace on a specific layer in a `LayerStackup`.
    pub fn calculate_trace_impedance_on_stackup(
        stackup: &LayerStackup,
        signal_layer_idx: usize,
        trace_width: Microns,
    ) -> Option<f64> {
        let ref_plane_idx = stackup.get_reference_plane(signal_layer_idx)?;
        let dielectric_h =
            stackup.get_dielectric_height_to_plane(signal_layer_idx, ref_plane_idx)?;
        let er = stackup.get_effective_er_between(signal_layer_idx, ref_plane_idx)?;
        let copper_t = stackup.layers[signal_layer_idx].thickness;

        // If this is an external layer (first or last copper), compute microstrip;
        // if inner layer, compute stripline or dual-reference.
        let copper_indices = stackup.copper_layer_indices();
        let is_outer = copper_indices.first() == Some(&signal_layer_idx)
            || copper_indices.last() == Some(&signal_layer_idx);

        if is_outer {
            Some(Self::calculate_microstrip(
                trace_width,
                dielectric_h,
                copper_t,
                er,
            ))
        } else {
            // For inner layers, total dielectric plane separation is approx 2 * h
            Some(Self::calculate_stripline(
                trace_width,
                dielectric_h * 2,
                copper_t,
                er,
            ))
        }
    }
}
