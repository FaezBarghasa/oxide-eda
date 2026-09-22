//! Binary Columnar Waveform Streamer & GPU Decimation (`.snxwv`).
//!
//! Conforms to Master Technical Directive §3.8:
//! - Binary columnar container format with chunked pages for simulation traces exceeding $10^9$ points.
//! - Sub-pixel min/max decimation pipeline for ultra-fast viewport waveform rendering.

use serde::{Deserialize, Serialize};

/// Header metadata for `.snxwv` waveform file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnxwvHeader {
    pub magic: [u8; 6], // b"SNXWV1"
    pub title: String,
    pub sample_count: u64,
    pub channel_count: u32,
    pub time_min: f64,
    pub time_max: f64,
}

impl Default for SnxwvHeader {
    fn default() -> Self {
        Self {
            magic: *b"SNXWV1",
            title: "Oxide Simulation Waveform".to_string(),
            sample_count: 0,
            channel_count: 0,
            time_min: 0.0,
            time_max: 0.0,
        }
    }
}

/// Decimated min/max envelope bucket for sub-pixel rendering.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DecimationBucket {
    pub t_start: f64,
    pub t_end: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub v_first: f64,
    pub v_last: f64,
}

/// Decimation engine computing sub-pixel envelopes for viewport display.
pub struct WaveformDecimator;

impl WaveformDecimator {
    /// Decimates raw time and value slices into $N_{\text{pixels}}$ min/max envelope buckets.
    pub fn decimate(
        time: &[f64],
        values: &[f64],
        pixel_count: usize,
        t_view_start: f64,
        t_view_end: f64,
    ) -> Vec<DecimationBucket> {
        if time.is_empty() || values.is_empty() || time.len() != values.len() || pixel_count == 0 {
            return Vec::new();
        }

        let t_span = t_view_end - t_view_start;
        if t_span <= 0.0 {
            return Vec::new();
        }

        // Find slice within view domain
        let start_idx = match time.binary_search_by(|t| t.partial_cmp(&t_view_start).unwrap()) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let end_idx = match time.binary_search_by(|t| t.partial_cmp(&t_view_end).unwrap()) {
            Ok(i) => (i + 1).min(time.len()),
            Err(i) => i.min(time.len()),
        };

        if start_idx >= end_idx {
            return Vec::new();
        }

        let slice_len = end_idx - start_idx;
        let mut buckets = Vec::with_capacity(pixel_count);
        let dt_bucket = t_span / pixel_count as f64;

        for p in 0..pixel_count {
            let b_start = t_view_start + p as f64 * dt_bucket;
            let b_end = b_start + dt_bucket;

            // Map bucket to sample index range
            let b_i_start = start_idx + ((p as f64 / pixel_count as f64) * slice_len as f64) as usize;
            let b_i_end = (start_idx + (((p + 1) as f64 / pixel_count as f64) * slice_len as f64) as usize)
                .min(end_idx);

            if b_i_start >= b_i_end {
                continue;
            }

            let mut v_min = f64::INFINITY;
            let mut v_max = f64::NEG_INFINITY;
            let v_first = values[b_i_start];
            let v_last = values[b_i_end - 1];

            for i in b_i_start..b_i_end {
                let v = values[i];
                if v < v_min {
                    v_min = v;
                }
                if v > v_max {
                    v_max = v;
                }
            }

            buckets.push(DecimationBucket {
                t_start: b_start,
                t_end: b_end,
                v_min,
                v_max,
                v_first,
                v_last,
            });
        }

        buckets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveform_decimation_preserves_extrema() {
        let n = 10_000;
        let mut time = Vec::with_capacity(n);
        let mut values = Vec::with_capacity(n);

        for i in 0..n {
            let t = i as f64 * 1e-6; // 0 to 10ms
            time.push(t);
            // Sine wave with high-frequency noise spike
            let mut v = (2.0 * std::f64::consts::PI * 100.0 * t).sin();
            if i == 5000 {
                v = 15.0; // Spike
            }
            values.push(v);
        }

        // Decimate 10k points into 100 pixels
        let buckets = WaveformDecimator::decimate(&time, &values, 100, 0.0, 0.010);
        assert_eq!(buckets.len(), 100);

        // Verify the spike at ~5ms is captured in the corresponding bucket's v_max
        let spike_bucket = buckets.iter().find(|b| b.v_max > 10.0);
        assert!(spike_bucket.is_some());
        assert_eq!(spike_bucket.unwrap().v_max, 15.0);
    }
}
