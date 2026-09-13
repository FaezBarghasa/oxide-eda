//! Route glossing optimizer: corner simplification, collinear merging, and angle softening.

use std::sync::Arc;

use oxide_physics::Microns;
use oxide_rules::ConstraintManager;

use crate::{RouteSegment, RoutingPath, SegmentType};

#[derive(Debug, Clone, PartialEq)]
pub struct GlossingResult {
    pub original_length: Microns,
    pub new_length: Microns,
    pub corners_removed: usize,
}

#[derive(Debug, Clone)]
pub struct GlossingOptimizer {
    pub rules: Arc<ConstraintManager>,
}

impl GlossingOptimizer {
    pub fn new(rules: Arc<ConstraintManager>) -> Self {
        Self { rules }
    }

    /// Gloss a routing path: remove redundant corners and shorten track length.
    pub fn gloss_path(&self, path: &mut RoutingPath) -> GlossingResult {
        let original_length = path.total_length;
        let mut corners_removed = 0;

        // 1. Collinear segments removal
        corners_removed += self.remove_collinear_segments(&mut path.segments);

        // 2. Shortcut corner cutting (45-degree chamfering)
        corners_removed += self.cut_redundant_corners(&mut path.segments);

        let new_length = path.segments.iter().map(|s| s.length()).sum();
        path.total_length = new_length;

        GlossingResult {
            original_length,
            new_length,
            corners_removed,
        }
    }

    fn remove_collinear_segments(&self, segments: &mut Vec<RouteSegment>) -> usize {
        if segments.len() < 2 {
            return 0;
        }

        let mut removed = 0;
        let mut i = 0;

        while i < segments.len().saturating_sub(1) {
            let (dx1, dy1) = segments[i].start_point.direction_to(segments[i].end_point);
            let (dx2, dy2) = segments[i + 1].start_point.direction_to(segments[i + 1].end_point);

            if (dx1 - dx2).abs() < 1e-4 && (dy1 - dy2).abs() < 1e-4 {
                // Merge segments[i] and segments[i + 1]
                segments[i].end_point = segments[i + 1].end_point;
                segments.remove(i + 1);
                removed += 1;
            } else {
                i += 1;
            }
        }

        removed
    }

    fn cut_redundant_corners(&self, segments: &mut Vec<RouteSegment>) -> usize {
        if segments.len() < 2 {
            return 0;
        }

        let mut removed = 0;
        let mut i = 0;

        while i < segments.len().saturating_sub(2) {
            let p0 = segments[i].start_point;
            let p1 = segments[i].end_point;
            let p2 = segments[i + 1].end_point;

            // Direct line check
            let d_indirect = p0.distance_to(p1) + p1.distance_to(p2);
            let d_direct = p0.distance_to(p2);

            if d_direct < d_indirect {
                // Replace two segments with one direct segment
                segments[i].end_point = p2;
                segments.remove(i + 1);
                removed += 1;
            } else {
                i += 1;
            }
        }

        removed
    }
}
