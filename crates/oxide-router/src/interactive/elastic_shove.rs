//! Bounded Elastic String Relaxation & Push-and-Shove Stabilization.
//!
//! Enforces:
//! - Hard recursion limit `MAX_SHOVE_RECURSION_DEPTH = 16`.
//! - Visited net bitset tracking to eliminate cycle loops.
//! - Quadratic deflection energy minimization with walkaround fallback.

use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

use crate::geometry::{Point2D, Vector2D};
use oxide_physics::Microns;

pub const MAX_SHOVE_RECURSION_DEPTH: usize = 16;
pub const MAX_RELAXATION_ITERATIONS: usize = 15;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ShoveError {
    #[error("Recursion limit exceeded ({0} levels); cycle detected in obstacle field")]
    RecursionLimitExceeded(usize),
    #[error("Corridor blocked by immovable obstacles; walkaround required")]
    CorridorBlocked,
    #[error("Clearance invariant violated at coordinate {0:?}")]
    ClearanceBreached(Point2D),
}

/// Thread-local shove context tracking recursion depth and visited nets.
#[derive(Debug, Clone)]
pub struct ShoveContext {
    pub visited_nets: HashSet<u32>,
    pub current_depth: usize,
    pub max_depth: usize,
    pub clearance_pad_nm: i64,
}

impl Default for ShoveContext {
    fn default() -> Self {
        Self {
            visited_nets: HashSet::new(),
            current_depth: 0,
            max_depth: MAX_SHOVE_RECURSION_DEPTH,
            clearance_pad_nm: 100_000,
        }
    }
}

impl ShoveContext {
    pub fn new(clearance_pad_nm: i64) -> Self {
        Self {
            visited_nets: HashSet::new(),
            current_depth: 0,
            max_depth: MAX_SHOVE_RECURSION_DEPTH,
            clearance_pad_nm,
        }
    }

    pub fn enter_net(&mut self, net_id: u32) -> Result<(), ShoveError> {
        if self.current_depth >= self.max_depth {
            return Err(ShoveError::RecursionLimitExceeded(self.current_depth));
        }
        if self.visited_nets.contains(&net_id) {
            return Err(ShoveError::RecursionLimitExceeded(self.current_depth));
        }
        self.visited_nets.insert(net_id);
        self.current_depth += 1;
        Ok(())
    }

    pub fn exit_net(&mut self, net_id: u32) {
        self.visited_nets.remove(&net_id);
        self.current_depth = self.current_depth.saturating_sub(1);
    }
}

/// Result of an elastic relaxation deflection.
#[derive(Debug, Clone, PartialEq)]
pub struct ElasticShoveResult {
    pub deflected_points: Vec<Point2D>,
    pub energy: f64,
    pub iterations: usize,
    pub converged: bool,
}

/// Bounded Elastic String Relaxation engine.
#[derive(Debug, Clone)]
pub struct ElasticShoveEngine {
    pub k_stretch: f64,
    pub k_stiff: f64,
    pub damping: f64,
}

impl Default for ElasticShoveEngine {
    fn default() -> Self {
        Self {
            k_stretch: 0.3,
            k_stiff: 0.6,
            damping: 0.5,
        }
    }
}

impl ElasticShoveEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Relaxes a polyline path around obstacles subject to clearance constraints.
    pub fn relax_path(
        &self,
        original_path: &[Point2D],
        obstacles: &[Point2D],
        clearance_microns: Microns,
    ) -> ElasticShoveResult {
        if original_path.len() < 2 {
            return ElasticShoveResult {
                deflected_points: original_path.to_vec(),
                energy: 0.0,
                iterations: 0,
                converged: true,
            };
        }

        let mut pts = original_path.to_vec();
        let clearance = clearance_microns as f64;
        let mut converged = false;
        let mut final_energy = 0.0;
        let mut iteration_count = 0;

        for iter in 0..MAX_RELAXATION_ITERATIONS {
            iteration_count = iter + 1;
            let mut max_displacement = 0.0;
            final_energy = 0.0;

            for i in 1..pts.len() - 1 {
                let orig = original_path[i];
                let prev = pts[i - 1];
                let next = pts[i + 1];

                // Spring forces: stretch toward neighbors, stiff toward original
                let fx_spring = self.k_stretch * ((prev.x + next.x) as f64 / 2.0 - pts[i].x as f64)
                    + self.k_stiff * (orig.x as f64 - pts[i].x as f64);
                let fy_spring = self.k_stretch * ((prev.y + next.y) as f64 / 2.0 - pts[i].y as f64)
                    + self.k_stiff * (orig.y as f64 - pts[i].y as f64);

                // Repulsive obstacle force
                let mut fx_repulse = 0.0;
                let mut fy_repulse = 0.0;

                for obs in obstacles {
                    let dx = (pts[i].x - obs.x) as f64;
                    let dy = (pts[i].y - obs.y) as f64;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < clearance && dist > 1e-3 {
                        let overlap = clearance - dist;
                        let nx = dx / dist;
                        let ny = dy / dist;
                        fx_repulse += nx * overlap * 1.2;
                        fy_repulse += ny * overlap * 1.2;
                    }
                }

                let disp_x = ((fx_spring + fx_repulse) * self.damping).round() as i64;
                let disp_y = ((fy_spring + fy_repulse) * self.damping).round() as i64;

                let d_mag = ((disp_x * disp_x + disp_y * disp_y) as f64).sqrt();
                if d_mag > max_displacement {
                    max_displacement = d_mag;
                }

                pts[i].x += disp_x;
                pts[i].y += disp_y;

                final_energy += d_mag;
            }

            if max_displacement <= 2.0 {
                converged = true;
                break;
            }
        }

        ElasticShoveResult {
            deflected_points: pts,
            energy: final_energy,
            iterations: iteration_count,
            converged,
        }
    }
}

