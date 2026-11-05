//! Live Ratsnest & Unrouted Connectivity Engine (Phase 1.6).
//!
//! Calculates minimal spanning trees (MST) of unrouted connections for all
//! pads sharing identical net IDs on a PCB layout, accounting for routed
//! copper segments and vias.

use oxide_types::pcb::{PcbBoard, Point};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// An unrouted connection between two physical points on a PCB layout.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RatsnestLine {
    pub net_number: u32,
    pub start: Point,
    pub end: Point,
    pub distance: f64,
}

/// Ratsnest calculation engine.
pub struct RatsnestEngine;

impl RatsnestEngine {
    /// Compute all unrouted ratsnest lines for a given PCB layout,
    /// accounting for routed copper track segments connecting pad terminals.
    pub fn compute_ratsnest(board: &PcbBoard) -> Vec<RatsnestLine> {
        let mut net_pads: HashMap<u32, Vec<Point>> = HashMap::new();

        // 1. Collect all pad positions grouped by assigned Net Number (ignoring Net 0 / unconnected)
        for fp in &board.footprints {
            for pad in &fp.pads {
                #[allow(clippy::collapsible_if)]
                if let Some(ref pad_net) = pad.net {
                    if pad_net.number > 0 {
                        // Pad absolute position is Footprint origin + Pad local offset
                        let abs_pos = Point {
                            x: fp.position.x + pad.position.x,
                            y: fp.position.y + pad.position.y,
                        };
                        net_pads.entry(pad_net.number).or_default().push(abs_pos);
                    }
                }
            }
        }

        // 2. Identify pads that already have connecting copper segments on the same net
        let mut net_segments: HashMap<u32, Vec<(&Point, &Point)>> = HashMap::new();
        for seg in &board.segments {
            if seg.net > 0 {
                net_segments
                    .entry(seg.net)
                    .or_default()
                    .push((&seg.start, &seg.end));
            }
        }

        let mut lines = Vec::new();

        // 3. Compute unrouted edges per Net
        for (net_num, points) in net_pads {
            if points.len() < 2 {
                continue;
            }

            let mst_edges = Self::compute_mst(&points);
            let routed_segs = net_segments.get(&net_num);

            for (p1, p2, dist) in mst_edges {
                // Check if an existing segment touches both terminals or bridges them
                let is_routed = routed_segs.is_some_and(|segs| {
                    segs.iter().any(|(s, e)| {
                        let touches_p1 = (s.x - p1.x).hypot(s.y - p1.y) < 0.25
                            || (e.x - p1.x).hypot(e.y - p1.y) < 0.25;
                        let touches_p2 = (s.x - p2.x).hypot(s.y - p2.y) < 0.25
                            || (e.x - p2.x).hypot(e.y - p2.y) < 0.25;
                        touches_p1 && touches_p2
                    })
                });

                if !is_routed {
                    lines.push(RatsnestLine {
                        net_number: net_num,
                        start: p1,
                        end: p2,
                        distance: dist,
                    });
                }
            }
        }

        lines
    }

    /// Compute the overall ratsnest routing completion ratio (0.0 to 1.0).
    /// Returns 1.0 if there are 0 required multi-point net connections.
    pub fn routing_completion_ratio(board: &PcbBoard) -> f64 {
        let mut total_required_connections = 0usize;
        let mut net_pads: HashMap<u32, usize> = HashMap::new();

        for fp in &board.footprints {
            for pad in &fp.pads {
                #[allow(clippy::collapsible_if)]
                if let Some(ref pad_net) = pad.net {
                    if pad_net.number > 0 {
                        *net_pads.entry(pad_net.number).or_default() += 1;
                    }
                }
            }
        }

        for count in net_pads.values() {
            if *count >= 2 {
                total_required_connections += count - 1;
            }
        }

        if total_required_connections == 0 {
            return 1.0;
        }

        let unrouted = Self::compute_ratsnest(board).len();
        if unrouted >= total_required_connections {
            0.0
        } else {
            (total_required_connections - unrouted) as f64 / total_required_connections as f64
        }
    }

    /// Compute Minimum Spanning Tree across points using Prim's algorithm.
    fn compute_mst(points: &[Point]) -> Vec<(Point, Point, f64)> {
        let n = points.len();
        if n < 2 {
            return Vec::new();
        }

        let mut edges = Vec::with_capacity(n - 1);
        let mut visited = HashSet::new();
        visited.insert(0);

        while visited.len() < n {
            let mut min_dist = f64::MAX;
            let mut best_u = 0;
            let mut best_v = 0;

            for &u in &visited {
                for v in 0..n {
                    if !visited.contains(&v) {
                        let dx = points[u].x - points[v].x;
                        let dy = points[u].y - points[v].y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist < min_dist {
                            min_dist = dist;
                            best_u = u;
                            best_v = v;
                        }
                    }
                }
            }

            visited.insert(best_v);
            edges.push((points[best_u], points[best_v], min_dist));
        }

        edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_types::pcb::{Footprint, Pad, PadNet, PadShape, PadType};
    use uuid::Uuid;

    #[test]
    fn test_compute_ratsnest_mst() {
        let mut board = PcbBoard::default();

        let fp1 = Footprint {
            uuid: Uuid::new_v4(),
            reference: "R1".to_string(),
            value: "10k".to_string(),
            footprint_id: "R_0603".to_string(),
            position: Point { x: 10.0, y: 10.0 },
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: vec![Pad {
                uuid: Uuid::new_v4(),
                number: "1".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Rect,
                position: Point { x: 0.0, y: 0.0 },
                size: Point { x: 1.0, y: 1.0 },
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet {
                    number: 1,
                    name: "GND".to_string(),
                }),
                roundrect_ratio: 0.0,
            }],
            graphics: vec![],
            properties: vec![],
        };

        let fp2 = Footprint {
            uuid: Uuid::new_v4(),
            reference: "C1".to_string(),
            value: "100nF".to_string(),
            footprint_id: "C_0402".to_string(),
            position: Point { x: 40.0, y: 50.0 },
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: vec![Pad {
                uuid: Uuid::new_v4(),
                number: "2".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Rect,
                position: Point { x: 0.0, y: 0.0 },
                size: Point { x: 1.0, y: 1.0 },
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet {
                    number: 1,
                    name: "GND".to_string(),
                }),
                roundrect_ratio: 0.0,
            }],
            graphics: vec![],
            properties: vec![],
        };

        board.footprints.push(fp1);
        board.footprints.push(fp2);

        let lines = RatsnestEngine::compute_ratsnest(&board);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].net_number, 1);
        assert_eq!(lines[0].start, Point { x: 10.0, y: 10.0 });
        assert_eq!(lines[0].end, Point { x: 40.0, y: 50.0 });
        assert!((lines[0].distance - 50.0).abs() < 1e-6);
    }
}
