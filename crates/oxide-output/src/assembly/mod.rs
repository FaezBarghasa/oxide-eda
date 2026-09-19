//! Pick-and-Place (Centroid / CPL / XY) & Assembly Exporter for Oxide EDA.
//!
//! Generates standard component placement files for automated Surface Mount Technology (SMT)
//! pick-and-place machines and contract manufacturers (JLCPCB, PCBWay, MacroFab, etc.).

use std::fmt::Write as FmtWrite;

use oxide_types::pcb::PcbBoard;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssemblyError {
    #[error("Formatting error: {0}")]
    Format(#[from] std::fmt::Error),
    #[error("No components found for assembly")]
    NoComponents,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssemblyLayer {
    Top,
    Bottom,
    Both,
}

#[derive(Debug, Clone)]
pub struct PickAndPlaceOptions {
    pub layer: AssemblyLayer,
    pub delimiter: char,
    pub include_header: bool,
    pub quote_strings: bool,
}

impl Default for PickAndPlaceOptions {
    fn default() -> Self {
        Self {
            layer: AssemblyLayer::Both,
            delimiter: ',',
            include_header: true,
            quote_strings: true,
        }
    }
}

pub struct PickAndPlaceExporter {
    options: PickAndPlaceOptions,
}

impl PickAndPlaceExporter {
    pub fn new(options: PickAndPlaceOptions) -> Self {
        Self { options }
    }

    /// Export Pick-and-Place CPL / Centroid data from a PCB board.
    pub fn export(&self, board: &PcbBoard) -> Result<String, AssemblyError> {
        let mut out = String::with_capacity(8 * 1024);

        if self.options.include_header {
            if self.options.delimiter == ',' {
                writeln!(
                    out,
                    "Designator,Footprint,Mid X,Mid Y,Ref X,Ref Y,Layer,Rotation,Comment"
                )?;
            } else {
                writeln!(
                    out,
                    "Designator\tFootprint\tMid X\tMid Y\tRef X\tRef Y\tLayer\tRotation\tComment"
                )?;
            }
        }

        for fp in &board.footprints {
            let is_top = fp.layer.is_empty() || fp.layer.to_lowercase().contains("top") || fp.layer == "F.Cu";
            let layer_name = if is_top { "Top" } else { "Bottom" };

            // Layer filtering
            match self.options.layer {
                AssemblyLayer::Top if !is_top => continue,
                AssemblyLayer::Bottom if is_top => continue,
                _ => {}
            }

            let designator = &fp.reference;
            let footprint_name = &fp.footprint_id;
            let mid_x = fp.position.x;
            let mid_y = fp.position.y;
            let ref_x = fp.position.x;
            let ref_y = fp.position.y;
            let rotation = fp.rotation;
            let comment = &fp.value;

            if self.options.delimiter == ',' {
                if self.options.quote_strings {
                    writeln!(
                        out,
                        "\"{designator}\",\"{footprint_name}\",{mid_x:.4},{mid_y:.4},{ref_x:.4},{ref_y:.4},\"{layer_name}\",{rotation:.1},\"{comment}\""
                    )?;
                } else {
                    writeln!(
                        out,
                        "{designator},{footprint_name},{mid_x:.4},{mid_y:.4},{ref_x:.4},{ref_y:.4},{layer_name},{rotation:.1},{comment}"
                    )?;
                }
            } else {
                writeln!(
                    out,
                    "{designator}\t{footprint_name}\t{mid_x:.4}\t{mid_y:.4}\t{ref_x:.4}\t{ref_y:.4}\t{layer_name}\t{rotation:.1}\t{comment}"
                )?;
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_types::pcb::{Footprint, Point};

    #[test]
    fn test_pick_and_place_export_format() {
        let mut board = PcbBoard::default();
        board.footprints.push(Footprint {
            uuid: uuid::Uuid::new_v4(),
            reference: "U1".to_string(),
            value: "STM32F401RCT6".to_string(),
            footprint_id: "LQFP-64_10x10mm_P0.5mm".to_string(),
            layer: "F.Cu".to_string(),
            position: Point { x: 50.0, y: 50.0 },
            rotation: 90.0,
            locked: false,
            pads: Vec::new(),
            graphics: Vec::new(),
            properties: Vec::new(),
        });

        let exporter = PickAndPlaceExporter::new(PickAndPlaceOptions::default());
        let csv = exporter.export(&board).unwrap();

        assert!(csv.contains("Designator,Footprint,Mid X,Mid Y"));
        assert!(csv.contains("\"U1\",\"LQFP-64_10x10mm_P0.5mm\",50.0000,50.0000"));
        assert!(csv.contains("\"Top\",90.0,\"STM32F401RCT6\""));
    }
}
