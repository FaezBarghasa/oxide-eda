//! `oxide-rf` — Telecommunications, RF & Signal Integrity Simulation Engine for Oxide EDA.
//!
//! Provides:
//! - [`SParameters2Port`] & [`SParameterDataset`]: 2-port network analysis, return/insertion loss, VSWR, Touchstone export, and Smith chart projections.
//! - [`ModulationScheme`] & [`Modulator`]: Digital/Analog modulation (AM, FM, ASK, FSK, BPSK, QPSK, 16/64/256-QAM) and I/Q baseband symbol generation.
//! - [`EyeDiagramDataset`]: Eye diagram windowing and signal integrity metrics (eye height, eye width, jitter RMS/p2p, SNR).
//! - [`ConstellationDataset`]: I/Q constellation mapping and Error Vector Magnitude (EVM) calculation.
//! - [`ChannelModel`]: AWGN noise, path loss, and phase/frequency offset impairment models.
//! - [`BerCurve`]: Bit Error Rate waterfall curves vs. $E_b/N_0$.

pub mod ber;
pub mod channel;
pub mod constellation;
pub mod eye_diagram;
pub mod modulation;
pub mod s_param;

pub use ber::{BerCurve, BerPoint};
pub use channel::ChannelModel;
pub use constellation::{ConstellationDataset, ConstellationPoint};
pub use eye_diagram::{EyeDiagramDataset, EyeMetrics, EyeTraceWindow};
pub use modulation::{IqSymbol, ModulationScheme, Modulator};
pub use s_param::{Complex64, SParameterDataset, SParameters2Port};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s_parameters_pi_attenuator() {
        let s2p = SParameters2Port {
            freq_hz: 1e9, // 1 GHz
            s11: Complex64::new(0.05, 0.0),
            s21: Complex64::new(0.5, 0.0), // -6 dB attenuation
            s12: Complex64::new(0.5, 0.0),
            s22: Complex64::new(0.05, 0.0),
            z0: 50.0,
        };

        assert!((s2p.return_loss_port1_db() - 26.02).abs() < 0.1);
        assert!((s2p.insertion_loss_db() - 6.02).abs() < 0.1);
        assert!(s2p.vswr_port1() > 1.0 && s2p.vswr_port1() < 1.2);
    }

    #[test]
    fn test_modulation_and_constellation() {
        let bits = vec![false, false, false, true, true, true, true, false];
        let symbols = Modulator::modulate_bits(&bits, ModulationScheme::Qpsk);
        assert_eq!(symbols.len(), 4);

        let channel = ChannelModel {
            snr_db: 30.0,
            path_loss_db: 0.0,
            phase_offset_deg: 0.0,
            frequency_offset_hz: 0.0,
        };
        let received = channel.apply(&symbols);
        assert_eq!(received.len(), 4);

        let constellation = ConstellationDataset::calculate(&received, &symbols);
        assert!(constellation.evm_rms_percent < 10.0);
    }

    #[test]
    fn test_eye_diagram_measurements() {
        let n_samples = 1000;
        let mut time = Vec::with_capacity(n_samples);
        let mut voltage = Vec::with_capacity(n_samples);

        for i in 0..n_samples {
            let t = i as f64 * 1e-9;
            time.push(t);
            // Simulated 100 MHz clock signal
            let v = (2.0 * std::f64::consts::PI * 100e6 * t).sin() * 1.5;
            voltage.push(v);
        }

        let eye = EyeDiagramDataset::from_signal(&time, &voltage, 100e6);
        assert!(!eye.traces.is_empty());
        assert_eq!(eye.symbol_rate_baud, 100e6);
    }
}
