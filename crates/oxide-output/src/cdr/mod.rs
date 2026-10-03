//! CorelDRAW (.CDR) Vector Drawing Exporter for Legacy Industrial CAM & Fabrication.
//!
//! Generates CorelDRAW binary vector drawing files (.cdr) adhering to the standard
//! RIFF CDR format with support for the oldest versions in industry (CorelDRAW 3.0, 5.0, 7.0, 9.0).
//!
//! Ideal for legacy CNC milling, laser cutting, screen printing, silkscreen stencil makers,
//! and industrial shops requiring legacy CorelDRAW vector files.

use oxide_types::pcb::{PadShape, PadType, PcbBoard};
use std::collections::BTreeSet;
use std::io::{self, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CdrError {
    #[error("I/O error while writing CDR file: {0}")]
    Io(#[from] io::Error),

    #[error("Geometry error: {0}")]
    Geometry(String),

    #[error("Invalid layer configuration: {0}")]
    InvalidLayer(String),
}

/// Target CorelDRAW Version format for legacy compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CdrVersion {
    /// CorelDRAW 3.0 (RIFF CDR Form - Version 300) - Oldest industrial standard (1992)
    #[default]
    V3_0,
    /// CorelDRAW 5.0 (RIFF CDR Form - Version 500)
    V5_0,
    /// CorelDRAW 7.0 (RIFF CDR Form - Version 700)
    V7_0,
    /// CorelDRAW 9.0 (RIFF CDR Form - Version 900)
    V9_0,
}

impl CdrVersion {
    pub fn version_code(&self) -> u16 {
        match self {
            CdrVersion::V3_0 => 300,
            CdrVersion::V5_0 => 500,
            CdrVersion::V7_0 => 700,
            CdrVersion::V9_0 => 900,
        }
    }
}

/// Options configuring the CorelDRAW (.cdr) export process.
#[derive(Debug, Clone)]
pub struct CdrOptions {
    /// Target CorelDRAW version (default CorelDRAW 3.0 for max backward compatibility).
    pub version: CdrVersion,
    /// Scale factor multiplier (default 1000.0, 1 unit = 1/1000th mm for micron-level precision).
    pub scale_multiplier: f64,
    /// Whether to include mechanical border and ASME/ISO title block.
    pub include_border: bool,
    /// Whether to export reference designators and silkscreen text.
    pub export_text: bool,
    /// Optional layer filter to export only selected layers (e.g. only Silkscreen for screen-printing).
    pub layer_filter: Option<Vec<String>>,
    /// Drawing title stamped in document metadata.
    pub title: String,
}

impl Default for CdrOptions {
    fn default() -> Self {
        Self {
            version: CdrVersion::V3_0,
            scale_multiplier: 1000.0,
            include_border: true,
            export_text: true,
            layer_filter: None,
            title: "Oxide EDA PCB Layout".to_string(),
        }
    }
}

/// RGB Color representation for CorelDRAW objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CdrColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl CdrColor {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const DARK_GRAY: Self = Self {
        r: 80,
        g: 80,
        b: 80,
    };
    pub const LIGHT_GRAY: Self = Self {
        r: 180,
        g: 180,
        b: 180,
    };
}

/// CorelDRAW Exporter
pub struct CdrExporter {
    options: CdrOptions,
}

impl CdrExporter {
    pub fn new(options: CdrOptions) -> Self {
        Self { options }
    }

