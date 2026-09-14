//! Digital and Analog Modulation & Demodulation Engine.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use crate::s_param::Complex64;

/// Modulation Scheme Kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModulationScheme {
    // Analog
    AmDsb,
    Fm,
    // Digital
    Ask,
    Fsk,
    Bpsk,
    Qpsk,
    Qam16,
    Qam64,
    Qam256,
}

/// Modulated I/Q Baseband Symbol.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IqSymbol {
    pub i: f64,
    pub q: f64,
}

impl IqSymbol {
    pub fn new(i: f64, q: f64) -> Self {
        Self { i, q }
    }

    pub fn to_complex(&self) -> Complex64 {
        Complex64::new(self.i, self.q)
    }

    pub fn magnitude(&self) -> f64 {
        (self.i * self.i + self.q * self.q).sqrt()
    }

    pub fn phase_rad(&self) -> f64 {
        self.q.atan2(self.i)
    }
}

/// Modulator Pipeline.
pub struct Modulator;

impl Modulator {
    /// Maps bit sequence into I/Q baseband symbols according to modulation scheme.
    pub fn modulate_bits(bits: &[bool], scheme: ModulationScheme) -> Vec<IqSymbol> {
        match scheme {
            ModulationScheme::Bpsk => bits
                .iter()
                .map(|&b| if b { IqSymbol::new(1.0, 0.0) } else { IqSymbol::new(-1.0, 0.0) })
                .collect(),

            ModulationScheme::Qpsk => {
                let mut symbols = Vec::with_capacity(bits.len() / 2 + 1);
                for chunk in bits.chunks(2) {
                    let b0 = chunk[0];
                    let b1 = if chunk.len() > 1 { chunk[1] } else { false };
                    let i = if b0 { 1.0 / (2.0f64).sqrt() } else { -1.0 / (2.0f64).sqrt() };
                    let q = if b1 { 1.0 / (2.0f64).sqrt() } else { -1.0 / (2.0f64).sqrt() };
                    symbols.push(IqSymbol::new(i, q));
                }
                symbols
            }

            ModulationScheme::Qam16 => {
                let mut symbols = Vec::with_capacity(bits.len() / 4 + 1);
                // 16-QAM maps 4 bits -> (I, Q) where each coord in {-3, -1, 1, 3} / sqrt(10)
                let norm = (10.0f64).sqrt();
                for chunk in bits.chunks(4) {
                    let mut b = [false; 4];
                    for (idx, &bit) in chunk.iter().enumerate() {
                        b[idx] = bit;
                    }
                    let i_val = match (b[0], b[1]) {
                        (false, false) => -3.0,
                        (false, true) => -1.0,
                        (true, true) => 1.0,
                        (true, false) => 3.0,
                    };
                    let q_val = match (b[2], b[3]) {
                        (false, false) => -3.0,
                        (false, true) => -1.0,
                        (true, true) => 1.0,
                        (true, false) => 3.0,
                    };
                    symbols.push(IqSymbol::new(i_val / norm, q_val / norm));
                }
                symbols
            }

            ModulationScheme::Ask => bits
                .iter()
                .map(|&b| if b { IqSymbol::new(1.0, 0.0) } else { IqSymbol::new(0.0, 0.0) })
                .collect(),

            ModulationScheme::Fsk => bits
                .iter()
                .map(|&b| if b { IqSymbol::new(1.0, 0.0) } else { IqSymbol::new(0.0, 1.0) })
                .collect(),

            ModulationScheme::Qam64 | ModulationScheme::Qam256 | ModulationScheme::AmDsb | ModulationScheme::Fm => {
                // Default fallback to QPSK mapping for higher orders
                Self::modulate_bits(bits, ModulationScheme::Qpsk)
            }
        }
    }

    /// Generates time-domain RF modulated carrier signal from baseband I/Q symbols.
    pub fn upconvert_to_carrier(
        symbols: &[IqSymbol],
        samples_per_symbol: usize,
        carrier_freq_hz: f64,
        sample_rate_hz: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        let total_samples = symbols.len() * samples_per_symbol;
        let mut time_vec = Vec::with_capacity(total_samples);
        let mut signal_vec = Vec::with_capacity(total_samples);

        for (sym_idx, sym) in symbols.iter().enumerate() {
            for sample_idx in 0..samples_per_symbol {
                let n = sym_idx * samples_per_symbol + sample_idx;
                let t = n as f64 / sample_rate_hz;
                time_vec.push(t);

                let carrier_cos = (2.0 * PI * carrier_freq_hz * t).cos();
                let carrier_sin = (2.0 * PI * carrier_freq_hz * t).sin();

                let rf_val = sym.i * carrier_cos - sym.q * carrier_sin;
                signal_vec.push(rf_val);
            }
        }

        (time_vec, signal_vec)
    }
}
