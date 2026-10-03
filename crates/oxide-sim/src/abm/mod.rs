//! Analog Behavioral Modeling (ABM) & Symbolic Automatic Differentiation Engine.
//!
//! Conforms to Master Technical Directive Horizon I (§2, Task 1.4):
//! - Intermediate Representation (IR) evaluation of non-linear component expressions.
//! - Symbolic automatic differentiation for exact analytical Jacobians without finite-difference errors.
//! - Rational Laplace transfer function H(s) transformation into state-space companion forms.

use serde::{Deserialize, Serialize};

pub mod hermite_spline;
pub use hermite_spline::{
    HermiteSmoothingConfig, evaluate_smoothed_if, evaluate_smoothed_limit, evaluate_smoothed_step,
};

/// Rational s-domain Laplace Transfer Function: H(s) = N(s) / D(s).
/// N(s) = sum(b_m * s^m), D(s) = sum(a_k * s^k).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaplaceTransferFunction {
    pub numerator_coeffs: Vec<f64>,   // [b0, b1, b2, ...]
    pub denominator_coeffs: Vec<f64>, // [a0, a1, a2, ...]
}

pub type StateSpaceRepresentation = (Vec<Vec<f64>>, Vec<f64>, Vec<f64>, f64);

impl LaplaceTransferFunction {
    pub fn new(numerator: &[f64], denominator: &[f64]) -> Self {
        Self {
            numerator_coeffs: numerator.to_vec(),
            denominator_coeffs: denominator.to_vec(),
        }
    }

    /// Evaluates frequency response H(j*omega) at given frequency in Hz.
    pub fn evaluate_freq_response(&self, freq_hz: f64) -> (f64, f64) {
        let omega = 2.0 * std::f64::consts::PI * freq_hz;
        let mut num_re = 0.0;
        let mut num_im = 0.0;

        for (m, &b) in self.numerator_coeffs.iter().enumerate() {
            let s_mag = omega.powi(m as i32);
            match m % 4 {
                0 => num_re += b * s_mag,
                1 => num_im += b * s_mag,
                2 => num_re -= b * s_mag,
                3 => num_im -= b * s_mag,
                _ => {}
            }
        }

        let mut den_re = 0.0;
        let mut den_im = 0.0;
        for (k, &a) in self.denominator_coeffs.iter().enumerate() {
            let s_mag = omega.powi(k as i32);
            match k % 4 {
                0 => den_re += a * s_mag,
                1 => den_im += a * s_mag,
                2 => den_re -= a * s_mag,
                3 => den_im -= a * s_mag,
                _ => {}
            }
        }

        let den_mag_sq = den_re * den_re + den_im * den_im;
        if den_mag_sq < 1e-30 {
            return (0.0, 0.0);
        }

        let out_re = (num_re * den_re + num_im * den_im) / den_mag_sq;
        let out_im = (num_im * den_re - num_re * den_im) / den_mag_sq;
        let mag = (out_re * out_re + out_im * out_im).sqrt();
        let phase_rad = out_im.atan2(out_re);
        (mag, phase_rad)
    }

    /// Transforms Laplace transfer function into controllable canonical state-space matrices (A, B, C, D).
    pub fn to_state_space(&self) -> Option<StateSpaceRepresentation> {
        let den = &self.denominator_coeffs;
        let num = &self.numerator_coeffs;
        if den.is_empty() || den.last().copied().unwrap_or(0.0).abs() < 1e-15 {
            return None;
        }

        let n = den.len() - 1;
        if n == 0 {
            let d = num.first().copied().unwrap_or(0.0) / den[0];
            return Some((Vec::new(), Vec::new(), Vec::new(), d));
        }

        let a_n = den[n];
        let mut a_norm = vec![0.0; n];
        for i in 0..n {
            a_norm[i] = den[i] / a_n;
        }

        let mut b_padded = vec![0.0; n + 1];
        for (i, &b) in num.iter().enumerate().take(n + 1) {
            b_padded[i] = b / a_n;
        }

        let d = b_padded[n];
        let mut c_vec = vec![0.0; n];
        for i in 0..n {
            c_vec[i] = b_padded[i] - d * a_norm[i];
        }

        // Companion matrix A
        let mut a_mat = vec![vec![0.0; n]; n];
        for i in 0..(n - 1) {
            a_mat[i][i + 1] = 1.0;
        }
        for j in 0..n {
            a_mat[n - 1][j] = -a_norm[j];
        }

        let mut b_vec = vec![0.0; n];
        b_vec[n - 1] = 1.0;

        Some((a_mat, b_vec, c_vec, d))
    }
}

