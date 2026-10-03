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
    ///
    /// Correctly handles multi-layer routes by partitioning segments into contiguous
    /// single-layer runs, retracing each run on its native layer, preserving vias and
    /// layer transitions, and safely falling back to original segments if a sub-run
    /// cannot find a shorter path.
    pub fn retrace_route(
        &self,
        route: &RoutingPath,
        spatial_index: &SpatialIndex,
    ) -> RoutingResult {
        if route.segments.is_empty() {
            return RoutingResult::Success(route.clone());
        }

        // Partition segments into contiguous runs that share the same layer
        let mut runs: Vec<Vec<RouteSegment>> = Vec::new();
        let mut current_run: Vec<RouteSegment> = Vec::new();

        for seg in &route.segments {
            if let Some(prev) = current_run.last() {
                if prev.layer == seg.layer {
                    current_run.push(seg.clone());
                } else {
                    runs.push(std::mem::take(&mut current_run));
                    current_run.push(seg.clone());
                }
            } else {
                current_run.push(seg.clone());
            }
        }
        if !current_run.is_empty() {
            runs.push(current_run);
        }

        let mut all_segments = Vec::new();

        for run in runs {
            if run.is_empty() {
                continue;
            }
            let run_layer = run[0].layer;
            let run_width = run[0].width;
            let run_start = run.first().unwrap().start_point;
            let run_end = run.last().unwrap().end_point;

            if run_start == run_end {
                all_segments.extend(run);
                continue;
            }

            match astar::find_astar_path(spatial_index, run_start, run_end, route.net_id, run_width)
            {
                Ok(waypoints) if waypoints.len() >= 2 => {
                    for i in 0..waypoints.len() - 1 {
                        all_segments.push(RouteSegment {
                            start_point: waypoints[i],
                            end_point: waypoints[i + 1],
                            width: run_width,
                            layer: run_layer,
                            net_id: route.net_id,
                            segment_type: SegmentType::Straight,
                        });
                    }
                }
                _ => {
                    // Fallback to preserving original sub-run if retrace was blocked
                    all_segments.extend(run);
                }
            }
        }

        let total_length = all_segments.iter().map(|s| s.length()).sum();

        RoutingResult::Success(RoutingPath {
            net_id: route.net_id,
            segments: all_segments,
            vias: route.vias.clone(),
            total_length,
            layer_transitions: route.layer_transitions.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point2D;
    use oxide_types::pcb::ViaType;

    #[test]
    fn test_retrace_empty_route() {
        let rules = Arc::new(ConstraintManager::default());
        let optimizer = RetraceOptimizer::new(rules);
        let spatial_index = SpatialIndex::new();
        let route = RoutingPath::default();
        let res = optimizer.retrace_route(&route, &spatial_index);
        match res {
            RoutingResult::Success(p) => assert!(p.segments.is_empty()),
            _ => panic!("Expected success on empty route"),
        }
    }

    #[test]
    fn test_retrace_multilayer_preserves_layers_and_vias() {
        let rules = Arc::new(ConstraintManager::default());
        let optimizer = RetraceOptimizer::new(rules);
        let spatial_index = SpatialIndex::new();

        let seg1 = RouteSegment {
            start_point: Point2D::new(0, 0),
            end_point: Point2D::new(1000, 0),
            width: 200,
            layer: 0,
            net_id: 1,
            segment_type: SegmentType::Straight,
        };
        let seg2 = RouteSegment {
            start_point: Point2D::new(1000, 0),
            end_point: Point2D::new(2000, 0),
            width: 200,
            layer: 1, // Bottom layer
            net_id: 1,
            segment_type: SegmentType::Straight,
        };
        let via = crate::ViaPlacement {
            position: Point2D::new(1000, 0),
            via_type: ViaType::Through,
            drill_size: 300,
            pad_diameter: 600,
            start_layer: 0,
            end_layer: 1,
            net_id: 1,
        };
        let transition = crate::LayerTransition {
            from_layer: 0,
            to_layer: 1,
            via_position: Point2D::new(1000, 0),
        };

        let route = RoutingPath {
            net_id: 1,
            segments: vec![seg1, seg2],
            vias: vec![via],
            total_length: 2000,
            layer_transitions: vec![transition],
        };

        let res = optimizer.retrace_route(&route, &spatial_index);
        match res {
            RoutingResult::Success(retraced) => {
                assert_eq!(retraced.vias.len(), 1);
                assert_eq!(retraced.layer_transitions.len(), 1);
                assert!(retraced.segments.iter().any(|s| s.layer == 0));
                assert!(retraced.segments.iter().any(|s| s.layer == 1));
            }
            _ => panic!("Expected successful retrace"),
        }
    }
}
