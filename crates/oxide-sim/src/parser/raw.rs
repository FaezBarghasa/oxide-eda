//! SPICE raw file parser (ASCII and Binary format).

use std::collections::BTreeMap;
use oxide_types::sim::{TraceUnit, WaveformDataset, WaveformTrace};
use crate::simulator::SimError;

/// Parses a SPICE raw data byte buffer into a [`WaveformDataset`].
pub fn parse_spice_raw(bytes: &[u8]) -> Result<WaveformDataset, SimError> {
    // Locate header end and determine if ASCII or Binary
    let content = String::from_utf8_lossy(bytes);
    let mut title = String::new();
    let mut analysis_name = "Transient".to_string();
    let mut is_complex = false;
    let mut num_variables = 0usize;
    let mut num_points = 0usize;
    let mut var_names: Vec<(String, TraceUnit)> = Vec::new();

    let mut lines = content.lines();
    let mut in_variables = false;
    let mut is_binary = false;
    let mut binary_offset = 0usize;

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.starts_with("Title:") {
            title = trimmed.trim_start_matches("Title:").trim().to_string();
        } else if trimmed.starts_with("Plotname:") {
            analysis_name = trimmed.trim_start_matches("Plotname:").trim().to_string();
        } else if trimmed.starts_with("Flags:") {
            if trimmed.contains("complex") {
                is_complex = true;
            }
        } else if trimmed.starts_with("No. Variables:") {
            if let Ok(n) = trimmed.trim_start_matches("No. Variables:").trim().parse::<usize>() {
                num_variables = n;
            }
        } else if trimmed.starts_with("No. Points:") {
            if let Ok(n) = trimmed.trim_start_matches("No. Points:").trim().parse::<usize>() {
                num_points = n;
            }
        } else if trimmed.starts_with("Variables:") {
            in_variables = true;
        } else if in_variables && (trimmed.starts_with("Values:") || trimmed.starts_with("Binary:")) {
            if trimmed.starts_with("Binary:") {
                is_binary = true;
                // Calculate byte offset where binary data starts
                let match_str = "Binary:\n";
                let match_str_crlf = "Binary:\r\n";
                if let Some(pos) = bytes.windows(match_str.len()).position(|w| w == match_str.as_bytes()) {
                    binary_offset = pos + match_str.len();
                } else if let Some(pos) = bytes.windows(match_str_crlf.len()).position(|w| w == match_str_crlf.as_bytes()) {
                    binary_offset = pos + match_str_crlf.len();
                }
            }
            break;
        } else if in_variables {
            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.len() >= 3 {
                let name = tokens[1].to_string();
                let type_str = tokens[2].to_ascii_lowercase();
                let unit = match type_str.as_str() {
                    "time" => TraceUnit::TimeSeconds,
                    "frequency" => TraceUnit::FrequencyHertz,
                    "voltage" => TraceUnit::VoltageVolts,
                    "current" => TraceUnit::CurrentAmperes,
                    _ => {
                        if name.starts_with("v(") || name.starts_with("V(") {
                            TraceUnit::VoltageVolts
                        } else if name.starts_with("i(") || name.starts_with("I(") {
                            TraceUnit::CurrentAmperes
                        } else {
                            TraceUnit::Dimensionless
                        }
                    }
                };
                var_names.push((name, unit));
            }
        }
    }

    if var_names.is_empty() {
        return Err(SimError::ParseError("No variables found in SPICE raw header".to_string()));
    }

    let mut traces_data: Vec<Vec<f64>> = vec![Vec::with_capacity(num_points); var_names.len()];

    if is_binary && binary_offset > 0 && binary_offset < bytes.len() {
        // Binary reading
        let raw_data = &bytes[binary_offset..];
        let float_size = 8; // f64 is standard in modern ngspice
        let point_stride = if is_complex {
            num_variables * float_size * 2
        } else {
            num_variables * float_size
        };

        let available_points = raw_data.len() / point_stride;
        let points_to_read = if num_points > 0 { num_points.min(available_points) } else { available_points };

        for p in 0..points_to_read {
            for v in 0..var_names.len() {
                let offset = if is_complex {
                    p * point_stride + v * float_size * 2
                } else {
                    p * point_stride + v * float_size
                };

                if offset + 8 <= raw_data.len() {
                    let val = f64::from_le_bytes(raw_data[offset..offset + 8].try_into().unwrap_or([0; 8]));
                    traces_data[v].push(val);
                }
            }
        }
    } else {
        // ASCII reading
        let mut current_var = 0;
        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.len() == 2 {
                // e.g., "0 0.00000e+00" (first variable with index)
                if let Ok(val) = tokens[1].parse::<f64>() {
                    current_var = 0;
                    if current_var < traces_data.len() {
                        traces_data[current_var].push(val);
                        current_var += 1;
                    }
                }
            } else if tokens.len() == 1 {
                // Subsequent variables for the current point
                if let Ok(val) = tokens[0].parse::<f64>() {
                    if current_var < traces_data.len() {
                        traces_data[current_var].push(val);
                        current_var += 1;
                    }
                }
            }
        }
    }

    let (x_name, x_unit) = var_names.first().cloned().unwrap_or(("time".to_string(), TraceUnit::TimeSeconds));
    let x_vals = if !traces_data.is_empty() { traces_data[0].clone() } else { Vec::new() };

    let mut traces = Vec::new();
    for (i, (name, unit)) in var_names.iter().enumerate().skip(1) {
        if i < traces_data.len() {
            traces.push(WaveformTrace {
                name: name.clone(),
                unit: *unit,
                values: traces_data[i].clone(),
            });
        }
    }

    Ok(WaveformDataset {
        title,
        analysis_name,
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
    fn parse_ascii_spice_raw() {
        let raw = r#"Title: Test Circuit
Date: Mon Sep 14 12:00:00 2026
Plotname: Transient Analysis
Flags: real
No. Variables: 3
No. Points: 2
Variables:
	0	time	time	defaultscale
	1	v(in)	voltage
	2	v(out)	voltage
Values:
0	0.000000e+00
	5.000000e+00
	0.000000e+00
1	1.000000e-03
	5.000000e+00
	3.160000e+00
"#;
        let ds = parse_spice_raw(raw.as_bytes()).expect("parsed raw");
        assert_eq!(ds.title, "Test Circuit");
        assert_eq!(ds.analysis_name, "Transient Analysis");
        assert_eq!(ds.x_trace.name, "time");
        assert_eq!(ds.x_trace.values.len(), 2);
        assert_eq!(ds.x_trace.values[1], 1e-3);
        assert_eq!(ds.traces.len(), 2);
        assert_eq!(ds.traces[0].name, "v(in)");
        assert_eq!(ds.traces[0].values[0], 5.0);
        assert_eq!(ds.traces[1].name, "v(out)");
        assert_eq!(ds.traces[1].values[1], 3.16);
    }
}
