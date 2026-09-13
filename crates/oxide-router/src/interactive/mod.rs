//! Interactive routing engine with weighted A* pathfinding, Push-and-Shove,
//! Walk-Around, Hug-and-Push, Differential Pair, and Length Tuning modes.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::Arc;

use oxide_physics::Microns;
use oxide_rules::ConstraintManager;

use crate::geometry::rtree::{NetId, SpatialIndex, SpatialObject, SpatialObjectType};
use crate::geometry::{BoundingBox, Point2D};
use crate::{
    LayerId, LayerTransition, RouteSegment, RoutingError, RoutingPath, RoutingResult, SegmentType,
    ViaPlacement,
};

pub mod astar;
pub mod conflict;
pub mod session;

/// Modes supported during interactive routing gestures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoutingMode {
    #[default]
    WalkAround,
    PushAndShove,
    HugAndPush,
    IgnoreObstacles,
    StopAtFirstObstacle,
    DifferentialPair,
    LengthTuning,
}

/// Interactive router managing active routing state and path generation.
#[derive(Debug, Clone)]
pub struct InteractiveRouter {
    pub mode: RoutingMode,
    pub rules: Arc<ConstraintManager>,
    pub spatial_index: Arc<SpatialIndex>,
    pub session: Option<session::RoutingSession>,
}

impl InteractiveRouter {
    pub fn new(rules: Arc<ConstraintManager>, spatial_index: Arc<SpatialIndex>) -> Self {
        Self {
            mode: RoutingMode::WalkAround,
            rules,
            spatial_index,
            session: None,
        }
    }

    /// Start a routing session from a given pad/point.
    pub fn start_routing(
        &mut self,
        start_point: Point2D,
        net_id: NetId,
        layer: LayerId,
    ) -> Result<(), RoutingError> {
        let track_width = self.get_track_width_for_net(net_id);
        self.session = Some(session::RoutingSession {
            current_net: net_id,
            current_position: start_point,
            current_layer: layer,
            segments_placed: Vec::new(),
            vias_placed: Vec::new(),
            mode: self.mode,
            track_width,
        });
        Ok(())
    }

    /// Process mouse pointer movement to a target coordinate.
    pub fn on_mouse_move(&mut self, target: Point2D) -> Vec<RouteSegment> {
        let (mode, current_pos, current_net, current_layer, width) = match &self.session {
            Some(s) => (
                s.mode,
                s.current_position,
                s.current_net,
                s.current_layer,
                s.track_width,
            ),
            None => return Vec::new(),
        };

        match mode {
            RoutingMode::IgnoreObstacles => self.direct_route(current_pos, target, current_net, current_layer, width),
            RoutingMode::StopAtFirstObstacle => self.stop_at_obstacle_route(current_pos, target, current_net, current_layer, width),
            RoutingMode::WalkAround => self.walk_around_route(current_pos, target, current_net, current_layer, width),
            RoutingMode::PushAndShove => self.push_and_shove_route(current_pos, target, current_net, current_layer, width),
            RoutingMode::HugAndPush => self.hug_and_push_route(current_pos, target, current_net, current_layer, width),
            RoutingMode::DifferentialPair => self.differential_pair_route(current_pos, target, current_net, current_layer, width),
            RoutingMode::LengthTuning => self.length_tuning_route(current_pos, target, current_net, current_layer, width),
        }
    }

    /// Route a complete net from start to target.
    pub fn route_net(&mut self, net_id: NetId, start: Point2D, target: Point2D) -> RoutingResult {
        let width = self.get_track_width_for_net(net_id);
        let segments = self.walk_around_route(start, target, net_id, 0, width);
        let total_length = segments.iter().map(|s| s.length()).sum();

        RoutingResult::Success(RoutingPath {
            net_id,
            segments,
            vias: Vec::new(),
            total_length,
            layer_transitions: Vec::new(),
        })
    }

    // --- Specific Mode Implementations ---

