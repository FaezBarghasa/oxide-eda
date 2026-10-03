//! AutoCAD Release 12 (AC1009) ASCII DXF Exporter for PCB Layers.
//!
//! Generates universally compatible AutoCAD R12 DXF files supported by legacy CAM tools,
//! CNC milling/routing, laser cutters, and classic CAD systems (AutoCAD R12/R14/2000,
//! CAM350, GC-Prevue, QCad, LibreCAD, FreeCAD).

use std::collections::BTreeSet;
use std::fmt::Write as FmtWrite;

use oxide_types::pcb::{PadShape, PadType, PcbBoard};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DxfError {
    #[error("Formatting error: {0}")]
    Format(#[from] std::fmt::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Empty board: no geometry found")]
    EmptyBoard,
}

/// AutoCAD Color Index (ACI) standard colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i16)]
pub enum AciColor {
    Red = 1,
    Yellow = 2,
    Green = 3,
    Cyan = 4,
    Blue = 5,
    Magenta = 6,
    WhiteOrBlack = 7,
    DarkGray = 8,
    LightGray = 9,
}

/// Standard PCB layer mapping for DXF export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxfLayerConfig {
    pub name: String,
    pub color: AciColor,
    pub description: String,
}

impl DxfLayerConfig {
    pub fn new(name: impl Into<String>, color: AciColor, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            color,
            description: description.into(),
        }
    }
}

/// Options for AutoCAD R12 DXF export.
#[derive(Debug, Clone)]
pub struct DxfOptions {
    /// Target AutoCAD version string in DXF header (default: "AC1009" for R12).
    pub acad_version: String,
    /// Export unit in millimeters (default: true).
    pub metric: bool,
    /// Export filled pads and zones as DXF SOLID / 3DFACE entities.
    pub export_solids: bool,
    /// Export silkscreen text elements.
    pub export_text: bool,
    /// Filter to specific layer names (if empty, all layers are exported).
    pub layer_filter: Option<Vec<String>>,
}

impl Default for DxfOptions {
    fn default() -> Self {
        Self {
            acad_version: "AC1009".to_string(),
            metric: true,
            export_solids: true,
            export_text: true,
            layer_filter: None,
        }
    }
}

/// DXF layer output container.
#[derive(Debug, Clone)]
pub struct DxfOutput {
    pub filename: String,
    pub content: String,
}

/// AutoCAD R12 ASCII DXF Exporter.
pub struct DxfExporter {
    options: DxfOptions,
}

impl DxfExporter {
    pub fn new(options: DxfOptions) -> Self {
        Self { options }
    }

    /// Export complete PCB board into a unified multi-layer AutoCAD R12 DXF file.
    pub fn export_board(&self, board: &PcbBoard) -> Result<String, DxfError> {
        let mut out = String::with_capacity(32 * 1024);

        // 1. Collect all active layer names from board
        let active_layers = self.collect_active_layers(board);

        // 2. Write DXF Header section
        self.write_header(&mut out, board)?;

        // 3. Write DXF Tables section (Linetypes and Layers)
        self.write_tables(&mut out, &active_layers)?;

        // 4. Write DXF Blocks section (minimal for R12)
        self.write_blocks(&mut out)?;

        // 5. Write DXF Entities section (PCB geometry)
        self.write_entities(&mut out, board)?;

        // 6. Write EOF
        writeln!(out, "  0\nEOF")?;

        Ok(out)
    }

    /// Export individual PCB layer into a standalone single-layer DXF file.
    pub fn export_single_layer(
        &self,
        board: &PcbBoard,
        layer_name: &str,
    ) -> Result<String, DxfError> {
        let mut single_opt = self.options.clone();
        single_opt.layer_filter = Some(vec![layer_name.to_string()]);
        let exporter = DxfExporter::new(single_opt);
        exporter.export_board(board)
    }

