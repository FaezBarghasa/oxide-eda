//! Bit Error Rate (BER) Analysis and Waterfall Curves.

use serde::{Deserialize, Serialize};

/// BER Point vs SNR / Eb/N0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BerPoint {
    pub eb_n0_db: f64,
    pub transmitted_bits: u64,
    pub bit_errors: u64,
    pub ber: f64,
}

/// Bit Error Rate Curve.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BerCurve {
    pub modulation_name: String,
    pub points: Vec<BerPoint>,
}

impl BerCurve {
    pub fn new(modulation_name: impl Into<String>) -> Self {
        Self {
            modulation_name: modulation_name.into(),
            points: Vec::new(),
        }
    }

    /// Theoretical BPSK/QPSK BER in AWGN: $P_b = \frac{1}{2} \text{erfc}\left(\sqrt{\frac{E_b}{N_0}}\right) = Q\left(\sqrt{\frac{2E_b}{N_0}}\right)$
    pub fn theoretical_bpsk_awgn(eb_n0_db_range: &[f64]) -> Self {
        let mut curve = Self::new("Theoretical BPSK (AWGN)");
        for &eb_n0_db in eb_n0_db_range {
            let eb_n0_lin = 10.0f64.powf(eb_n0_db / 10.0);
            let ber = 0.5 * erfc((eb_n0_lin).sqrt());
            curve.points.push(BerPoint {
                eb_n0_db,
                transmitted_bits: 1_000_000,
                bit_errors: (ber * 1_000_000.0).round() as u64,
                ber,
            });
        }
        curve
    }
}

/// Complementary error function approximation.
fn erfc(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.5 * x.abs());
    let tau = t * (-x * x - 1.26551223
        + t * (1.00002368
            + t * (0.37409196
                + t * (0.09678418
                    + t * (-0.18628806
                        + t * (0.27886807
                            + t * (-1.13520398
                                + t * (1.48851587
                                    + t * (-0.82215223
                                        + t * 0.17087277)))))))))
        .exp();
    if x >= 0.0 { tau } else { 2.0 - tau }
}
