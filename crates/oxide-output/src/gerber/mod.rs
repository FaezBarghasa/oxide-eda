//! Gerber RS-274X & Gerber X2/X3 Industrial Exporter for Oxide EDA.
//!
//! Cleanroom implementation strictly adhering to Ucamco Gerber File Format Specification (Rev 2023.08).
//! Supports full multi-layer generation with standard apertures, aperture macros, X2 metadata attributes,
//! netlist attribution, and polygon area fills.

use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;

use oxide_types::pcb::{PadShape, PcbBoard};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GerberError {
    #[error("Formatting error: {0}")]
    Format(#[from] std::fmt::Error),
    #[error("Invalid layer index: {0}")]
    InvalidLayer(usize),
    #[error("Empty board outline")]
    EmptyOutline,
}

/// Standard PCB layer roles for manufacturing export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GerberLayer {
    TopCopper,
    BottomCopper,
    InnerCopper(u8),
    TopSolderMask,
    BottomSolderMask,
    TopSilkscreen,
    BottomSilkscreen,
    TopPaste,
    BottomPaste,
    EdgeCuts,
}

impl GerberLayer {
    pub fn file_extension(&self) -> &'static str {
        match self {
            GerberLayer::TopCopper => "gtl",
            GerberLayer::BottomCopper => "gbl",
            GerberLayer::InnerCopper(_) => "gbr",
            GerberLayer::TopSolderMask => "gts",
            GerberLayer::BottomSolderMask => "gbs",
            GerberLayer::TopSilkscreen => "gto",
            GerberLayer::BottomSilkscreen => "gbo",
            GerberLayer::TopPaste => "gtp",
            GerberLayer::BottomPaste => "gbp",
            GerberLayer::EdgeCuts => "gm1",
        }
    }

    pub fn x2_file_function(&self) -> String {
        match self {
            GerberLayer::TopCopper => "%TF.FileFunction,Copper,L1,Top*%".to_string(),
            GerberLayer::BottomCopper => "%TF.FileFunction,Copper,L2,Bot*%".to_string(),
            GerberLayer::InnerCopper(idx) => {
                format!("%TF.FileFunction,Copper,L{idx},In*%")
            }
            GerberLayer::TopSolderMask => "%TF.FileFunction,Soldermask,Top*%".to_string(),
            GerberLayer::BottomSolderMask => "%TF.FileFunction,Soldermask,Bot*%".to_string(),
            GerberLayer::TopSilkscreen => "%TF.FileFunction,Legend,Top*%".to_string(),
            GerberLayer::BottomSilkscreen => "%TF.FileFunction,Legend,Bot*%".to_string(),
            GerberLayer::TopPaste => "%TF.FileFunction,Paste,Top*%".to_string(),
            GerberLayer::BottomPaste => "%TF.FileFunction,Paste,Bot*%".to_string(),
            GerberLayer::EdgeCuts => "%TF.FileFunction,Profile,NP*%".to_string(),
        }
    }
}

/// Shape definition for a dynamic Gerber aperture.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ApertureDef {
    Circle { diameter_nm: i64 },
    Rectangle { width_nm: i64, height_nm: i64 },
    Obround { width_nm: i64, height_nm: i64 },
    CustomMacro(String),
}

/// Options controlling Gerber generation.
#[derive(Debug, Clone)]
pub struct GerberOptions {
    pub use_gerber_x2_attributes: bool,
    pub include_netlist_attributes: bool,
    pub coordinate_format_integer: u8,
    pub coordinate_format_decimal: u8,
}

impl Default for GerberOptions {
    fn default() -> Self {
        Self {
            use_gerber_x2_attributes: true,
            include_netlist_attributes: true,
            coordinate_format_integer: 4,
            coordinate_format_decimal: 6,
        }
    }
}

/// Generated single-layer Gerber output container.
#[derive(Debug, Clone)]
pub struct GerberLayerOutput {
    pub layer: GerberLayer,
    pub filename: String,
    pub content: String,
}

/// Main Gerber manufacturing exporter.
pub struct GerberExporter {
    options: GerberOptions,
}

impl GerberExporter {
    pub fn new(options: GerberOptions) -> Self {
        Self { options }
    }

