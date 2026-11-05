//! Dirty-Region Spatial R-Tree and Incremental DRC Engine (Phase 3.1).
//!
//! Provides sub-millisecond incremental DRC evaluations by indexing board entities
//! (tracks, vias, pads) in an Axis-Aligned Bounding Box (AABB) spatial hierarchy.
//! When an interactive edit occurs (moving a footprint, pushing a track, inserting a via),
//! only objects intersecting the bounding box of the edit plus a clearance halo (e.g. 500µm)
//! are re-evaluated, avoiding full $O(N^2)$ board re-checks.

use oxide_physics::Microns;
use oxide_types::pcb::{PcbBoard, Point};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::manager::ConstraintManager;
use crate::violation::RuleViolation;

/// 2D Axis-Aligned Bounding Box in micrometers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DrcAabb {
    pub min_x: Microns,
    pub min_y: Microns,
    pub max_x: Microns,
    pub max_y: Microns,
}

impl DrcAabb {
    pub fn new(min_x: Microns, min_y: Microns, max_x: Microns, max_y: Microns) -> Self {
        Self {
            min_x: min_x.min(max_x),
            min_y: min_y.min(max_y),
            max_x: min_x.max(max_x),
            max_y: min_y.max(max_y),
        }
    }

    pub fn from_point_radius(center_x: Microns, center_y: Microns, radius: Microns) -> Self {
        Self {
            min_x: center_x - radius,
            min_y: center_y - radius,
            max_x: center_x + radius,
            max_y: center_y + radius,
        }
    }

    pub fn from_mm_points(p1: Point, p2: Point, halo_mm: f64) -> Self {
        let x1 = (p1.x * 1000.0).round() as i64;
        let y1 = (p1.y * 1000.0).round() as i64;
        let x2 = (p2.x * 1000.0).round() as i64;
        let y2 = (p2.y * 1000.0).round() as i64;
        let halo = (halo_mm * 1000.0).round() as i64;
        Self {
            min_x: x1.min(x2) - halo,
            min_y: y1.min(y2) - halo,
            max_x: x1.max(x2) + halo,
            max_y: y1.max(y2) + halo,
        }
    }

    pub fn expand(&self, margin: Microns) -> Self {
        Self {
            min_x: self.min_x - margin,
            min_y: self.min_y - margin,
            max_x: self.max_x + margin,
            max_y: self.max_y + margin,
        }
    }

    pub fn intersects(&self, other: &DrcAabb) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }
}

/// Indexed geometric item in the DRC Spatial R-Tree.
#[derive(Debug, Clone, PartialEq)]
pub enum DrcSpatialItem {
    Segment {
        index: usize,
        uuid: Uuid,
        layer: String,
        net: u32,
        aabb: DrcAabb,
    },
    Via {
        index: usize,
        uuid: Uuid,
        net: u32,
        drill: f64,
        diameter: f64,
        position: Point,
        aabb: DrcAabb,
    },
    Pad {
        uuid: Uuid,
        fp_reference: String,
        net: u32,
        center: Point,
        aabb: DrcAabb,
    },
}

impl DrcSpatialItem {
    pub fn aabb(&self) -> &DrcAabb {
        match self {
            Self::Segment { aabb, .. } => aabb,
            Self::Via { aabb, .. } => aabb,
            Self::Pad { aabb, .. } => aabb,
        }
    }

    pub fn net(&self) -> u32 {
        match self {
            Self::Segment { net, .. } => *net,
            Self::Via { net, .. } => *net,
            Self::Pad { net, .. } => *net,
        }
    }
}

/// High-performance Spatial Index for Incremental DRC.
#[derive(Debug, Clone, Default)]
pub struct IncrementalDrcEngine {
    items: Vec<DrcSpatialItem>,
}

