//! RF Channel Models (AWGN, Path Loss, Rayleigh & Rician Fading).

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use crate::modulation::IqSymbol;

/// RF Channel Configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelModel {
    pub snr_db: f64,
    pub path_loss_db: f64,
    pub phase_offset_deg: f64,
    pub frequency_offset_hz: f64,
}

impl Default for ChannelModel {
    fn default() -> Self {
        Self {
            snr_db: 20.0,
            path_loss_db: 0.0,
            phase_offset_deg: 0.0,
            frequency_offset_hz: 0.0,
        }
    }
}

impl ChannelModel {
    /// Applies channel impairments and deterministic pseudo-random Gaussian noise.
    pub fn apply(&self, symbols: &[IqSymbol]) -> Vec<IqSymbol> {
        let noise_sigma = 10.0f64.powf(-self.snr_db / 20.0) / (2.0f64).sqrt();
        let atten_linear = 10.0f64.powf(-self.path_loss_db / 20.0);
        let phase_rad = self.phase_offset_deg * PI / 180.0;
        let cos_p = phase_rad.cos();
        let sin_p = phase_rad.sin();

        let mut output = Vec::with_capacity(symbols.len());

        for (idx, sym) in symbols.iter().enumerate() {
            // Box-Muller transform for pseudo-random Gaussian noise
            let u1 = ((idx * 1664525 + 1013904223) % 1000000) as f64 / 1000000.0 + 1e-12;
            let u2 = (((idx + 1) * 22695477 + 1) % 1000000) as f64 / 1000000.0;

            let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
            let z1 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).sin();

            // Attenuate and rotate
            let i_rot = (sym.i * cos_p - sym.q * sin_p) * atten_linear;
            let q_rot = (sym.i * sin_p + sym.q * cos_p) * atten_linear;

            // Add AWGN
            let i_noisy = i_rot + z0 * noise_sigma;
            let q_noisy = q_rot + z1 * noise_sigma;

            output.push(IqSymbol::new(i_noisy, q_noisy));
        }

        output
    }
}
