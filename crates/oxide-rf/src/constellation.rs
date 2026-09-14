//! Constellation Diagram & Error Vector Magnitude (EVM) Calculation.

use serde::{Deserialize, Serialize};
use crate::modulation::IqSymbol;

/// Constellation Point with ideal reference and noisy received coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConstellationPoint {
    pub received: IqSymbol,
    pub ideal: Option<IqSymbol>,
    pub error_vector: IqSymbol,
}

/// Constellation Diagram Dataset & Metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstellationDataset {
    pub points: Vec<ConstellationPoint>,
    /// RMS Error Vector Magnitude as a percentage (e.g. 2.5%).
    pub evm_rms_percent: f64,
    /// RMS EVM in dB: $EVM_{dB} = 20 \log_{10}(EVM_{\%}/100)$
    pub evm_rms_db: f64,
    /// Peak EVM percentage.
    pub evm_peak_percent: f64,
    /// Estimated Signal-to-Noise Ratio derived from EVM.
    pub snr_est_db: f64,
}

impl ConstellationDataset {
    /// Constructs dataset and calculates EVM metrics from received and ideal symbols.
    pub fn calculate(received_symbols: &[IqSymbol], ideal_symbols: &[IqSymbol]) -> Self {
        let n = received_symbols.len().min(ideal_symbols.len());
        if n == 0 {
            return Self {
                points: Vec::new(),
                evm_rms_percent: 0.0,
                evm_rms_db: -100.0,
                evm_peak_percent: 0.0,
                snr_est_db: 100.0,
            };
        }

        let mut points = Vec::with_capacity(n);
        let mut err_sq_sum = 0.0;
        let mut ref_pwr_sum = 0.0;
        let mut peak_err_sq = 0.0f64;

        for i in 0..n {
            let rx = received_symbols[i];
            let id = ideal_symbols[i];

            let err_i = rx.i - id.i;
            let err_q = rx.q - id.q;
            let err_pwr = err_i * err_i + err_q * err_q;
            let ref_pwr = id.i * id.i + id.q * id.q;

            err_sq_sum += err_pwr;
            ref_pwr_sum += ref_pwr;
            if err_pwr > peak_err_sq {
                peak_err_sq = err_pwr;
            }

            points.push(ConstellationPoint {
                received: rx,
                ideal: Some(id),
                error_vector: IqSymbol::new(err_i, err_q),
            });
        }

        let avg_ref_pwr = (ref_pwr_sum / n as f64).max(1e-12);
        let evm_rms_linear = (err_sq_sum / (n as f64 * avg_ref_pwr)).sqrt();
        let evm_rms_percent = evm_rms_linear * 100.0;
        let evm_rms_db = if evm_rms_linear <= 1e-15 { -100.0 } else { 20.0 * evm_rms_linear.log10() };
        let evm_peak_percent = (peak_err_sq / avg_ref_pwr).sqrt() * 100.0;
        let snr_est_db = -evm_rms_db;

        Self {
            points,
            evm_rms_percent,
            evm_rms_db,
            evm_peak_percent,
            snr_est_db,
        }
    }
}