    fn collect_active_layers(&self, board: &PcbBoard) -> Vec<DxfLayerConfig> {
        // Standard layer palette with industrial ACI color assignments
        let mut known_layers = vec![
            DxfLayerConfig::new("BOARD_OUTLINE", AciColor::WhiteOrBlack, "Board Boundary"),
            DxfLayerConfig::new("F_CU", AciColor::Red, "Top Copper Layer"),
            DxfLayerConfig::new("B_CU", AciColor::Blue, "Bottom Copper Layer"),
            DxfLayerConfig::new("F_SILK", AciColor::Yellow, "Top Silkscreen"),
            DxfLayerConfig::new("B_SILK", AciColor::Magenta, "Bottom Silkscreen"),
            DxfLayerConfig::new("F_MASK", AciColor::DarkGray, "Top Solder Mask"),
            DxfLayerConfig::new("B_MASK", AciColor::LightGray, "Bottom Solder Mask"),
            DxfLayerConfig::new("DRILL", AciColor::Cyan, "Through-hole and Via Drills"),
        ];

        // Add custom inner layers if declared
        for l in &board.layers {
            let canon = canonicalize_layer_name(&l.name);
            if !known_layers.iter().any(|k| k.name == canon) {
                known_layers.push(DxfLayerConfig::new(canon, AciColor::Green, &l.name));
            }
        }

        if let Some(ref filter) = self.options.layer_filter {
            let filter_set: BTreeSet<String> =
                filter.iter().map(|s| canonicalize_layer_name(s)).collect();
            known_layers.retain(|l| filter_set.contains(&l.name));
        }

        known_layers
    }

    fn write_header(&self, out: &mut String, board: &PcbBoard) -> Result<(), DxfError> {
        writeln!(out, "  0\nSECTION")?;
        writeln!(out, "  2\nHEADER")?;

        // AutoCAD Version (AC1009 = R12)
        writeln!(out, "  9\n$ACADVER")?;
        writeln!(out, "  1\n{}", self.options.acad_version)?;

        // Drawing units: 4 = Millimeters, 1 = Inches
        writeln!(out, "  9\n$INSUNITS")?;
        writeln!(out, " 70\n{}", if self.options.metric { 4 } else { 1 })?;

        // Measurement: 1 = Metric, 0 = English
        writeln!(out, "  9\n$MEASUREMENT")?;
        writeln!(out, " 70\n{}", if self.options.metric { 1 } else { 0 })?;

        // Extents calculation
        let (min_x, min_y, max_x, max_y) = self.calculate_extents(board);
        writeln!(out, "  9\n$EXTMIN")?;
        writeln!(out, " 10\n{:.4}", min_x)?;
        writeln!(out, " 20\n{:.4}", min_y)?;
        writeln!(out, " 30\n0.0")?;

        writeln!(out, "  9\n$EXTMAX")?;
        writeln!(out, " 10\n{:.4}", max_x)?;
        writeln!(out, " 20\n{:.4}", max_y)?;
        writeln!(out, " 30\n0.0")?;

        writeln!(out, "  0\nENDSEC")?;
        Ok(())
    }

    fn write_tables(&self, out: &mut String, layers: &[DxfLayerConfig]) -> Result<(), DxfError> {
        writeln!(out, "  0\nSECTION")?;
        writeln!(out, "  2\nTABLES")?;

        // 1. Linetype table
        writeln!(out, "  0\nTABLE")?;
        writeln!(out, "  2\nLTYPE")?;
        writeln!(out, " 70\n1")?;

        writeln!(out, "  0\nLTYPE")?;
        writeln!(out, "  2\nCONTINUOUS")?;
        writeln!(out, " 70\n0")?;
        writeln!(out, "  3\nSolid line")?;
        writeln!(out, " 72\n65")?;
        writeln!(out, " 73\n0")?;
        writeln!(out, " 40\n0.0")?;

        writeln!(out, "  0\nENDTAB")?;

        // 2. Layer table
        writeln!(out, "  0\nTABLE")?;
        writeln!(out, "  2\nLAYER")?;
        writeln!(out, " 70\n{}", layers.len() + 1)?;

        // Default layer 0
        writeln!(out, "  0\nLAYER")?;
        writeln!(out, "  2\n0")?;
        writeln!(out, " 70\n0")?;
        writeln!(out, " 62\n7")?;
        writeln!(out, "  6\nCONTINUOUS")?;

        for layer in layers {
            writeln!(out, "  0\nLAYER")?;
            writeln!(out, "  2\n{}", layer.name)?;
            writeln!(out, " 70\n0")?;
            writeln!(out, " 62\n{}", layer.color as i16)?;
            writeln!(out, "  6\nCONTINUOUS")?;
        }

        writeln!(out, "  0\nENDTAB")?;
        writeln!(out, "  0\nENDSEC")?;
        Ok(())
    }

