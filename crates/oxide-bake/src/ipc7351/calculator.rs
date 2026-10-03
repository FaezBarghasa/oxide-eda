use oxide_library::primitive::footprint::{FpPasteAperture, LayerId, Polygon};
use serde::{Deserialize, Serialize};

/// IPC-7351C Density Level target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DensityLevel {
    /// Level A: Most (Maximum Land Protrusion) — Prototyping, military, harsh environment.
    Most,
    /// Level B: Nominal (Median Land Protrusion) — Standard commercial SMT assembly.
    #[default]
    Nominal,
    /// Level C: Least (Minimum Land Protrusion) — Ultra-dense, handheld, mobile layout.
    Least,
}

/// Solder fillet goals (Toe, Heel, Side) and Courtyard excess in millimeters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilletTargets {
    pub toe_jt: f64,
    pub heel_jh: f64,
    pub side_js: f64,
    pub courtyard_excess: f64,
}

impl FilletTargets {
    /// Gull-wing packages: SOIC, SOP, TSSOP, QFP, SOT23
    pub fn gull_wing(density: DensityLevel) -> Self {
        match density {
            DensityLevel::Most => Self {
                toe_jt: 0.55,
                heel_jh: 0.45,
                side_js: 0.05,
                courtyard_excess: 0.50,
            },
            DensityLevel::Nominal => Self {
                toe_jt: 0.35,
                heel_jh: 0.35,
                side_js: 0.03,
                courtyard_excess: 0.25,
            },
            DensityLevel::Least => Self {
                toe_jt: 0.15,
                heel_jh: 0.25,
                side_js: 0.01,
                courtyard_excess: 0.12,
            },
        }
    }

    /// No-lead packages: QFN, DFN, SON
    pub fn no_lead(density: DensityLevel) -> Self {
        match density {
            DensityLevel::Most => Self {
                toe_jt: 0.40,
                heel_jh: 0.00,
                side_js: -0.04,
                courtyard_excess: 0.50,
            },
            DensityLevel::Nominal => Self {
                toe_jt: 0.30,
                heel_jh: 0.00,
                side_js: -0.04,
                courtyard_excess: 0.25,
            },
            DensityLevel::Least => Self {
                toe_jt: 0.20,
                heel_jh: 0.00,
                side_js: -0.04,
                courtyard_excess: 0.12,
            },
        }
    }

    /// Rectangular chip passives: 0402, 0603, 0805, 1206
    pub fn leadless_chip(density: DensityLevel) -> Self {
        match density {
            DensityLevel::Most => Self {
                toe_jt: 0.55,
                heel_jh: 0.00,
                side_js: -0.05,
                courtyard_excess: 0.50,
            },
            DensityLevel::Nominal => Self {
                toe_jt: 0.35,
                heel_jh: 0.00,
                side_js: -0.05,
                courtyard_excess: 0.25,
            },
            DensityLevel::Least => Self {
                toe_jt: 0.15,
                heel_jh: 0.00,
                side_js: -0.05,
                courtyard_excess: 0.12,
            },
        }
    }

    /// Selects fillet targets matching package class string.
    pub fn for_package(pkg_class: &str, density: DensityLevel) -> Self {
        let u = pkg_class.to_uppercase();
        if u.contains("QFN") || u.contains("DFN") || u.contains("SON") || u.contains("LGA") {
            Self::no_lead(density)
        } else if u.contains("CHIP")
            || u.contains("0402")
            || u.contains("0603")
            || u.contains("0805")
            || u.contains("1206")
        {
            Self::leadless_chip(density)
        } else {
            Self::gull_wing(density)
        }
    }
}

/// Solved IPC-7351C Pad geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct SolvedPadGeometry {
    pub length_x: f64,
    pub width_y: f64,
    pub center_to_center_c: f64,
}

