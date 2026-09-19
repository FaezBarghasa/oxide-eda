//! Interactive routing engine with weighted A* pathfinding, Push-and-Shove,
//! Walk-Around, Hug-and-Push, Differential Pair, and Length Tuning modes.

use std::sync::Arc;

use oxide_physics::Microns;
use oxide_rules::ConstraintManager;

use crate::geometry::Point2D;
use crate::geometry::rtree::{NetId, SpatialIndex};
use crate::{LayerId, RouteSegment, RoutingError, RoutingPath, RoutingResult, SegmentType};

pub mod astar;
pub mod conflict;
pub mod session;

use conflict::{PushAndShoveEngine, PushResult};

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
    pub push_engine: PushAndShoveEngine,
}

impl InteractiveRouter {
    pub fn new(rules: Arc<ConstraintManager>, spatial_index: Arc<SpatialIndex>) -> Self {
        Self {
            mode: RoutingMode::WalkAround,
            rules,
            spatial_index,
            session: None,
            push_engine: PushAndShoveEngine::default(),
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
            tuning_hud: None,
            corner_style: session::CornerStyle::default(),
        });
        Ok(())
    }

    /// Enable interactive length tuning with a target length and optional package delay.
    pub fn enable_length_tuning(&mut self, target_length_microns: Microns, package_delay_microns: Microns) {
        if let Some(s) = &mut self.session {
            s.mode = RoutingMode::LengthTuning;
            let mut hud = session::InteractiveTuningHudState::new(target_length_microns);
            hud.package_delay_microns = package_delay_microns;
            s.tuning_hud = Some(hud);
        }
    }

    /// Hotkey '1': Increase meander amplitude by +100µm
    pub fn hotkey_increase_amplitude(&mut self) {
        if let Some(s) = &mut self.session {
            if let Some(hud) = &mut s.tuning_hud {
                hud.adjust_amplitude(100);
            }
        }
    }

    /// Hotkey '2': Decrease meander amplitude by -100µm
    pub fn hotkey_decrease_amplitude(&mut self) {
        if let Some(s) = &mut self.session {
            if let Some(hud) = &mut s.tuning_hud {
                hud.adjust_amplitude(-100);
            }
        }
    }

    /// Hotkey '3': Increase meander pitch / wavelength by +100µm
    pub fn hotkey_increase_pitch(&mut self) {
        if let Some(s) = &mut self.session {
            if let Some(hud) = &mut s.tuning_hud {
                hud.adjust_pitch(100);
            }
        }
    }

    /// Hotkey '4': Decrease meander pitch / wavelength by -100µm
    pub fn hotkey_decrease_pitch(&mut self) {
        if let Some(s) = &mut self.session {
            if let Some(hud) = &mut s.tuning_hud {
                hud.adjust_pitch(-100);
            }
        }
    }

    /// Hotkey 'Shift+Space': Cycle corner routing style (45° -> 45° arc -> 90° -> Full Arc)
    pub fn hotkey_cycle_corner_style(&mut self) -> session::CornerStyle {
        if let Some(s) = &mut self.session {
            s.corner_style = s.corner_style.next();
            if let Some(hud) = &mut s.tuning_hud {
                hud.corner_style = s.corner_style;
            }
            s.corner_style
        } else {
            session::CornerStyle::default()
        }
    }

    /// Hotkey '*': Drop a via at current position and switch active routing layer
    pub fn hotkey_drop_via_and_switch_layer(
        &mut self,
        target_layer: LayerId,
        diameter_microns: Microns,
        drill_microns: Microns,
    ) -> Option<crate::ViaPlacement> {
        let s = self.session.as_mut()?;
        let via = crate::ViaPlacement {
            position: s.current_position,
            pad_diameter: diameter_microns,
            drill_size: drill_microns,
            start_layer: s.current_layer,
            end_layer: target_layer,
            net_id: s.current_net,
            via_type: oxide_types::pcb::ViaType::Through,
        };
        s.commit_via(via.clone());
        Some(via)
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
            RoutingMode::IgnoreObstacles => {
                self.direct_route(current_pos, target, current_net, current_layer, width)
            }
            RoutingMode::StopAtFirstObstacle => {
                self.stop_at_obstacle_route(current_pos, target, current_net, current_layer, width)
            }
            RoutingMode::WalkAround => {
                self.walk_around_route(current_pos, target, current_net, current_layer, width)
            }
            RoutingMode::PushAndShove => {
                self.push_and_shove_route(current_pos, target, current_net, current_layer, width)
            }
            RoutingMode::HugAndPush => {
                self.hug_and_push_route(current_pos, target, current_net, current_layer, width)
            }
            RoutingMode::DifferentialPair => {
                self.differential_pair_route(current_pos, target, current_net, current_layer, width)
            }
            RoutingMode::LengthTuning => {
                self.length_tuning_route(current_pos, target, current_net, current_layer, width)
            }
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
        let bbox = crate::geometry::BoundingBox::new(start, end);
        let obstacles = self.spatial_index.check_collision(&bbox, &[net_id]);

        if obstacles.is_empty() {
            self.direct_route(start, end, net_id, layer, width)
        } else {
            let first_obs = &obstacles[0];
            let stop_point = Point2D::new(
                (start.x + first_obs.bbox.min.x) / 2,
                (start.y + first_obs.bbox.min.y) / 2,
            );
            self.direct_route(start, stop_point, net_id, layer, width)
        }
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

    /// Push-and-shove routing: generates direct / A* trace while computing obstacle displacement.
    pub fn push_and_shove_with_displacements(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> (Vec<RouteSegment>, Vec<PushResult>) {
        let clearance = 150; // 150µm minimum clearance
        let pushes = self.push_engine.resolve_cascade_push(
            &self.spatial_index,
            start,
            end,
            net_id,
            clearance,
        );

        let segments = self.direct_route(start, end, net_id, layer, width);
        (segments, pushes)
    }

    fn push_and_shove_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        let (segments, _) = self.push_and_shove_with_displacements(start, end, net_id, layer, width);
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
        let gap = 150; // 150µm diff pair gap
        let centerline = astar::find_astar_path(&self.spatial_index, start, end, net_id, width);
        let mut segments = Vec::new();

        for i in 0..centerline.len().saturating_sub(1) {
            let p1 = centerline[i];
            let p2 = centerline[i + 1];
            let (dx, dy) = p1.direction_to(p2);
            let perp_x = (-dy * ((gap + width) as f64 / 2.0)).round() as i64;
            let perp_y = (dx * ((gap + width) as f64 / 2.0)).round() as i64;

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
        let (amplitude, pitch, target_len, pkg_delay) = if let Some(s) = &self.session {
            if let Some(hud) = &s.tuning_hud {
                (
                    hud.amplitude_microns,
                    hud.pitch_microns,
                    hud.target_length_microns,
                    hud.package_delay_microns,
                )
            } else {
                (800, 600, start.distance_to(end) + 4000, 0)
            }
        } else {
            (800, 600, start.distance_to(end) + 4000, 0)
        };

        let direct_dist = start.distance_to(end);
        let needed_trace_length = (target_len - pkg_delay).max(direct_dist);
        let extra_len = needed_trace_length.saturating_sub(direct_dist);

        self.generate_meander_with_params(start, end, extra_len, net_id, layer, width, amplitude, pitch)
    }

    /// Generate accordion / trombone meander pattern with default params
    pub fn generate_meander(
        &self,
        start: Point2D,
        end: Point2D,
        extra_length: Microns,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> {
        self.generate_meander_with_params(start, end, extra_length, net_id, layer, width, 800, 600)
    }

    /// Generate accordion / trombone meander pattern for length and phase-delay matching
    pub fn generate_meander_with_params(
        &self,
        start: Point2D,
        end: Point2D,
        extra_length: Microns,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
        amplitude_microns: Microns,
        pitch_microns: Microns,
    ) -> Vec<RouteSegment> {
        let mut segments = Vec::new();
        let (dx, dy) = start.direction_to(end);
        let (perp_x, perp_y) = (-dy, dx);

        let amplitude = amplitude_microns.max(100);
        let step = pitch_microns.max(100);

        let mut current = start;
        let mut remaining = extra_length;
        let mut flip = true;

        while current.distance_to(end) > step && remaining > 0 {
            let next_base = Point2D::new(
                current.x + (dx * step as f64).round() as i64,
                current.y + (dy * step as f64).round() as i64,
            );

            let offset_sign = if flip { 1.0 } else { -1.0 };
            let peak1 = Point2D::new(
                current.x + (perp_x * amplitude as f64 * offset_sign).round() as i64,
                current.y + (perp_y * amplitude as f64 * offset_sign).round() as i64,
            );
            let peak2 = Point2D::new(
                next_base.x + (perp_x * amplitude as f64 * offset_sign).round() as i64,
                next_base.y + (perp_y * amplitude as f64 * offset_sign).round() as i64,
            );

            let seg1_len = current.distance_to(peak1);
            let seg2_len = peak1.distance_to(peak2);
            let seg3_len = peak2.distance_to(next_base);
            let added = (seg1_len + seg2_len + seg3_len).saturating_sub(step);

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
            remaining = remaining.saturating_sub(added);
            flip = !flip;
        }

        if current != end {
            segments.push(RouteSegment {
                start_point: current,
                end_point: end,
                width,
                layer,
                net_id,
                segment_type: SegmentType::Straight,
            });
        }

        segments
    }

    fn get_track_width_for_net(&self, _net_id: NetId) -> Microns {
        200 // Default 200 µm
    }
}