/// Single hop record in a multi-obstacle cascade shove propagation.
#[derive(Debug, Clone, PartialEq)]
pub struct ShovePropagationStep {
    pub source_net: u32,
    pub target_net: u32,
    pub deflection_vector: Vector2D,
    pub hop_index: usize,
    pub contact_point: Point2D,
}

/// Representation of an obstacle for cascade shoving (trace, pad, or boundary).
#[derive(Debug, Clone, PartialEq)]
pub struct CascadeObstacle {
    pub net_id: u32,
    pub points: Vec<Point2D>,
    pub original_points: Vec<Point2D>,
    pub is_fixed: bool,
    pub is_via: bool,
    pub is_locked: bool,
    pub max_displacement_um: f64,
}

impl CascadeObstacle {
    pub fn new_trace(net_id: u32, points: Vec<Point2D>, is_fixed: bool) -> Self {
        let original_points = points.clone();
        Self {
            net_id,
            points,
            original_points,
            is_fixed,
            is_via: false,
            is_locked: is_fixed,
            max_displacement_um: 0.0,
        }
    }

    pub fn new_via(
        net_id: u32,
        position: Point2D,
        is_locked: bool,
        max_displacement_um: f64,
    ) -> Self {
        let original_points = vec![position];
        Self {
            net_id,
            points: vec![position],
            original_points,
            is_fixed: is_locked,
            is_via: true,
            is_locked,
            max_displacement_um,
        }
    }

    /// Relaxes obstacle points toward their original resting coordinates when clearance permits.
    pub fn relax_towards_rest(&mut self, primary_path: &[Point2D], clearance: i64, damping: f64) {
        if self.is_locked || self.is_fixed {
            return;
        }
        for (pt, orig) in self.points.iter_mut().zip(&self.original_points) {
            let dx = orig.x - pt.x;
            let dy = orig.y - pt.y;
            let dist_to_orig = ((dx * dx + dy * dy) as f64).sqrt();
            if dist_to_orig > 1.0 {
                let step_x = (dx as f64 * damping.clamp(0.0, 1.0)).round() as i64;
                let step_y = (dy as f64 * damping.clamp(0.0, 1.0)).round() as i64;
                let candidate = Point2D::new(pt.x + step_x, pt.y + step_y);
                let mut violates = false;
                for prim_pt in primary_path {
                    if prim_pt.distance_to(candidate) < clearance {
                        violates = true;
                        break;
                    }
                }
                if !violates {
                    *pt = candidate;
                }
            }
        }
    }
}

/// Outcome of a multi-obstacle topological cascade shove.
#[derive(Debug, Clone, PartialEq)]
pub struct CascadeShoveResult {
    pub primary_deflected_path: Vec<Point2D>,
    pub displaced_obstacles: Vec<(u32, Vec<Point2D>)>,
    pub displaced_vias: Vec<(u32, Point2D)>,
    pub propagation_steps: Vec<ShovePropagationStep>,
    pub max_hop_reached: usize,
    pub deflected_around_fixed: bool,
}