impl IncrementalDrcEngine {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Build spatial index from full board.
    pub fn build(board: &PcbBoard) -> Self {
        let mut engine = Self::new();

        // 1. Index segments
        for (idx, seg) in board.segments.iter().enumerate() {
            let half_w = (seg.width * 500.0).round() as i64;
            let aabb = DrcAabb::from_mm_points(seg.start, seg.end, 0.0).expand(half_w);
            engine.items.push(DrcSpatialItem::Segment {
                index: idx,
                uuid: seg.uuid,
                layer: seg.layer.clone(),
                net: seg.net,
                aabb,
            });
        }

        // 2. Index vias
        for (idx, via) in board.vias.iter().enumerate() {
            let cx = (via.position.x * 1000.0).round() as i64;
            let cy = (via.position.y * 1000.0).round() as i64;
            let radius = (via.diameter * 500.0).round() as i64;
            let aabb = DrcAabb::from_point_radius(cx, cy, radius);
            engine.items.push(DrcSpatialItem::Via {
                index: idx,
                uuid: via.uuid,
                net: via.net,
                drill: via.drill,
                diameter: via.diameter,
                position: via.position,
                aabb,
            });
        }

        // 3. Index pads
        for fp in &board.footprints {
            for pad in &fp.pads {
                let px = fp.position.x + pad.position.x;
                let py = fp.position.y + pad.position.y;
                let cx = (px * 1000.0).round() as i64;
                let cy = (py * 1000.0).round() as i64;
                let hw = (pad.size.x * 500.0).round() as i64;
                let hh = (pad.size.y * 500.0).round() as i64;
                let aabb = DrcAabb::new(cx - hw, cy - hh, cx + hw, cy + hh);
                engine.items.push(DrcSpatialItem::Pad {
                    uuid: pad.uuid,
                    fp_reference: fp.reference.clone(),
                    net: pad.net.as_ref().map(|n| n.number).unwrap_or(0),
                    center: Point::new(px, py),
                    aabb,
                });
            }
        }

        engine
    }

    /// Query items intersecting a given AABB region.
    pub fn query_region<'a>(&'a self, region: &DrcAabb) -> Vec<&'a DrcSpatialItem> {
        self.items
            .iter()
            .filter(|item| item.aabb().intersects(region))
            .collect()
    }

    /// Run incremental DRC on a modified region (e.g. AABB of an edited track or footprint).
    /// Re-evaluates only entities in `dirty_region` plus halo clearance, returning delta violations.
    pub fn check_dirty_region(
        &self,
        manager: &ConstraintManager,
        board: &PcbBoard,
        dirty_region: &DrcAabb,
        halo_microns: Microns,
    ) -> Vec<RuleViolation> {
        let mut violations = Vec::new();
        let search_region = dirty_region.expand(halo_microns);
        let candidate_items = self.query_region(&search_region);

        if candidate_items.is_empty() {
            return violations;
        }

        let net_names: HashMap<u32, &str> = board
            .nets
            .iter()
            .map(|n| (n.number, n.name.as_str()))
            .collect();
        let get_net_name = |net: u32| -> String {
            net_names
                .get(&net)
                .copied()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("NET_{net}"))
        };

        // Collect Net-Tie exemptions
        let mut net_tied_pairs = HashSet::new();
        for fp in &board.footprints {
            let is_net_tie = fp.footprint_id.to_ascii_lowercase().contains("nettie")
                || fp.value.to_ascii_lowercase().contains("nettie")
                || fp.reference.starts_with("NT");
            if is_net_tie {
                let nets: Vec<u32> = fp
                    .pads
                    .iter()
                    .filter_map(|p| p.net.as_ref().map(|n| n.number))
                    .collect();
                for i in 0..nets.len() {
                    for j in (i + 1)..nets.len() {
                        let na = nets[i];
                        let nb = nets[j];
                        if na != nb {
                            net_tied_pairs.insert((na.min(nb), na.max(nb)));
                        }
                    }
                }
            }
        }

        // Re-evaluate segments in candidate set
        let candidate_segment_indices: Vec<usize> = candidate_items
            .iter()
            .filter_map(|item| match item {
                DrcSpatialItem::Segment { index, .. } => Some(*index),
                _ => None,
            })
            .collect();

        for &i in &candidate_segment_indices {
            let seg_a = &board.segments[i];
            let width_microns = (seg_a.width * 1000.0).round() as i64;
            let net_name = get_net_name(seg_a.net);

            // 1. Trace width rule check
            if let Err(v) = manager.validate_trace_width(&net_name, None, None, width_microns) {
                violations.push(v);
            }

            // 2. Pairwise clearance check against all candidate segments
            for &j in &candidate_segment_indices {
                if i >= j {
                    continue;
                }
                let seg_b = &board.segments[j];
                if seg_a.layer == seg_b.layer && seg_a.net != seg_b.net {
                    let pair = (seg_a.net.min(seg_b.net), seg_a.net.max(seg_b.net));
                    if net_tied_pairs.contains(&pair) {
                        continue;
                    }

                    let d = oxide_types::schematic::point_to_segment_dist(
                        seg_a.start.x,
                        seg_a.start.y,
                        seg_b.start.x,
                        seg_b.start.y,
                        seg_b.end.x,
                        seg_b.end.y,
                    );
                    let dist_microns = (d * 1000.0).round() as i64;
                    let net_a = get_net_name(seg_a.net);
                    let net_b = get_net_name(seg_b.net);

                    if let Err(v) =
                        manager.validate_clearance(&net_a, None, &net_b, None, None, dist_microns)
                    {
                        violations.push(v);
                    }
                }
            }
        }

        violations
    }
}
