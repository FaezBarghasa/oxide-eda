//! Cadence PSpice CSDF (Common Simulation Data Format) parser.

use std::collections::BTreeMap;
use oxide_types::sim::{TraceUnit, WaveformDataset, WaveformTrace};
use crate::simulator::SimError;

/// Parse a Cadence PSpice CSDF simulation output text into a [`WaveformDataset`].
pub fn parse_csdf(content: &str) -> Result<WaveformDataset, SimError> {
    let mut title = "PSpice Simulation".to_string();
    let mut x_name = "time".to_string();
    let mut x_unit = TraceUnit::TimeSeconds;
    let mut x_vals = Vec::new();
    let mut trace_defs: Vec<(String, TraceUnit)> = Vec::new();
    let mut trace_data: Vec<Vec<f64>> = Vec::new();

    let mut in_data = false;
    let mut in_header = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("#H") {
            in_header = true;
        } else if trimmed.starts_with("#N") {
            // Signal name definitions, e.g. `#N V(OUT) V`
            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.len() >= 2 {
                let name = tokens[1].to_string();
                let unit = if tokens.len() >= 3 {
                    match tokens[2].to_ascii_uppercase().as_str() {
                        "V" => TraceUnit::VoltageVolts,
                        "A" => TraceUnit::CurrentAmperes,
                        "S" | "SEC" => TraceUnit::TimeSeconds,
                        "HZ" => TraceUnit::FrequencyHertz,
                        _ => TraceUnit::Dimensionless,
                    }
                } else if name.starts_with("V(") {
                    TraceUnit::VoltageVolts
                } else if name.starts_with("I(") {
                    TraceUnit::CurrentAmperes
                } else {
                    TraceUnit::Dimensionless
                };

                if trace_defs.is_empty() && (unit == TraceUnit::TimeSeconds || unit == TraceUnit::FrequencyHertz) {
                    x_name = name;
                    x_unit = unit;
                } else {
                    trace_defs.push((name, unit));
                    trace_data.push(Vec::new());
                }
            }
        } else if trimmed.starts_with("#D") {
            in_data = true;
            in_header = false;
        } else if in_data && !trimmed.starts_with('#') {
            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if !tokens.is_empty() {
                if let Ok(x) = tokens[0].parse::<f64>() {
                    x_vals.push(x);
                    for (i, tok) in tokens[1..].iter().enumerate() {
                        if i < trace_data.len() {
                            if let Ok(val) = tok.parse::<f64>() {
                                trace_data[i].push(val);
                            }
                        }
                    }
                }
            }
        } else if in_header && trimmed.starts_with("TITLE") {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                title = parts[1].trim().to_string();
            }
        }
    }

    let traces = trace_defs
        .into_iter()
        .zip(trace_data.into_iter())
        .map(|((name, unit), values)| WaveformTrace { name, unit, values })
        .collect();

    Ok(WaveformDataset {
        title,
        analysis_name: "PSpice Analysis".to_string(),
        x_trace: WaveformTrace {
            name: x_name,
            unit: x_unit,
            values: x_vals,
        },
        traces,
        operating_point: BTreeMap::new(),
        log: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_csdf_sample() {
        let sample = r#"#H
TITLE = RC Transient Analysis
#N TIME S
#N V(OUT) V
#N I(R1) A
#D
0.000000 0.000000 0.005000
0.000100 0.632000 0.004368
0.000200 0.864000 0.003816
"#;
        let ds = parse_csdf(sample).expect("parsed CSDF");
        assert_eq!(ds.title, "RC Transient Analysis");
        assert_eq!(ds.x_trace.name, "TIME");
        assert_eq!(ds.x_trace.values.len(), 3);
        assert_eq!(ds.traces.len(), 2);
        assert_eq!(ds.traces[0].name, "V(OUT)");
        assert_eq!(ds.traces[0].values[1], 0.632);
        assert_eq!(ds.traces[1].name, "I(R1)");
        assert_eq!(ds.traces[1].values[0], 0.005);
    }
}
