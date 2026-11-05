//! Smart Dynamic Copper Polygonal Pour & Teardrop Generation Engine.
//!
//! Handles prioritized copper zone clipping, thermal relief spoke calculations (ortho/diagonal),
//! minimum area island removal, and automated curvilinear teardrop fillet generation on tracks/vias/pads.

use crate::geometry::Point2D;
use crate::geometry::rtree::NetId;
use crate::{LayerId, RouteSegment, SegmentType, ViaPlacement};
use oxide_physics::Microns;
use oxide_types::pcb::{PcbBoard, Point, Segment};
use serde::{Deserialize, Serialize};

/// Thermal relief connection style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThermalReliefStyle {
    #[default]
    FourSpokeOrtho, // 4 spokes at 0°, 90°, 180°, 270°
    FourSpokeDiagonal, // 4 spokes at 45°, 135°, 225°, 315°
    TwoSpoke,          // 2 spokes
    DirectConnect,     // Solid copper fill directly touching pad
    NoConnect,         // Anti-pad isolation gap without connection
}

/// Dynamic copper zone definition with priority and thermal relief configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopperZoneConfig {
    pub net_id: NetId,
    pub layer: LayerId,
    pub priority: u32,
    pub clearance: Microns,
    pub min_island_area_sq_microns: i64,
    pub thermal_relief: ThermalReliefStyle,
    pub thermal_spoke_width: Microns,
    pub thermal_gap: Microns,
}

impl Default for CopperZoneConfig {
    fn default() -> Self {
        Self {
            net_id: 0,
            layer: 0,
            priority: 1,
            clearance: 200,                      // 200 µm clearance (~8 mil)
            min_island_area_sq_microns: 500_000, // 0.5 mm² minimum island area
            thermal_relief: ThermalReliefStyle::FourSpokeOrtho,
            thermal_spoke_width: 250, // 250 µm spoke width (~10 mil)
            thermal_gap: 300,         // 300 µm thermal relief gap
        }
    }
}

/// Spoke segment in a thermal relief connection.
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalSpoke {
    pub start: Point2D,
    pub end: Point2D,
    pub width: Microns,
}

/// Smart copper pour clipping and thermal generator.
pub struct CopperPourEngine;

impl CopperPourEngine {
    /// Calculate thermal relief spokes around a pad center.
    pub fn generate_thermal_spokes(
        center: Point2D,
        pad_radius: Microns,
        config: &CopperZoneConfig,
    ) -> Vec<ThermalSpoke> {
        let mut spokes = Vec::new();
        if config.thermal_relief == ThermalReliefStyle::DirectConnect
            || config.thermal_relief == ThermalReliefStyle::NoConnect
        {
            return spokes;
        }

        let outer_r = pad_radius + config.thermal_gap;
        let spoke_w = config.thermal_spoke_width;

        let angles_deg: Vec<f64> = match config.thermal_relief {
            ThermalReliefStyle::FourSpokeOrtho => vec![0.0, 90.0, 180.0, 270.0],
            ThermalReliefStyle::FourSpokeDiagonal => vec![45.0, 135.0, 225.0, 315.0],
            ThermalReliefStyle::TwoSpoke => vec![0.0, 180.0],
            _ => vec![],
        };

        for angle in angles_deg {
            let rad = angle.to_radians();
            let cos = rad.cos();
            let sin = rad.sin();

            let start = Point2D::new(
                center.x + (pad_radius as f64 * cos).round() as i64,
                center.y + (pad_radius as f64 * sin).round() as i64,
            );
            let end = Point2D::new(
                center.x + (outer_r as f64 * cos).round() as i64,
                center.y + (outer_r as f64 * sin).round() as i64,
            );

            spokes.push(ThermalSpoke {
                start,
                end,
                width: spoke_w,
            });
        }

        spokes
    }

    /// Check if a detected polygon island meets the minimum area requirement to remain.
    pub fn is_valid_island(polygon: &[Point2D], min_area_sq_microns: i64) -> bool {
        if polygon.len() < 3 {
            return false;
        }

        // Shoelace formula for polygon area
        let mut area_twice: i64 = 0;
        for i in 0..polygon.len() {
            let j = (i + 1) % polygon.len();
            area_twice += polygon[i].x * polygon[j].y - polygon[j].x * polygon[i].y;
        }

        let area = (area_twice.abs()) / 2;
        area >= min_area_sq_microns
    }
}

/// Automated teardrop fillet generator.
pub struct TeardropGenerator;

