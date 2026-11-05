//! Dirty-Region Spatial R-Tree and Incremental DRC Engine (Phase 3.1 & 3.2).
//!
//! Provides sub-millisecond incremental DRC evaluations by indexing board entities
//! (tracks, vias, pads) in an Axis-Aligned Bounding Box (AABB) spatial hierarchy.
//! When an interactive edit occurs (moving a footprint, pushing a track, inserting a via),
//! only objects intersecting the bounding box of the edit plus a clearance halo (e.g. 500µm)
//! are re-evaluated, avoiding full $O(N^2)$ board re-checks.

use oxide_physics::Microns;
use oxide_types::pcb::{PcbBoard, Point};
use rayon::prelude::*;
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

/// High-performance Spatial Hash Index for Incremental & Parallel DRC.
#[derive(Debug, Clone)]
pub struct IncrementalDrcEngine {
    items: Vec<DrcSpatialItem>,
    cell_size: i64,
    grid: HashMap<(i32, i32), Vec<usize>>,
}

impl Default for IncrementalDrcEngine {
    fn default() -> Self {
        Self::new(2000)
    }
}

impl IncrementalDrcEngine {
    pub fn new(cell_size_microns: i64) -> Self {
        Self {
            items: Vec::new(),
            cell_size: cell_size_microns.max(500),
            grid: HashMap::new(),
        }
    }

    /// Build spatial index from full board.
    pub fn build(board: &PcbBoard) -> Self {
        let mut engine = Self::new(2000); // 2mm spatial cells

        // 1. Index segments
        for (idx, seg) in board.segments.iter().enumerate() {
            let half_w = (seg.width * 500.0).round() as i64;
            let aabb = DrcAabb::from_mm_points(seg.start, seg.end, 0.0).expand(half_w);
            let item_idx = engine.items.len();
            engine.items.push(DrcSpatialItem::Segment {
                index: idx,
                uuid: seg.uuid,
                layer: seg.layer.clone(),
                net: seg.net,
                aabb,
            });
            engine.insert_into_grid(item_idx, &aabb);
        }

        // 2. Index vias
        for (idx, via) in board.vias.iter().enumerate() {
            let cx = (via.position.x * 1000.0).round() as i64;
            let cy = (via.position.y * 1000.0).round() as i64;
            let radius = (via.diameter * 500.0).round() as i64;
            let aabb = DrcAabb::from_point_radius(cx, cy, radius);
            let item_idx = engine.items.len();
            engine.items.push(DrcSpatialItem::Via {
                index: idx,
                uuid: via.uuid,
                net: via.net,
                drill: via.drill,
                diameter: via.diameter,
                position: via.position,
                aabb,
            });
            engine.insert_into_grid(item_idx, &aabb);
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
                let item_idx = engine.items.len();
                engine.items.push(DrcSpatialItem::Pad {
                    uuid: pad.uuid,
                    fp_reference: fp.reference.clone(),
                    net: pad.net.as_ref().map(|n| n.number).unwrap_or(0),
                    center: Point::new(px, py),
                    aabb,
                });
                engine.insert_into_grid(item_idx, &aabb);
            }
        }

