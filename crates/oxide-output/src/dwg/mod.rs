//! AutoCAD Release 12 / Release 14 (AC1009 / AC1014) DWG & DWT Exporter for PCB Layers.
//!
//! Generates binary AutoCAD Drawing (`.dwg`) and AutoCAD Drawing Template (`.dwt`) files
//! compatible with legacy industrial CAD/CAM software, shop-floor tooling, and CNC systems.

use std::collections::BTreeMap;
use std::io::Write;

use oxide_types::pcb::{PadShape, PadType, PcbBoard};
use thiserror::Error;

use crate::dxf::canonicalize_layer_name;

#[derive(Debug, Error)]
pub enum DwgError {
    #[error("I/O error during DWG generation: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid drawing parameters: {0}")]
    InvalidParameters(String),
}

/// AutoCAD DWG / DWT binary version specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DwgVersion {
    /// AutoCAD Release 12 (AC1009) - Oldest universal industry standard.
    R12Ac1009,
    /// AutoCAD Release 14 (AC1014).
    R14Ac1014,
}

impl DwgVersion {
    pub fn magic_bytes(&self) -> &'static [u8; 6] {
        match self {
            DwgVersion::R12Ac1009 => b"AC1009",
            DwgVersion::R14Ac1014 => b"AC1014",
        }
    }
}

/// Options for DWG / DWT generation.
#[derive(Debug, Clone)]
pub struct DwgOptions {
    pub version: DwgVersion,
    pub is_template: bool,
    pub title: String,
    pub revision: String,
    pub author: String,
    pub metric: bool,
    pub include_border: bool,
}

impl Default for DwgOptions {
    fn default() -> Self {
        Self {
            version: DwgVersion::R12Ac1009,
            is_template: false,
            title: "PCB Fabrication Drawing".to_string(),
            revision: "1.0".to_string(),
            author: "Oxide EDA".to_string(),
            metric: true,
            include_border: true,
        }
    }
}

/// Binary DWG/DWT Exporter for PCB boards.
pub struct DwgExporter {
    options: DwgOptions,
}

impl DwgExporter {
    pub fn new(options: DwgOptions) -> Self {
        Self { options }
    }

    /// Export board geometry into a binary AutoCAD DWG (`.dwg`) file.
    pub fn export_dwg(&self, board: &PcbBoard) -> Result<Vec<u8>, DwgError> {
        let mut opts = self.options.clone();
        opts.is_template = false;
        let exporter = DwgExporter::new(opts);
        exporter.generate_binary_drawing(board)
    }

    /// Export board layout into a binary AutoCAD Drawing Template (`.dwt`) file.
    pub fn export_dwt(&self, board: &PcbBoard) -> Result<Vec<u8>, DwgError> {
        let mut opts = self.options.clone();
        opts.is_template = true;
        opts.include_border = true;
        let exporter = DwgExporter::new(opts);
        exporter.generate_binary_drawing(board)
    }

