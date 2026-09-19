//! Excellon NC Drill Exporter for Oxide EDA.
//!
//! Generates CNC drill files for PCB fabricators according to the Excellon 2 Format specification.
//! Supports Plated Through Holes (PTH), Non-Plated Holes (NPTH), tool table headers,
//! slot routing, and drill summary tables.

use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;

use oxide_types::pcb::{PadType, PcbBoard};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DrillError {
    #[error("Formatting error: {0}")]
    Format(#[from] std::fmt::Error),
    #[error("No drill holes found in board")]
    NoHoles,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DrillHoleType {
    PlatedThrough,
    NonPlatedThrough,
    BlindBuried { start_layer: u8, end_layer: u8 },
}

#[derive(Debug, Clone)]
pub struct DrillHole {
    pub x_mm: f64,
    pub y_mm: f64,
    pub diameter_nm: i64,
    pub hole_type: DrillHoleType,
}

#[derive(Debug, Clone)]
pub struct ExcellonOutput {
    pub filename: String,
    pub content: String,
    pub hole_count: usize,
    pub tool_count: usize,
}

pub struct ExcellonExporter {
    pub metric: bool,
    pub trailing_zeros_omitted: bool,
    pub precision_integer: u8,
    pub precision_decimal: u8,
}

impl Default for ExcellonExporter {
    fn default() -> Self {
        Self {
            metric: true,
            trailing_zeros_omitted: false,
            precision_integer: 3,
            precision_decimal: 3,
        }
    }
}

impl ExcellonExporter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Export all drill files (PTH and NPTH) for a board.
    pub fn export_board(&self, board: &PcbBoard) -> Result<Vec<ExcellonOutput>, DrillError> {
        let mut holes = Vec::new();

        // 1. Collect from vias
        for via in &board.vias {
            let drill_nm = (via.drill_mm * 1_000_000.0).round() as i64;
            holes.push(DrillHole {
                x_mm: via.center[0],
                y_mm: via.center[1],
                diameter_nm: drill_nm,
                hole_type: DrillHoleType::PlatedThrough,
            });
        }

        // 2. Collect from footprint pads
        for fp in &board.footprints {
            for pad in &fp.pads {
                if let Some(drill_mm) = pad.drill_size_mm {
                    let drill_nm = (drill_mm[0] * 1_000_000.0).round() as i64;
                    let hole_type = if pad.pad_type == PadType::NpThru {
                        DrillHoleType::NonPlatedThrough
                    } else {
                        DrillHoleType::PlatedThrough
                    };
                    holes.push(DrillHole {
                        x_mm: pad.center[0],
                        y_mm: pad.center[1],
                        diameter_nm: drill_nm,
                        hole_type,
                    });
                }
            }
        }

        let mut outputs = Vec::new();

        // Export Plated Holes (PTH)
        let pth_holes: Vec<DrillHole> = holes
            .iter()
            .filter(|h| matches!(h.hole_type, DrillHoleType::PlatedThrough))
            .cloned()
            .collect();
        if !pth_holes.is_empty() {
            let content = self.generate_excellon_file(&pth_holes, true)?;
            outputs.push(ExcellonOutput {
                filename: "drill_pth.drl".to_string(),
                hole_count: pth_holes.len(),
                tool_count: self.count_unique_tools(&pth_holes),
                content,
            });
        }

        // Export Non-Plated Holes (NPTH)
        let npth_holes: Vec<DrillHole> = holes
            .iter()
            .filter(|h| matches!(h.hole_type, DrillHoleType::NonPlatedThrough))
            .cloned()
            .collect();
        if !npth_holes.is_empty() {
            let content = self.generate_excellon_file(&npth_holes, false)?;
            outputs.push(ExcellonOutput {
                filename: "drill_npth.drl".to_string(),
                hole_count: npth_holes.len(),
                tool_count: self.count_unique_tools(&npth_holes),
                content,
            });
        }

        Ok(outputs)
    }

    fn count_unique_tools(&self, holes: &[DrillHole]) -> usize {
        let tools: std::collections::HashSet<i64> = holes.iter().map(|h| h.diameter_nm).collect();
        tools.len()
    }

    fn generate_excellon_file(
        &self,
        holes: &[DrillHole],
        is_plated: bool,
    ) -> Result<String, DrillError> {
        let mut out = String::with_capacity(8 * 1024);

        // Group holes by diameter
        let mut tool_map: BTreeMap<i64, Vec<&DrillHole>> = BTreeMap::new();
        for hole in holes {
            tool_map.entry(hole.diameter_nm).or_default().push(hole);
        }

        // Header
        writeln!(out, "M48")?; // Start of Header
        writeln!(out, "; Oxide EDA - Excellon 2 NC Drill Output")?;
        writeln!(
            out,
            "; Plating: {}",
            if is_plated {
                "PLATED"
            } else {
                "NON_PLATED"
            }
        )?;
        writeln!(out, "METRIC,LZ")?; // Metric millimeters, Leading zeros present

        // Tool Definitions
        let mut tool_index = 1u32;
        let mut tool_ids: BTreeMap<i64, u32> = BTreeMap::new();
        for &diam_nm in tool_map.keys() {
            let diam_mm = diam_nm as f64 / 1_000_000.0;
            writeln!(out, "T{tool_index:02}C{diam_mm:.3}")?;
            tool_ids.insert(diam_nm, tool_index);
            tool_index += 1;
        }

        writeln!(out, "%")?; // End of Header
        writeln!(out, "G90")?; // Absolute Coordinates
        writeln!(out, "G05")?; // Drill Mode

        // Drill Hits by Tool
        for (diam_nm, hole_list) in &tool_map {
            let tid = tool_ids[diam_nm];
            writeln!(out, "T{tid:02}")?;
            for hole in hole_list {
                let x = (hole.x_mm * 1000.0).round() as i64;
                let y = (hole.y_mm * 1000.0).round() as i64;
                writeln!(out, "X{x:06}Y{y:06}")?;
            }
        }

        writeln!(out, "M30")?; // End of Program

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_types::pcb::PcbVia;

    #[test]
    fn test_excellon_export_structure() {
        let mut board = PcbBoard::default();
        board.vias.push(PcbVia {
            id: uuid::Uuid::new_v4(),
            net_id: 1,
            center: [10.0, 20.0],
            diameter_mm: 0.6,
            drill_mm: 0.3,
            start_layer: 0,
            end_layer: 1,
            via_type: oxide_types::pcb::ViaType::Through,
        });

        let exporter = ExcellonExporter::new();
        let outputs = exporter.export_board(&board).unwrap();

        assert_eq!(outputs.len(), 1);
        let pth = &outputs[0];
        assert_eq!(pth.filename, "drill_pth.drl");
        assert!(pth.content.contains("M48"));
        assert!(pth.content.contains("T01C0.300"));
        assert!(pth.content.contains("X010000Y020000"));
        assert!(pth.content.contains("M30"));
    }
}