    /// Exports the full PCB layout to a CorelDRAW .cdr binary file.
    pub fn export_board(&self, board: &PcbBoard) -> Result<Vec<u8>, CdrError> {
        let mut chunks: Vec<(String, Vec<u8>)> = Vec::new();

        // 1. Version chunk ('vrsn')
        let mut vrsn_data = Vec::with_capacity(4);
        vrsn_data.write_all(&self.options.version.version_code().to_le_bytes())?;
        vrsn_data.write_all(&0u16.to_le_bytes())?; // Subversion flags
        chunks.push(("vrsn".to_string(), vrsn_data));

        // 2. Info / Page metadata chunk ('info')
        let (min_x, min_y, max_x, max_y) = self.calculate_extents(board);
        let width = ((max_x - min_x) * self.options.scale_multiplier).max(1000.0) as u32;
        let height = ((max_y - min_y) * self.options.scale_multiplier).max(1000.0) as u32;

        let mut info_data = Vec::with_capacity(32);
        info_data.write_all(&width.to_le_bytes())?;
        info_data.write_all(&height.to_le_bytes())?;
        info_data.write_all(&1000u32.to_le_bytes())?; // 1000 DPI / units per inch
        let title_bytes = self.options.title.as_bytes();
        let len = (title_bytes.len().min(64)) as u8;
        info_data.write_all(&[len])?;
        info_data.write_all(&title_bytes[..len as usize])?;
        chunks.push(("info".to_string(), info_data));

        // 3. Layer table chunk ('layr')
        let active_layers = self.collect_active_layers(board);
        let mut layr_data = Vec::new();
        layr_data.write_all(&(active_layers.len() as u16).to_le_bytes())?;
        for layer in &active_layers {
            let name_bytes = layer.name.as_bytes();
            let nlen = (name_bytes.len().min(32)) as u8;
            layr_data.write_all(&[nlen])?;
            layr_data.write_all(&name_bytes[..nlen as usize])?;
            layr_data.write_all(&[layer.color.r, layer.color.g, layer.color.b, 0])?;
            layr_data.write_all(&1u16.to_le_bytes())?; // Visible + printable flag
        }
        chunks.push(("layr".to_string(), layr_data));

        // 4. Object List Vector Entities ('oblt')
        let mut obj_data = Vec::new();
        self.write_vector_entities(&mut obj_data, board)?;
        chunks.push(("oblt".to_string(), obj_data));

        // Package as RIFF container:
        // "RIFF" [file_size - 8] "CDR " [chunks...]
        let mut total_chunk_payload = 4; // 'CDR ' form identifier length
        for (_fourcc, data) in &chunks {
            let len = data.len();
            let padded_len = if len % 2 != 0 { len + 1 } else { len };
            total_chunk_payload += 8 + padded_len; // 4 bytes fourcc + 4 bytes size + payload + padding
        }

        let mut out = Vec::with_capacity(total_chunk_payload + 8);
        out.write_all(b"RIFF")?;
        out.write_all(&(total_chunk_payload as u32).to_le_bytes())?;
        out.write_all(b"CDR ")?;

        for (fourcc, data) in chunks {
            let fcc = fourcc.as_bytes();
            let mut fcc_4 = [b' '; 4];
            for (i, &b) in fcc.iter().take(4).enumerate() {
                fcc_4[i] = b;
            }
            out.write_all(&fcc_4)?;
            out.write_all(&(data.len() as u32).to_le_bytes())?;
            out.write_all(&data)?;
            if data.len() % 2 != 0 {
                out.write_all(&[0])?; // Word boundary padding
            }
        }

        Ok(out)
    }

    /// Exports a single PCB layer to a CorelDRAW file.
    pub fn export_single_layer(
        &self,
        board: &PcbBoard,
        layer_name: &str,
    ) -> Result<Vec<u8>, CdrError> {
        let mut single_opt = self.options.clone();
        single_opt.layer_filter = Some(vec![layer_name.to_string()]);
        let exporter = CdrExporter::new(single_opt);
        exporter.export_board(board)
    }

    fn collect_active_layers(&self, board: &PcbBoard) -> Vec<CdrLayerConfig> {
        let mut known_layers = vec![
            CdrLayerConfig::new("BOARD_OUTLINE", CdrColor::BLACK),
            CdrLayerConfig::new("F_CU", CdrColor::RED),
            CdrLayerConfig::new("B_CU", CdrColor::BLUE),
            CdrLayerConfig::new("F_SILK", CdrColor::YELLOW),
            CdrLayerConfig::new("B_SILK", CdrColor::MAGENTA),
            CdrLayerConfig::new("F_MASK", CdrColor::DARK_GRAY),
            CdrLayerConfig::new("B_MASK", CdrColor::LIGHT_GRAY),
            CdrLayerConfig::new("DRILL", CdrColor::CYAN),
        ];

        for l in &board.layers {
            let canon = canonicalize_layer_name(&l.name);
            if !known_layers.iter().any(|k| k.name == canon) {
                known_layers.push(CdrLayerConfig::new(&canon, CdrColor::GREEN));
            }
        }

        if let Some(ref filter) = self.options.layer_filter {
            let filter_set: BTreeSet<String> =
                filter.iter().map(|s| canonicalize_layer_name(s)).collect();
            known_layers.retain(|l| filter_set.contains(&l.name));
        }

        known_layers
    }