/// Computes IPC-7351C mathematical land pattern parameters.
pub fn calculate_ipc7351c_pad(
    lead_span_min: f64,
    lead_span_max: f64,
    inner_span_min: f64,
    inner_span_max: f64,
    lead_width_min: f64,
    lead_width_max: f64,
    fillet: &FilletTargets,
) -> SolvedPadGeometry {
    // Tolerances: fabrication F = 0.05mm, placement P = 0.05mm
    let f = 0.05;
    let p = 0.05;

    let c_l = (lead_span_max - lead_span_min).abs();
    let c_s = (inner_span_max - inner_span_min).abs();
    let c_w = (lead_width_max - lead_width_min).abs();

    // Solder fillet calculations
    // Z_max = L_min + 2*J_T + sqrt(C_L^2 + F^2 + P^2)
    let s_l = (c_l * c_l + f * f + p * p).sqrt();
    let z_max = lead_span_min + 2.0 * fillet.toe_jt + s_l;

    // G_min = S_max - 2*J_H - sqrt(C_S^2 + F^2 + P^2)
    let s_s = (c_s * c_s + f * f + p * p).sqrt();
    let g_min = (inner_span_max - 2.0 * fillet.heel_jh - s_s).max(0.1);

    // Pad Length X = (Z_max - G_min) / 2
    let length_x = ((z_max - g_min) / 2.0).max(0.2);

    // Pad Width Y = b_max + 2*J_S + sqrt(C_W^2 + F^2 + P^2)
    let s_w = (c_w * c_w + f * f + p * p).sqrt();
    let width_y = (lead_width_max + 2.0 * fillet.side_js + s_w).max(0.15);

    // Center-to-center distance C = (Z_max + G_min) / 2
    let center_to_center_c = (z_max + g_min) / 2.0;

    SolvedPadGeometry {
        length_x,
        width_y,
        center_to_center_c,
    }
}

/// Synthesizes paste aperture window panes for thermal pad (achieving 60-70% coverage).
pub fn synthesize_thermal_paste_panes(
    thermal_w: f64,
    thermal_h: f64,
    target_coverage: f64, // e.g. 0.65 (65%)
) -> Vec<FpPasteAperture> {
    let mut apertures = Vec::new();
    let layer = LayerId::new("F.Paste");

    if thermal_w <= 2.0 || thermal_h <= 2.0 {
        // Small thermal pad: single scaled pane
        let scale = target_coverage.sqrt();
        let pane_w = thermal_w * scale;
        let pane_h = thermal_h * scale;
        apertures.push(FpPasteAperture {
            boundary: Polygon::new(vec![
                [-pane_w / 2.0, -pane_h / 2.0],
                [pane_w / 2.0, -pane_h / 2.0],
                [pane_w / 2.0, pane_h / 2.0],
                [-pane_w / 2.0, pane_h / 2.0],
            ]),
            layer,
        });
        return apertures;
    }

    // Large thermal pad: 2x2 or 3x3 matrix of panes
    let cols = if thermal_w > 4.0 { 3 } else { 2 };
    let rows = if thermal_h > 4.0 { 3 } else { 2 };
    let dam_web_mm = 0.25;

    let avail_w = thermal_w - (cols as f64 - 1.0) * dam_web_mm;
    let avail_h = thermal_h - (rows as f64 - 1.0) * dam_web_mm;

    let base_pane_w = avail_w / cols as f64;
    let base_pane_h = avail_h / rows as f64;

    let scale = target_coverage.sqrt();
    let pane_w = base_pane_w * scale;
    let pane_h = base_pane_h * scale;

    let step_x = base_pane_w + dam_web_mm;
    let step_y = base_pane_h + dam_web_mm;

    let start_x = -((cols as f64 - 1.0) * step_x) / 2.0;
    let start_y = -((rows as f64 - 1.0) * step_y) / 2.0;

    for r in 0..rows {
        for c in 0..cols {
            let cx = start_x + c as f64 * step_x;
            let cy = start_y + r as f64 * step_y;
            apertures.push(FpPasteAperture {
                boundary: Polygon::new(vec![
                    [cx - pane_w / 2.0, cy - pane_h / 2.0],
                    [cx + pane_w / 2.0, cy - pane_h / 2.0],
                    [cx + pane_w / 2.0, cy + pane_h / 2.0],
                    [cx - pane_w / 2.0, cy + pane_h / 2.0],
                ]),
                layer: layer.clone(),
            });
        }
    }

    apertures
}