    fn write_blocks(&self, out: &mut String) -> Result<(), DxfError> {
        writeln!(out, "  0\nSECTION")?;
        writeln!(out, "  2\nBLOCKS")?;
        writeln!(out, "  0\nENDSEC")?;
        Ok(())
    }

    fn write_entities(&self, out: &mut String, board: &PcbBoard) -> Result<(), DxfError> {
        writeln!(out, "  0\nSECTION")?;
        writeln!(out, "  2\nENTITIES")?;

        // 1. Board outline and board-level graphics
        if self.should_include_layer("BOARD_OUTLINE") {
            self.write_board_graphics(out, board)?;
        }

        // 2. Trace segments
        for seg in &board.segments {
            let layer_name = canonicalize_layer_name(&seg.layer);
            if !self.should_include_layer(&layer_name) {
                continue;
            }

            self.write_line(
                out,
                &layer_name,
                seg.start.x,
                seg.start.y,
                seg.end.x,
                seg.end.y,
            )?;
        }

        // 3. Vias
        if self.should_include_layer("DRILL") || self.should_include_layer("F_CU") {
            for via in &board.vias {
                let r = via.diameter / 2.0;
                let drill_r = via.drill / 2.0;

                if self.should_include_layer("F_CU") {
                    self.write_circle(out, "F_CU", via.position.x, via.position.y, r)?;
                }
                if self.should_include_layer("B_CU") {
                    self.write_circle(out, "B_CU", via.position.x, via.position.y, r)?;
                }
                if self.should_include_layer("DRILL") && drill_r > 0.0 {
                    self.write_circle(out, "DRILL", via.position.x, via.position.y, drill_r)?;
                }
            }
        }

        // 4. Footprints (Pads, silkscreen lines, and text)
        for fp in &board.footprints {
            let fp_rot = fp.rotation.to_radians();

            // Footprint pads
            for pad in &fp.pads {
                let (px, py) = rotate_point(pad.position.x, pad.position.y, fp_rot);
                let center_x = fp.position.x + px;
                let center_y = fp.position.y + py;

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
                    match pad.shape {
                        PadShape::Circle => {
                            let radius = pad.size.x.max(pad.size.y) / 2.0;
                            self.write_circle(out, pad_layer, center_x, center_y, radius)?;
                        }
                        PadShape::Rect | PadShape::RoundRect | PadShape::Custom => {
                            let hw = pad.size.x / 2.0;
                            let hh = pad.size.y / 2.0;
                            let (c1x, c1y) = rotate_point(-hw, -hh, fp_rot);
                            let (c2x, c2y) = rotate_point(hw, -hh, fp_rot);
                            let (c3x, c3y) = rotate_point(hw, hh, fp_rot);
                            let (c4x, c4y) = rotate_point(-hw, hh, fp_rot);

                            let p1 = (center_x + c1x, center_y + c1y);
                            let p2 = (center_x + c2x, center_y + c2y);
                            let p3 = (center_x + c3x, center_y + c3y);
                            let p4 = (center_x + c4x, center_y + c4y);

                            if self.options.export_solids {
                                self.write_solid(out, pad_layer, p1, p2, p4, p3)?;
                            } else {
                                self.write_closed_polyline(out, pad_layer, &[p1, p2, p3, p4])?;
                            }
                        }
                        PadShape::Oval | PadShape::Trapezoid => {
                            let radius = pad.size.x.max(pad.size.y) / 2.0;
                            self.write_circle(out, pad_layer, center_x, center_y, radius)?;
                        }
                    }
                }

                // Drill hole
                if let Some(drill) = pad
                    .drill
                    .as_ref()
                    .filter(|d| self.should_include_layer("DRILL") && d.diameter > 0.0)
                {
                    self.write_circle(out, "DRILL", center_x, center_y, drill.diameter / 2.0)?;
                }
            }

            // Footprint graphics / silkscreen
            let silk_layer = if fp.layer.to_ascii_lowercase().contains("bottom") {
                "B_SILK"
            } else {
                "F_SILK"
            };

            if self.should_include_layer(silk_layer) {
                for g in &fp.graphics {
                    if let (Some(s), Some(e)) = (g.start, g.end) {
                        let (s_rot_x, s_rot_y) = rotate_point(s.x, s.y, fp_rot);
                        let (e_rot_x, e_rot_y) = rotate_point(e.x, e.y, fp_rot);
                        self.write_line(
                            out,
                            silk_layer,
                            fp.position.x + s_rot_x,
                            fp.position.y + s_rot_y,
                            fp.position.x + e_rot_x,
                            fp.position.y + e_rot_y,
                        )?;
                    } else if !g.points.is_empty() {
                        let pts: Vec<(f64, f64)> = g
                            .points
                            .iter()
                            .map(|p| {
                                let (rx, ry) = rotate_point(p.x, p.y, fp_rot);
                                (fp.position.x + rx, fp.position.y + ry)
                            })
                            .collect();
                        self.write_polyline(out, silk_layer, &pts, false)?;
                    }
                }

                // Footprint reference / value text
                if self.options.export_text && !fp.reference.is_empty() {
                    self.write_text(
                        out,
                        silk_layer,
                        fp.position.x,
                        fp.position.y,
                        1.2,
                        &fp.reference,
                        fp.rotation,
                    )?;
                }
            }
        }

