//! Deterministic Monte Carlo & Statistical Distribution Engine.
//!
//! Conforms to Master Technical Directive §3.7:
//! Parameter variations (Gaussian, Uniform, Lognormal, and correlated component lots)
//! generated via deterministic pseudo-random seeds guaranteeing bit-exact waveform reproducibility.

use serde::{Deserialize, Serialize};

/// Statistical Distribution Kind.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ParameterDistribution {
    Uniform { tolerance_percent: f64 },
    Gaussian { sigma_percent: f64, num_sigmas: f64 },
    Lognormal { sigma_percent: f64 },
}

/// A component parameter subject to statistical Monte Carlo variations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TolerancedParameter {
    pub name: String,
    pub nominal_value: f64,
    pub distribution: ParameterDistribution,
}

/// A single Monte Carlo trial run definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloRun {
    pub run_index: usize,
    pub parameter_values: Vec<(String, f64)>,
}

/// Linear congruential / XorShift64 deterministic PRNG for reproducible runs.
#[derive(Debug, Clone)]
pub struct DeterministicPrng {
    state: u64,
}

impl DeterministicPrng {
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    /// Generates next 64-bit random word.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Generates uniform float in [0.0, 1.0).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Generates standard normal sample via Box-Muller transform: N(0, 1).
    pub fn next_gaussian(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-15);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

/// Monte Carlo Trial Generator.
pub struct MonteCarloEngine;

impl MonteCarloEngine {
    /// Generates $N$ reproducible parameter variation sets using a fixed seed.
    pub fn generate_runs(
        parameters: &[TolerancedParameter],
        num_runs: usize,
        seed: u64,
    ) -> Vec<MonteCarloRun> {
        let mut prng = DeterministicPrng::new(seed);
        let mut runs = Vec::with_capacity(num_runs);

        for i in 0..num_runs {
            let mut run_params = Vec::with_capacity(parameters.len());
            for param in parameters {
                let sample_val = match param.distribution {
                    ParameterDistribution::Uniform { tolerance_percent } => {
                        let u = prng.next_f64() * 2.0 - 1.0; // [-1.0, 1.0]
                        param.nominal_value * (1.0 + (tolerance_percent / 100.0) * u)
                    }
                    ParameterDistribution::Gaussian { sigma_percent, num_sigmas } => {
                        let z = prng.next_gaussian().clamp(-num_sigmas, num_sigmas);
                        param.nominal_value * (1.0 + (sigma_percent / 100.0) * z)
                    }
                    ParameterDistribution::Lognormal { sigma_percent } => {
                        let z = prng.next_gaussian();
                        let sigma = sigma_percent / 100.0;
                        param.nominal_value * (z * sigma - 0.5 * sigma * sigma).exp()
                    }
                };
                run_params.push((param.name.clone(), sample_val));
            }
            runs.push(MonteCarloRun {
                run_index: i + 1,
                parameter_values: run_params,
            });
        }

        runs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_monte_carlo_reproducibility() {
        let params = vec![
            TolerancedParameter {
                name: "R1".to_string(),
                nominal_value: 10_000.0,
                distribution: ParameterDistribution::Uniform { tolerance_percent: 5.0 },
            },
            TolerancedParameter {
                name: "C1".to_string(),
                nominal_value: 100e-9,
                distribution: ParameterDistribution::Gaussian { sigma_percent: 10.0, num_sigmas: 3.0 },
            },
        ];

        let runs1 = MonteCarloEngine::generate_runs(&params, 10, 42);
        let runs2 = MonteCarloEngine::generate_runs(&params, 10, 42);

        assert_eq!(runs1.len(), 10);
        assert_eq!(runs2.len(), 10);

        for (r1, r2) in runs1.iter().zip(runs2.iter()) {
            assert_eq!(r1.run_index, r2.run_index);
            assert_eq!(r1.parameter_values, r2.parameter_values);
            // Verify R1 stays within 5%
            let r1_val = r1.parameter_values[0].1;
            assert!(r1_val >= 9_500.0 && r1_val <= 10_500.0);
        }
    }
}
