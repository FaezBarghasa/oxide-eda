//! Git-Native Visual Schematic and PCB Diff-Merge Engine.
//!
//! Categorizes geometric mutations between revisions:
//! - Added (Green): entities present in comparison revision but not in baseline.
//! - Removed (Red): entities present in baseline revision but not in comparison.
//! - Modified (Amber/Yellow): entities present in both with shifted coordinates, changed layers, or altered net/value.
//!
//! Supports side-by-side split rendering and unified alpha-blended overlay comparison.

use oxide_types::pcb::{Footprint, PcbBoard, Segment, Via};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffClassification {
    Added,    // Green
    Removed,  // Red
    Modified, // Amber / Yellow
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffItem<T> {
    pub classification: DiffClassification,
    pub before: Option<T>,
    pub after: Option<T>,
    pub details: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PcbVisualDiff {
    pub added_count: usize,
    pub removed_count: usize,
    pub modified_count: usize,
    pub footprint_diffs: Vec<DiffItem<Footprint>>,
    pub segment_diffs: Vec<DiffItem<Segment>>,
    pub via_diffs: Vec<DiffItem<Via>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VisualDiffMode {
    #[default]
    UnifiedOverlay, // Color-coded alpha blend
    SplitSwipe,     // Vertical swipe divider
    SideBySide,     // Dual viewports
}

impl PcbVisualDiff {
    /// Computes geometric difference between baseline board (A/HEAD) and comparison board (B/working tree).
    pub fn compute_diff(board_a: &PcbBoard, board_b: &PcbBoard) -> Self {
        let mut diff = Self::default();

        // 1. Footprints diff (by reference designator or UUID)
        let map_a: HashMap<&str, &Footprint> = board_a
            .footprints
            .iter()
            .map(|f| (f.reference.as_str(), f))
            .collect();
        let map_b: HashMap<&str, &Footprint> = board_b
            .footprints
            .iter()
            .map(|f| (f.reference.as_str(), f))
            .collect();

        let all_refs: HashSet<&str> = map_a.keys().copied().chain(map_b.keys().copied()).collect();

        for r in all_refs {
            match (map_a.get(r), map_b.get(r)) {
                (Some(fa), None) => {
                    diff.removed_count += 1;
                    diff.footprint_diffs.push(DiffItem {
                        classification: DiffClassification::Removed,
                        before: Some((*fa).clone()),
                        after: None,
                        details: format!("Removed footprint {} ({})", fa.reference, fa.value),
                    });
                }
                (None, Some(fb)) => {
                    diff.added_count += 1;
                    diff.footprint_diffs.push(DiffItem {
                        classification: DiffClassification::Added,
                        before: None,
                        after: Some((*fb).clone()),
                        details: format!("Added footprint {} ({})", fb.reference, fb.value),
                    });
                }
                (Some(fa), Some(fb)) => {
                    let pos_changed =
                        fa.position.x != fb.position.x || fa.position.y != fb.position.y;
                    let rot_changed = (fa.rotation - fb.rotation).abs() > 1e-3;
                    let val_changed = fa.value != fb.value;
                    let layer_changed = fa.layer != fb.layer;

                    if pos_changed || rot_changed || val_changed || layer_changed {
                        diff.modified_count += 1;
                        let mut changes = Vec::new();
                        if pos_changed {
                            changes.push(format!(
                                "pos ({:.2}, {:.2}) -> ({:.2}, {:.2})",
                                fa.position.x, fa.position.y, fb.position.x, fb.position.y
                            ));
                        }
                        if rot_changed {
                            changes.push(format!("rot {}° -> {}°", fa.rotation, fb.rotation));
                        }
                        if val_changed {
                            changes.push(format!("val '{}' -> '{}'", fa.value, fb.value));
                        }
                        if layer_changed {
                            changes.push(format!("layer '{}' -> '{}'", fa.layer, fb.layer));
                        }

                        diff.footprint_diffs.push(DiffItem {
                            classification: DiffClassification::Modified,
                            before: Some((*fa).clone()),
                            after: Some((*fb).clone()),
                            details: format!("Modified {}: {}", fa.reference, changes.join(", ")),
                        });
                    }
                }
                (None, None) => {}
            }
        }

        // 2. Segments diff (matched by uuid or geometric coordinates)
        let seg_map_a: HashMap<Uuid, &Segment> =
            board_a.segments.iter().map(|s| (s.uuid, s)).collect();
        let seg_map_b: HashMap<Uuid, &Segment> =
            board_b.segments.iter().map(|s| (s.uuid, s)).collect();
        let all_seg_uuids: HashSet<Uuid> = seg_map_a
            .keys()
            .copied()
            .chain(seg_map_b.keys().copied())
            .collect();

        for uid in all_seg_uuids {
            match (seg_map_a.get(&uid), seg_map_b.get(&uid)) {
                (Some(sa), None) => {
                    diff.removed_count += 1;
                    diff.segment_diffs.push(DiffItem {
                        classification: DiffClassification::Removed,
                        before: Some((*sa).clone()),
                        after: None,
                        details: format!(
                            "Removed track segment on layer {} net {}",
                            sa.layer, sa.net
                        ),
                    });
                }
                (None, Some(sb)) => {
                    diff.added_count += 1;
                    diff.segment_diffs.push(DiffItem {
                        classification: DiffClassification::Added,
                        before: None,
                        after: Some((*sb).clone()),
                        details: format!(
                            "Added track segment on layer {} net {}",
                            sb.layer, sb.net
                        ),
                    });
                }
                (Some(sa), Some(sb)) => {
                    let geom_changed = sa.start.x != sb.start.x
                        || sa.start.y != sb.start.y
                        || sa.end.x != sb.end.x
                        || sa.end.y != sb.end.y;
                    let width_changed = (sa.width - sb.width).abs() > 1e-4;
                    let net_changed = sa.net != sb.net;
                    let layer_changed = sa.layer != sb.layer;

                    if geom_changed || width_changed || net_changed || layer_changed {
                        diff.modified_count += 1;
                        diff.segment_diffs.push(DiffItem {
                            classification: DiffClassification::Modified,
                            before: Some((*sa).clone()),
                            after: Some((*sb).clone()),
                            details: format!(
                                "Modified track segment net {} layer {}",
                                sb.net, sb.layer
                            ),
                        });
                    }
                }
                (None, None) => {}
            }
        }

        // 3. Vias diff
        let via_map_a: HashMap<Uuid, &Via> = board_a.vias.iter().map(|v| (v.uuid, v)).collect();
        let via_map_b: HashMap<Uuid, &Via> = board_b.vias.iter().map(|v| (v.uuid, v)).collect();
        let all_via_uuids: HashSet<Uuid> = via_map_a
            .keys()
            .copied()
            .chain(via_map_b.keys().copied())
            .collect();

        for uid in all_via_uuids {
            match (via_map_a.get(&uid), via_map_b.get(&uid)) {
                (Some(va), None) => {
                    diff.removed_count += 1;
                    diff.via_diffs.push(DiffItem {
                        classification: DiffClassification::Removed,
                        before: Some((*va).clone()),
                        after: None,
                        details: format!(
                            "Removed via at ({:.2}, {:.2}) net {}",
                            va.position.x, va.position.y, va.net
                        ),
                    });
                }
                (None, Some(vb)) => {
                    diff.added_count += 1;
                    diff.via_diffs.push(DiffItem {
                        classification: DiffClassification::Added,
                        before: None,
                        after: Some((*vb).clone()),
                        details: format!(
                            "Added via at ({:.2}, {:.2}) net {}",
                            vb.position.x, vb.position.y, vb.net
                        ),
                    });
                }
                (Some(va), Some(vb)) => {
                    let pos_changed =
                        va.position.x != vb.position.x || va.position.y != vb.position.y;
                    let net_changed = va.net != vb.net;
                    let drill_changed = (va.drill - vb.drill).abs() > 1e-4;

                    if pos_changed || net_changed || drill_changed {
                        diff.modified_count += 1;
                        diff.via_diffs.push(DiffItem {
                            classification: DiffClassification::Modified,
                            before: Some((*va).clone()),
                            after: Some((*vb).clone()),
                            details: format!(
                                "Modified via at ({:.2}, {:.2})",
                                vb.position.x, vb.position.y
                            ),
                        });
                    }
                }
                (None, None) => {}
            }
        }

        diff
    }

    /// RGBA color representation for overlay visual rendering.
    pub fn classification_color(classification: DiffClassification) -> [f32; 4] {
        match classification {
            DiffClassification::Added => [0.15, 0.85, 0.30, 0.80],    // Crisp Green
            DiffClassification::Removed => [0.90, 0.20, 0.20, 0.80],  // Bright Red
            DiffClassification::Modified => [1.00, 0.75, 0.10, 0.85], // Amber / Gold
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_types::pcb::Point;

    #[test]
    pub fn test_git_visual_diff_geometry_classification() {
        let mut board_a = PcbBoard::default();
        let mut board_b = PcbBoard::default();

        // 1. Footprint R1 in A (value 10k), R1 in B modified to 20k
        let r1_a = Footprint {
            uuid: Uuid::new_v4(),
            reference: "R1".to_string(),
            value: "10k".to_string(),
            footprint_id: "R_0603".to_string(),
            position: Point { x: 10.0, y: 10.0 },
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: Vec::new(),
            graphics: Vec::new(),
            properties: Vec::new(),
        };
        let mut r1_b = r1_a.clone();
        r1_b.value = "20k".to_string();
        r1_b.position.x = 12.0;

        board_a.footprints.push(r1_a);
        board_b.footprints.push(r1_b);

        // 2. Footprint C1 only in A (Removed in B)
        let c1_a = Footprint {
            uuid: Uuid::new_v4(),
            reference: "C1".to_string(),
            value: "100nF".to_string(),
            footprint_id: "C_0603".to_string(),
            position: Point { x: 30.0, y: 10.0 },
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: Vec::new(),
            graphics: Vec::new(),
            properties: Vec::new(),
        };
        board_a.footprints.push(c1_a);

        // 3. Footprint U2 only in B (Added in B)
        let u2_b = Footprint {
            uuid: Uuid::new_v4(),
            reference: "U2".to_string(),
            value: "ATmega328P".to_string(),
            footprint_id: "QFP-32".to_string(),
            position: Point { x: 50.0, y: 50.0 },
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: Vec::new(),
            graphics: Vec::new(),
            properties: Vec::new(),
        };
        board_b.footprints.push(u2_b);

        // 4. Segments: Track 1 removed, Track 2 added
        let seg_del = Segment {
            uuid: Uuid::new_v4(),
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 10.0, y: 0.0 },
            width: 0.25,
            layer: "F.Cu".to_string(),
            net: 1,
        };
        board_a.segments.push(seg_del);

        let seg_add = Segment {
            uuid: Uuid::new_v4(),
            start: Point { x: 20.0, y: 0.0 },
            end: Point { x: 30.0, y: 0.0 },
            width: 0.35,
            layer: "F.Cu".to_string(),
            net: 2,
        };
        board_b.segments.push(seg_add);

        let diff = PcbVisualDiff::compute_diff(&board_a, &board_b);

        // Assert classifications
        assert_eq!(diff.added_count, 2, "Expected 1 added footprint + 1 added segment");
        assert_eq!(diff.removed_count, 2, "Expected 1 removed footprint + 1 removed segment");
        assert_eq!(diff.modified_count, 1, "Expected 1 modified footprint (R1)");

        let r1_diff = diff
            .footprint_diffs
            .iter()
            .find(|d| d.details.contains("R1"))
            .expect("R1 diff must exist");
        assert_eq!(r1_diff.classification, DiffClassification::Modified);

        let c1_diff = diff
            .footprint_diffs
            .iter()
            .find(|d| d.details.contains("C1"))
            .expect("C1 diff must exist");
        assert_eq!(c1_diff.classification, DiffClassification::Removed);

        let u2_diff = diff
            .footprint_diffs
            .iter()
            .find(|d| d.details.contains("U2"))
            .expect("U2 diff must exist");
        assert_eq!(u2_diff.classification, DiffClassification::Added);

        // Colors
        let green = PcbVisualDiff::classification_color(DiffClassification::Added);
        assert!(green[1] > green[0], "Green channel must dominate added color");
        let red = PcbVisualDiff::classification_color(DiffClassification::Removed);
        assert!(red[0] > red[1], "Red channel must dominate removed color");
    }
}