/// Topological Multi-Obstacle Cascade Shove Engine.
///
/// Propagates transverse deflection across successive obstacles with attenuation,
/// cycle detection, and fallback to convex-boundary walkaround on fixed boundaries.
#[derive(Debug, Clone)]
pub struct CascadeShoveEngine {
    pub max_cascade_hops: usize,
    pub attenuation_factor: f64,
    pub clearance_microns: Microns,
    pub elastic_engine: ElasticShoveEngine,
}

impl Default for CascadeShoveEngine {
    fn default() -> Self {
        Self {
            max_cascade_hops: 5,
            attenuation_factor: 0.85,
            clearance_microns: 150,
            elastic_engine: ElasticShoveEngine::default(),
        }
    }
}

impl CascadeShoveEngine {
    pub fn new(max_cascade_hops: usize, clearance_microns: Microns) -> Self {
        Self {
            max_cascade_hops,
            attenuation_factor: 0.85,
            clearance_microns,
            elastic_engine: ElasticShoveEngine::default(),
        }
    }

    /// Propagates cascade deflection through a collection of obstacles.
    pub fn propagate_cascade(
        &self,
        primary_path: &[Point2D],
        primary_net: u32,
        obstacles: &[CascadeObstacle],
    ) -> CascadeShoveResult {
        let mut displaced_map: HashMap<u32, Vec<Point2D>> = obstacles
            .iter()
            .map(|obs| (obs.net_id, obs.points.clone()))
            .collect();
        let fixed_map: HashMap<u32, bool> = obstacles
            .iter()
            .map(|obs| (obs.net_id, obs.is_fixed))
            .collect();
        let obs_map: HashMap<u32, &CascadeObstacle> =
            obstacles.iter().map(|obs| (obs.net_id, obs)).collect();

        let mut queue: VecDeque<(u32, Vector2D, usize, Point2D)> = VecDeque::new();
        let mut visited_nets = HashSet::new();
        visited_nets.insert(primary_net);

        let mut propagation_steps = Vec::new();
        let mut displaced_vias = Vec::new();
        let mut max_hop_reached = 0;
        let mut deflected_around_fixed = false;

        // Step 1: Detect direct collision of primary path with obstacles
        let mut initial_colliding_obstacles = Vec::new();
        for obs in obstacles {
            if obs.net_id == primary_net {
                continue;
            }
            for pt in &obs.points {
                for prim_pt in primary_path {
                    let dist = prim_pt.distance_to(*pt);
                    if dist < self.clearance_microns {
                        let dir = Vector2D::from_points(*prim_pt, *pt).normalize();
                        let push_mag = (self.clearance_microns - dist) as f64 + 10.0;
                        let push_vec = Vector2D::new(dir.dx * push_mag, dir.dy * push_mag);
                        initial_colliding_obstacles.push((obs.net_id, push_vec, *pt));
                        break;
                    }
                }
            }
        }

        // Enqueue initial hops
        for (obs_net, push_vec, contact_pt) in initial_colliding_obstacles {
            if *fixed_map.get(&obs_net).unwrap_or(&false) {
                deflected_around_fixed = true;
            } else if !visited_nets.contains(&obs_net) {
                visited_nets.insert(obs_net);
                queue.push_back((obs_net, push_vec, 1, contact_pt));
                propagation_steps.push(ShovePropagationStep {
                    source_net: primary_net,
                    target_net: obs_net,
                    deflection_vector: push_vec,
                    hop_index: 1,
                    contact_point: contact_pt,
                });
            }
        }

        // Step 2: Propagate cascade through queue
        while let Some((curr_net, mut push_vec, hop, _contact_pt)) = queue.pop_front() {
            max_hop_reached = max_hop_reached.max(hop);

            // Handle via constraints
            if let Some(obs) = obs_map.get(&curr_net)
                && obs.is_via
            {
                if obs.is_locked {
                    deflected_around_fixed = true;
                    continue;
                }
                let req_len = push_vec.length();
                if req_len > obs.max_displacement_um {
                    let scale = obs.max_displacement_um / req_len.max(1e-9);
                    push_vec = Vector2D::new(push_vec.dx * scale, push_vec.dy * scale);
                    deflected_around_fixed = true;
                }
            }

            // Displace target obstacle points
            if let Some(pts) = displaced_map.get_mut(&curr_net) {
                for p in pts.iter_mut() {
                    p.x += push_vec.dx.round() as i64;
                    p.y += push_vec.dy.round() as i64;
                }
                if let Some(obs) = obs_map.get(&curr_net)
                    && obs.is_via
                    && !pts.is_empty()
                {
                    displaced_vias.push((curr_net, pts[0]));
                }
            }

            if hop >= self.max_cascade_hops {
                continue;
            }

            // Check if displaced points collide with any other obstacle
            let curr_pts = displaced_map.get(&curr_net).cloned().unwrap_or_default();
            for obs in obstacles {
                let other_net = obs.net_id;
                if other_net == curr_net || other_net == primary_net {
                    continue;
                }

                let other_pts = displaced_map.get(&other_net).cloned().unwrap_or_default();
                for pt_curr in &curr_pts {
                    for pt_other in &other_pts {
                        let dist = pt_curr.distance_to(*pt_other);
                        if dist < self.clearance_microns {
                            let is_other_fixed = *fixed_map.get(&other_net).unwrap_or(&false);
                            if is_other_fixed {
                                deflected_around_fixed = true;
                            } else if !visited_nets.contains(&other_net) {
                                visited_nets.insert(other_net);
                                let mut dir =
                                    Vector2D::from_points(*pt_curr, *pt_other).normalize();
                                if dir.length() < 1e-6 {
                                    dir = push_vec.normalize();
                                }
                                let next_mag = ((self.clearance_microns - dist) as f64 + 10.0)
                                    * self.attenuation_factor;
                                let next_vec = Vector2D::new(dir.dx * next_mag, dir.dy * next_mag);

                                propagation_steps.push(ShovePropagationStep {
                                    source_net: curr_net,
                                    target_net: other_net,
                                    deflection_vector: next_vec,
                                    hop_index: hop + 1,
                                    contact_point: *pt_other,
                                });

                                queue.push_back((other_net, next_vec, hop + 1, *pt_other));
                            }
                        }
                    }
                }
            }
        }

        // Deflect primary path around obstacles or relax
        let all_obstacle_points: Vec<Point2D> = displaced_map
            .values()
            .flat_map(|pts| pts.iter().copied())
            .collect();

        let primary_deflected_path = if deflected_around_fixed {
            // Deflect primary path cleanly along boundary around fixed obstacles
            let mut deflected = primary_path.to_vec();
            for pt in &mut deflected {
                for obs in obstacles.iter().filter(|o| o.is_fixed) {
                    for obs_pt in &obs.points {
                        let dist = pt.distance_to(*obs_pt);
                        if dist < self.clearance_microns {
                            let push = (self.clearance_microns - dist) + 50;
                            let dir = Vector2D::from_points(*obs_pt, *pt).normalize();
                            pt.x += (dir.dx * push as f64).round() as i64;
                            pt.y += (dir.dy * push as f64).round() as i64;
                        }
                    }
                }
            }
            deflected
        } else {
            let relax_res = self.elastic_engine.relax_path(
                primary_path,
                &all_obstacle_points,
                self.clearance_microns,
            );
            relax_res.deflected_points
        };

        CascadeShoveResult {
            primary_deflected_path,
            displaced_obstacles: displaced_map.into_iter().collect(),
            displaced_vias,
            propagation_steps,
            max_hop_reached,
            deflected_around_fixed,
        }
    }

