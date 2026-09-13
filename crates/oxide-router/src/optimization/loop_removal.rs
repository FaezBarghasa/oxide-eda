//! Redundant loop detection and automatic route loop removal.

use std::sync::Arc;

use oxide_physics::Microns;
use oxide_rules::ConstraintManager;

use crate::{RouteSegment, RoutingPath};

#[derive(Debug, Clone, PartialEq)]
pub struct LoopRemovalResult {
    pub loops_removed: usize,
    pub new_length: Microns,
}

#[derive(Debug, Clone)]
pub struct LoopRemovalOptimizer {
    pub rules: Arc<ConstraintManager>,
}

impl LoopRemovalOptimizer {
    pub fn new(rules: Arc<ConstraintManager>) -> Self {
        Self { rules }
    }

    /// Automatically find and remove self-intersecting loops in a routing path.
    pub fn remove_loops(&self, path: &mut RoutingPath) -> LoopRemovalResult {
        let mut loops_removed = 0;
        let mut i = 0;

        while i < path.segments.len() {
            let mut found_loop = false;
            let p_start = path.segments[i].start_point;

            for j in (i + 1)..path.segments.len() {
                let p_end = path.segments[j].end_point;
                if p_start == p_end {
                    // Loop detected between segment i and segment j: drain internal segments
                    path.segments.drain(i..=j);
                    loops_removed += 1;
                    found_loop = true;
                    break;
                }
            }

            if !found_loop {
                i += 1;
            }
        }

        path.total_length = path.segments.iter().map(|s| s.length()).sum();

        LoopRemovalResult {
            loops_removed,
            new_length: path.total_length,
        }
    }
}
