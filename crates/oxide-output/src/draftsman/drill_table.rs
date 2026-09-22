//! Automated Drill Table & Legend Synthesizer for Draftsman.
//!
//! Conforms to Master Technical Directive §4.5:
//! - Live calculation and grouping of drill symbols by diameter, tolerance, plating condition, and hole count.
//! - Direct associativity with PCB layout database (pads, vias, mounting holes).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use oxide_types::pcb::{PcbBoard, PadType};

/// Standard Drill Hole Plating Condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HolePlating {
    Plated,
    NonPlated,
}

/// A row in the fabrication drill table.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrillTableRow {
    pub symbol_char: char,
    pub diameter_nm: i64,
    pub diameter_mm: f64,
    pub diameter_mil: f64,
    pub plating: HolePlating,
    pub count: usize,
    pub tolerance_plus_mm: f64,
    pub tolerance_minus_mm: f64,
    pub layer_pair: String,
}

/// Drill Table Legend synthesized directly from PCB board layout.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DrillTable {
    pub rows: Vec<DrillTableRow>,
    pub total_hole_count: usize,
}

impl DrillTable {
    /// Available standard symbol characters for drill legends.
    const SYMBOL_PALETTE: &'static [char] = &[
        '⊕', '⊗', '⊙', '⊘', '⊚', '⊛', '⊜', '⊝',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'J', 'K', 'L', 'M', 'N', 'P', 'R', 'S',
    ];

    /// Synthesizes a live, grouped drill table from a `PcbBoard`.
    pub fn from_board(board: &PcbBoard) -> Self {
        // Grouping key: (diameter_nm, plating, layer_pair)
        let mut hole_groups: BTreeMap<(i64, HolePlating, String), usize> = BTreeMap::new();
        let mut total_holes = 0;

        // 1. Collect footprint through-hole and mounting pads
        for footprint in &board.footprints {
            for pad in &footprint.pads {
                if pad.pad_type == PadType::ThroughHole {
                    if let Some(drill_nm) = pad.drill_diameter {
                        if drill_nm > 0 {
                            let plating = HolePlating::Plated;
                            let key = (drill_nm, plating, "Top-Bottom".to_string());
                            *hole_groups.entry(key).or_insert(0) += 1;
                            total_holes += 1;
                        }
                    }
                }
            }
        }

        // 2. Collect through-hole, blind, and buried vias
        for via in &board.vias {
            if via.drill_nm > 0 {
                let plating = HolePlating::Plated;
                let layer_pair = if let Some(ref span) = via.via_span {
                    format!("{}-{}", span.start_layer, span.end_layer)
                } else {
                    "Top-Bottom".to_string()
                };
                let key = (via.drill_nm, plating, layer_pair);
                *hole_groups.entry(key).or_insert(0) += 1;
                total_holes += 1;
            }
        }

        // 3. Assemble table rows and assign unique legend symbols
        let mut rows = Vec::new();
        let mut symbol_idx = 0;

        for ((diam_nm, plating, layer_pair), count) in hole_groups {
            let symbol_char = Self::SYMBOL_PALETTE[symbol_idx % Self::SYMBOL_PALETTE.len()];
            symbol_idx += 1;

            let diam_mm = diam_nm as f64 / 1_000_000.0;
            let diam_mil = diam_mm / 0.0254;

            // Default IPC-2221 drill hole tolerances: +0.08mm / -0.05mm for PTH, +/-0.05mm for NPTH
            let (tol_plus, tol_minus) = match plating {
                HolePlating::Plated => (0.08, 0.05),
                HolePlating::NonPlated => (0.05, 0.05),
            };

            rows.push(DrillTableRow {
                symbol_char,
                diameter_nm: diam_nm,
                diameter_mm: diam_mm,
                diameter_mil: diam_mil,
                plating,
                count,
                tolerance_plus_mm: tol_plus,
                tolerance_minus_mm: tol_minus,
                layer_pair,
            });
        }

        Self {
            rows,
            total_hole_count: total_holes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_types::pcb::{Footprint, Pad, Point, Via};

    #[test]
    fn test_drill_table_synthesis() {
        let mut board = PcbBoard::default();

        // Add 2 identical vias
        board.vias.push(Via {
            id: uuid::Uuid::new_v4(),
            position: Point::new(1000, 1000),
            net_id: None,
            size_nm: 600_000,
            drill_nm: 300_000,
            via_type: oxide_types::pcb::ViaType::Through,
            via_span: None,
        });
        board.vias.push(Via {
            id: uuid::Uuid::new_v4(),
            position: Point::new(2000, 2000),
            net_id: None,
            size_nm: 600_000,
            drill_nm: 300_000,
            via_type: oxide_types::pcb::ViaType::Through,
            via_span: None,
        });

        // Add 1 through-hole component with 1.0mm (1_000_000 nm) drill
        let mut fp = Footprint::default();
        fp.pads.push(Pad {
            number: "1".to_string(),
            net_id: None,
            position: Point::new(5000, 5000),
            size_nm: Point::new(1_600_000, 1_600_000),
            pad_type: PadType::ThroughHole,
            drill_diameter: Some(1_000_000),
            shape: oxide_types::pcb::PadShape::Circle,
            layer: oxide_types::pcb::Layer::F_Cu,
        });
        board.footprints.push(fp);

        let table = DrillTable::from_board(&board);
        assert_eq!(table.total_hole_count, 3);
        assert_eq!(table.rows.len(), 2);
        
        let via_row = table.rows.iter().find(|r| r.diameter_nm == 300_000).unwrap();
        assert_eq!(via_row.count, 2);
        assert_eq!(via_row.diameter_mm, 0.3);

        let th_row = table.rows.iter().find(|r| r.diameter_nm == 1_000_000).unwrap();
        assert_eq!(th_row.count, 1);
        assert_eq!(th_row.diameter_mm, 1.0);
    }
}
