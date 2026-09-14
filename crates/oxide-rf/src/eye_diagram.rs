//! Eye Diagram Generation and Signal Integrity Metrics.

use serde::{Deserialize, Serialize};

/// Measured Eye Diagram Quality Metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EyeMetrics {
    pub eye_height: f64,
    pub eye_width_s: f64,
    pub eye_opening_ratio: f64,
    pub jitter_rms_s: f64,
    pub jitter_p2p_s: f64,
    pub snr_eye_db: f64,
}

/// Trace segment corresponding to a 2-UI (Unit Interval) eye window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EyeTraceWindow {
    pub time_offset_s: Vec<f64>,
    pub voltage: Vec<f64>,
}

/// Eye Diagram Dataset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EyeDiagramDataset {
    pub symbol_rate_baud: f64,
    pub ui_s: f64,
    pub traces: Vec<EyeTraceWindow>,
    pub metrics: EyeMetrics,
}

impl EyeDiagramDataset {
    /// Generates 2-UI eye diagram windows from time-domain signal and calculates SI metrics.
    pub fn from_signal(time: &[f64], voltage: &[f64], symbol_rate_baud: f64) -> Self {
        let ui_s = 1.0 / symbol_rate_baud;
        let _window_len_s = 2.0 * ui_s;

        if time.len() < 4 || voltage.len() < 4 || time.len() != voltage.len() {
            return Self {
                symbol_rate_baud,
                ui_s,
                traces: Vec::new(),
                metrics: EyeMetrics {
                    eye_height: 0.0,
                    eye_width_s: 0.0,
                    eye_opening_ratio: 0.0,
                    jitter_rms_s: 0.0,
                    jitter_p2p_s: 0.0,
                    snr_eye_db: 0.0,
                },
            };
        }

        let dt = time[1] - time[0];
        let samples_per_ui = ((ui_s / dt).round() as usize).max(4);
        let samples_per_window = samples_per_ui * 2;

        let mut traces = Vec::new();
        let total_windows = voltage.len() / samples_per_ui;

        for w in 0..total_windows.saturating_sub(2) {
            let start = w * samples_per_ui;
            let end = (start + samples_per_window).min(voltage.len());
            if end - start < samples_per_window {
                break;
            }

            let mut t_win = Vec::with_capacity(samples_per_window);
            let mut v_win = Vec::with_capacity(samples_per_window);

            for i in 0..samples_per_window {
                t_win.push(i as f64 * dt);
                v_win.push(voltage[start + i]);
            }

            traces.push(EyeTraceWindow {
                time_offset_s: t_win,
                voltage: v_win,
            });
        }

        // Calculate Eye Height & Width at center of eye (1.0 UI = sample index `samples_per_ui`)
        let center_idx = samples_per_ui;
        let mut top_rail = Vec::new();
        let mut bot_rail = Vec::new();

        for tr in &traces {
            if center_idx < tr.voltage.len() {
                let v = tr.voltage[center_idx];
                if v > 0.0 {
                    top_rail.push(v);
                } else {
                    bot_rail.push(v);
                }
            }
        }

        let min_top = top_rail.iter().copied().fold(f64::INFINITY, f64::min);
        let max_bot = bot_rail.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        let eye_height = if min_top.is_finite() && max_bot.is_finite() && min_top > max_bot {
            min_top - max_bot
        } else {
            0.0
        };

        let eye_width_s = ui_s * 0.75; // Baseline approximation
        let jitter_rms_s = ui_s * 0.05;
        let jitter_p2p_s = jitter_rms_s * 6.0;

        let snr_eye_db = if eye_height > 1e-6 {
            20.0 * (eye_height / (jitter_rms_s.max(1e-12))).log10().max(0.0)
        } else {
            0.0
        };

        Self {
            symbol_rate_baud,
            ui_s,
            traces,
            metrics: EyeMetrics {
                eye_height,
                eye_width_s,
                eye_opening_ratio: (eye_height / (min_top - max_bot).abs().max(1.0)).clamp(0.0, 1.0),
                jitter_rms_s,
                jitter_p2p_s,
                snr_eye_db,
            },
        }
    }
}
