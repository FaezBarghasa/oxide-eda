//! Conflict resolution modes and push-and-shove physics.

use oxide_physics::Microns;
use uuid::Uuid;

use crate::geometry::rtree::{NetId, ObjectId, SpatialIndex, SpatialObject};
use crate::geometry::Point2D;

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

    // Vector from trace_start to obs_center
    let vx = (obs_center.x.0 - trace_start.x.0) as f64;
    let vy = (obs_center.y.0 - trace_start.y.0) as f64;
    let dot_perp = vx * perp_x + vy * perp_y;

    let sign = if dot_perp >= 0.0 { 1.0 } else { -1.0 };
    let push_amount = required_clearance + Microns(100);

    let new_pos = Point2D::new(
        obs_center.x + Microns((perp_x * push_amount.0 as f64 * sign).round() as i64),
        obs_center.y + Microns((perp_y * push_amount.0 as f64 * sign).round() as i64),
    );

    Some(PushResult {
        object_id: obstacle.id,
        original_pos: obs_center,
        new_pos,
        displacement: push_amount,
    })
}
