//! Net scoping and wire topology redundancy pruning.

use oxide_types::schematic::Wire;
use serde::{Deserialize, Serialize};

/// Net naming and hierarchy scoping mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NetScope {
    /// Unique to the local sheet instance; names are scoped as `SheetID/NetName`.
    #[default]
    Local,
    /// Shared across the entire project (e.g. power rails like GND, VCC_3V3, +5V).
    Global,
    /// Inter-sheet connectivity resolved strictly across SheetSymbol pins and child sheet Ports.
    Hierarchical,
}

/// Resolve a net identifier based on its hierarchy scope.
pub fn resolve_scoped_net_name(sheet_id: &str, raw_name: &str, scope: NetScope) -> String {
    match scope {
        NetScope::Global => raw_name.to_string(),
        NetScope::Local => {
            if sheet_id.is_empty() {
                raw_name.to_string()
            } else {
                format!("{sheet_id}/{raw_name}")
            }
        }
        NetScope::Hierarchical => {
            if sheet_id.is_empty() {
                raw_name.to_string()
            } else {
                format!("{sheet_id}:{raw_name}")
            }
        }
    }
}

/// Detect if a power rail name implies global net scoping.
pub fn is_canonical_global_power_rail(net_name: &str) -> bool {
    let upper = net_name.to_uppercase();
    matches!(
        upper.as_str(),
        "GND"
            | "AGND"
            | "DGND"
            | "PGND"
            | "EARTH"
            | "VCC"
            | "VDD"
            | "VEE"
            | "VSS"
            | "VBAT"
            | "3V3"
            | "+3V3"
            | "VCC_3V3"
            | "5V"
            | "+5V"
            | "VCC_5V"
            | "12V"
            | "+12V"
            | "-12V"
            | "1V8"
            | "+1V8"
            | "1V2"
            | "+1V2"
            | "0V85"
    )
}

/// Prune collinear overlapping, duplicate, and zero-length wires from a sheet.
/// Returns the number of redundant wire elements eliminated.
pub fn prune_wire_redundancies(wires: &mut Vec<Wire>) -> usize {
    let initial_count = wires.len();
    if initial_count <= 1 {
        return 0;
    }

    // 1. Remove zero-length wires
    wires.retain(|w| {
        let dx = (w.end.x - w.start.x).abs();
        let dy = (w.end.y - w.start.y).abs();
        dx > 1e-6 || dy > 1e-6
    });

    // 2. Canonicalize wire endpoints (min point first)
    for w in wires.iter_mut() {
        if w.start.x > w.end.x || ((w.start.x - w.end.x).abs() < 1e-6 && w.start.y > w.end.y) {
            std::mem::swap(&mut w.start, &mut w.end);
        }
    }

    // 3. Merge collinear overlapping segments on horizontal or vertical axes
    let mut merged = Vec::with_capacity(wires.len());
    let mut visited = vec![false; wires.len()];

    for i in 0..wires.len() {
        if visited[i] {
            continue;
        }
        let mut cur = wires[i].clone();
        visited[i] = true;

        for j in (i + 1)..wires.len() {
            if visited[j] {
                continue;
            }
            let other = &wires[j];

            // Check if both are horizontal and share same Y
            if (cur.start.y - cur.end.y).abs() < 1e-6
                && (other.start.y - other.end.y).abs() < 1e-6
                && (cur.start.y - other.start.y).abs() < 1e-6
            {
                // Check if intervals [cur.start.x, cur.end.x] and [other.start.x, other.end.x] overlap or touch
                if other.start.x <= cur.end.x + 1e-6 && other.end.x >= cur.start.x - 1e-6 {
                    cur.start.x = cur.start.x.min(other.start.x);
                    cur.end.x = cur.end.x.max(other.end.x);
                    visited[j] = true;
                }
            }
            // Check if both are vertical and share same X
            else if (cur.start.x - cur.end.x).abs() < 1e-6
                && (other.start.x - other.end.x).abs() < 1e-6
                && (cur.start.x - other.start.x).abs() < 1e-6
            {
                // Check if intervals [cur.start.y, cur.end.y] and [other.start.y, other.end.y] overlap or touch
                if other.start.y <= cur.end.y + 1e-6 && other.end.y >= cur.start.y - 1e-6 {
                    cur.start.y = cur.start.y.min(other.start.y);
                    cur.end.y = cur.end.y.max(other.end.y);
                    visited[j] = true;
                }
            }
        }
        merged.push(cur);
    }

    let pruned = initial_count.saturating_sub(merged.len());
    *wires = merged;
    pruned
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_types::schematic::Point;
    use uuid::Uuid;

    #[test]
    fn test_resolve_scoped_net_names() {
        assert_eq!(
            resolve_scoped_net_name("Sheet1", "RESET", NetScope::Local),
            "Sheet1/RESET"
        );
        assert_eq!(
            resolve_scoped_net_name("Sheet1", "GND", NetScope::Global),
            "GND"
        );
        assert_eq!(
            resolve_scoped_net_name("Sheet1", "DATA0", NetScope::Hierarchical),
            "Sheet1:DATA0"
        );
    }

    #[test]
    fn test_canonical_power_rails() {
        assert!(is_canonical_global_power_rail("GND"));
        assert!(is_canonical_global_power_rail("vcc_3v3"));
        assert!(is_canonical_global_power_rail("+5V"));
        assert!(!is_canonical_global_power_rail("SPI0_MOSI"));
    }

    #[test]
    fn test_prune_redundant_collinear_wires() {
        let mut wires = vec![
            Wire {
                uuid: Uuid::new_v4(),
                start: Point::new(0.0, 10.0),
                end: Point::new(10.0, 10.0),
                stroke_width: 0.15,
            },
            Wire {
                uuid: Uuid::new_v4(),
                start: Point::new(5.0, 10.0),
                end: Point::new(20.0, 10.0),
                stroke_width: 0.15,
            },
            Wire {
                uuid: Uuid::new_v4(),
                start: Point::new(2.0, 2.0),
                end: Point::new(2.0, 2.0), // Zero-length
                stroke_width: 0.15,
            },
        ];

        let pruned = prune_wire_redundancies(&mut wires);
        assert_eq!(pruned, 2);
        assert_eq!(wires.len(), 1);
        assert_eq!(wires[0].start.x, 0.0);
        assert_eq!(wires[0].end.x, 20.0);
    }
}