impl TeardropGenerator {
    /// Generate a curvilinear/straight teardrop fillet between a track and a circular pad/via.
    pub fn generate_teardrop_for_pad(
        track_start: Point2D,
        pad_center: Point2D,
        track_width: Microns,
        pad_radius: Microns,
        net_id: NetId,
        layer: LayerId,
    ) -> Option<Vec<RouteSegment>> {
        let dist = track_start.distance_to(pad_center);
        if dist < pad_radius {
            return None;
        }

        let (dx, dy) = track_start.direction_to(pad_center);
        let perp_x = -dy;
        let perp_y = dx;

        // Teardrop length is typically 100% of pad radius extending along track
        let fillet_len = pad_radius;
        let fillet_anchor = Point2D::new(
            pad_center.x - (dx * (pad_radius + fillet_len) as f64).round() as i64,
            pad_center.y - (dy * (pad_radius + fillet_len) as f64).round() as i64,
        );

        let pad_wing_r = (pad_radius as f64 * 0.8).round() as i64;
        let wing1 = Point2D::new(
            pad_center.x + (perp_x * pad_wing_r as f64).round() as i64,
            pad_center.y + (perp_y * pad_wing_r as f64).round() as i64,
        );
        let wing2 = Point2D::new(
            pad_center.x - (perp_x * pad_wing_r as f64).round() as i64,
            pad_center.y - (perp_y * pad_wing_r as f64).round() as i64,
        );

        Some(vec![
            RouteSegment {
                start_point: fillet_anchor,
                end_point: wing1,
                width: track_width / 2,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            },
            RouteSegment {
                start_point: fillet_anchor,
                end_point: wing2,
                width: track_width / 2,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            },
        ])
    }

    /// Add teardrops across all vias in a routed path.
    pub fn apply_teardrops_to_path(
        segments: &[RouteSegment],
        vias: &[ViaPlacement],
    ) -> Vec<RouteSegment> {
        let mut teardrops = Vec::new();

        for via in vias {
            let via_r = via.pad_diameter / 2;
            for seg in segments {
                if seg.net_id == via.net_id && seg.layer == via.start_layer {
                    if seg.end_point.distance_to(via.position) <= via_r
                        && let Some(td) = Self::generate_teardrop_for_pad(
                            seg.start_point,
                            via.position,
                            seg.width,
                            via_r,
                            seg.net_id,
                            seg.layer,
                        )
                    {
                        teardrops.extend(td);
                    } else if seg.start_point.distance_to(via.position) <= via_r
                        && let Some(td) = Self::generate_teardrop_for_pad(
                            seg.end_point,
                            via.position,
                            seg.width,
                            via_r,
                            seg.net_id,
                            seg.layer,
                        )
                    {
                        teardrops.extend(td);
                    }
                }
            }
        }

        teardrops
    }