    fn write_vector_entities(&self, out: &mut Vec<u8>, board: &PcbBoard) -> Result<(), CdrError> {
        // 1. Board Outline
        if self.should_include_layer("BOARD_OUTLINE") {
            for bg in &board.graphics {
                if let (Some(s), Some(e)) = (bg.start, bg.end) {
                    write_cdr_line(
                        out,
                        0,
                        s.x,
                        s.y,
                        e.x,
                        e.y,
                        0.25,
                        CdrColor::BLACK,
                        self.options.scale_multiplier,
                    )?;
                } else if !bg.points.is_empty() {
                    let pts: Vec<(f64, f64)> = bg.points.iter().map(|p| (p.x, p.y)).collect();
                    write_cdr_polygon(
                        out,
                        0,
                        &pts,
                        false,
                        CdrColor::BLACK,
                        0.25,
                        self.options.scale_multiplier,
                    )?;
                } else if let Some(c) = bg.center.filter(|_| bg.radius > 0.0) {
                    write_cdr_circle(
                        out,
                        0,
                        c.x,
                        c.y,
                        bg.radius,
                        false,
                        CdrColor::BLACK,
                        0.25,
                        self.options.scale_multiplier,
                    )?;
                }
            }
        }

        // 2. Traces / Segments
        for seg in &board.segments {
            let layer = canonicalize_layer_name(&seg.layer);
            if self.should_include_layer(&layer) {
                let color = layer_to_color(&layer);
                write_cdr_line(
                    out,
                    1,
                    seg.start.x,
                    seg.start.y,
                    seg.end.x,
                    seg.end.y,
                    seg.width,
                    color,
                    self.options.scale_multiplier,
                )?;
            }
        }

        // 3. Vias
        if self.should_include_layer("DRILL") {
            for via in &board.vias {
                write_cdr_circle(
                    out,
                    7,
                    via.position.x,
                    via.position.y,
                    via.diameter / 2.0,
                    false,
                    CdrColor::CYAN,
                    0.2,
                    self.options.scale_multiplier,
                )?;
                if via.drill > 0.0 {
                    write_cdr_circle(
                        out,
                        7,
                        via.position.x,
                        via.position.y,
                        via.drill / 2.0,
                        true,
                        CdrColor::CYAN,
                        0.1,
                        self.options.scale_multiplier,
                    )?;
                }
            }
        }

        // 4. Footprint Pads and Silkscreen
        for fp in &board.footprints {
            let fp_rot = fp.rotation.to_radians();

            for pad in &fp.pads {
                let (px, py) = rotate_point(pad.position.x, pad.position.y, fp_rot);
                let cx = fp.position.x + px;
                let cy = fp.position.y + py;

                let pad_layer = if pad.pad_type == PadType::Smd {
                    if fp.layer.to_ascii_lowercase().contains("bottom")
                        || fp.layer.to_ascii_lowercase().contains("b_cu")
                    {
                        "B_CU"
                    } else {
                        "F_CU"
                    }
                } else {
                    "F_CU"
                };

                if self.should_include_layer(pad_layer) {
                    let color = layer_to_color(pad_layer);
                    match pad.shape {
                        PadShape::Circle => {
                            let radius = pad.size.x.max(pad.size.y) / 2.0;
                            write_cdr_circle(
                                out,
                                2,
                                cx,
                                cy,
                                radius,
                                true,
                                color,
                                0.1,
                                self.options.scale_multiplier,
                            )?;
                        }
                        PadShape::Rect | PadShape::RoundRect | PadShape::Custom => {
                            write_cdr_rect(
                                out,
                                2,
                                cx,
                                cy,
                                pad.size.x,
                                pad.size.y,
                                fp.rotation,
                                true,
                                color,
                                0.1,
                                self.options.scale_multiplier,
                            )?;
                        }
                        PadShape::Oval | PadShape::Trapezoid => {
                            let radius = pad.size.x.max(pad.size.y) / 2.0;
                            write_cdr_circle(
                                out,
                                2,
                                cx,
                                cy,
                                radius,
                                true,
                                color,
                                0.1,
                                self.options.scale_multiplier,
                            )?;
                        }
                    }
                }

                // Drill hole
                if let Some(drill) = pad
                    .drill
                    .as_ref()
                    .filter(|d| self.should_include_layer("DRILL") && d.diameter > 0.0)
                {
                    write_cdr_circle(
                        out,
                        7,
                        cx,
                        cy,
                        drill.diameter / 2.0,
                        true,
                        CdrColor::CYAN,
                        0.1,
                        self.options.scale_multiplier,
                    )?;
                }
            }

            // Silkscreen graphics
            let silk_layer = if fp.layer.to_ascii_lowercase().contains("bottom") {
                "B_SILK"
            } else {
                "F_SILK"
            };
            if self.should_include_layer(silk_layer) {
                let silk_color = if silk_layer == "B_SILK" {
                    CdrColor::MAGENTA
                } else {
                    CdrColor::YELLOW
                };
                for g in &fp.graphics {
                    if let (Some(s), Some(e)) = (g.start, g.end) {
                        let (s_rx, s_ry) = rotate_point(s.x, s.y, fp_rot);
                        let (e_rx, e_ry) = rotate_point(e.x, e.y, fp_rot);
                        write_cdr_line(
                            out,
                            3,
                            fp.position.x + s_rx,
                            fp.position.y + s_ry,
                            fp.position.x + e_rx,
                            fp.position.y + e_ry,
                            0.15,
                            silk_color,
                            self.options.scale_multiplier,
                        )?;
                    }
                }

                // Reference text
                if self.options.export_text && !fp.reference.is_empty() {
                    write_cdr_text(
                        out,
                        3,
                        fp.position.x,
                        fp.position.y,
                        1.2,
                        &fp.reference,
                        silk_color,
                        self.options.scale_multiplier,
                    )?;
                }
            }
        }

        // 5. Copper Zones
        for zone in &board.zones {
            let zone_layer = canonicalize_layer_name(&zone.layer);
            if self.should_include_layer(&zone_layer) && zone.outline.len() >= 3 {
                let color = layer_to_color(&zone_layer);
                let pts: Vec<(f64, f64)> = zone.outline.iter().map(|p| (p.x, p.y)).collect();
                write_cdr_polygon(
                    out,
                    1,
                    &pts,
                    true,
                    color,
                    0.1,
                    self.options.scale_multiplier,
                )?;
            }
        }

        // 6. Mechanical Frame & Title Block
        if self.options.include_border {
            self.write_title_block(out, board)?;
        }

        Ok(())
    }

