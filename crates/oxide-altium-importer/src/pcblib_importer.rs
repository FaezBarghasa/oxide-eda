//! Altium Designer Footprint Library (.PcbLib) importer.

use std::collections::HashMap;
use uuid::Uuid;

use oxide_library::primitive::footprint::{
    ComponentType, Footprint, FpGraphic, FpGraphicKind, LayerId, Pad, PadKind, PadShape, Polygon,
};

use crate::cfb::CfbContainer;
use crate::error::AltiumImportError;
use crate::record::{parse_record_stream, AltiumRecord};

/// Import all footprints from an Altium `.PcbLib` file byte slice.
pub fn import_pcblib_bytes(bytes: &[u8]) -> Result<Vec<Footprint>, AltiumImportError> {
    let cfb = CfbContainer::parse(bytes)?;
    let mut footprints = Vec::new();

    // Check FileHeader or specific library sections
    if let Ok(header_data) = cfb.get_stream("FileHeader") {
        let records = parse_record_stream(&header_data)?;
        let parsed = parse_footprints_from_records(&records)?;
        footprints.extend(parsed);
    }

    // Check individual storage streams (often named after the component footprint pattern)
    for (name, stream_bytes) in &cfb.streams {
        if name != "FileHeader" && name != "Library" && !name.starts_with('/') {
            if let Ok(records) = parse_record_stream(stream_bytes) {
                if let Ok(parsed) = parse_footprints_from_records(&records) {
                    for fp in parsed {
                        if !footprints.iter().any(|f| f.name == fp.name) {
                            footprints.push(fp);
                        }
                    }
                }
            }
        }
    }

    Ok(footprints)
}

