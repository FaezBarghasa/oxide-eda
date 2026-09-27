//! Automated Four-Stage Convergence Recovery Cascade for Stiff Non-Linear Circuits.
//!
//! Conforms to Master Technical Directive Horizon I (§2, Task 1.3):
//! - Stage 0: Standard Newton-Raphson Iteration.
//! - Stage 1: Damped Line-Search Newton-Raphson.
//! - Stage 2: Adaptive Gmin Conductance Stepping ($10^{-2}\,\text{S} \to 10^{-12}\,\text{S}$).
//! - Stage 3: Dynamic Source Stepping Continuation ($\alpha \in [0.0 \to 1.0]$).
//! - Stage 4: Pseudo-Transient Continuation (PTC) with virtual node capacitances.

use serde::{Deserialize, Serialize};
use super::ConvergenceStage;

/// Configuration and state for the 4-stage convergence recovery cascade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceCascade {
    pub current_stage: ConvergenceStage,
    pub line_search_alpha: f64,
    pub gmin_start: f64,
    pub gmin_target: f64,
    pub gmin_current: f64,
    pub gmin_step_count: usize,
    pub source_alpha: f64,
    pub source_step_size: f64,
    pub ptc_tau: f64,
    pub ptc_pseudo_dt: f64,
}

impl Default for ConvergenceCascade {
    fn default() -> Self {
        Self {
            current_stage: ConvergenceStage::StandardNewtonRaphson,
            line_search_alpha: 1e-4,
            gmin_start: 1e-2,
            gmin_target: 1e-12,
            gmin_current: 1e-12,
            gmin_step_count: 10,
            source_alpha: 1.0,
            source_step_size: 0.1,
            ptc_tau: 1e-6,
            ptc_pseudo_dt: 1e-9,
        }
    }
}

impl ConvergenceCascade {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets cascade back to standard Newton-Raphson stage.
    pub fn reset(&mut self) {
        self.current_stage = ConvergenceStage::StandardNewtonRaphson;
        self.gmin_current = self.gmin_target;
        self.source_alpha = 1.0;
        self.source_step_size = 0.1;
    }

    /// Advances to the next fallback recovery stage upon non-linear divergence.
    pub fn advance_fallback_stage(&mut self) -> Option<ConvergenceStage> {
        match self.current_stage {
            ConvergenceStage::StandardNewtonRaphson => {
                self.current_stage = ConvergenceStage::DampedLineSearch;
                Some(self.current_stage)
            }
            ConvergenceStage::DampedLineSearch => {
                self.current_stage = ConvergenceStage::GminStepping;
                self.gmin_current = self.gmin_start;
                Some(self.current_stage)
            }
            ConvergenceStage::GminStepping => {
                self.current_stage = ConvergenceStage::SourceStepping;
                self.source_alpha = 0.0;
                self.source_step_size = 0.1;
                Some(self.current_stage)
            }
            ConvergenceStage::SourceStepping => {
                self.current_stage = ConvergenceStage::PseudoTransientContinuation;
                Some(self.current_stage)
            }
            ConvergenceStage::PseudoTransientContinuation => None, // All stages exhausted
        }
    }

    /// Computes optimal line-search damping factor $\lambda \in (0, 1]$ to satisfy Armijo condition.
    pub fn compute_line_search_damping(
        &self,
        residual_norm_prev: f64,
        residual_norm_next: f64,
        proposed_lambda: f64,
    ) -> f64 {
        if residual_norm_next < (1.0 - self.line_search_alpha * proposed_lambda) * residual_norm_prev {
            proposed_lambda
        } else {
            (proposed_lambda * 0.5).max(0.01)
        }
    }

    /// Steps Gmin logarithmically from `gmin_start` down toward `gmin_target`.
    pub fn step_gmin(&mut self, step_index: usize) -> f64 {
        let n = self.gmin_step_count.max(1) as f64;
        let frac = (step_index as f64 / n).clamp(0.0, 1.0);
        let ratio = self.gmin_target / self.gmin_start;
        self.gmin_current = self.gmin_start * ratio.powf(frac);
        self.gmin_current
    }

    /// Steps source scaling parameter $\alpha \in [0.0 \to 1.0]$.
    pub fn step_source(&mut self, success: bool) -> (f64, bool) {
        if success {
            self.source_alpha = (self.source_alpha + self.source_step_size).min(1.0);
            let is_complete = (self.source_alpha - 1.0).abs() < 1e-9;
            (self.source_alpha, is_complete)
        } else {
            // Bifurcate step size upon sub-step failure
            self.source_step_size = (self.source_step_size * 0.5).max(1e-4);
            (self.source_alpha, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cascade_stage_transitions() {
        let mut cascade = ConvergenceCascade::new();
        assert_eq!(cascade.current_stage, ConvergenceStage::StandardNewtonRaphson);

        assert_eq!(cascade.advance_fallback_stage(), Some(ConvergenceStage::DampedLineSearch));
        assert_eq!(cascade.advance_fallback_stage(), Some(ConvergenceStage::GminStepping));
        assert_eq!(cascade.advance_fallback_stage(), Some(ConvergenceStage::SourceStepping));
        assert_eq!(cascade.advance_fallback_stage(), Some(ConvergenceStage::PseudoTransientContinuation));
        assert_eq!(cascade.advance_fallback_stage(), None);
    }

    #[test]
    fn test_gmin_and_source_stepping() {
        let mut cascade = ConvergenceCascade::new();
        let g0 = cascade.step_gmin(0);
        let g_end = cascade.step_gmin(10);
        assert!((g0 - 1e-2).abs() < 1e-6);
        assert!((g_end - 1e-12).abs() < 1e-15);

        cascade.source_alpha = 0.0;
        let (a1, done1) = cascade.step_source(true);
        assert_eq!(a1, 0.1);
        assert!(!done1);
    }
}