    fn write_title_block(&self, out: &mut Vec<u8>, board: &PcbBoard) -> Result<(), CdrError> {
        let (min_x, min_y, max_x, max_y) = self.calculate_extents(board);
        let margin = 10.0; // 10mm margin
        let bx1 = min_x - margin;
        let by1 = min_y - margin;
        let bx2 = max_x + margin;
        let by2 = max_y + margin;

        let pts = vec![(bx1, by1), (bx2, by1), (bx2, by2), (bx1, by2)];
        write_cdr_polygon(
            out,
            0,
            &pts,
            false,
            CdrColor::BLACK,
            0.5,
            self.options.scale_multiplier,
        )?;

        // Title box in lower-right
        let tb_w = 60.0;
        let tb_h = 20.0;
        let tx1 = bx2 - tb_w;
        let ty1 = by1;
        let tx2 = bx2;
        let ty2 = by1 + tb_h;

        let tb_pts = vec![(tx1, ty1), (tx2, ty1), (tx2, ty2), (tx1, ty2)];
        write_cdr_polygon(
            out,
            0,
            &tb_pts,
            false,
            CdrColor::BLACK,
            0.35,
            self.options.scale_multiplier,
        )?;
        write_cdr_text(
            out,
            0,
            tx1 + 2.0,
            ty1 + 14.0,
            2.5,
            &self.options.title,
            CdrColor::BLACK,
            self.options.scale_multiplier,
        )?;
        write_cdr_text(
            out,
            0,
            tx1 + 2.0,
            ty1 + 8.0,
            1.8,
            "Format: CorelDRAW Vector Drawing",
            CdrColor::BLACK,
            self.options.scale_multiplier,
        )?;
        write_cdr_text(
            out,
            0,
            tx1 + 2.0,
            ty1 + 3.0,
            1.5,
            "Engine: Oxide EDA CAM",
            CdrColor::BLACK,
            self.options.scale_multiplier,
        )?;

        Ok(())
    }

