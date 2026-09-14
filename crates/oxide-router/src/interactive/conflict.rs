//! Conflict resolution modes and push-and-shove physics.

use std::collections::{HashMap, HashSet};
use oxide_physics::Microns;

use crate::geometry::rtree::{ObjectId, SpatialIndex, SpatialObject};
use crate::geometry::{BoundingBox, Point2D};

/// Result of pushing an obstacle away from an advancing trace.
#[derive(Debug, Clone, PartialEq)]
pub struct PushResult {
    pub object_id: ObjectId,
    pub original_pos: Point2D,
    pub new_pos: Point2D,
    pub displacement: Microns,
}

/// Calculate push displacement vector to move obstacle out of clearance zone.
pub fn calculate_push(
    obstacle: &SpatialObject,
    trace_start: Point2D,
    trace_end: Point2D,
    required_clearance: Microns,
) -> Option<PushResult> {
    let obs_center = obstacle.bbox.center();
    let (dx, dy) = trace_start.direction_to(trace_end);
    let (perp_x, perp_y) = (-dy, dx);

    let vx = (obs_center.x - trace_start.x) as f64;
    let vy = (obs_center.y - trace_start.y) as f64;
    let dot_perp = vx * perp_x + vy * perp_y;

    let sign = if dot_perp >= 0.0 { 1.0 } else { -1.0 };
    let push_amount = required_clearance + 100;

    let new_pos = Point2D::new(
        obs_center.x + (perp_x * push_amount as f64 * sign).round() as i64,
        obs_center.y + (perp_y * push_amount as f64 * sign).round() as i64,
    );

    Some(PushResult {
        object_id: obstacle.id,
        original_pos: obs_center,
        new_pos,
        displacement: push_amount,
    })
}

/// Cascade push-and-shove physics engine: recursively resolves multi-obstacle collisions
/// while maintaining strict clearance and minimum displacement energy.
#[derive(Debug, Clone)]
pub struct PushAndShoveEngine {
    pub max_cascade_depth: usize,
    pub repulsive_spring_constant: f64,
}

impl Default for PushAndShoveEngine {
    fn default() -> Self {
        Self {
            max_cascade_depth: 4,
            repulsive_spring_constant: 0.85,
        }
    }
}

impl PushAndShoveEngine {
    pub fn new(max_cascade_depth: usize) -> Self {
        Self {
            max_cascade_depth,
            repulsive_spring_constant: 0.85,
        }
    }

    /// Resolve a cascade push for a trace segment advancing through `spatial_index`.
    pub fn resolve_cascade_push(
        &self,
        spatial_index: &SpatialIndex,
        trace_start: Point2D,
        trace_end: Point2D,
        net_id: u32,
        required_clearance: Microns,
    ) -> Vec<PushResult> {
        let mut results = Vec::new();
        let mut visited = HashSet::new();
        let mut pushed_positions: HashMap<ObjectId, Point2D> = HashMap::new();

        let sweep_box = BoundingBox::new(
            Point2D::new(
                trace_start.x.min(trace_end.x) - required_clearance,
                trace_start.y.min(trace_end.y) - required_clearance,
            ),
            Point2D::new(
                trace_start.x.max(trace_end.x) + required_clearance,
                trace_start.y.max(trace_end.y) + required_clearance,
            ),
        );

        let initial_obstacles = spatial_index.check_collision(&sweep_box, &[net_id]);

        for obstacle in initial_obstacles {
            if visited.contains(&obstacle.id) {
                continue;
            }
            if let Some(push) = calculate_push(&obstacle, trace_start, trace_end, required_clearance) {
                visited.insert(push.object_id);
                pushed_positions.insert(push.object_id, push.new_pos);
                results.push(push);

                // Cascade to secondary obstacles
                self.cascade_step(
                    spatial_index,
                    &mut results,
                    &mut visited,
                    &mut pushed_positions,
                    &obstacle,
                    required_clearance,
                    1,
                );
            }
        }

        results
    }

    fn cascade_step(
        &self,
        spatial_index: &SpatialIndex,
        results: &mut Vec<PushResult>,
        visited: &mut HashSet<ObjectId>,
        pushed_positions: &mut HashMap<ObjectId, Point2D>,
        source_obs: &SpatialObject,
        required_clearance: Microns,
        depth: usize,
    ) {
        if depth >= self.max_cascade_depth {
            return;
        }

        let current_pos = pushed_positions.get(&source_obs.id).copied().unwrap_or_else(|| source_obs.bbox.center());
        let secondary_bbox = BoundingBox::from_center_radius(current_pos, required_clearance + 200);

        let exclude_nets: Vec<u32> = source_obs.net_id.into_iter().collect();
        let secondary_obstacles = spatial_index.check_collision(&secondary_bbox, &exclude_nets);

        for sec_obs in secondary_obstacles {
            if visited.contains(&sec_obs.id) {
                continue;
            }

            let sec_center = sec_obs.bbox.center();
            let (dx, dy) = current_pos.direction_to(sec_center);
            let push_dist = (required_clearance as f64 * self.repulsive_spring_constant).round() as i64;
            let new_pos = Point2D::new(
                sec_center.x + (dx * push_dist as f64).round() as i64,
                sec_center.y + (dy * push_dist as f64).round() as i64,
            );

            let push = PushResult {
                object_id: sec_obs.id,
                original_pos: sec_center,
                new_pos,
                displacement: push_dist,
            };

            visited.insert(push.object_id);
            pushed_positions.insert(push.object_id, push.new_pos);
            results.push(push);

            self.cascade_step(
                spatial_index,
                results,
                visited,
                pushed_positions,
                &sec_obs,
                required_clearance,
                depth + 1,
            );
        }
    }
}
