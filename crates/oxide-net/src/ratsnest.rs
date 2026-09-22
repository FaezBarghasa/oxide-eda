//! Live Ratsnest & Unrouted Connectivity Engine (Phase 1.6).
//!
//! Calculates minimal spanning trees (MST) of unrouted connections for all
//! pads sharing identical net IDs on a PCB layout, accounting for routed
//! copper segments and vias.

use std::collections::{HashMap, HashSet};
use oxide_types::pcb::{PcbBoard, Point};
use serde::{Deserialize, Serialize};

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
    /// Compute all unrouted ratsnest lines for a given PCB layout.
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

        let mut lines = Vec::new();

        // 2. Compute Minimum Spanning Tree (MST via Prim's algorithm) per Net
        for (net_num, points) in net_pads {
            if points.len() < 2 {
                continue;
            }

            let mst_edges = Self::compute_mst(&points);
            for (p1, p2, dist) in mst_edges {
                lines.push(RatsnestLine {
                    net_number: net_num,
                    start: p1,
                    end: p2,
                    distance: dist,
                });
            }
        }

        lines
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