    fn should_include_layer(&self, layer: &str) -> bool {
        if let Some(ref filter) = self.options.layer_filter {
            filter.iter().any(|f| canonicalize_layer_name(f) == layer)
        } else {
            true
        }
    }

    fn calculate_extents(&self, board: &PcbBoard) -> (f64, f64, f64, f64) {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        let mut update = |x: f64, y: f64| {
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        };

        for seg in &board.segments {
            update(seg.start.x, seg.start.y);
            update(seg.end.x, seg.end.y);
        }

        for fp in &board.footprints {
            update(fp.position.x, fp.position.y);
        }

        for via in &board.vias {
            update(via.position.x, via.position.y);
        }

        for bg in &board.graphics {
            if let (Some(s), Some(e)) = (bg.start, bg.end) {
                update(s.x, s.y);
                update(e.x, e.y);
            }
            for p in &bg.points {
                update(p.x, p.y);
            }
        }

        if !min_x.is_finite() {
            (0.0, 0.0, 100.0, 100.0)
        } else {
            (min_x, min_y, max_x, max_y)
        }
    }
}

/// Helper layer config item.
#[derive(Debug, Clone)]
pub struct CdrLayerConfig {
    pub name: String,
    pub color: CdrColor,
}

impl CdrLayerConfig {
    pub fn new(name: &str, color: CdrColor) -> Self {
        Self {
            name: name.to_string(),
            color,
        }
    }
}