/// Parse multiple [`Footprint`] primitives from a record slice.
pub fn parse_footprints_from_records(records: &[AltiumRecord]) -> Result<Vec<Footprint>, AltiumImportError> {
    let mut footprints = Vec::new();
    let mut current_footprint: Option<Footprint> = None;

    for rec in records {
        let record_type = rec.get("RECORD").or_else(|| rec.get("OBJECTTYPE")).unwrap_or("");

        if record_type.eq_ignore_ascii_case("Component") || record_type == "Component" || record_type == "1" {
            if let Some(prev) = current_footprint.take() {
                footprints.push(prev);
            }

            let name = rec
                .get("PATTERN")
                .or_else(|| rec.get("NAME"))
                .unwrap_or("ALT_FP")
                .to_string();
            let desc = rec.get("DESCRIPTION").unwrap_or("").to_string();

            current_footprint = Some(Footprint {
                uuid: Uuid::now_v7(),
                name,
                anchor: [0.0, 0.0],
                pads: Vec::new(),
                courtyard: Polygon::default(),
                silk_f: Vec::new(),
                silk_b: Vec::new(),
                fab_f: Vec::new(),
                fab_b: Vec::new(),
                v_scores: Vec::new(),
                mask_openings: Vec::new(),
                mask_excludes: Vec::new(),
                paste_apertures: Vec::new(),
                drills: Vec::new(),
                slots: Vec::new(),
                bodies3d: Vec::new(),
                step_model: None,
                tags: Vec::new(),
                description: desc,
                component_type: ComponentType::Standard,
                color_silkscreen: None,
                color_fab: None,
                color_courtyard: None,
                density_level: None,
                fiducials: Vec::new(),
                testpoints: Vec::new(),
                glue_spots: Vec::new(),
                lead_spans: Vec::new(),
                keepouts: Vec::new(),
                heatsinks: Vec::new(),
                pours: Vec::new(),
                edge_clearance_mm: None,
                assembly_notes: Vec::new(),
            });
        } else if record_type.eq_ignore_ascii_case("Pad") || record_type == "Pad" || record_type == "2" {
            if let Some(fp) = current_footprint.as_mut() {
                let number = rec.get("NAME").or_else(|| rec.get("DESIGNATOR")).unwrap_or("1").to_string();
                let x = rec.get_coord_mm("X").or_else(|| rec.get_coord_mm("LOCATION.X")).unwrap_or(0.0);
                let y = rec.get_coord_mm("Y").or_else(|| rec.get_coord_mm("LOCATION.Y")).unwrap_or(0.0);
                let x_size = rec.get_coord_mm("TOPXSIZE").or_else(|| rec.get_coord_mm("XSIZE")).unwrap_or(1.5);
                let y_size = rec.get_coord_mm("TOPYSIZE").or_else(|| rec.get_coord_mm("YSIZE")).unwrap_or(1.5);
                let rotation = rec.get_f64("ROTATION").unwrap_or(0.0);
                let hole_size = rec.get_coord_mm("HOLESIZE").unwrap_or(0.0);

                let is_tht = hole_size > 0.0;
                let kind = if is_tht { PadKind::Tht } else { PadKind::Smd };

                let shape = match rec.get("TOPSHAPE").or_else(|| rec.get("SHAPE")).unwrap_or("Round") {
                    "Rectangular" | "Rect" => PadShape::Rect,
                    "RoundedRectangle" | "RoundRect" => PadShape::RoundRect { radius_ratio: 0.25 },
                    "Octagonal" => PadShape::Chamfered {
                        chamfer_ratio: 0.25,
                        corners: oxide_library::primitive::footprint::ChamferedCorners::all(),
                    },
                    _ => PadShape::Round,
                };

                let layers = if is_tht {
                    vec![
                        LayerId::new("F.Cu"),
                        LayerId::new("B.Cu"),
                        LayerId::new("F.Mask"),
                        LayerId::new("B.Mask"),
                    ]
                } else {
                    vec![
                        LayerId::new("F.Cu"),
                        LayerId::new("F.Mask"),
                        LayerId::new("F.Paste"),
                    ]
                };

                let drill = if is_tht {
                    Some(oxide_library::primitive::footprint::Drill {
                        diameter: hole_size,
                        slot_length: None,
                    })
                } else {
                    None
                };

                fp.pads.push(Pad {
                    number,
                    kind,
                    shape,
                    size: [x_size, y_size],
                    position: [x, y],
                    rotation,
                    layers,
                    drill,
                    solder_mask_margin: None,
                    paste_margin: None,
                    pad_template: String::new(),
                    pad_stack: None,
                    chamfer: None,
                    top_feature: None,
                    bottom_feature: None,
                    thermal_relief: None,
                    testpoint: None,
                    electrical_type: None,
                    pin_signal_type: None,
                    trace_attachment: None,
                    plated: is_tht,
                    hole_type: None,
                    hole_wall_plating_thickness_um: None,
                    counterbore_top: None,
                    counterbore_bottom: None,
                    countersink_top: None,
                    countersink_bottom: None,
                    edge_connector_bevel: None,
                    side_wetting_flank: None,
                    soldering_technology: None,
                    backdrill: None,
                    thermal_via_pattern: None,
                    force_visual_anchor: false,
                    mask_margin_top: None,
                    mask_margin_bottom: None,
                    paste_margin_top: None,
                    paste_margin_bottom: None,
                    paste_coverage_top: None,
                    paste_coverage_bottom: None,
                    custom_shape_polygon: None,
                });
            }
        } else if record_type.eq_ignore_ascii_case("Track") || record_type == "Track" {
            if let Some(fp) = current_footprint.as_mut() {
                let x1 = rec.get_coord_mm("X1").unwrap_or(0.0);
                let y1 = rec.get_coord_mm("Y1").unwrap_or(0.0);
                let x2 = rec.get_coord_mm("X2").unwrap_or(0.0);
                let y2 = rec.get_coord_mm("Y2").unwrap_or(0.0);
                let width = rec.get_coord_mm("WIDTH").unwrap_or(0.15);

                fp.silk_f.push(FpGraphic {
                    kind: FpGraphicKind::Line {
                        from: [x1, y1],
                        to: [x2, y2],
                    },
                    stroke_width: width,
                    filled: false,
                });
            }
        }
    }

    if let Some(last) = current_footprint {
        footprints.push(last);
    }

    Ok(footprints)
}