        // 5. Copper Zones
        for zone in &board.zones {
            let zone_layer = canonicalize_layer_name(&zone.layer);
            if !self.should_include_layer(&zone_layer) || zone.outline.len() < 3 {
                continue;
            }

            let pts: Vec<(f64, f64)> = zone.outline.iter().map(|p| (p.x, p.y)).collect();
            self.write_closed_polyline(out, &zone_layer, &pts)?;
        }

        writeln!(out, "  0\nENDSEC")?;
        Ok(())
    }

    fn write_board_graphics(&self, out: &mut String, board: &PcbBoard) -> Result<(), DxfError> {
        for bg in &board.graphics {
            if let (Some(s), Some(e)) = (bg.start, bg.end) {
                self.write_line(out, "BOARD_OUTLINE", s.x, s.y, e.x, e.y)?;
            } else if !bg.points.is_empty() {
                let pts: Vec<(f64, f64)> = bg.points.iter().map(|p| (p.x, p.y)).collect();
                self.write_closed_polyline(out, "BOARD_OUTLINE", &pts)?;
            } else if let Some(c) = bg.center.filter(|_| bg.radius > 0.0) {
                self.write_circle(out, "BOARD_OUTLINE", c.x, c.y, bg.radius)?;
            }
        }
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

        for via in &board.vias {
            update(via.position.x - via.diameter, via.position.y - via.diameter);
            update(via.position.x + via.diameter, via.position.y + via.diameter);
        }

        for fp in &board.footprints {
            update(fp.position.x - 5.0, fp.position.y - 5.0);
            update(fp.position.x + 5.0, fp.position.y + 5.0);
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

        if min_x.is_infinite() {
            (0.0, 0.0, 100.0, 100.0)
        } else {
            (min_x - 5.0, min_y - 5.0, max_x + 5.0, max_y + 5.0)
        }
    }

    fn write_line(
        &self,
        out: &mut String,
        layer: &str,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> Result<(), DxfError> {
        writeln!(out, "  0\nLINE")?;
        writeln!(out, "  8\n{}", layer)?;
        writeln!(out, " 10\n{:.4}", x1)?;
        writeln!(out, " 20\n{:.4}", y1)?;
        writeln!(out, " 30\n0.0")?;
        writeln!(out, " 11\n{:.4}", x2)?;
        writeln!(out, " 21\n{:.4}", y2)?;
        writeln!(out, " 31\n0.0")?;
        Ok(())
    }

    fn write_circle(
        &self,
        out: &mut String,
        layer: &str,
        cx: f64,
        cy: f64,
        r: f64,
    ) -> Result<(), DxfError> {
        writeln!(out, "  0\nCIRCLE")?;
        writeln!(out, "  8\n{}", layer)?;
        writeln!(out, " 10\n{:.4}", cx)?;
        writeln!(out, " 20\n{:.4}", cy)?;
        writeln!(out, " 30\n0.0")?;
        writeln!(out, " 40\n{:.4}", r)?;
        Ok(())
    }

    fn write_solid(
        &self,
        out: &mut String,
        layer: &str,
        p1: (f64, f64),
        p2: (f64, f64),
        p3: (f64, f64),
        p4: (f64, f64),
    ) -> Result<(), DxfError> {
        // AutoCAD R12 SOLID entity order: P1, P2, P4, P3 (bow-tie prevention)
        writeln!(out, "  0\nSOLID")?;
        writeln!(out, "  8\n{}", layer)?;
        writeln!(out, " 10\n{:.4}", p1.0)?;
        writeln!(out, " 20\n{:.4}", p1.1)?;
        writeln!(out, " 30\n0.0")?;
        writeln!(out, " 11\n{:.4}", p2.0)?;
        writeln!(out, " 21\n{:.4}", p2.1)?;
        writeln!(out, " 31\n0.0")?;
        writeln!(out, " 12\n{:.4}", p3.0)?;
        writeln!(out, " 22\n{:.4}", p3.1)?;
        writeln!(out, " 32\n0.0")?;
        writeln!(out, " 13\n{:.4}", p4.0)?;
        writeln!(out, " 23\n{:.4}", p4.1)?;
        writeln!(out, " 33\n0.0")?;
        Ok(())
    }

    fn write_polyline(
        &self,
        out: &mut String,
        layer: &str,
        points: &[(f64, f64)],
        closed: bool,
    ) -> Result<(), DxfError> {
        if points.is_empty() {
            return Ok(());
        }

        // AutoCAD R12 standard 2D POLYLINE entity
        writeln!(out, "  0\nPOLYLINE")?;
        writeln!(out, "  8\n{}", layer)?;
        writeln!(out, " 66\n1")?; // Vertices follow flag
        writeln!(out, " 70\n{}", if closed { 1 } else { 0 })?;
        writeln!(out, " 10\n0.0\n 20\n0.0\n 30\n0.0")?;

        for &(x, y) in points {
            writeln!(out, "  0\nVERTEX")?;
            writeln!(out, "  8\n{}", layer)?;
            writeln!(out, " 10\n{:.4}", x)?;
            writeln!(out, " 20\n{:.4}", y)?;
            writeln!(out, " 30\n0.0")?;
        }

        writeln!(out, "  0\nSEQEND")?;
        writeln!(out, "  8\n{}", layer)?;
        Ok(())
    }

    fn write_closed_polyline(
        &self,
        out: &mut String,
        layer: &str,
        points: &[(f64, f64)],
    ) -> Result<(), DxfError> {
        self.write_polyline(out, layer, points, true)
    }

    #[allow(clippy::too_many_arguments)]
    fn write_text(
        &self,
        out: &mut String,
        layer: &str,
        x: f64,
        y: f64,
        height: f64,
        text: &str,
        rotation_deg: f64,
    ) -> Result<(), DxfError> {
        writeln!(out, "  0\nTEXT")?;
        writeln!(out, "  8\n{}", layer)?;
        writeln!(out, " 10\n{:.4}", x)?;
        writeln!(out, " 20\n{:.4}", y)?;
        writeln!(out, " 30\n0.0")?;
        writeln!(out, " 40\n{:.4}", height)?;
        writeln!(out, "  1\n{}", text)?;
        if rotation_deg.abs() > 1e-4 {
            writeln!(out, " 50\n{:.2}", rotation_deg)?;
        }
        Ok(())
    }
}

/// Helper function to normalize layer names into valid uppercase DXF identifiers.
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

fn rotate_point(x: f64, y: f64, angle_rad: f64) -> (f64, f64) {
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}