    /// Relaxes displaced obstacles back toward their resting state when primary path clearance allows.
    pub fn relax_obstacles(
        &self,
        primary_path: &[Point2D],
        obstacles: &mut [CascadeObstacle],
        damping: f64,
    ) {
        for obs in obstacles.iter_mut() {
            obs.relax_towards_rest(primary_path, self.clearance_microns, damping);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shove_context_recursion_limit() {
        let mut ctx = ShoveContext::new(100_000);
        for net in 0..16 {
            assert!(ctx.enter_net(net).is_ok());
        }
        // Exceeding MAX_SHOVE_RECURSION_DEPTH (16) must return error
        assert!(ctx.enter_net(16).is_err());
    }

    #[test]
    fn test_shove_context_cycle_detection() {
        let mut ctx = ShoveContext::new(100_000);
        assert!(ctx.enter_net(1).is_ok());
        assert!(ctx.enter_net(2).is_ok());
        // Re-entering visited net 1 must return error
        assert!(ctx.enter_net(1).is_err());
    }

    #[test]
    fn test_elastic_shove_relaxation() {
        let engine = ElasticShoveEngine::new();
        let path = vec![
            Point2D::new(0, 0),
            Point2D::new(500, 0),
            Point2D::new(1000, 0),
        ];
        let obstacles = vec![Point2D::new(500, 20)]; // obstacle directly on path

        let result = engine.relax_path(&path, &obstacles, 150);
        assert!(result.converged);
        assert_eq!(result.deflected_points.len(), 3);
        // Middle point must have been deflected away from obstacle
        assert!(result.deflected_points[1].y != 0);
    }

    #[test]
    fn test_multi_obstacle_cascade_shove_propagation() {
        let cascade_engine = CascadeShoveEngine::new(5, 100);
        let primary_path = vec![
            Point2D::new(0, 0),
            Point2D::new(500, 0),
            Point2D::new(1000, 0),
        ];

        // Obstacle 1 at (500, 50) is movable (distance to path = 50 < clearance 100)
        let obs1 = CascadeObstacle::new_trace(10, vec![Point2D::new(500, 50)], false);
        // Obstacle 2 at (500, 110) is also movable (when obs1 is shoved to y ~ 110, obs1 hits obs2)
        let obs2 = CascadeObstacle::new_trace(20, vec![Point2D::new(500, 110)], false);

        let result = cascade_engine.propagate_cascade(&primary_path, 1, &[obs1, obs2]);

        assert!(
            result.max_hop_reached >= 2,
            "cascade must propagate at least 2 hops, got {}",
            result.max_hop_reached
        );
        assert!(
            result.propagation_steps.len() >= 2,
            "must have at least 2 propagation steps"
        );
        assert_eq!(result.propagation_steps[0].source_net, 1);
        assert_eq!(result.propagation_steps[0].target_net, 10);
        assert_eq!(result.propagation_steps[1].source_net, 10);
        assert_eq!(result.propagation_steps[1].target_net, 20);

        // Displaced obstacle coordinates must have moved upwards
        let displaced_obs2 = result
            .displaced_obstacles
            .iter()
            .find(|(net, _)| *net == 20)
            .unwrap();
        assert!(
            displaced_obs2.1[0].y > 110,
            "obstacle 2 must be displaced upwards by cascade"
        );
    }

    #[test]
    fn test_cascade_shove_deflects_on_fixed_boundary() {
        let cascade_engine = CascadeShoveEngine::new(5, 100);
        let primary_path = vec![
            Point2D::new(0, 0),
            Point2D::new(500, 0),
            Point2D::new(1000, 0),
        ];

        // Fixed obstacle directly colliding with primary path
        let fixed_obs = CascadeObstacle::new_trace(99, vec![Point2D::new(500, 30)], true);

        let result = cascade_engine.propagate_cascade(&primary_path, 1, &[fixed_obs]);

        assert!(
            result.deflected_around_fixed,
            "must flag deflected_around_fixed"
        );
        // Primary path at middle point must be deflected away from fixed obstacle
        assert!(
            result.primary_deflected_path[1].y != 0,
            "primary path must deflect around fixed boundary"
        );
    }

    #[test]
    fn test_cascade_shove_movable_via_displacement() {
        let cascade_engine = CascadeShoveEngine::new(5, 100);
        let primary_path = vec![
            Point2D::new(0, 0),
            Point2D::new(500, 0),
            Point2D::new(1000, 0),
        ];

        // Movable signal via at (500, 60), distance to primary path = 60 < clearance 100
        let movable_via = CascadeObstacle::new_via(15, Point2D::new(500, 60), false, 150.0);

        let result = cascade_engine.propagate_cascade(&primary_path, 1, &[movable_via]);

        assert_eq!(result.displaced_vias.len(), 1, "must record displaced via");
        let (via_net, new_pos) = result.displaced_vias[0];
        assert_eq!(via_net, 15);
        assert!(
            new_pos.y > 60,
            "movable signal via must be shoved away from trace (got y={})",
            new_pos.y
        );
    }

    #[test]
    fn test_cascade_shove_elastic_springback_relaxation() {
        let cascade_engine = CascadeShoveEngine::new(5, 100);
        let orig_pos = Point2D::new(500, 60);
        let mut via = CascadeObstacle::new_via(21, orig_pos, false, 150.0);

        // Displace via manually or by a collision
        via.points[0] = Point2D::new(500, 180);
        assert_eq!(via.points[0].y, 180);

        // When primary path has moved far away (e.g. at Y = -500), clearance is satisfied
        let distant_path = vec![
            Point2D::new(0, -500),
            Point2D::new(500, -500),
            Point2D::new(1000, -500),
        ];

        // Relax toward rest with damping 0.8
        cascade_engine.relax_obstacles(&distant_path, std::slice::from_mut(&mut via), 0.8);

        // Via should have moved significantly back towards original Y=60
        assert!(
            via.points[0].y < 180,
            "via must relax back towards resting state (now at y={})",
            via.points[0].y
        );
        assert!(
            via.points[0].y >= 60,
            "via must not overshoot resting position"
        );
    }
}
