//! Parser for Altium pipe-delimited property list records.

use std::collections::HashMap;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::AltiumImportError;

/// A parsed Altium property record with typed getters.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AltiumRecord {
    pub properties: HashMap<String, String>,
}

impl AltiumRecord {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| v.parse::<i64>().ok())
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| v.parse::<f64>().ok())
    }

    pub fn get_bool(&self, key: &str) -> bool {
        match self.get(key) {
            Some("TRUE") | Some("true") | Some("1") | Some("T") => true,
            _ => false,
        }
    }

    /// Convert Altium DXP internal coordinate (1/10000 inch = 0.1 mil = 0.00254 mm) or point to mm.
    pub fn get_coord_mm(&self, key: &str) -> Option<f64> {
        // In Altium schematics, coordinates are typically in 10-mil (pt) or 1-mil units
        // 1 pt = 0.127 mm (10 mil = 0.254 mm)
        self.get_f64(key).map(|val| val * 0.0254) // 1 mil = 0.0254 mm
    }

    /// Parse a single pipe-delimited string (e.g. `|RECORD=1|LOCATION.X=100|...`) into an [`AltiumRecord`].
    pub fn from_pipe_str(s: &str) -> Self {
        let mut properties = HashMap::new();
        for item in s.split('|') {
            if let Some((k, v)) = item.split_once('=') {
                properties.insert(k.trim().to_uppercase(), v.trim().to_string());
            }
        }
        Self { properties }
    }
}

/// Parse an entire stream of length-prefixed pipe-delimited records.
pub fn parse_record_stream(data: &[u8]) -> Result<Vec<AltiumRecord>, AltiumImportError> {
    let mut records = Vec::new();
    let mut cursor = 0;

    while cursor < data.len() {
        if cursor + 4 > data.len() {
            // Check if remaining bytes are plain text
            let remaining = String::from_utf8_lossy(&data[cursor..]);
            if remaining.contains('|') {
                records.push(AltiumRecord::from_pipe_str(&remaining));
            }
            break;
        }

        let len = (&data[cursor..cursor + 4]).read_u32::<LittleEndian>()? as usize;
        cursor += 4;

        if len == 0 || cursor + len > data.len() {
            // Try fallback text scan if length prefix is invalid
            let text = String::from_utf8_lossy(&data[cursor - 4..]);
            for chunk in text.split('\0') {
                if chunk.contains('|') {
                    records.push(AltiumRecord::from_pipe_str(chunk));
                }
            }
            break;
        }

        let record_bytes = &data[cursor..cursor + len];
        cursor += len;

        // Skip potential null terminator or padding
        if cursor < data.len() && data[cursor] == 0 {
            cursor += 1;
        }

        let s = String::from_utf8_lossy(record_bytes);
        if s.contains('|') {
            records.push(AltiumRecord::from_pipe_str(&s));
        }
    }

    Ok(records)
}