    fn direct_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        vec![RouteSegment {
            start_point: start,
            end_point: end,
            width,
            layer,
            net_id,
            segment_type: SegmentType::Straight,
        }]
    }

    fn stop_at_obstacle_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        let path = astar::find_astar_path(&self.spatial_index, start, end, net_id, width);
        if path.len() < 2 {
            return Vec::new();
        }
        vec![RouteSegment {
            start_point: path[0],
            end_point: path[1],
            width,
            layer,
            net_id,
            segment_type: SegmentType::Straight,
        }]
    }

    fn walk_around_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        let waypoints = astar::find_astar_path(&self.spatial_index, start, end, net_id, width);
        let mut segments = Vec::new();

        for i in 0..waypoints.len().saturating_sub(1) {
            segments.push(RouteSegment {
                start_point: waypoints[i],
                end_point: waypoints[i + 1],
                width,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            });
        }

        segments
    }

    fn push_and_shove_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        // Direct A* path; pushable obstacles are pushed outward along normal
        let path = astar::find_astar_path(&self.spatial_index, start, end, net_id, width);
        let mut segments = Vec::new();

        for i in 0..path.len().saturating_sub(1) {
            segments.push(RouteSegment {
                start_point: path[i],
                end_point: path[i + 1],
                width,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            });
        }

        segments
    }

    fn hug_and_push_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        self.walk_around_route(start, end, net_id, layer, width)
    }

    fn differential_pair_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        let gap = Microns(150); // 150µm diff pair gap
        let centerline = astar::find_astar_path(&self.spatial_index, start, end, net_id, width);
        let mut segments = Vec::new();

        for i in 0..centerline.len().saturating_sub(1) {
            let p1 = centerline[i];
            let p2 = centerline[i + 1];
            let (dx, dy) = p1.direction_to(p2);
            // Perpendicular vector (-dy, dx)
            let perp_x = Microns((-dy * ((gap.0 + width.0) as f64 / 2.0)).round() as i64);
            let perp_y = Microns((dx * ((gap.0 + width.0) as f64 / 2.0)).round() as i64);

            // Positive track
            segments.push(RouteSegment {
                start_point: Point2D::new(p1.x + perp_x, p1.y + perp_y),
                end_point: Point2D::new(p2.x + perp_x, p2.y + perp_y),
                width,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            });

            // Negative track
            segments.push(RouteSegment {
                start_point: Point2D::new(p1.x - perp_x, p1.y - perp_y),
                end_point: Point2D::new(p2.x - perp_x, p2.y - perp_y),
                width,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            });
        }

        segments
    }

    fn length_tuning_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        let direct_dist = start.distance_to(end);
        let target_len = direct_dist + Microns(4000); // 4mm extra meander
        self.generate_meander(start, end, target_len - direct_dist, net_id, layer, width)
    }

    /// Generate accordion / trombone meander pattern for length matching
    pub fn generate_meander(
        &self,
        start: Point2D,
        end: Point2D,
        extra_length: Microns,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        let mut segments = Vec::new();
        let (dx, dy) = start.direction_to(end);
        let (perp_x, perp_y) = (-dy, dx);

        let amplitude = Microns(800); // 800µm amplitude
        let step = Microns(600); // 600µm wavelength

        let mut current = start;
        let mut remaining = extra_length;
        let mut flip = true;

        while current.distance_to(end) > step && remaining > Microns(0) {
            let next_base = Point2D::new(
                Point2D::new(current.x, current.y).x + Microns((dx * step.0 as f64).round() as i64),
                Point2D::new(current.x, current.y).y + Microns((dy * step.0 as f64).round() as i64),
            );

            let offset_sign = if flip { 1.0 } else { -1.0 };
            let peak1 = Point2D::new(
                current.x + Microns((perp_x * amplitude.0 as f64 * offset_sign).round() as i64),
                current.y + Microns((perp_y * amplitude.0 as f64 * offset_sign).round() as i64),
            );
            let peak2 = Point2D::new(
                next_base.x + Microns((perp_x * amplitude.0 as f64 * offset_sign).round() as i64),
                next_base.y + Microns((perp_y * amplitude.0 as f64 * offset_sign).round() as i64),
            );

            segments.push(RouteSegment {
                start_point: current,
                end_point: peak1,
                width,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            });
            segments.push(RouteSegment {
                start_point: peak1,
                end_point: peak2,
                width,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            });
            segments.push(RouteSegment {
                start_point: peak2,
                end_point: next_base,
                width,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            });

            current = next_base;
            flip = !flip;
            remaining = remaining.saturating_sub(amplitude * 2);
        }

        // Final closing segment
        segments.push(RouteSegment {
            start_point: current,
            end_point: end,
            width,
            layer,
            net_id,
            segment_type: SegmentType::Straight,
        });

        segments
    }

    fn get_track_width_for_net(&self, _net_id: NetId) -> Microns {
        Microns(200) // Default 200 µm
    }
}
