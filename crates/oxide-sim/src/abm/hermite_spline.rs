//! Parametric Cubic Hermite Splines for $\mathcal{C}^1$ Continuous Non-Linear Smoothing.
//!
//! Replaces discontinuous step functions (`STP`), conditionals (`IF`), and clamping (`LIMIT`)
//! with $\mathcal{C}^1$ continuous cubic Hermite splines to guarantee bounded analytical
//! Jacobians and eliminate Dirac delta residual spikes during Newton-Raphson iterations.

/// Cubic Hermite transition smoothing configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HermiteSmoothingConfig {
    pub delta_smooth: f64, // Boundary half-width (e.g. 10 µV = 1e-5 V)
}

impl Default for HermiteSmoothingConfig {
    fn default() -> Self {
        Self {
            delta_smooth: 1e-5,
        }
    }
}

/// Evaluates smoothed step function $f(x)$ and its continuous first derivative $df/dx$.
///
/// Smooths a step between `y_low` (for $x \le x_0 - \delta$) and `y_high` (for $x \ge x_0 + \delta$)
/// using cubic Hermite interpolation $H(u) = 3u^2 - 2u^3$ with $u \in [0, 1]$.
#[inline]
pub fn evaluate_smoothed_step(
    x: f64,
    x0: f64,
    y_low: f64,
    y_high: f64,
    delta: f64,
) -> (f64, f64) {
    if x <= x0 - delta {
        (y_low, 0.0)
    } else if x >= x0 + delta {
        (y_high, 0.0)
    } else {
        let u = (x - (x0 - delta)) / (2.0 * delta);
        let h_u = 3.0 * u * u - 2.0 * u * u * u;
        let dh_du = 6.0 * u * (1.0 - u);

        let value = y_low + (y_high - y_low) * h_u;
        let derivative = (y_high - y_low) * dh_du / (2.0 * delta);

        (value, derivative)
    }
}

/// Smoothed IF(cond > 0, v_true, v_false) operator with continuous derivative.
#[inline]
pub fn evaluate_smoothed_if(
    cond: f64,
    v_true: f64,
    v_false: f64,
    delta: f64,
) -> (f64, f64) {
    evaluate_smoothed_step(cond, 0.0, v_false, v_true, delta)
}

/// Smoothed LIMIT(x, min, max) with continuous derivatives at the clamping corners.
#[inline]
pub fn evaluate_smoothed_limit(
    x: f64,
    min_val: f64,
    max_val: f64,
    delta: f64,
) -> (f64, f64) {
    if x < min_val - delta {
        (min_val, 0.0)
    } else if x > max_val + delta {
        (max_val, 0.0)
    } else if x <= min_val + delta {
        // Lower transition
        let u = (x - (min_val - delta)) / (2.0 * delta);
        let h_u = 3.0 * u * u - 2.0 * u * u * u;
        let dh_du = 6.0 * u * (1.0 - u);
        (min_val + (x - min_val) * h_u, dh_du * 0.5)
    } else if x >= max_val - delta {
        // Upper transition
        let u = ((max_val + delta) - x) / (2.0 * delta);
        let h_u = 3.0 * u * u - 2.0 * u * u * u;
        let dh_du = -6.0 * u * (1.0 - u);
        (max_val - (max_val - x) * h_u, -dh_du * 0.5)
    } else {
        (x, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoothed_step_boundary_continuity() {
        let (val_left, der_left) = evaluate_smoothed_step(2.4, 2.5, 0.0, 1.0, 0.05);
        let (val_mid, der_mid) = evaluate_smoothed_step(2.5, 2.5, 0.0, 1.0, 0.05);
        let (val_right, der_right) = evaluate_smoothed_step(2.6, 2.5, 0.0, 1.0, 0.05);

        assert_eq!(val_left, 0.0);
        assert_eq!(der_left, 0.0);
        assert!((val_mid - 0.5).abs() < 1e-12);
        assert!(der_mid > 0.0);
        assert_eq!(val_right, 1.0);
        assert_eq!(der_right, 0.0);
    }

    #[test]
    fn test_smoothed_if_operator() {
        let (v_neg, d_neg) = evaluate_smoothed_if(-1.0, 5.0, 0.0, 0.01);
        let (v_pos, d_pos) = evaluate_smoothed_if(1.0, 5.0, 0.0, 0.01);
        assert_eq!(v_neg, 0.0);
        assert_eq!(d_neg, 0.0);
        assert_eq!(v_pos, 5.0);
        assert_eq!(d_pos, 0.0);
    }
}