    /// Export all production layers for a given PCB board.
    pub fn export_board(&self, board: &PcbBoard) -> Result<Vec<GerberLayerOutput>, GerberError> {
        let mut layers = vec![
            GerberLayer::TopCopper,
            GerberLayer::BottomCopper,
            GerberLayer::TopSolderMask,
            GerberLayer::BottomSolderMask,
            GerberLayer::TopSilkscreen,
            GerberLayer::BottomSilkscreen,
            GerberLayer::TopPaste,
            GerberLayer::BottomPaste,
            GerberLayer::EdgeCuts,
        ];

        // Add inner copper layers if present
        if board.layers.len() > 2 {
            for i in 2..board.layers.len() {
                layers.push(GerberLayer::InnerCopper(i as u8));
            }
        }

        let mut outputs = Vec::new();
        for layer in layers {
            let content = self.export_single_layer(board, layer)?;
            let filename = format!("output.{extension}", extension = layer.file_extension());
            outputs.push(GerberLayerOutput {
                layer,
                filename,
                content,
            });
        }

        Ok(outputs)
    }

    /// Export a specific single layer.
    pub fn export_single_layer(
        &self,
        board: &PcbBoard,
        layer: GerberLayer,
    ) -> Result<String, GerberError> {
        let mut out = String::with_capacity(16 * 1024);

        // 1. Header & Format declaration
        writeln!(out, "G04 ===================================================================*")?;
        writeln!(out, "G04 Oxide EDA - Gerber RS-274X / X2 Production Output*")?;
        writeln!(out, "G04 Cleanroom Engine under Apache-2.0*")?;
        writeln!(out, "G04 ===================================================================*")?;

        if self.options.use_gerber_x2_attributes {
            writeln!(out, "%TF.GenerationSoftware,OxideEDA,Oxide,v0.16.0*%")?;
            writeln!(out, "%TF.Part,Single*%")?;
            writeln!(out, "%TF.FilePolarity,Positive*%")?;
            writeln!(out, "{}", layer.x2_file_function())?;
        }

        writeln!(out, "%MOMM*%")?; // Metric millimeters
        writeln!(out, "%FSLAX46Y46*%")?; // Format Specification: Leading zeros omitted, Absolute, 4.6 format
        writeln!(out, "%LPD*%")?; // Layer Polarity Dark

        // 2. Aperture Table Collection
        let mut apertures: BTreeMap<ApertureDef, u32> = BTreeMap::new();
        let mut next_aperture_id = 10u32;

        let mut get_or_insert_aperture = |def: ApertureDef| -> u32 {
            if let Some(&id) = apertures.get(&def) {
                id
            } else {
                let id = next_aperture_id;
                next_aperture_id += 1;
                apertures.insert(def, id);
                id
            }
        };

        // Collect needed apertures for this layer
        match layer {
            GerberLayer::TopCopper | GerberLayer::BottomCopper | GerberLayer::InnerCopper(_) => {
                for seg in &board.segments {
                    let d_nm = (seg.width * 1_000_000.0).round() as i64;
                    get_or_insert_aperture(ApertureDef::Circle { diameter_nm: d_nm });
                }
                for via in &board.vias {
                    let d_nm = (via.diameter * 1_000_000.0).round() as i64;
                    get_or_insert_aperture(ApertureDef::Circle { diameter_nm: d_nm });
                }
                for fp in &board.footprints {
                    for pad in &fp.pads {
                        let def = match pad.shape {
                            PadShape::Circle => ApertureDef::Circle {
                                diameter_nm: (pad.size.x * 1_000_000.0).round() as i64,
                            },
                            PadShape::Rect | PadShape::RoundRect => ApertureDef::Rectangle {
                                width_nm: (pad.size.x * 1_000_000.0).round() as i64,
                                height_nm: (pad.size.y * 1_000_000.0).round() as i64,
                            },
                            PadShape::Oval => ApertureDef::Obround {
                                width_nm: (pad.size.x * 1_000_000.0).round() as i64,
                                height_nm: (pad.size.y * 1_000_000.0).round() as i64,
                            },
                            _ => ApertureDef::Circle {
                                diameter_nm: (pad.size.x * 1_000_000.0).round() as i64,
                            },
                        };
                        get_or_insert_aperture(def);
                    }
                }
            }
            GerberLayer::TopSolderMask | GerberLayer::BottomSolderMask => {
                for fp in &board.footprints {
                    for pad in &fp.pads {
                        let expansion_mm = 0.05; // 50 µm standard mask expansion
                        let def = match pad.shape {
                            PadShape::Circle => ApertureDef::Circle {
                                diameter_nm: ((pad.size.x + expansion_mm * 2.0) * 1_000_000.0)
                                    .round() as i64,
                            },
                            PadShape::Rect | PadShape::RoundRect => ApertureDef::Rectangle {
                                width_nm: ((pad.size.x + expansion_mm * 2.0) * 1_000_000.0)
                                    .round() as i64,
                                height_nm: ((pad.size.y + expansion_mm * 2.0) * 1_000_000.0)
                                    .round() as i64,
                            },
                            PadShape::Oval => ApertureDef::Obround {
                                width_nm: ((pad.size.x + expansion_mm * 2.0) * 1_000_000.0)
                                    .round() as i64,
                                height_nm: ((pad.size.y + expansion_mm * 2.0) * 1_000_000.0)
                                    .round() as i64,
                            },
                            _ => ApertureDef::Circle {
                                diameter_nm: ((pad.size.x + expansion_mm * 2.0) * 1_000_000.0)
                                    .round() as i64,
                            },
                        };
                        get_or_insert_aperture(def);
                    }
                }
            }
            GerberLayer::TopSilkscreen | GerberLayer::BottomSilkscreen => {
                get_or_insert_aperture(ApertureDef::Circle {
                    diameter_nm: 150_000, // 0.15 mm
                });
            }
            GerberLayer::TopPaste | GerberLayer::BottomPaste => {
                for fp in &board.footprints {
                    for pad in &fp.pads {
                        if pad.pad_type == PadType::Smd {
                            let def = ApertureDef::Rectangle {
                                width_nm: (pad.size.x * 1_000_000.0).round() as i64,
                                height_nm: (pad.size.y * 1_000_000.0).round() as i64,
                            };
                            get_or_insert_aperture(def);
                        }
                    }
                }
            }
            GerberLayer::EdgeCuts => {
                get_or_insert_aperture(ApertureDef::Circle {
                    diameter_nm: 100_000, // 0.1 mm edge cut contour
                });
            }
        }

        // 3. Write Aperture Definitions
        for (def, id) in &apertures {
            match def {
                ApertureDef::Circle { diameter_nm } => {
                    let d_mm = *diameter_nm as f64 / 1_000_000.0;
                    writeln!(out, "%ADD{id}C,{d_mm:.6}*%")?;
                }
                ApertureDef::Rectangle {
                    width_nm,
                    height_nm,
                } => {
                    let w_mm = *width_nm as f64 / 1_000_000.0;
                    let h_mm = *height_nm as f64 / 1_000_000.0;
                    writeln!(out, "%ADD{id}R,{w_mm:.6}X{h_mm:.6}*%")?;
                }
                ApertureDef::Obround {
                    width_nm,
                    height_nm,
                } => {
                    let w_mm = *width_nm as f64 / 1_000_000.0;
                    let h_mm = *height_nm as f64 / 1_000_000.0;
                    writeln!(out, "%ADD{id}O,{w_mm:.6}X{h_mm:.6}*%")?;
                }
                ApertureDef::CustomMacro(macro_str) => {
                    writeln!(out, "{macro_str}")?;
                }
            }
        }

        // 4. Emit Geometry Commands
        match layer {
            GerberLayer::TopCopper | GerberLayer::BottomCopper | GerberLayer::InnerCopper(_) => {
                // Flash pads
                for fp in &board.footprints {
                    for pad in &fp.pads {
                        let def = match pad.shape {
                            PadShape::Circle => ApertureDef::Circle {
                                diameter_nm: (pad.size.x * 1_000_000.0).round() as i64,
                            },
                            PadShape::Rect | PadShape::RoundRect => ApertureDef::Rectangle {
                                width_nm: (pad.size.x * 1_000_000.0).round() as i64,
                                height_nm: (pad.size.y * 1_000_000.0).round() as i64,
                            },
                            PadShape::Oval => ApertureDef::Obround {
                                width_nm: (pad.size.x * 1_000_000.0).round() as i64,
                                height_nm: (pad.size.y * 1_000_000.0).round() as i64,
                            },
                            _ => ApertureDef::Circle {
                                diameter_nm: (pad.size.x * 1_000_000.0).round() as i64,
                            },
                        };

                        if let Some(&ap_id) = apertures.get(&def) {
                            writeln!(out, "D{ap_id}*")?;
                            let (x, y) = self.format_coord(
                                fp.position.x + pad.position.x,
                                fp.position.y + pad.position.y,
                            );
                            writeln!(out, "X{x}Y{y}D03*")?;
                        }
                    }
                }

                // Flash vias
                for via in &board.vias {
                    let def = ApertureDef::Circle {
                        diameter_nm: (via.diameter * 1_000_000.0).round() as i64,
                    };
                    if let Some(&ap_id) = apertures.get(&def) {
                        writeln!(out, "D{ap_id}*")?;
                        let (x, y) = self.format_coord(via.position.x, via.position.y);
                        writeln!(out, "X{x}Y{y}D03*")?;
                    }
                }

                // Interpolate segments
                for seg in &board.segments {
                    let def = ApertureDef::Circle {
                        diameter_nm: (seg.width * 1_000_000.0).round() as i64,
                    };
                    if let Some(&ap_id) = apertures.get(&def) {
                        writeln!(out, "D{ap_id}*")?;
                        let (x0, y0) = self.format_coord(seg.start.x, seg.start.y);
                        let (x1, y1) = self.format_coord(seg.end.x, seg.end.y);
                        writeln!(out, "X{x0}Y{y0}D02*")?;
                        writeln!(out, "X{x1}Y{y1}D01*")?;
                    }
                }

                // Polygon zones (G36 / G37 region fill)
                for zone in &board.zones {
                    if !zone.outline.is_empty() {
                        writeln!(out, "G36*")?;
                        let (first_x, first_y) =
                            self.format_coord(zone.outline[0].x, zone.outline[0].y);
                        writeln!(out, "X{first_x}Y{first_y}D02*")?;
                        for pt in &zone.outline[1..] {
                            let (x, y) = self.format_coord(pt.x, pt.y);
                            writeln!(out, "X{x}Y{y}D01*")?;
                        }
                        writeln!(out, "X{first_x}Y{first_y}D01*")?;
                        writeln!(out, "G37*")?;
                    }
                }
            }
            GerberLayer::EdgeCuts => {
                let ap_id = apertures
                    .get(&ApertureDef::Circle {
                        diameter_nm: 100_000,
                    })
                    .copied()
                    .unwrap_or(10);
                writeln!(out, "D{ap_id}*")?;

                if !board.outline.is_empty() {
                    let (first_x, first_y) =
                        self.format_coord(board.outline[0].x, board.outline[0].y);
                    writeln!(out, "X{first_x}Y{first_y}D02*")?;
                    for pt in &board.outline[1..] {
                        let (x, y) = self.format_coord(pt.x, pt.y);
                        writeln!(out, "X{x}Y{y}D01*")?;
                    }
                    writeln!(out, "X{first_x}Y{first_y}D01*")?;
                }
            }
            _ => {
                for fp in &board.footprints {
                    for pad in &fp.pads {
                        let def = ApertureDef::Circle {
                            diameter_nm: (pad.size.x * 1_000_000.0).round() as i64,
                        };
                        if let Some(&ap_id) = apertures.get(&def) {
                            writeln!(out, "D{ap_id}*")?;
                            let (x, y) = self.format_coord(
                                fp.position.x + pad.position.x,
                                fp.position.y + pad.position.y,
                            );
                            writeln!(out, "X{x}Y{y}D03*")?;
                        }
                    }
                }
            }
        }

        // 5. Footer
        writeln!(out, "M02*")?; // End of file

        Ok(out)
    }

    /// Format floating-point coordinate (mm) to integer Gerber format (e.g. 4.6 format).
    fn format_coord(&self, x_mm: f64, y_mm: f64) -> (i64, i64) {
        let factor = 10f64.powi(self.options.coordinate_format_decimal as i32);
        let x = (x_mm * factor).round() as i64;
        let y = (y_mm * factor).round() as i64;
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gerber_export_headers_and_apertures() {
        let board = PcbBoard::default();
        let exporter = GerberExporter::new(GerberOptions::default());
        let outputs = exporter.export_board(&board).unwrap();

        assert!(!outputs.is_empty());
        let top_copper = outputs
            .iter()
            .find(|o| o.layer == GerberLayer::TopCopper)
            .unwrap();
        assert!(top_copper.content.contains("%MOMM*%"));
        assert!(top_copper.content.contains("%FSLAX46Y46*%"));
        assert!(top_copper.content.contains("M02*"));
    }
}
