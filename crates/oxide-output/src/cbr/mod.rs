//! Copper Bottom Routing (`.cbr`) Legacy CAM Exporter for Oxide EDA.
//!
//! Generates classic industrial `.cbr` (Copper Bottom Routing) files strictly compatible with
//! legacy photoplotters, CAM350, GC-Prevue, Protel, P-CAD, and industrial board fabrication equipment.

use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;

use oxide_types::pcb::{PadShape, PadType, PcbBoard};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CbrError {
    #[error("Formatting error: {0}")]
    Format(#[from] std::fmt::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Empty bottom copper layer")]
    EmptyLayer,
}

/// Options for `.cbr` (Copper Bottom Routing) generation.
#[derive(Debug, Clone)]
pub struct CbrOptions {
    /// Coordinate format (e.g. 2.4 format with 4 decimal digits).
    pub scale_multiplier: f64,
    /// Include copper zones on bottom layer.
    pub include_zones: bool,
    /// Include bottom-layer silkscreen fiducials or alignment targets.
    pub include_vias: bool,
    /// Board project title.
    pub title: String,
}

impl Default for CbrOptions {
    fn default() -> Self {
        Self {
            scale_multiplier: 10_000.0, // 2.4 fixed-point format (1 mm = 10,000 units)
            include_zones: true,
            include_vias: true,
            title: "Oxide Bottom Copper".to_string(),
        }
    }
}

/// Dedicated `.cbr` (Copper Bottom Routing) Exporter.
pub struct CbrExporter {
    options: CbrOptions,
}

impl CbrExporter {
    pub fn new(options: CbrOptions) -> Self {
        Self { options }
    }

    /// Export bottom copper layer into `.cbr` file format string.
    pub fn export_bottom_copper(&self, board: &PcbBoard) -> Result<String, CbrError> {
        let mut bytes = Vec::with_capacity(32 * 1024);
        self.export_bottom_copper_to_writer(board, &mut bytes)?;
        String::from_utf8(bytes)
            .map_err(|e| CbrError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
    }

    /// Stream bottom copper layer directly into any `std::io::Write` target.
    pub fn export_bottom_copper_to_writer<W: std::io::Write>(
        &self,
        board: &PcbBoard,
        writer: &mut W,
    ) -> Result<(), CbrError> {
        let mut out = String::with_capacity(32 * 1024);

        // 1. Collect Apertures for bottom layer traces, pads, and vias
        let mut apertures = BTreeMap::new();
        let mut next_dcode = 10u32;

        // Trace width apertures
        let mut trace_apertures = BTreeMap::new();
        for seg in &board.segments {
            if is_bottom_copper_layer(&seg.layer) {
                let w_nm = (seg.width * 1000.0).round() as i64;
                if let std::collections::btree_map::Entry::Vacant(e) = trace_apertures.entry(w_nm) {
                    e.insert(next_dcode);
                    apertures.insert(next_dcode, ApertureDef::Circle(seg.width));
                    next_dcode += 1;
                }
            }
        }

        // Pad apertures
        let mut pad_apertures = BTreeMap::new();
        for fp in &board.footprints {
            let is_bottom_fp = fp.layer.to_ascii_lowercase().contains("bottom")
                || fp.layer.to_ascii_lowercase().contains("b_cu");
            for pad in &fp.pads {
                let on_bottom = pad.pad_type == PadType::Thru
                    || (pad.pad_type == PadType::Smd && is_bottom_fp)
                    || pad.layers.iter().any(|l| is_bottom_copper_layer(l));

                if on_bottom {
                    let key = pad_aperture_key(pad);
                    if let std::collections::btree_map::Entry::Vacant(e) = pad_apertures.entry(key)
                    {
                        e.insert(next_dcode);
                        let ap = match pad.shape {
                            PadShape::Circle => ApertureDef::Circle(pad.size.x.max(pad.size.y)),
                            PadShape::Rect | PadShape::RoundRect | PadShape::Custom => {
                                ApertureDef::Rect(pad.size.x, pad.size.y)
                            }
                            PadShape::Oval | PadShape::Trapezoid => {
                                ApertureDef::Oval(pad.size.x, pad.size.y)
                            }
                        };
                        apertures.insert(next_dcode, ap);
                        next_dcode += 1;
                    }
                }
            }
        }

        // Via apertures
        let mut via_aperture = None;
        if self.options.include_vias && !board.vias.is_empty() {
            let via_diam = board.vias[0].diameter;
            via_aperture = Some(next_dcode);
            apertures.insert(next_dcode, ApertureDef::Circle(via_diam));
        }

        // 2. Write Standard Photoplotter Header
        writeln!(
            out,
            "G04 ===================================================================*"
        )?;
        writeln!(
            out,
            "G04 File: CBR (Copper Bottom Routing) - Oxide EDA CAM Exporter*"
        )?;
        writeln!(out, "G04 Title: {}*", self.options.title)?;
        writeln!(out, "G04 Format: RS-274X Compatible Bottom Copper Layer*")?;
        writeln!(
            out,
            "G04 ===================================================================*"
        )?;
        writeln!(out, "%FSLAX24Y24*%")?;
        writeln!(out, "%MOMM*%")?;
        writeln!(out, "%LPD*%")?;

        // 3. Declare Aperture Definitions (%ADD...%)
        for (&dcode, ap) in &apertures {
            match ap {
                ApertureDef::Circle(d) => {
                    writeln!(out, "%ADD{dcode}C,{d:.4}*%")?;
                }
                ApertureDef::Rect(w, h) => {
                    writeln!(out, "%ADD{dcode}R,{w:.4}X{h:.4}*%")?;
                }
                ApertureDef::Oval(w, h) => {
                    writeln!(out, "%ADD{dcode}O,{w:.4}X{h:.4}*%")?;
                }
            }
        }

        // 4. Emit Trace Segments (D02 Move + D01 Draw)
        let mut current_dcode = 0u32;

        for seg in &board.segments {
            if is_bottom_copper_layer(&seg.layer) {
                let w_nm = (seg.width * 1000.0).round() as i64;
                if let Some(&dcode) = trace_apertures.get(&w_nm) {
                    if current_dcode != dcode {
                        writeln!(out, "D{dcode}*")?;
                        current_dcode = dcode;
                    }

                    let x1 = self.format_coord(seg.start.x);
                    let y1 = self.format_coord(seg.start.y);
                    let x2 = self.format_coord(seg.end.x);
                    let y2 = self.format_coord(seg.end.y);

                    writeln!(out, "X{x1}Y{y1}D02*")?; // Move to start (light off)
                    writeln!(out, "X{x2}Y{y2}D01*")?; // Draw to end (light on)
                }
            }
        }

        // 5. Emit Pads (D03 Flash)
        for fp in &board.footprints {
            let is_bottom_fp = fp.layer.to_ascii_lowercase().contains("bottom")
                || fp.layer.to_ascii_lowercase().contains("b_cu");
            let fp_rot = fp.rotation.to_radians();

            for pad in &fp.pads {
                let on_bottom = pad.pad_type == PadType::Thru
                    || (pad.pad_type == PadType::Smd && is_bottom_fp)
                    || pad.layers.iter().any(|l| is_bottom_copper_layer(l));

                if on_bottom {
                    let key = pad_aperture_key(pad);
                    if let Some(&dcode) = pad_apertures.get(&key) {
                        if current_dcode != dcode {
                            writeln!(out, "D{dcode}*")?;
                            current_dcode = dcode;
                        }

                        let (px, py) = rotate_point(pad.position.x, pad.position.y, fp_rot);
                        let cx = fp.position.x + px;
                        let cy = fp.position.y + py;

                        let x = self.format_coord(cx);
                        let y = self.format_coord(cy);
                        writeln!(out, "X{x}Y{y}D03*")?; // Flash pad
                    }
                }
            }
        }

        // 6. Emit Vias (D03 Flash)
        if let (true, Some(via_dc)) = (self.options.include_vias, via_aperture) {
            if current_dcode != via_dc {
                writeln!(out, "D{via_dc}*")?;
            }

            for via in &board.vias {
                let x = self.format_coord(via.position.x);
                let y = self.format_coord(via.position.y);
                writeln!(out, "X{x}Y{y}D03*")?; // Flash via
            }
        }

        // 7. Emit Bottom Copper Zones (G36 / G37 polygon fills)
        if self.options.include_zones {
            for zone in &board.zones {
                if is_bottom_copper_layer(&zone.layer) && zone.outline.len() >= 3 {
                    writeln!(out, "G36*")?; // Start polygon fill
                    if let Some(first) = zone.outline.first() {
                        let fx = self.format_coord(first.x);
                        let fy = self.format_coord(first.y);
                        writeln!(out, "X{fx}Y{fy}D02*")?;

                        for pt in zone.outline.iter().skip(1) {
                            let px = self.format_coord(pt.x);
                            let py = self.format_coord(pt.y);
                            writeln!(out, "X{px}Y{py}D01*")?;
                        }

                        // Close polygon back to first vertex
                        writeln!(out, "X{fx}Y{fy}D01*")?;
                    }
                    writeln!(out, "G37*")?; // End polygon fill
                }
            }
        }

        // 8. End of File
        writeln!(out, "M02*")?;

        writer.write_all(out.as_bytes())?;
        Ok(())
    }

    fn format_coord(&self, val: f64) -> i64 {
        (val * self.options.scale_multiplier).round() as i64
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ApertureDef {
    Circle(f64),
    Rect(f64, f64),
    Oval(f64, f64),
}

fn pad_aperture_key(pad: &oxide_types::pcb::Pad) -> String {
    format!("{:?}_{:.4}_{:.4}", pad.shape, pad.size.x, pad.size.y)
}

fn is_bottom_copper_layer(layer: &str) -> bool {
    let lower = layer.to_ascii_lowercase();
    lower.contains("bottom") || lower == "b.cu" || lower == "b_cu" || lower == "layer 2"
}

fn rotate_point(x: f64, y: f64, angle_rad: f64) -> (f64, f64) {
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}
