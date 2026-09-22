//! Component Stress & Smoke Analysis (Safe Operating Area - SOA).
//!
//! Conforms to Master Technical Directive §3.7:
//! Real-time monitoring of instantaneous and time-averaged stresses against manufacturer limit databases:
//! $\text{Stress Ratio} = \max(V_{\text{peak}}/V_{\text{breakdown}}, I_{\text{RMS}}/I_{\text{max}}, P_{\text{avg}}/P_{\text{rated}}, T_j/T_{j, \text{max}})$

use serde::{Deserialize, Serialize};

/// Maximum ratings / Safe Operating Area limits for a component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentLimits {
    pub max_voltage_v: f64,
    pub max_current_a: f64,
    pub max_power_w: f64,
    pub max_junction_temp_c: f64,
    pub derating_factor: f64, // e.g. 0.8 for 20% safety margin
}

impl Default for ComponentLimits {
    fn default() -> Self {
        Self {
            max_voltage_v: 50.0,
            max_current_a: 1.0,
            max_power_w: 0.5,
            max_junction_temp_c: 125.0,
            derating_factor: 0.8,
        }
    }
}

/// Simulated electrical and thermal stresses on a component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedStress {
    pub peak_voltage_v: f64,
    pub rms_current_a: f64,
    pub avg_power_w: f64,
    pub junction_temp_c: f64,
}

/// Component stress ratio evaluation results.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StressEvaluation {
    pub designator: String,
    pub voltage_ratio: f64,
    pub current_ratio: f64,
    pub power_ratio: f64,
    pub temp_ratio: f64,
    pub max_stress_ratio: f64,
    pub is_overstressed: bool,
    pub primary_violation: Option<String>,
}

/// Safe Operating Area & Smoke Analyzer.
#[derive(Debug, Default)]
pub struct SmokeAnalyzer;

impl SmokeAnalyzer {
    /// Evaluates component stresses against rated limits with safety derating.
    pub fn evaluate_stress(
        designator: &str,
        limits: &ComponentLimits,
        stress: &SimulatedStress,
    ) -> StressEvaluation {
        let derated_v = limits.max_voltage_v * limits.derating_factor;
        let derated_i = limits.max_current_a * limits.derating_factor;
        let derated_p = limits.max_power_w * limits.derating_factor;
        let derated_t = limits.max_junction_temp_c * limits.derating_factor;

        let v_ratio = if derated_v > 0.0 { stress.peak_voltage_v / derated_v } else { 0.0 };
        let i_ratio = if derated_i > 0.0 { stress.rms_current_a / derated_i } else { 0.0 };
        let p_ratio = if derated_p > 0.0 { stress.avg_power_w / derated_p } else { 0.0 };
        let t_ratio = if derated_t > 0.0 { stress.junction_temp_c / derated_t } else { 0.0 };

        let max_ratio = v_ratio.max(i_ratio).max(p_ratio).max(t_ratio);
        let is_overstressed = max_ratio > 1.0;

        let primary_violation = if is_overstressed {
            if max_ratio == v_ratio {
                Some(format!("Peak Voltage ({:.2}V > rated {:.2}V)", stress.peak_voltage_v, derated_v))
            } else if max_ratio == i_ratio {
                Some(format!("RMS Current ({:.3}A > rated {:.3}A)", stress.rms_current_a, derated_i))
            } else if max_ratio == p_ratio {
                Some(format!("Avg Power ({:.3}W > rated {:.3}W)", stress.avg_power_w, derated_p))
            } else {
                Some(format!("Junction Temp ({:.1}°C > rated {:.1}°C)", stress.junction_temp_c, derated_t))
            }
        } else {
            None
        };

        StressEvaluation {
            designator: designator.to_string(),
            voltage_ratio: v_ratio,
            current_ratio: i_ratio,
            power_ratio: p_ratio,
            temp_ratio: t_ratio,
            max_stress_ratio: max_ratio,
            is_overstressed,
            primary_violation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_analysis_within_safe_area() {
        let limits = ComponentLimits {
            max_voltage_v: 25.0,
            max_current_a: 2.0,
            max_power_w: 1.0,
            max_junction_temp_c: 125.0,
            derating_factor: 0.8, // 20V, 1.6A, 0.8W, 100C derated
        };
        let stress = SimulatedStress {
            peak_voltage_v: 12.0,
            rms_current_a: 0.5,
            avg_power_w: 0.2,
            junction_temp_c: 45.0,
        };

        let eval = SmokeAnalyzer::evaluate_stress("R1", &limits, &stress);
        assert!(!eval.is_overstressed);
        assert!(eval.max_stress_ratio < 1.0);
        assert!(eval.primary_violation.is_none());
    }

    #[test]
    fn test_smoke_analysis_voltage_violation() {
        let limits = ComponentLimits {
            max_voltage_v: 16.0,
            max_current_a: 1.0,
            max_power_w: 0.5,
            max_junction_temp_c: 125.0,
            derating_factor: 1.0,
        };
        let stress = SimulatedStress {
            peak_voltage_v: 24.0, // 24V on 16V part
            rms_current_a: 0.1,
            avg_power_w: 0.1,
            junction_temp_c: 35.0,
        };

        let eval = SmokeAnalyzer::evaluate_stress("C1", &limits, &stress);
        assert!(eval.is_overstressed);
        assert_eq!(eval.max_stress_ratio, 1.5);
        assert!(eval.primary_violation.unwrap().contains("Peak Voltage"));
    }
}
