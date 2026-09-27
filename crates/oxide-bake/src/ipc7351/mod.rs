pub mod calculator;
pub mod extrusion_3d;

pub use calculator::{
    calculate_ipc7351c_pad, synthesize_thermal_paste_panes, DensityLevel, FilletTargets,
    SolvedPadGeometry,
};
pub use extrusion_3d::Package3DExtruder;

use oxide_library::harvester::PackageDimensions;
use oxide_library::primitive::footprint::{
    Footprint, FpGraphic, FpGraphicKind, LayerId, Pad, PadKind, PadShape, Polygon,
};

/// High-level IPC-7351C Footprint Generator.
pub struct Ipc7351Generator;

impl Ipc7351Generator {
    /// Mathematically compiles package dimensions into an IPC-7351C compliant Footprint.
    pub fn generate_footprint(name: &str, dims: &PackageDimensions, density: DensityLevel) -> Footprint {
        let mut fp = Footprint::empty(name);
        let fillet = FilletTargets::for_package(&dims.package_class, density);

        let top_copper = vec![
            LayerId::new("F.Cu"),
            LayerId::new("F.Mask"),
            LayerId::new("F.Paste"),
        ];

        let pkg_upper = dims.package_class.to_uppercase();

        if pkg_upper.contains("CHIP") || dims.pin_count == 2 {
            // 2-terminal leadless chip component (Resistors/Capacitors)
            let length_min = dims.body_length_mm[0];
            let length_max = dims.body_length_mm[2];
            let width_min = dims.body_width_mm[0];
            let width_max = dims.body_width_mm[2];
            let lead_l_min = dims.lead_length_mm[0];
            let lead_l_max = dims.lead_length_mm[2];

            let inner_min = length_min - 2.0 * lead_l_max;
            let inner_max = length_max - 2.0 * lead_l_min;

            let solved = calculate_ipc7351c_pad(
                length_min,
                length_max,
                inner_min,
                inner_max,
                width_min,
                width_max,
                &fillet,
            );

            let pad_x = solved.center_to_center_c / 2.0;

            // Pad 1 (Left)
            fp.pads.push(Pad {
                number: "1".into(),
                kind: PadKind::Smd,
                shape: PadShape::Rect,
                size: [solved.length_x, solved.width_y],
                position: [-pad_x, 0.0],
                layers: top_copper.clone(),
                ..Pad::default()
            });

            // Pad 2 (Right)
            fp.pads.push(Pad {
                number: "2".into(),
                kind: PadKind::Smd,
                shape: PadShape::Rect,
                size: [solved.length_x, solved.width_y],
                position: [pad_x, 0.0],
                layers: top_copper.clone(),
                ..Pad::default()
            });

            // Courtyard
            let cy_w = solved.center_to_center_c + solved.length_x + fillet.courtyard_excess * 2.0;
            let cy_h = solved.width_y + fillet.courtyard_excess * 2.0;
            fp.courtyard = Polygon::new(vec![
                [-cy_w / 2.0, -cy_h / 2.0],
                [cy_w / 2.0, -cy_h / 2.0],
                [cy_w / 2.0, cy_h / 2.0],
                [-cy_w / 2.0, cy_h / 2.0],
            ]);
        } else if pkg_upper.contains("QFP") || (pkg_upper.contains("QFN") && dims.pin_count >= 16) {
            // 4-sided Quad Flat Package (QFP / QFN)
            let side_pins = dims.pin_count / 4;
            let pitch = dims.lead_pitch_mm;
            let body_w_nom = dims.body_width_mm[1];
            let _body_l_nom = dims.body_length_mm[1];

            let solved = calculate_ipc7351c_pad(
                dims.body_width_mm[0] + 2.0 * dims.lead_length_mm[0],
                dims.body_width_mm[2] + 2.0 * dims.lead_length_mm[2],
                dims.body_width_mm[0],
                dims.body_width_mm[2],
                dims.lead_width_mm[0],
                dims.lead_width_mm[2],
                &fillet,
            );

            let offset = solved.center_to_center_c / 2.0;
            let span_start = -((side_pins as f64 - 1.0) * pitch) / 2.0;

            let mut pin_num = 1;
            // 1. Left side (pins 1 .. side_pins)
            for i in 0..side_pins {
                let y = span_start + (i as f64 * pitch);
                fp.pads.push(Pad {
                    number: pin_num.to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [solved.length_x, solved.width_y],
                    position: [-offset, y],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                pin_num += 1;
            }

            // 2. Bottom side
            for i in 0..side_pins {
                let x = span_start + (i as f64 * pitch);
                fp.pads.push(Pad {
                    number: pin_num.to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [solved.width_y, solved.length_x],
                    position: [x, offset],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                pin_num += 1;
            }

            // 3. Right side
            for i in 0..side_pins {
                let y = span_start + ((side_pins - 1 - i) as f64 * pitch);
                fp.pads.push(Pad {
                    number: pin_num.to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [solved.length_x, solved.width_y],
                    position: [offset, y],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                pin_num += 1;
            }

            // 4. Top side
            for i in 0..side_pins {
                let x = span_start + ((side_pins - 1 - i) as f64 * pitch);
                fp.pads.push(Pad {
                    number: pin_num.to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [solved.width_y, solved.length_x],
                    position: [x, -offset],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                pin_num += 1;
            }

            // Optional Center Exposed Thermal Pad
            if let Some(therm) = dims.thermal_pad_mm {
                fp.pads.push(Pad {
                    number: "EP".into(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [therm[0], therm[1]],
                    position: [0.0, 0.0],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                fp.paste_apertures = synthesize_thermal_paste_panes(therm[0], therm[1], 0.65);
            }

            // Courtyard
            let cy = (offset + solved.length_x / 2.0 + fillet.courtyard_excess).max(body_w_nom / 2.0 + 0.5);
            fp.courtyard = Polygon::new(vec![
                [-cy, -cy],
                [cy, -cy],
                [cy, cy],
                [-cy, cy],
            ]);

            // Silk Outline + Pin 1 marker
            fp.silk_f.push(FpGraphic {
                kind: FpGraphicKind::Circle {
                    center: [-offset - 0.5, span_start - 0.5],
                    radius: 0.25,
                },
                stroke_width: 0.15,
                filled: true,
            });
        } else {
            // Dual inline / SOIC / SOP / SOT
            let half = dims.pin_count / 2;
            let pitch = dims.lead_pitch_mm;
            let _body_w_nom = dims.body_width_mm[1];
            let _body_l_nom = dims.body_length_mm[1];

            let solved = calculate_ipc7351c_pad(
                dims.body_width_mm[0] + 2.0 * dims.lead_length_mm[0],
                dims.body_width_mm[2] + 2.0 * dims.lead_length_mm[2],
                dims.body_width_mm[0],
                dims.body_width_mm[2],
                dims.lead_width_mm[0],
                dims.lead_width_mm[2],
                &fillet,
            );

            let x_span = solved.center_to_center_c / 2.0;
            let y_start = -((half as f64 - 1.0) * pitch) / 2.0;

            // Left row: pins 1 ..= half (top to bottom)
            for i in 0..half {
                let y = y_start + (i as f64 * pitch);
                fp.pads.push(Pad {
                    number: (i + 1).to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [solved.length_x, solved.width_y],
                    position: [-x_span, y],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
            }

            // Right row: pins half+1 ..= pin_count (bottom to top)
            for i in 0..half {
                let y = y_start + ((half - 1 - i) as f64 * pitch);
                fp.pads.push(Pad {
                    number: (half + 1 + i).to_string(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [solved.length_x, solved.width_y],
                    position: [x_span, y],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
            }

            // Optional Center Exposed Thermal Pad
            if let Some(therm) = dims.thermal_pad_mm {
                fp.pads.push(Pad {
                    number: "EP".into(),
                    kind: PadKind::Smd,
                    shape: PadShape::Rect,
                    size: [therm[0], therm[1]],
                    position: [0.0, 0.0],
                    layers: top_copper.clone(),
                    ..Pad::default()
                });
                fp.paste_apertures = synthesize_thermal_paste_panes(therm[0], therm[1], 0.65);
            }

            // Courtyard
            let cy_w = solved.center_to_center_c + solved.length_x + fillet.courtyard_excess * 2.0;
            let cy_h = ((half as f64) * pitch) + fillet.courtyard_excess * 2.0;
            fp.courtyard = Polygon::new(vec![
                [-cy_w / 2.0, -cy_h / 2.0],
                [cy_w / 2.0, -cy_h / 2.0],
                [cy_w / 2.0, cy_h / 2.0],
                [-cy_w / 2.0, cy_h / 2.0],
            ]);

            // Silk Pin 1 Polarity Marker
            fp.silk_f.push(FpGraphic {
                kind: FpGraphicKind::Circle {
                    center: [-x_span - 0.5, y_start - 0.5],
                    radius: 0.25,
                },
                stroke_width: 0.15,
                filled: true,
            });
        }

        // Procedural 3D body representation
        fp.body_3d = Package3DExtruder::synthesize_body_3d(dims);

        fp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipc_soic8_synthesis_all_density_levels() {
        let dims = PackageDimensions::standard_soic(8);

        let fp_most = Ipc7351Generator::generate_footprint("SOIC8_MOST", &dims, DensityLevel::Most);
        let fp_nom = Ipc7351Generator::generate_footprint("SOIC8_NOM", &dims, DensityLevel::Nominal);
        let fp_least = Ipc7351Generator::generate_footprint("SOIC8_LEAST", &dims, DensityLevel::Least);

        assert_eq!(fp_most.pads.len(), 8);
        assert_eq!(fp_nom.pads.len(), 8);
        assert_eq!(fp_least.pads.len(), 8);

        // Density level A (Most) pads must be larger than Density level C (Least)
        assert!(fp_most.pads[0].size[0] > fp_least.pads[0].size[0]);
        assert!(fp_most.pads[0].size[1] > fp_least.pads[0].size[1]);
    }

    #[test]
    fn ipc_thermal_pad_window_panning() {
        let dims = PackageDimensions::standard_qfn(32, 5.0, 0.5, Some([3.0, 3.0]));
        let fp = Ipc7351Generator::generate_footprint("QFN32", &dims, DensityLevel::Nominal);

        // Should have 32 peripheral pads + 1 thermal pad
        assert_eq!(fp.pads.len(), 33);
        // Thermal pad > 2.0mm should produce paste aperture matrix
        assert_eq!(fp.paste_apertures.len(), 4);
    }
}
