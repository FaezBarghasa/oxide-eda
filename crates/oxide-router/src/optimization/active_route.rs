//! ActiveRoute - River Route algorithm for multi-net guide routing.

use std::sync::Arc;
use uuid::Uuid;

use oxide_physics::Microns;
use oxide_rules::ConstraintManager;

use crate::geometry::Point2D;
use crate::geometry::rtree::NetId;
use crate::{LayerId, RouteSegment, RoutingPath, RoutingResult, SegmentType};

pub type RouteGuideId = Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct RouteGuide {
    pub id: RouteGuideId,
    pub path: Vec<Point2D>,
    pub width: Microns,
    pub layer: LayerId,
}

#[derive(Debug, Clone)]
pub struct ActiveRouteOptimizer {
    pub route_guides: Vec<RouteGuide>,
    pub rules: Arc<ConstraintManager>,
}

impl ActiveRouteOptimizer {
    pub fn new(rules: Arc<ConstraintManager>) -> Self {
        Self {
            route_guides: Vec::new(),
            rules,
        }
    }

    /// Route multiple nets in parallel along a guide curve (River Routing)
    pub fn route_along_guide(&self, nets: &[NetId], guide: &RouteGuide) -> Vec<RoutingResult> {
        let mut results = Vec::new();
        if nets.is_empty() || guide.path.len() < 2 {
            return results;
        }

        let track_width = 200;
        let clearance = 150;
        let pitch = track_width + clearance;
        let total_bundle_width = pitch * nets.len().saturating_sub(1) as i64;
        let start_offset = -total_bundle_width / 2;

        for (i, &net_id) in nets.iter().enumerate() {
            let offset_microns = start_offset + i as i64 * pitch;
            let mut segments = Vec::new();

            for seg_idx in 0..guide.path.len().saturating_sub(1) {
                let p1 = guide.path[seg_idx];
                let p2 = guide.path[seg_idx + 1];
                let (dx, dy) = p1.direction_to(p2);
                let (perp_x, perp_y) = (-dy, dx);

                let start_pt = Point2D::new(
                    p1.x + (perp_x * offset_microns as f64).round() as i64,
                    p1.y + (perp_y * offset_microns as f64).round() as i64,
                );
                let end_pt = Point2D::new(
                    p2.x + (perp_x * offset_microns as f64).round() as i64,
                    p2.y + (perp_y * offset_microns as f64).round() as i64,
                );

                segments.push(RouteSegment {
                    start_point: start_pt,
                    end_point: end_pt,
                    width: track_width,
                    layer: guide.layer,
                    net_id,
                    segment_type: SegmentType::Straight,
                });
            }

            let total_length = segments.iter().map(|s| s.length()).sum();
            results.push(RoutingResult::Success(RoutingPath {
                net_id,
                segments,
                vias: Vec::new(),
                total_length,
                layer_transitions: Vec::new(),
            }));
        }

        results
    }
}