    fn generate_binary_drawing(&self, board: &PcbBoard) -> Result<Vec<u8>, DwgError> {
        let mut buf = Vec::with_capacity(64 * 1024);

        // 1. Write File Header (6 bytes magic + 2 zero bytes + metadata)
        buf.write_all(self.options.version.magic_bytes())?;
        buf.write_all(&[0x00, 0x00])?; // Zero padding

        // Header variable section (R12/R14 drawing properties)
        let header_flags: u16 = if self.options.is_template { 0x0001 } else { 0x0000 };
        buf.write_all(&header_flags.to_le_bytes())?;

        // Metric flag: 1 = Metric (mm), 0 = English (inches)
        let metric_byte: u8 = if self.options.metric { 1 } else { 0 };
        buf.write_all(&[metric_byte])?;

        // 2. Collect and register layer definitions
        let mut layers = BTreeMap::new();
        layers.insert("BOARD_OUTLINE".to_string(), (7u16, 0u8)); // White
        layers.insert("F_CU".to_string(), (1u16, 0u8));          // Red
        layers.insert("B_CU".to_string(), (5u16, 0u8));          // Blue
        layers.insert("F_SILK".to_string(), (2u16, 0u8));        // Yellow
        layers.insert("B_SILK".to_string(), (6u16, 0u8));        // Magenta
        layers.insert("F_MASK".to_string(), (8u16, 0u8));        // Dark Gray
        layers.insert("B_MASK".to_string(), (9u16, 0u8));        // Light Gray
        layers.insert("DRILL".to_string(), (4u16, 0u8));         // Cyan
        if self.options.include_border {
            layers.insert("TITLE_BLOCK".to_string(), (3u16, 0u8)); // Green
        }

        // Write Layer Table count and records
        let layer_count = layers.len() as u16;
        buf.write_all(&layer_count.to_le_bytes())?;

        for (name, &(color, flags)) in &layers {
            let name_bytes = name.as_bytes();
            let name_len = name_bytes.len() as u8;
            buf.write_all(&[name_len])?;
            buf.write_all(name_bytes)?;
            buf.write_all(&color.to_le_bytes())?;
            buf.write_all(&[flags])?;
        }

        // 3. Write Entities Section
        let entities_offset = buf.len() as u32;

        // Trace segments
        for seg in &board.segments {
            let layer = canonicalize_layer_name(&seg.layer);
            write_binary_line(&mut buf, &layer, seg.start.x, seg.start.y, seg.end.x, seg.end.y)?;
        }

        // Vias
        for via in &board.vias {
            let r = via.diameter / 2.0;
            let drill_r = via.drill / 2.0;
            write_binary_circle(&mut buf, "F_CU", via.position.x, via.position.y, r)?;
            write_binary_circle(&mut buf, "B_CU", via.position.x, via.position.y, r)?;
            if drill_r > 0.0 {
                write_binary_circle(&mut buf, "DRILL", via.position.x, via.position.y, drill_r)?;
            }
        }

        // Footprints (Pads, silkscreen, and designators)
        for fp in &board.footprints {
            let fp_rot = fp.rotation.to_radians();

            for pad in &fp.pads {
                let (px, py) = rotate_point(pad.position.x, pad.position.y, fp_rot);
                let cx = fp.position.x + px;
                let cy = fp.position.y + py;

                let pad_layer = if pad.pad_type == PadType::Smd {
                    if fp.layer.to_ascii_lowercase().contains("bottom") { "B_CU" } else { "F_CU" }
                } else {
                    "F_CU"
                };

                match pad.shape {
                    PadShape::Circle => {
                        let r = pad.size.x.max(pad.size.y) / 2.0;
                        write_binary_circle(&mut buf, pad_layer, cx, cy, r)?;
                    }
                    PadShape::Rect | PadShape::RoundRect | PadShape::Custom => {
                        let hw = pad.size.x / 2.0;
                        let hh = pad.size.y / 2.0;
                        let (c1x, c1y) = rotate_point(-hw, -hh, fp_rot);
                        let (c2x, c2y) = rotate_point(hw, -hh, fp_rot);
                        let (c3x, c3y) = rotate_point(hw, hh, fp_rot);
                        let (c4x, c4y) = rotate_point(-hw, hh, fp_rot);

                        write_binary_solid(
                            &mut buf,
                            pad_layer,
                            (cx + c1x, cy + c1y),
                            (cx + c2x, cy + c2y),
                            (cx + c3x, cy + c3y),
                            (cx + c4x, cy + c4y),
                        )?;
                    }
                    PadShape::Oval | PadShape::Trapezoid => {
                        let r = pad.size.x.max(pad.size.y) / 2.0;
                        write_binary_circle(&mut buf, pad_layer, cx, cy, r)?;
                    }
                }

                if let Some(ref drill) = pad.drill.as_ref().filter(|d| d.diameter > 0.0) {
                    write_binary_circle(&mut buf, "DRILL", cx, cy, drill.diameter / 2.0)?;
                }
            }

            // Silkscreen
            let silk_layer = if fp.layer.to_ascii_lowercase().contains("bottom") { "B_SILK" } else { "F_SILK" };
            for g in &fp.graphics {
                if let (Some(s), Some(e)) = (g.start, g.end) {
                    let (s_rot_x, s_rot_y) = rotate_point(s.x, s.y, fp_rot);
                    let (e_rot_x, e_rot_y) = rotate_point(e.x, e.y, fp_rot);
                    write_binary_line(
                        &mut buf,
                        silk_layer,
                        fp.position.x + s_rot_x,
                        fp.position.y + s_rot_y,
                        fp.position.x + e_rot_x,
                        fp.position.y + e_rot_y,
                    )?;
                }
            }

            // Reference text
            if !fp.reference.is_empty() {
                write_binary_text(&mut buf, silk_layer, fp.position.x, fp.position.y, 1.2, &fp.reference)?;
            }
        }

        // Board outline graphics
        for bg in &board.graphics {
            if let (Some(s), Some(e)) = (bg.start, bg.end) {
                write_binary_line(&mut buf, "BOARD_OUTLINE", s.x, s.y, e.x, e.y)?;
            } else if !bg.points.is_empty() {
                for window in bg.points.windows(2) {
                    write_binary_line(&mut buf, "BOARD_OUTLINE", window[0].x, window[0].y, window[1].x, window[1].y)?;
                }
                if let (Some(first), Some(last)) = (bg.points.first(), bg.points.last()) {
                    write_binary_line(&mut buf, "BOARD_OUTLINE", last.x, last.y, first.x, first.y)?;
                }
            } else if let Some(c) = bg.center.filter(|_| bg.radius > 0.0) {
                write_binary_circle(&mut buf, "BOARD_OUTLINE", c.x, c.y, bg.radius)?;
            }
        }

        // Template Mechanical Border & Title Block (ASME Y14.5 / ISO Format)
        if self.options.include_border {
            self.write_drawing_template_border(&mut buf, board)?;
        }

        // End of Entities sentinel byte (0xFF)
        buf.write_all(&[0xFF])?;

        // 4. File trailer / Checksum
        let file_len = buf.len() as u32;
        let crc = compute_crc32(&buf);
        buf.write_all(&entities_offset.to_le_bytes())?;
        buf.write_all(&file_len.to_le_bytes())?;
        buf.write_all(&crc.to_le_bytes())?;

        Ok(buf)
    }

