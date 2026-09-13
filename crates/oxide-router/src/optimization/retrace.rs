//! Route retrace optimizer for re-running pathfinding over existing layout.

use std::sync::Arc;

use oxide_rules::ConstraintManager;

use crate::geometry::rtree::SpatialIndex;
use crate::interactive::astar;
use crate::{RouteSegment, RoutingPath, RoutingResult, SegmentType};

#[derive(Debug, Clone)]
pub struct RetraceOptimizer {
    pub rules: Arc<ConstraintManager>,
}

impl RetraceOptimizer {
    pub fn new(rules: Arc<ConstraintManager>) -> Self {
        Self { rules }
    }

    /// Retrace an existing route with current spatial index and clearance rules.
    pub fn retrace_route(&self, route: &RoutingPath, spatial_index: &SpatialIndex) -> RoutingResult {
        let start = match route.segments.first() {
            Some(s) => s.start_point,
            None => return RoutingResult::Success(route.clone()),
        };
        let end = match route.segments.last() {
            Some(s) => s.end_point,
            None => return RoutingResult::Success(route.clone()),
        };

        let width = route.segments[0].width;
        let waypoints = astar::find_astar_path(spatial_index, start, end, route.net_id, width);
        let mut segments = Vec::new();

        for i in 0..waypoints.len().saturating_sub(1) {
            segments.push(RouteSegment {
                start_point: waypoints[i],
                end_point: waypoints[i + 1],
                width,
                layer: route.segments[0].layer,
                net_id: route.net_id,
                segment_type: SegmentType::Straight,
            });
        }

        let total_length = segments.iter().map(|s| s.length()).sum();

        RoutingResult::Success(RoutingPath {
            net_id: route.net_id,
            segments,
            vias: route.vias.clone(),
            total_length,
            layer_transitions: route.layer_transitions.clone(),
        })
    }
}
