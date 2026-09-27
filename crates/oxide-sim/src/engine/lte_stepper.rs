//! Variable-Order Integration (Gear BDF 1-6 / Trapezoidal) & Milne Local Truncation Error (LTE) Controller.
//!
//! Conforms to Master Technical Directive Horizon I (§2, Task 1.2):
//! - Gear BDF Orders 1 through 6 and variable Trapezoidal integration.
//! - Milne predictor-corrector error estimation across dynamic storage nodes.
//! - Adaptive dynamic timestep bounds based on user-defined RELTOL and ABSTOL.

use serde::{Deserialize, Serialize};

/// Integration Scheme Kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationMethod {
    Trapezoidal,
    GearBdf(u8), // Order 1 to 6
}

/// Local Truncation Error (LTE) Stepper Controller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LteController {
    pub method: IntegrationMethod,
    pub reltol: f64,
    pub abstol: f64,
    pub min_dt: f64,
    pub max_dt: f64,
    pub current_dt: f64,
    pub current_order: u8,
}

impl Default for LteController {
    fn default() -> Self {
        Self {
            method: IntegrationMethod::Trapezoidal,
            reltol: 1e-3,
            abstol: 1e-6,
            min_dt: 1e-15,
            max_dt: 1e-3,
            current_dt: 1e-9,
            current_order: 2,
        }
    }
}

impl LteController {
    pub fn new(reltol: f64, abstol: f64, initial_dt: f64) -> Self {
        Self {
            method: IntegrationMethod::Trapezoidal,
            reltol: reltol.clamp(1e-9, 1e-1),
            abstol: abstol.clamp(1e-15, 1e-3),
            min_dt: 1e-15,
            max_dt: 1.0,
            current_dt: initial_dt.max(1e-15),
            current_order: 2,
        }
    }

    /// Evaluates Milne predictor-corrector error and determines next adaptive timestep $h_{\text{next}}$.
    pub fn compute_next_timestep(
        &mut self,
        v_current: &[f64],
        v_predicted: &[f64],
        dt_current: f64,
    ) -> (f64, f64) {
        let n = v_current.len();
        if n == 0 || v_predicted.len() != n {
            return (dt_current, 0.0);
        }

        let mut max_scaled_error: f64 = 0.0;
        let p = self.current_order as f64;

        for i in 0..n {
            let lte_error = (v_current[i] - v_predicted[i]).abs();
            let tol = self.reltol * v_current[i].abs() + self.abstol;
            let scaled_error = if tol > 0.0 { lte_error / tol } else { 0.0 };
            if scaled_error > max_scaled_error {
                max_scaled_error = scaled_error;
            }
        }

        let scale_factor = if max_scaled_error > 1e-15 {
            0.9 * (1.0 / max_scaled_error).powf(1.0 / (p + 1.0))
        } else {
            2.0
        };

        let clamped_scale = scale_factor.clamp(0.1, 2.0);
        let mut next_dt = (dt_current * clamped_scale).clamp(self.min_dt, self.max_dt);

        // If step error is excessive, clamp next_dt strictly lower
        if max_scaled_error > 1.0 {
            next_dt = (dt_current * 0.5).max(self.min_dt);
        }

        self.current_dt = next_dt;
        (next_dt, max_scaled_error)
    }

    /// Companion dynamic coefficients $\alpha_0, \beta_0$ for BDF / Trapezoidal discretizations.
    pub fn integration_coefficients(&self, dt: f64) -> (f64, f64) {
        match self.method {
            IntegrationMethod::Trapezoidal => {
                // (2.0 / dt) conductance stamp coefficient
                (2.0 / dt.max(1e-15), 1.0)
            }
            IntegrationMethod::GearBdf(order) => {
                let alpha = match order {
                    1 => 1.0 / dt,
                    2 => 1.5 / dt,
                    3 => (11.0 / 6.0) / dt,
                    4 => (25.0 / 12.0) / dt,
                    5 => (137.0 / 60.0) / dt,
                    _ => (147.0 / 60.0) / dt,
                };
                (alpha, 1.0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lte_timestep_adaptation() {
        let mut controller = LteController::new(1e-3, 1e-6, 1e-6);
        let v_curr = vec![1.0, 5.0, 0.5];
        // Predicted is very close: error is small, dt should expand
        let v_pred = vec![1.00001, 5.00002, 0.50001];

        let (next_dt, error) = controller.compute_next_timestep(&v_curr, &v_pred, 1e-6);
        assert!(error < 0.1);
        assert!(next_dt > 1e-6);
    }

    #[test]
    fn test_lte_timestep_reduction_on_large_error() {
        let mut controller = LteController::new(1e-3, 1e-6, 1e-6);
        let v_curr = vec![1.0, 5.0];
        // Predicted has huge divergence: error > 1.0, dt should reduce
        let v_pred = vec![1.5, 7.0];

        let (next_dt, error) = controller.compute_next_timestep(&v_curr, &v_pred, 1e-6);
        assert!(error > 1.0);
        assert!(next_dt < 1e-6);
    }
}