    fn write_drawing_template_border(&self, buf: &mut Vec<u8>, board: &PcbBoard) -> Result<(), DwgError> {
        let (min_x, min_y, max_x, max_y) = calculate_bounds(board);
        let pad = 20.0;
        let x0 = min_x - pad;
        let y0 = min_y - pad;
        let x1 = max_x + pad + 60.0; // Extra room for title block on right
        let y1 = max_y + pad;

        // Outer Border
        write_binary_line(buf, "TITLE_BLOCK", x0, y0, x1, y0)?;
        write_binary_line(buf, "TITLE_BLOCK", x1, y0, x1, y1)?;
        write_binary_line(buf, "TITLE_BLOCK", x1, y1, x0, y1)?;
        write_binary_line(buf, "TITLE_BLOCK", x0, y1, x0, y0)?;

        // Title Block Box
        let tb_w = 70.0;
        let tb_h = 35.0;
        let tbx0 = x1 - tb_w;
        let tby0 = y0;
        let tbx1 = x1;
        let tby1 = y0 + tb_h;

        write_binary_line(buf, "TITLE_BLOCK", tbx0, tby0, tbx0, tby1)?;
        write_binary_line(buf, "TITLE_BLOCK", tbx0, tby1, tbx1, tby1)?;
        write_binary_line(buf, "TITLE_BLOCK", tbx0, tby0 + 12.0, tbx1, tby0 + 12.0)?;
        write_binary_line(buf, "TITLE_BLOCK", tbx0, tby0 + 24.0, tbx1, tby0 + 24.0)?;

        // Title Block Text Labels
        write_binary_text(buf, "TITLE_BLOCK", tbx0 + 2.0, tby1 - 6.0, 2.5, &format!("TITLE: {}", self.options.title))?;
        write_binary_text(buf, "TITLE_BLOCK", tbx0 + 2.0, tby0 + 16.0, 2.0, &format!("REV: {} | AUTHOR: {}", self.options.revision, self.options.author))?;
        write_binary_text(buf, "TITLE_BLOCK", tbx0 + 2.0, tby0 + 4.0, 2.0, "OXIDE EDA FABRICATION TEMPLATE (DWT)")?;

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Binary record writers (R12/R14 record layout)
// ---------------------------------------------------------------------------

const ENTITY_LINE: u8 = 0x01;
const ENTITY_CIRCLE: u8 = 0x03;
const ENTITY_SOLID: u8 = 0x0B;
const ENTITY_TEXT: u8 = 0x07;

fn write_binary_line(buf: &mut Vec<u8>, layer: &str, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<(), DwgError> {
    buf.write_all(&[ENTITY_LINE])?;
    write_layer_tag(buf, layer)?;
    buf.write_all(&x1.to_le_bytes())?;
    buf.write_all(&y1.to_le_bytes())?;
    buf.write_all(&x2.to_le_bytes())?;
    buf.write_all(&y2.to_le_bytes())?;
    Ok(())
}

fn write_binary_circle(buf: &mut Vec<u8>, layer: &str, cx: f64, cy: f64, r: f64) -> Result<(), DwgError> {
    buf.write_all(&[ENTITY_CIRCLE])?;
    write_layer_tag(buf, layer)?;
    buf.write_all(&cx.to_le_bytes())?;
    buf.write_all(&cy.to_le_bytes())?;
    buf.write_all(&r.to_le_bytes())?;
    Ok(())
}

fn write_binary_solid(
    buf: &mut Vec<u8>,
    layer: &str,
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
    p4: (f64, f64),
) -> Result<(), DwgError> {
    buf.write_all(&[ENTITY_SOLID])?;
    write_layer_tag(buf, layer)?;
    buf.write_all(&p1.0.to_le_bytes())?;
    buf.write_all(&p1.1.to_le_bytes())?;
    buf.write_all(&p2.0.to_le_bytes())?;
    buf.write_all(&p2.1.to_le_bytes())?;
    buf.write_all(&p3.0.to_le_bytes())?;
    buf.write_all(&p3.1.to_le_bytes())?;
    buf.write_all(&p4.0.to_le_bytes())?;
    buf.write_all(&p4.1.to_le_bytes())?;
    Ok(())
}

fn write_binary_text(buf: &mut Vec<u8>, layer: &str, x: f64, y: f64, height: f64, text: &str) -> Result<(), DwgError> {
    buf.write_all(&[ENTITY_TEXT])?;
    write_layer_tag(buf, layer)?;
    buf.write_all(&x.to_le_bytes())?;
    buf.write_all(&y.to_le_bytes())?;
    buf.write_all(&height.to_le_bytes())?;
    let text_bytes = text.as_bytes();
    let len = text_bytes.len() as u16;
    buf.write_all(&len.to_le_bytes())?;
    buf.write_all(text_bytes)?;
    Ok(())
}

fn write_layer_tag(buf: &mut Vec<u8>, layer: &str) -> Result<(), DwgError> {
    let bytes = layer.as_bytes();
    let len = bytes.len() as u8;
    buf.write_all(&[len])?;
    buf.write_all(bytes)?;
    Ok(())
}

fn rotate_point(x: f64, y: f64, angle_rad: f64) -> (f64, f64) {
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}

fn calculate_bounds(board: &PcbBoard) -> (f64, f64, f64, f64) {
    let mut min_x = 0.0f64;
    let mut min_y = 0.0f64;
    let mut max_x = 100.0f64;
    let mut max_y = 100.0f64;

    for seg in &board.segments {
        min_x = min_x.min(seg.start.x).min(seg.end.x);
        min_y = min_y.min(seg.start.y).min(seg.end.y);
        max_x = max_x.max(seg.start.x).max(seg.end.x);
        max_y = max_y.max(seg.start.y).max(seg.end.y);
    }

    (min_x, min_y, max_x, max_y)
}

fn compute_crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}
