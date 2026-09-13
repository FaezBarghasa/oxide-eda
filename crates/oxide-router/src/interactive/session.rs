//! Interactive routing session state.

use oxide_physics::Microns;

use super::RoutingMode;
use crate::geometry::Point2D;
use crate::geometry::rtree::NetId;
use crate::{LayerId, RouteSegment, ViaPlacement};

/// Active state while user is dragging or clicking to lay down a trace.
#[derive(Debug, Clone)]
pub struct RoutingSession {
    pub current_net: NetId,
    pub current_position: Point2D,
    pub current_layer: LayerId,
    pub segments_placed: Vec<RouteSegment>,
    pub vias_placed: Vec<ViaPlacement>,
    pub mode: RoutingMode,
    pub track_width: Microns,
}

impl RoutingSession {
    pub fn commit_segment(&mut self, segment: RouteSegment) {
        self.current_position = segment.end_point;
        self.segments_placed.push(segment);
    }

    pub fn commit_via(&mut self, via: ViaPlacement) {
        self.current_layer = via.end_layer;
        self.current_position = via.position;
        self.vias_placed.push(via);
    }
}
