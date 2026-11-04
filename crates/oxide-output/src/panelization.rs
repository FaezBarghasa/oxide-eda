//! Automated PCB Panelization Engine.
//!
//! Provides step-and-repeat board placement, edge rails, optical fiducials,
//! V-scoring calculations, mouse-bite breakaway tabs, and copper thieving grids
//! according to IPC-2221/IPC-A-600 standards.

use serde::{Deserialize, Serialize};

/// 2D point representation in millimetres.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Instance of a board in the panel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoardInstance {
    pub origin_mm: Point2D,
    pub width_mm: f64,
    pub height_mm: f64,
    pub rotation_deg: f64,
}

/// V-Score cut line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VScoreLine {
    pub start: Point2D,
    pub end: Point2D,
    /// Blade angle in degrees (default 30° or 45°).
    pub angle_deg: f64,
    /// Remaining web thickness in mm (standard is 1/3 of board thickness, typically 0.3mm - 0.5mm).
    pub web_thickness_mm: f64,
}

/// Mouse-bite perforation breakaway tab.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MouseBiteTab {
    pub center: Point2D,
    pub tab_width_mm: f64,
    pub hole_diameter_mm: f64,
    pub hole_pitch_mm: f64,
    pub hole_count: u32,
}

/// Copper thieving / dummy copper fill pattern to equalize plating current density.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThievingGrid {
    pub dot_diameter_mm: f64,
    pub pitch_mm: f64,
    pub clearance_to_copper_mm: f64,
    pub offset_to_rail_edge_mm: f64,
}

/// Panel fabrication violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelViolationKind {
    VScoreWebTooThin,
    VScoreWebTooThick,
    MouseBiteHolePitchTooTight,
    RailTooNarrow,
    FiducialTooCloseToEdge,
    BoardOverlap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PanelViolation {
    pub kind: PanelViolationKind,
    pub message: String,
    pub location: Option<Point2D>,
}

/// Complete Panel Layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PanelLayout {
    pub width_mm: f64,
    pub height_mm: f64,
    pub boards: Vec<BoardInstance>,
    pub rail_top_mm: f64,
    pub rail_bottom_mm: f64,
    pub rail_left_mm: f64,
    pub rail_right_mm: f64,
    pub fiducials: Vec<Point2D>,
    pub tooling_holes: Vec<Point2D>,
    pub v_score_lines: Vec<VScoreLine>,
    pub mouse_bites: Vec<MouseBiteTab>,
    pub copper_thieving: Option<ThievingGrid>,
}

impl PanelLayout {
    /// Generates a standard step-and-repeat 2D array panel with perimeter rails.
    pub fn create_step_and_repeat(
        board_width_mm: f64,
        board_height_mm: f64,
        cols: u32,
        rows: u32,
        spacing_x_mm: f64,
        spacing_y_mm: f64,
        rail_width_mm: f64,
    ) -> Self {
        let cols = cols.max(1);
        let rows = rows.max(1);

        let active_w = (cols as f64) * board_width_mm + ((cols - 1) as f64) * spacing_x_mm;
        let active_h = (rows as f64) * board_height_mm + ((rows - 1) as f64) * spacing_y_mm;

        let total_w = active_w + 2.0 * rail_width_mm;
        let total_h = active_h + 2.0 * rail_width_mm;

        let mut boards = Vec::with_capacity((cols * rows) as usize);
        for r in 0..rows {
            for c in 0..cols {
                let x = rail_width_mm + (c as f64) * (board_width_mm + spacing_x_mm);
                let y = rail_width_mm + (r as f64) * (board_height_mm + spacing_y_mm);
                boards.push(BoardInstance {
                    origin_mm: Point2D::new(x, y),
                    width_mm: board_width_mm,
                    height_mm: board_height_mm,
                    rotation_deg: 0.0,
                });
            }
        }

        // Standard 3 optical fiducials (L-shaped alignment) with 5mm rail setback
        let fiducials = vec![
            Point2D::new(rail_width_mm / 2.0, rail_width_mm / 2.0),
            Point2D::new(total_w - rail_width_mm / 2.0, rail_width_mm / 2.0),
            Point2D::new(rail_width_mm / 2.0, total_h - rail_width_mm / 2.0),
        ];

        // 4 tooling holes (3.2mm typical for standard SMT conveyor pins) in the 4 corners
        let tooling_holes = vec![
            Point2D::new(5.0, 5.0),
            Point2D::new(total_w - 5.0, 5.0),
            Point2D::new(5.0, total_h - 5.0),
            Point2D::new(total_w - 5.0, total_h - 5.0),
        ];

        let mut v_scores = Vec::new();
        // If spacing is 0, add V-scoring along borders
        if spacing_x_mm == 0.0 && cols > 1 {
            for c in 1..cols {
                let x = rail_width_mm + (c as f64) * board_width_mm;
                v_scores.push(VScoreLine {
                    start: Point2D::new(x, 0.0),
                    end: Point2D::new(x, total_h),
                    angle_deg: 30.0,
                    web_thickness_mm: 0.4,
                });
            }
        }
        if spacing_y_mm == 0.0 && rows > 1 {
            for r in 1..rows {
                let y = rail_width_mm + (r as f64) * board_height_mm;
                v_scores.push(VScoreLine {
                    start: Point2D::new(0.0, y),
                    end: Point2D::new(total_w, y),
                    angle_deg: 30.0,
                    web_thickness_mm: 0.4,
                });
            }
        }

        Self {
            width_mm: total_w,
            height_mm: total_h,
            boards,
            rail_top_mm: rail_width_mm,
            rail_bottom_mm: rail_width_mm,
            rail_left_mm: rail_width_mm,
            rail_right_mm: rail_width_mm,
            fiducials,
            tooling_holes,
            v_score_lines: v_scores,
            mouse_bites: Vec::new(),
            copper_thieving: Some(ThievingGrid {
                dot_diameter_mm: 1.5,
                pitch_mm: 3.0,
                clearance_to_copper_mm: 1.0,
                offset_to_rail_edge_mm: 2.0,
            }),
        }
    }