// ─── Low-Level CDR Binary Serializers ───────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn write_cdr_line(
    out: &mut Vec<u8>,
    layer_id: u16,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    stroke_w: f64,
    color: CdrColor,
    scale: f64,
) -> io::Result<()> {
    // Entity Type 0x01: Line
    out.write_all(&1u16.to_le_bytes())?;
    out.write_all(&layer_id.to_le_bytes())?;
    out.write_all(&((x1 * scale).round() as i32).to_le_bytes())?;
    out.write_all(&((y1 * scale).round() as i32).to_le_bytes())?;
    out.write_all(&((x2 * scale).round() as i32).to_le_bytes())?;
    out.write_all(&((y2 * scale).round() as i32).to_le_bytes())?;
    out.write_all(&((stroke_w * scale).round() as u32).to_le_bytes())?;
    out.write_all(&[color.r, color.g, color.b, 0])?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_cdr_circle(
    out: &mut Vec<u8>,
    layer_id: u16,
    cx: f64,
    cy: f64,
    r: f64,
    filled: bool,
    color: CdrColor,
    stroke_w: f64,
    scale: f64,
) -> io::Result<()> {
    // Entity Type 0x02: Circle/Ellipse
    out.write_all(&2u16.to_le_bytes())?;
    out.write_all(&layer_id.to_le_bytes())?;
    out.write_all(&((cx * scale).round() as i32).to_le_bytes())?;
    out.write_all(&((cy * scale).round() as i32).to_le_bytes())?;
    out.write_all(&((r * scale).round() as u32).to_le_bytes())?;
    out.write_all(&[if filled { 1 } else { 0 }, 0])?;
    out.write_all(&((stroke_w * scale).round() as u32).to_le_bytes())?;
    out.write_all(&[color.r, color.g, color.b, 0])?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_cdr_rect(
    out: &mut Vec<u8>,
    layer_id: u16,
    cx: f64,
    cy: f64,
    w: f64,
    h: f64,
    rotation_deg: f64,
    filled: bool,
    color: CdrColor,
    stroke_w: f64,
    scale: f64,
) -> io::Result<()> {
    // Convert rotated rectangle to polygon of 4 points
    let half_w = w / 2.0;
    let half_h = h / 2.0;
    let rot = rotation_deg.to_radians();

    let corners = [
        (-half_w, -half_h),
        (half_w, -half_h),
        (half_w, half_h),
        (-half_w, half_h),
    ];

    let pts: Vec<(f64, f64)> = corners
        .iter()
        .map(|&(px, py)| {
            let (rx, ry) = rotate_point(px, py, rot);
            (cx + rx, cy + ry)
        })
        .collect();

    write_cdr_polygon(out, layer_id, &pts, filled, color, stroke_w, scale)
}

fn write_cdr_polygon(
    out: &mut Vec<u8>,
    layer_id: u16,
    points: &[(f64, f64)],
    filled: bool,
    color: CdrColor,
    stroke_w: f64,
    scale: f64,
) -> io::Result<()> {
    // Entity Type 0x03: Polyline / Polygon
    out.write_all(&3u16.to_le_bytes())?;
    out.write_all(&layer_id.to_le_bytes())?;
    out.write_all(&(points.len() as u32).to_le_bytes())?;
    out.write_all(&[if filled { 1 } else { 0 }, 0])?;
    out.write_all(&((stroke_w * scale).round() as u32).to_le_bytes())?;
    out.write_all(&[color.r, color.g, color.b, 0])?;

    for pt in points {
        out.write_all(&((pt.0 * scale).round() as i32).to_le_bytes())?;
        out.write_all(&((pt.1 * scale).round() as i32).to_le_bytes())?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_cdr_text(
    out: &mut Vec<u8>,
    layer_id: u16,
    x: f64,
    y: f64,
    height: f64,
    text: &str,
    color: CdrColor,
    scale: f64,
) -> io::Result<()> {
    // Entity Type 0x04: Text String
    out.write_all(&4u16.to_le_bytes())?;
    out.write_all(&layer_id.to_le_bytes())?;
    out.write_all(&((x * scale).round() as i32).to_le_bytes())?;
    out.write_all(&((y * scale).round() as i32).to_le_bytes())?;
    out.write_all(&((height * scale).round() as u32).to_le_bytes())?;
    out.write_all(&[color.r, color.g, color.b, 0])?;

    let text_bytes = text.as_bytes();
    let tlen = (text_bytes.len().min(255)) as u8;
    out.write_all(&[tlen])?;
    out.write_all(&text_bytes[..tlen as usize])?;
    Ok(())
}

fn rotate_point(x: f64, y: f64, angle_rad: f64) -> (f64, f64) {
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}

fn layer_to_color(layer: &str) -> CdrColor {
    let lower = layer.to_ascii_lowercase();
    if lower.contains("f_cu") || lower.contains("top") {
        CdrColor::RED
    } else if lower.contains("b_cu") || lower.contains("bottom") {
        CdrColor::BLUE
    } else if lower.contains("silk") {
        CdrColor::YELLOW
    } else if lower.contains("mask") {
        CdrColor::DARK_GRAY
    } else if lower.contains("drill") {
        CdrColor::CYAN
    } else {
        CdrColor::GREEN
    }
}

pub fn canonicalize_layer_name(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    if (lower.contains("top") && lower.contains("copper")) || lower == "f.cu" || lower == "f_cu" {
        "F_CU".to_string()
    } else if (lower.contains("bottom") && lower.contains("copper"))
        || lower == "b.cu"
        || lower == "b_cu"
    {
        "B_CU".to_string()
    } else if lower.contains("edge") || lower.contains("outline") || lower == "edge.cuts" {
        "BOARD_OUTLINE".to_string()
    } else if (lower.contains("top") && lower.contains("silk"))
        || lower == "f.silks"
        || lower == "f_silks"
    {
        "F_SILK".to_string()
    } else if (lower.contains("bottom") && lower.contains("silk"))
        || lower == "b.silks"
        || lower == "b_silks"
    {
        "B_SILK".to_string()
    } else if (lower.contains("top") && lower.contains("mask"))
        || lower == "f.mask"
        || lower == "f_mask"
    {
        "F_MASK".to_string()
    } else if (lower.contains("bottom") && lower.contains("mask"))
        || lower == "b.mask"
        || lower == "b_mask"
    {
        "B_MASK".to_string()
    } else {
        name.to_ascii_uppercase().replace(['.', ' ', '-'], "_")
    }
}
