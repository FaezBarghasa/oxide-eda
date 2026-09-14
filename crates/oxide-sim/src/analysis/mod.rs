//! Waveform mathematical analysis and measurements.

use oxide_types::sim::WaveformTrace;

/// Statistical summary of a waveform trace.
#[derive(Debug, Clone, PartialEq)]
pub struct TraceStats {
    pub min: f64,
    pub max: f64,
    pub peak_to_peak: f64,
    pub mean: f64,
    pub rms: f64,
}

/// Calculate basic statistics for a waveform trace.
pub fn calculate_stats(trace: &WaveformTrace) -> Option<TraceStats> {
    if trace.values.is_empty() {
        return None;
    }

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for &v in &trace.values {
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
        sum += v;
        sum_sq += v * v;
    }

    let n = trace.values.len() as f64;
    let mean = sum / n;
    let rms = (sum_sq / n).sqrt();
    let peak_to_peak = max - min;

    Some(TraceStats {
        min,
        max,
        peak_to_peak,
        mean,
        rms,
    })
}

/// Find 10% to 90% rise time of a step waveform.
pub fn calculate_rise_time(time_vals: &[f64], trace_vals: &[f64]) -> Option<f64> {
    if time_vals.len() != trace_vals.len() || trace_vals.len() < 2 {
        return None;
    }

    let min = trace_vals.iter().copied().fold(f64::INFINITY, f64::min);
    let max = trace_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let v10 = min + 0.1 * (max - min);
    let v90 = min + 0.9 * (max - min);

    let mut t10 = None;
    let mut t90 = None;

    for (i, &v) in trace_vals.iter().enumerate() {
        if t10.is_none() && v >= v10 {
            t10 = Some(time_vals[i]);
        }
        if t90.is_none() && v >= v90 {
            t90 = Some(time_vals[i]);
            break;
        }
    }

    match (t10, t90) {
        (Some(t1), Some(t2)) => Some((t2 - t1).abs()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_types::sim::TraceUnit;

    #[test]
    fn calculate_stats_basic() {
        let trace = WaveformTrace {
            name: "V(1)".to_string(),
            unit: TraceUnit::VoltageVolts,
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };
        let stats = calculate_stats(&trace).expect("stats");
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.peak_to_peak, 4.0);
        assert_eq!(stats.mean, 3.0);
    }

    #[test]
    fn calculate_rise_time_step() {
        let t = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let v = vec![0.0, 0.05, 0.1, 0.3, 0.5, 0.7, 0.9, 0.95, 1.0, 1.0, 1.0];
        let rt = calculate_rise_time(&t, &v).expect("rise time");
        assert_eq!(rt, 4.0); // from t=2.0 (v=0.1) to t=6.0 (v=0.9)
    }
}