    /// Generate and apply teardrops to an entire PCB board pre-CAM.
    ///
    /// Identifies all track segments terminating at circular pads or vias,
    /// and generates matching-net teardrop fillet segments on the same copper layer.
    pub fn apply_teardrops_to_board(board: &mut PcbBoard) -> usize {
        let mut new_segments = Vec::new();

        // 1. Gather all via targets: (center, radius_mm, net_id, layers)
        let via_targets: Vec<(Point, f64, u32, Vec<String>)> = board
            .vias
            .iter()
            .map(|v| (v.position, v.diameter / 2.0, v.net, v.layers.clone()))
            .collect();

        // 2. Gather all pad targets: (center, radius_mm, net_id, layers)
        let mut pad_targets: Vec<(Point, f64, u32, Vec<String>)> = Vec::new();
        for fp in &board.footprints {
            for pad in &fp.pads {
                let rot_rad = fp.rotation.to_radians();
                let cos_r = rot_rad.cos();
                let sin_r = rot_rad.sin();
                let px = fp.position.x + pad.position.x * cos_r - pad.position.y * sin_r;
                let py = fp.position.y + pad.position.x * sin_r + pad.position.y * cos_r;
                let radius = pad.size.x.max(pad.size.y) / 2.0;
                let net = pad.net.as_ref().map(|n| n.number).unwrap_or(0);
                pad_targets.push((Point::new(px, py), radius, net, pad.layers.clone()));
            }
        }

        // 3. For each existing segment, check ends against vias and pads
        for seg in &board.segments {
            if seg.width <= 0.0 {
                continue;
            }
            let seg_len =
                ((seg.end.x - seg.start.x).powi(2) + (seg.end.y - seg.start.y).powi(2)).sqrt();
            if seg_len < 0.05 {
                continue;
            }

            // Check against vias
            for (via_pos, via_r, via_net, via_layers) in &via_targets {
                if *via_net != seg.net {
                    continue;
                }
                let layer_matches = via_layers.is_empty()
                    || via_layers.iter().any(|l| {
                        l.eq_ignore_ascii_case(&seg.layer) || l.eq_ignore_ascii_case("Multi-Layer")
                    });
                if !layer_matches {
                    continue;
                }

                let d_start =
                    ((seg.start.x - via_pos.x).powi(2) + (seg.start.y - via_pos.y).powi(2)).sqrt();
                let d_end =
                    ((seg.end.x - via_pos.x).powi(2) + (seg.end.y - via_pos.y).powi(2)).sqrt();

                if d_start <= *via_r * 1.2 && seg_len > *via_r {
                    let dir_x = (seg.end.x - seg.start.x) / seg_len;
                    let dir_y = (seg.end.y - seg.start.y) / seg_len;
                    let perp_x = -dir_y;
                    let perp_y = dir_x;
                    let anchor = Point::new(
                        via_pos.x + dir_x * (*via_r * 1.5),
                        via_pos.y + dir_y * (*via_r * 1.5),
                    );
                    let wing_r = *via_r * 0.8;
                    let w1 = Point::new(via_pos.x + perp_x * wing_r, via_pos.y + perp_y * wing_r);
                    let w2 = Point::new(via_pos.x - perp_x * wing_r, via_pos.y - perp_y * wing_r);
                    let td_width = seg.width.max(0.15);
                    new_segments.push(Segment {
                        uuid: uuid::Uuid::now_v7(),
                        start: anchor,
                        end: w1,
                        width: td_width,
                        layer: seg.layer.clone(),
                        net: seg.net,
                    });
                    new_segments.push(Segment {
                        uuid: uuid::Uuid::now_v7(),
                        start: anchor,
                        end: w2,
                        width: td_width,
                        layer: seg.layer.clone(),
                        net: seg.net,
                    });
                } else if d_end <= *via_r * 1.2 && seg_len > *via_r {
                    let dir_x = (seg.start.x - seg.end.x) / seg_len;
                    let dir_y = (seg.start.y - seg.end.y) / seg_len;
                    let perp_x = -dir_y;
                    let perp_y = dir_x;
                    let anchor = Point::new(
                        via_pos.x + dir_x * (*via_r * 1.5),
                        via_pos.y + dir_y * (*via_r * 1.5),
                    );
                    let wing_r = *via_r * 0.8;
                    let w1 = Point::new(via_pos.x + perp_x * wing_r, via_pos.y + perp_y * wing_r);
                    let w2 = Point::new(via_pos.x - perp_x * wing_r, via_pos.y - perp_y * wing_r);
                    let td_width = seg.width.max(0.15);
                    new_segments.push(Segment {
                        uuid: uuid::Uuid::now_v7(),
                        start: anchor,
                        end: w1,
                        width: td_width,
                        layer: seg.layer.clone(),
                        net: seg.net,
                    });
                    new_segments.push(Segment {
                        uuid: uuid::Uuid::now_v7(),
                        start: anchor,
                        end: w2,
                        width: td_width,
                        layer: seg.layer.clone(),
                        net: seg.net,
                    });
                }
            }

            // Check against pads
            for (pad_pos, pad_r, pad_net, pad_layers) in &pad_targets {
                if *pad_net != 0 && *pad_net != seg.net {
                    continue;
                }
                let layer_matches = pad_layers.is_empty()
                    || pad_layers.iter().any(|l| {
                        l.eq_ignore_ascii_case(&seg.layer) || l.eq_ignore_ascii_case("Multi-Layer")
                    });
                if !layer_matches {
                    continue;
                }

                let d_start =
                    ((seg.start.x - pad_pos.x).powi(2) + (seg.start.y - pad_pos.y).powi(2)).sqrt();
                let d_end =
                    ((seg.end.x - pad_pos.x).powi(2) + (seg.end.y - pad_pos.y).powi(2)).sqrt();

                if d_start <= *pad_r * 1.2 && seg_len > *pad_r {
                    let dir_x = (seg.end.x - seg.start.x) / seg_len;
                    let dir_y = (seg.end.y - seg.start.y) / seg_len;
                    let perp_x = -dir_y;
                    let perp_y = dir_x;
                    let anchor = Point::new(
                        pad_pos.x + dir_x * (*pad_r * 1.5),
                        pad_pos.y + dir_y * (*pad_r * 1.5),
                    );
                    let wing_r = *pad_r * 0.8;
                    let w1 = Point::new(pad_pos.x + perp_x * wing_r, pad_pos.y + perp_y * wing_r);
                    let w2 = Point::new(pad_pos.x - perp_x * wing_r, pad_pos.y - perp_y * wing_r);
                    let td_width = seg.width.max(0.15);
                    new_segments.push(Segment {
                        uuid: uuid::Uuid::now_v7(),
                        start: anchor,
                        end: w1,
                        width: td_width,
                        layer: seg.layer.clone(),
                        net: seg.net,
                    });
                    new_segments.push(Segment {
                        uuid: uuid::Uuid::now_v7(),
                        start: anchor,
                        end: w2,
                        width: td_width,
                        layer: seg.layer.clone(),
                        net: seg.net,
                    });
                } else if d_end <= *pad_r * 1.2 && seg_len > *pad_r {
                    let dir_x = (seg.start.x - seg.end.x) / seg_len;
                    let dir_y = (seg.start.y - seg.end.y) / seg_len;
                    let perp_x = -dir_y;
                    let perp_y = dir_x;
                    let anchor = Point::new(
                        pad_pos.x + dir_x * (*pad_r * 1.5),
                        pad_pos.y + dir_y * (*pad_r * 1.5),
                    );
                    let wing_r = *pad_r * 0.8;
                    let w1 = Point::new(pad_pos.x + perp_x * wing_r, pad_pos.y + perp_y * wing_r);
                    let w2 = Point::new(pad_pos.x - perp_x * wing_r, pad_pos.y - perp_y * wing_r);
                    let td_width = seg.width.max(0.15);
                    new_segments.push(Segment {
                        uuid: uuid::Uuid::now_v7(),
                        start: anchor,
                        end: w1,
                        width: td_width,
                        layer: seg.layer.clone(),
                        net: seg.net,
                    });
                    new_segments.push(Segment {
                        uuid: uuid::Uuid::now_v7(),
                        start: anchor,
                        end: w2,
                        width: td_width,
                        layer: seg.layer.clone(),
                        net: seg.net,
                    });
                }
            }
        }

        let count = new_segments.len();
        board.segments.extend(new_segments);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_relief_spokes() {
        let center = Point2D::new(1000, 1000);
        let pad_r = 500; // 500µm radius (1mm pad)
        let config = CopperZoneConfig {
            thermal_relief: ThermalReliefStyle::FourSpokeOrtho,
            thermal_gap: 300,
            thermal_spoke_width: 250,
            ..Default::default()
        };

        let spokes = CopperPourEngine::generate_thermal_spokes(center, pad_r, &config);
        assert_eq!(spokes.len(), 4);
        assert_eq!(spokes[0].width, 250);
        // East spoke (0°) start at x=1500, end at x=1800
        assert_eq!(spokes[0].start, Point2D::new(1500, 1000));
        assert_eq!(spokes[0].end, Point2D::new(1800, 1000));
    }

    #[test]
    fn test_island_area_removal() {
        let small_square = vec![
            Point2D::new(0, 0),
            Point2D::new(100, 0),
            Point2D::new(100, 100),
            Point2D::new(0, 100),
        ]; // Area = 10,000 µm²

        let large_square = vec![
            Point2D::new(0, 0),
            Point2D::new(1000, 0),
            Point2D::new(1000, 1000),
            Point2D::new(0, 1000),
        ]; // Area = 1,000,000 µm²

        assert!(!CopperPourEngine::is_valid_island(&small_square, 500_000));
        assert!(CopperPourEngine::is_valid_island(&large_square, 500_000));
    }

    #[test]
    fn test_board_level_teardrop_application() {
        use oxide_types::pcb::{PcbBoard, Point, Segment, Via};
        use uuid::Uuid;

        let mut board = PcbBoard::default();

        // Add a via at (10.0, 10.0) with diameter 0.6mm (radius 0.3mm)
        board.vias.push(Via {
            uuid: Uuid::now_v7(),
            position: Point::new(10.0, 10.0),
            diameter: 0.6,
            drill: 0.3,
            layers: vec!["Top Layer".to_string(), "Bottom Layer".to_string()],
            net: 1,
            via_type: oxide_types::pcb::ViaType::Through,
            via_span: None,
        });

        // Add a segment connecting to the via from (5.0, 10.0) to (10.0, 10.0)
        board.segments.push(Segment {
            uuid: Uuid::now_v7(),
            start: Point::new(5.0, 10.0),
            end: Point::new(10.0, 10.0),
            width: 0.25,
            layer: "Top Layer".to_string(),
            net: 1,
        });

        assert_eq!(board.segments.len(), 1);
        let added = TeardropGenerator::apply_teardrops_to_board(&mut board);
        assert_eq!(added, 2, "Expected 2 teardrop fillet wing segments");
        assert_eq!(board.segments.len(), 3);

        // Teardrop segments must inherit net and layer
        assert_eq!(board.segments[1].net, 1);
        assert_eq!(board.segments[1].layer, "Top Layer");
        assert_eq!(board.segments[2].net, 1);
        assert_eq!(board.segments[2].layer, "Top Layer");
    }

    #[test]
    fn test_teardrop_generation() {
        let track_start = Point2D::new(0, 1000);
        let pad_center = Point2D::new(1000, 1000);
        let td =
            TeardropGenerator::generate_teardrop_for_pad(track_start, pad_center, 200, 300, 1, 0);

        assert!(td.is_some());
        let segments = td.unwrap();
        assert_eq!(segments.len(), 2);
    }
}