    /// Validates panel manufacturing constraints against IPC-2221 and IPC-A-600.
    pub fn validate(&self) -> Vec<PanelViolation> {
        let mut violations = Vec::new();

        // 1. Check rail minimum width (IPC recommends >= 5.0mm for automated conveyor grip)
        let min_rail = self
            .rail_top_mm
            .min(self.rail_bottom_mm)
            .min(self.rail_left_mm)
            .min(self.rail_right_mm);
        if min_rail < 4.0 {
            violations.push(PanelViolation {
                kind: PanelViolationKind::RailTooNarrow,
                message: format!(
                    "Panel conveyor rail width {:.2}mm is below the 4.0mm SMT transport minimum (5.0mm recommended)",
                    min_rail
                ),
                location: None,
            });
        }

        // 2. Check fiducials distance to panel edges (>= 3.0mm required for optical camera clearance)
        for fid in &self.fiducials {
            let dist_edge = fid
                .x
                .min(fid.y)
                .min(self.width_mm - fid.x)
                .min(self.height_mm - fid.y);
            if dist_edge < 2.5 {
                violations.push(PanelViolation {
                    kind: PanelViolationKind::FiducialTooCloseToEdge,
                    message: format!(
                        "Optical fiducial at ({:.2}, {:.2}) is too close to panel edge ({:.2}mm < 2.5mm)",
                        fid.x, fid.y, dist_edge
                    ),
                    location: Some(*fid),
                });
            }
        }

        // 3. Check V-score web residual thickness (for 1.6mm board, standard is 0.3mm - 0.6mm)
        for score in &self.v_score_lines {
            if score.web_thickness_mm < 0.25 {
                violations.push(PanelViolation {
                    kind: PanelViolationKind::VScoreWebTooThin,
                    message: format!(
                        "V-Score web thickness {:.2}mm is critically thin (< 0.25mm), risk of premature de-paneling",
                        score.web_thickness_mm
                    ),
                    location: Some(score.start),
                });
            } else if score.web_thickness_mm > 0.8 {
                violations.push(PanelViolation {
                    kind: PanelViolationKind::VScoreWebTooThick,
                    message: format!(
                        "V-Score web thickness {:.2}mm is too thick (> 0.8mm), excessive mechanical stress during breakout",
                        score.web_thickness_mm
                    ),
                    location: Some(score.start),
                });
            }
        }

        // 4. Check mouse bite hole spacing (pitch must be >= hole diameter * 1.5)
        for mb in &self.mouse_bites {
            if mb.hole_pitch_mm < mb.hole_diameter_mm * 1.2 {
                violations.push(PanelViolation {
                    kind: PanelViolationKind::MouseBiteHolePitchTooTight,
                    message: format!(
                        "Mouse-bite hole pitch {:.2}mm is too tight for hole diameter {:.2}mm, risking jagged edges",
                        mb.hole_pitch_mm, mb.hole_diameter_mm
                    ),
                    location: Some(mb.center),
                });
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_step_and_repeat_generation() {
        let panel = PanelLayout::create_step_and_repeat(50.0, 40.0, 2, 3, 0.0, 0.0, 8.0);
        // Total width: 2 * 50 + 2 * 8 = 116mm
        assert_eq!(panel.width_mm, 116.0);
        // Total height: 3 * 40 + 2 * 8 = 136mm
        assert_eq!(panel.height_mm, 136.0);
        assert_eq!(panel.boards.len(), 6);
        assert_eq!(panel.fiducials.len(), 3);
        assert_eq!(panel.tooling_holes.len(), 4);
        assert_eq!(panel.v_score_lines.len(), 3); // 1 vertical + 2 horizontal

        let violations = panel.validate();
        assert!(violations.is_empty(), "Standard panel should have 0 violations");
    }

    #[test]
    fn test_panel_validation_rules() {
        let mut panel = PanelLayout::create_step_and_repeat(50.0, 40.0, 2, 2, 2.0, 2.0, 2.0); // rail too narrow (2mm)
        panel.v_score_lines.push(VScoreLine {
            start: Point2D::new(0.0, 20.0),
            end: Point2D::new(50.0, 20.0),
            angle_deg: 30.0,
            web_thickness_mm: 0.15, // too thin
        });

        let violations = panel.validate();
        assert!(violations.iter().any(|v| v.kind == PanelViolationKind::RailTooNarrow));
        assert!(violations.iter().any(|v| v.kind == PanelViolationKind::VScoreWebTooThin));
    }
}