        engine
    }

    fn insert_into_grid(&mut self, item_idx: usize, aabb: &DrcAabb) {
        let min_cx = (aabb.min_x / self.cell_size) as i32;
        let max_cx = (aabb.max_x / self.cell_size) as i32;
        let min_cy = (aabb.min_y / self.cell_size) as i32;
        let max_cy = (aabb.max_y / self.cell_size) as i32;

        for cx in min_cx..=max_cx {
            for cy in min_cy..=max_cy {
                self.grid.entry((cx, cy)).or_default().push(item_idx);
            }
        }
    }

    /// Query items intersecting a given AABB region using spatial grid cells.
    pub fn query_region<'a>(&'a self, region: &DrcAabb) -> Vec<&'a DrcSpatialItem> {
        let min_cx = (region.min_x / self.cell_size) as i32;
        let max_cx = (region.max_x / self.cell_size) as i32;
        let min_cy = (region.min_y / self.cell_size) as i32;
        let max_cy = (region.max_y / self.cell_size) as i32;

        let mut candidate_indices = HashSet::new();
        for cx in min_cx..=max_cx {
            for cy in min_cy..=max_cy {
                if let Some(list) = self.grid.get(&(cx, cy)) {
                    for &idx in list {
                        candidate_indices.insert(idx);
                    }
                }
            }
        }

        candidate_indices
            .into_iter()
            .map(|idx| &self.items[idx])
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
                || fp.reference.starts_with("NT")
                || fp.properties.iter().any(|p| {
                    (p.key.eq_ignore_ascii_case("componentkind")
                        && p.value.eq_ignore_ascii_case("nettie"))
                        || (p.key.eq_ignore_ascii_case("nettie")
                            && (p.value.eq_ignore_ascii_case("true") || p.value == "1"))
                });
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

        // 1. Re-evaluate segments in dirty region
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

            // Trace width rule check (only if segment intersects dirty region directly)
            let half_w = (seg_a.width * 500.0).round() as i64;
            let seg_aabb = DrcAabb::from_mm_points(seg_a.start, seg_a.end, 0.0).expand(half_w);
            if seg_aabb.intersects(dirty_region)
                && let Err(v) = manager.validate_trace_width(&net_name, None, None, width_microns)
            {
                violations.push(v);
            }

            // Pairwise clearance check against candidate segments
            for &j in &candidate_segment_indices {
                if i >= j {
                    continue;
                }
                let seg_b = &board.segments[j];
                let b_half_w = (seg_b.width * 500.0).round() as i64;
                let b_aabb = DrcAabb::from_mm_points(seg_b.start, seg_b.end, 0.0).expand(b_half_w);

                // At least one of the pair must intersect dirty region
                if !seg_aabb.intersects(dirty_region) && !b_aabb.intersects(dirty_region) {
                    continue;
                }

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

        // 2. Re-evaluate vias in candidate set
        let candidate_via_indices: Vec<usize> = candidate_items
            .iter()
            .filter_map(|item| match item {
                DrcSpatialItem::Via { index, .. } => Some(*index),
                _ => None,
            })
            .collect();

        for &i in &candidate_via_indices {
            let via = &board.vias[i];
            let cx = (via.position.x * 1000.0).round() as i64;
            let cy = (via.position.y * 1000.0).round() as i64;
            let radius = (via.diameter * 500.0).round() as i64;
            let via_aabb = DrcAabb::from_point_radius(cx, cy, radius);

            if via_aabb.intersects(dirty_region) {
                let drill_microns = (via.drill * 1000.0).round() as i64;
                let pad_microns = (via.diameter * 1000.0).round() as i64;
                let net_name = get_net_name(via.net);
                let obj_id = format!("Via_{}", via.uuid);

                if let Some(rule) = manager.resolve_via_style_rule(&net_name, None, None) {
                    if drill_microns < rule.min_drill {
                        violations.push(RuleViolation {
                            violation_type: crate::violation::RuleViolationType::ViaDrillTooSmall,
                            message: format!(
                                "Via drill violation on '{obj_id}' (net '{net_name}'): drill is {:.3}mm ({}µm), rule requires minimum {:.3}mm ({}µm)",
                                via.drill, drill_microns, rule.min_drill as f64 / 1000.0, rule.min_drill
                            ),
                            scope: rule.scope.clone(),
                            required_value: format!("{:.3}mm", rule.min_drill as f64 / 1000.0),
                            actual_value: format!("{:.3}mm", via.drill),
                            location: Some((via.position.x, via.position.y)),
                            object_ids: vec![obj_id.clone()],
                        });
                    }
                    if pad_microns < rule.min_diameter {
                        violations.push(RuleViolation {
                            violation_type: crate::violation::RuleViolationType::ViaDiameterTooSmall,
                            message: format!(
                                "Via diameter violation on '{obj_id}' (net '{net_name}'): diameter is {:.3}mm ({}µm), rule requires minimum {:.3}mm ({}µm)",
                                via.diameter, pad_microns, rule.min_diameter as f64 / 1000.0, rule.min_diameter
                            ),
                            scope: rule.scope.clone(),
                            required_value: format!("{:.3}mm", rule.min_diameter as f64 / 1000.0),
                            actual_value: format!("{:.3}mm", via.diameter),
                            location: Some((via.position.x, via.position.y)),
                            object_ids: vec![obj_id.clone()],
                        });
                    }
                }

                if let Err(v) = manager.validate_annular_ring(
                    &obj_id,
                    &net_name,
                    None,
                    None,
                    pad_microns,
                    drill_microns,
                ) {
                    violations.push(v);
                }
            }

            // Hole-to-hole checks against neighbor candidate vias
            for &j in &candidate_via_indices {
                if i >= j {
                    continue;
                }
                let via_b = &board.vias[j];
                let b_cx = (via_b.position.x * 1000.0).round() as i64;
                let b_cy = (via_b.position.y * 1000.0).round() as i64;
                let b_rad = (via_b.diameter * 500.0).round() as i64;
                let b_aabb = DrcAabb::from_point_radius(b_cx, b_cy, b_rad);

                if !via_aabb.intersects(dirty_region) && !b_aabb.intersects(dirty_region) {
                    continue;
                }

                let center_dist = ((via.position.x - via_b.position.x).powi(2)
                    + (via.position.y - via_b.position.y).powi(2))
                .sqrt();
                let edge_dist = center_dist - (via.drill / 2.0) - (via_b.drill / 2.0);
                let edge_microns = (edge_dist.max(0.0) * 1000.0).round() as i64;

                let id_a = format!("Via_{}", via.uuid);
                let id_b = format!("Via_{}", via_b.uuid);
                let net_a = get_net_name(via.net);
                let net_b = get_net_name(via_b.net);

                if let Err(v) =
                    manager.validate_hole_to_hole(&id_a, &net_a, &id_b, &net_b, None, edge_microns)
                {
                    violations.push(v);
                }
            }
        }

        violations
    }

    /// Full-board DRC accelerated by Rayon parallelization (Phase 3.2).
    pub fn run_drc_parallel(
        &self,
        manager: &ConstraintManager,
        board: &PcbBoard,
    ) -> Vec<RuleViolation> {
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

        // Collect Net-Tie exempted pairs
        let mut net_tied_pairs = HashSet::new();
        for fp in &board.footprints {
            let is_net_tie = fp.footprint_id.to_ascii_lowercase().contains("nettie")
                || fp.value.to_ascii_lowercase().contains("nettie")
                || fp.reference.starts_with("NT")
                || fp.properties.iter().any(|p| {
                    (p.key.eq_ignore_ascii_case("componentkind")
                        && p.value.eq_ignore_ascii_case("nettie"))
                        || (p.key.eq_ignore_ascii_case("nettie")
                            && (p.value.eq_ignore_ascii_case("true") || p.value == "1"))
                });
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

        // 1. Parallel Trace Width Checks
        let width_violations: Vec<RuleViolation> = board
            .segments
            .par_iter()
            .filter_map(|seg| {
                let width_microns = (seg.width * 1000.0).round() as i64;
                let net_name = get_net_name(seg.net);
                manager
                    .validate_trace_width(&net_name, None, None, width_microns)
                    .err()
            })
            .collect();

        // 2. Parallel Via Style & Annular Ring Checks
        let via_violations: Vec<RuleViolation> = board
            .vias
            .par_iter()
            .flat_map(|via| {
                let mut list = Vec::new();
                let drill_microns = (via.drill * 1000.0).round() as i64;
                let pad_microns = (via.diameter * 1000.0).round() as i64;
                let net_name = get_net_name(via.net);
                let obj_id = format!("Via_{}", via.uuid);

                if let Some(rule) = manager.resolve_via_style_rule(&net_name, None, None) {
                    if drill_microns < rule.min_drill {
                        list.push(RuleViolation {
                            violation_type: crate::violation::RuleViolationType::ViaDrillTooSmall,
                            message: format!(
                                "Via drill violation on '{obj_id}' (net '{net_name}'): drill is {:.3}mm ({}µm), rule requires minimum {:.3}mm ({}µm)",
                                via.drill, drill_microns, rule.min_drill as f64 / 1000.0, rule.min_drill
                            ),
                            scope: rule.scope.clone(),
                            required_value: format!("{:.3}mm", rule.min_drill as f64 / 1000.0),
                            actual_value: format!("{:.3}mm", via.drill),
                            location: Some((via.position.x, via.position.y)),
                            object_ids: vec![obj_id.clone()],
                        });
                    }
                    if pad_microns < rule.min_diameter {
                        list.push(RuleViolation {
                            violation_type: crate::violation::RuleViolationType::ViaDiameterTooSmall,
                            message: format!(
                                "Via diameter violation on '{obj_id}' (net '{net_name}'): diameter is {:.3}mm ({}µm), rule requires minimum {:.3}mm ({}µm)",
                                via.diameter, pad_microns, rule.min_diameter as f64 / 1000.0, rule.min_diameter
                            ),
                            scope: rule.scope.clone(),
                            required_value: format!("{:.3}mm", rule.min_diameter as f64 / 1000.0),
                            actual_value: format!("{:.3}mm", via.diameter),
                            location: Some((via.position.x, via.position.y)),
                            object_ids: vec![obj_id.clone()],
                        });
                    }
                }

                if let Err(v) = manager.validate_annular_ring(
                    &obj_id,
                    &net_name,
                    None,
                    None,
                    pad_microns,
                    drill_microns,
                ) {
                    list.push(v);
                }
                list
            })
            .collect();

        // 3. Parallel Clearance Validations across segments
        let clearance_violations: Vec<RuleViolation> = (0..board.segments.len())
            .into_par_iter()
            .flat_map(|i| {
                let mut list = Vec::new();
                let seg_a = &board.segments[i];
                for j in (i + 1)..board.segments.len() {
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

                        if let Err(v) = manager.validate_clearance(
                            &net_a,
                            None,
                            &net_b,
                            None,
                            None,
                            dist_microns,
                        ) {
                            list.push(v);
                        }
                    }
                }
                list
            })
            .collect();

        // 4. Parallel Hole-to-Hole Validations
        let hole_violations: Vec<RuleViolation> = (0..board.vias.len())
            .into_par_iter()
            .flat_map(|i| {
                let mut list = Vec::new();
                let via_a = &board.vias[i];
                for j in (i + 1)..board.vias.len() {
                    let via_b = &board.vias[j];
                    let center_dist = ((via_a.position.x - via_b.position.x).powi(2)
                        + (via_a.position.y - via_b.position.y).powi(2))
                    .sqrt();
                    let edge_dist = center_dist - (via_a.drill / 2.0) - (via_b.drill / 2.0);
                    let edge_microns = (edge_dist.max(0.0) * 1000.0).round() as i64;

                    let id_a = format!("Via_{}", via_a.uuid);
                    let id_b = format!("Via_{}", via_b.uuid);
                    let net_a = get_net_name(via_a.net);
                    let net_b = get_net_name(via_b.net);

                    if let Err(v) = manager.validate_hole_to_hole(
                        &id_a,
                        &net_a,
                        &id_b,
                        &net_b,
                        None,
                        edge_microns,
                    ) {
                        list.push(v);
                    }
                }
                list
            })
            .collect();

        let mut violations = Vec::with_capacity(
            width_violations.len()
                + via_violations.len()
                + clearance_violations.len()
                + hole_violations.len(),
        );
        violations.extend(width_violations);
        violations.extend(via_violations);
        violations.extend(clearance_violations);
        violations.extend(hole_violations);

        violations
    }
}