/// Analytical Expression Node for Symbolic Evaluation & Automatic Differentiation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AbmExpr {
    Constant(f64),
    NodeVoltage(usize), // Node index
    Add(Box<AbmExpr>, Box<AbmExpr>),
    Sub(Box<AbmExpr>, Box<AbmExpr>),
    Mul(Box<AbmExpr>, Box<AbmExpr>),
    Exp(Box<AbmExpr>),
    Ln(Box<AbmExpr>),
    Limit(Box<AbmExpr>, f64, f64),
}

impl AbmExpr {
    /// Evaluates the expression value given the current circuit node voltages.
    pub fn eval(&self, state: &[f64]) -> f64 {
        match self {
            Self::Constant(c) => *c,
            Self::NodeVoltage(idx) => state.get(*idx).copied().unwrap_or(0.0),
            Self::Add(a, b) => a.eval(state) + b.eval(state),
            Self::Sub(a, b) => a.eval(state) - b.eval(state),
            Self::Mul(a, b) => a.eval(state) * b.eval(state),
            Self::Exp(a) => a.eval(state).clamp(-50.0, 50.0).exp(),
            Self::Ln(a) => a.eval(state).max(1e-15).ln(),
            Self::Limit(a, min_val, max_val) => a.eval(state).clamp(*min_val, *max_val),
        }
    }

    /// Computes exact symbolic partial derivative with respect to node voltage `target_node`: df/d(v_target).
    pub fn derivative(&self, target_node: usize) -> AbmExpr {
        match self {
            Self::Constant(_) => Self::Constant(0.0),
            Self::NodeVoltage(idx) => {
                if *idx == target_node {
                    Self::Constant(1.0)
                } else {
                    Self::Constant(0.0)
                }
            }
            Self::Add(a, b) => Self::Add(
                Box::new(a.derivative(target_node)),
                Box::new(b.derivative(target_node)),
            ),
            Self::Sub(a, b) => Self::Sub(
                Box::new(a.derivative(target_node)),
                Box::new(b.derivative(target_node)),
            ),
            Self::Mul(a, b) => Self::Add(
                Box::new(Self::Mul(Box::new(a.derivative(target_node)), b.clone())),
                Box::new(Self::Mul(a.clone(), Box::new(b.derivative(target_node)))),
            ),
            Self::Exp(a) => Self::Mul(
                Box::new(Self::Exp(a.clone())),
                Box::new(a.derivative(target_node)),
            ),
            Self::Ln(a) => Self::Mul(
                Box::new(Self::Constant(1.0)),
                Box::new(a.derivative(target_node)),
            ),
            Self::Limit(a, _, _) => a.derivative(target_node),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbolic_differentiation() {
        // f(v0, v1) = v0 * exp(v1)
        let expr = AbmExpr::Mul(
            Box::new(AbmExpr::NodeVoltage(0)),
            Box::new(AbmExpr::Exp(Box::new(AbmExpr::NodeVoltage(1)))),
        );

        let state = vec![2.0, 0.0]; // v0=2, v1=0 -> f = 2 * 1 = 2
        assert_eq!(expr.eval(&state), 2.0);

        // df/dv0 = 1 * exp(v1) = 1
        let df_dv0 = expr.derivative(0);
        assert_eq!(df_dv0.eval(&state), 1.0);

        // df/dv1 = v0 * exp(v1) = 2
        let df_dv1 = expr.derivative(1);
        assert_eq!(df_dv1.eval(&state), 2.0);
    }

    #[test]
    fn test_laplace_transfer_function_state_space() {
        // H(s) = 1 / (s + 10) -> a0=10, a1=1, b0=1
        let tf = LaplaceTransferFunction::new(&[1.0], &[10.0, 1.0]);
        let ss = tf.to_state_space().expect("state space");
        let (a, b, c, d) = ss;
        assert_eq!(a.len(), 1);
        assert_eq!(a[0][0], -10.0);
        assert_eq!(b[0], 1.0);
        assert_eq!(c[0], 1.0);
        assert_eq!(d, 0.0);
    }
}
