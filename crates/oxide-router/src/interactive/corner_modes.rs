//! Interactive routing corner mode state machine and waypoint generators.

use crate::geometry::Point2D;
use serde::{Deserialize, Serialize};

/// Routing corner transition mode during interactive drag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CornerMode {
    /// Standard 45-degree angle routing with straight bevels (default in Altium).
    #[default]
    FortyFiveDeg,
    /// 45-degree angle routing with smooth curved circular arcs.
    FortyFiveArc,
    /// 90-degree orthogonal Manhattan routing.
    NinetyDeg,
    /// Freeform direct line / any-angle routing.
    AnyAngle,
}

impl CornerMode {
    /// Cycle to the next corner mode (Shift+Space gesture).
    #[must_use]
    pub fn cycle_forward(self) -> Self {
        match self {
            Self::FortyFiveDeg => Self::FortyFiveArc,
            Self::FortyFiveArc => Self::NinetyDeg,
            Self::NinetyDeg => Self::AnyAngle,
            Self::AnyAngle => Self::FortyFiveDeg,
        }
    }

    /// Cycle to the previous corner mode.
    #[must_use]
    pub fn cycle_backward(self) -> Self {
        match self {
            Self::FortyFiveDeg => Self::AnyAngle,
            Self::FortyFiveArc => Self::FortyFiveDeg,
            Self::NinetyDeg => Self::FortyFiveArc,
            Self::AnyAngle => Self::NinetyDeg,
        }
    }

    /// Human-readable label for HUD and status bar display.
    pub fn label(self) -> &'static str {
        match self {
            Self::FortyFiveDeg => "45° Bevel",
            Self::FortyFiveArc => "45° Arc",
            Self::NinetyDeg => "90° Orthogonal",
            Self::AnyAngle => "Any Angle",
        }
    }
}

/// Synthesize route intermediate waypoints from start to target cursor position.
pub fn generate_corner_waypoints(
    start: Point2D,
    target: Point2D,
    mode: CornerMode,
    horizontal_first: bool,
) -> Vec<Point2D> {
    if start.x == target.x && start.y == target.y {
        return vec![start];
    }

    match mode {
        CornerMode::AnyAngle => {
            vec![start, target]
        }
        CornerMode::NinetyDeg => {
            if horizontal_first {
                vec![start, Point2D::new(target.x, start.y), target]
            } else {
                vec![start, Point2D::new(start.x, target.y), target]
            }
        }
        CornerMode::FortyFiveDeg => {
            let dx = target.x - start.x;
            let dy = target.y - start.y;
            let abs_dx = dx.abs();
            let abs_dy = dy.abs();

            if abs_dx == 0 || abs_dy == 0 || abs_dx == abs_dy {
                return vec![start, target];
            }

            if horizontal_first {
                if abs_dx > abs_dy {
                    // Long horizontal run, then 45-degree bevel to target
                    let mid_x = target.x - dy.signum() * dx.signum() * abs_dy;
                    vec![start, Point2D::new(mid_x, start.y), target]
                } else {
                    // 45-degree bevel first, then vertical run to target
                    let mid_y = start.y + dy.signum() * dx.signum() * abs_dx;
                    vec![start, Point2D::new(target.x, mid_y), target]
                }
            } else if abs_dy > abs_dx {
                // Long vertical run, then 45-degree bevel to target
                let mid_y = target.y - dx.signum() * dy.signum() * abs_dx;
                vec![start, Point2D::new(start.x, mid_y), target]
            } else {
                // 45-degree bevel first, then horizontal run to target
                let mid_x = start.x + dx.signum() * dy.signum() * abs_dy;
                vec![start, Point2D::new(mid_x, target.y), target]
            }
        }
        CornerMode::FortyFiveArc => {
            // Generates 45-degree waypoints with intermediate arc approximation points
            let base_waypoints = generate_corner_waypoints(
                start,
                target,
                CornerMode::FortyFiveDeg,
                horizontal_first,
            );
            if base_waypoints.len() <= 2 {
                return base_waypoints;
            }

            let mut smoothed = Vec::with_capacity(base_waypoints.len() + 2);
            smoothed.push(base_waypoints[0]);

            for i in 1..base_waypoints.len() - 1 {
                let p_prev = base_waypoints[i - 1];
                let p_curr = base_waypoints[i];
                let p_next = base_waypoints[i + 1];

                let (d1_x, d1_y) = p_curr.direction_to(p_prev);
                let (d2_x, d2_y) = p_curr.direction_to(p_next);
                let len1 = p_curr.distance_to(p_prev);
                let len2 = p_curr.distance_to(p_next);

                let fillet_r = 100i64.min(len1 * 2 / 5).min(len2 * 2 / 5); // Max 100um fillet

                if fillet_r > 1 {
                    let arc_start = Point2D::new(
                        p_curr.x + (d1_x * fillet_r as f64).round() as i64,
                        p_curr.y + (d1_y * fillet_r as f64).round() as i64,
                    );
                    let arc_end = Point2D::new(
                        p_curr.x + (d2_x * fillet_r as f64).round() as i64,
                        p_curr.y + (d2_y * fillet_r as f64).round() as i64,
                    );
                    smoothed.push(arc_start);
                    smoothed.push(arc_end);
                } else {
                    smoothed.push(p_curr);
                }
            }

            smoothed.push(*base_waypoints.last().unwrap());
            smoothed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_mode_cycling() {
        let mode = CornerMode::FortyFiveDeg;
        assert_eq!(mode.cycle_forward(), CornerMode::FortyFiveArc);
        assert_eq!(mode.cycle_forward().cycle_forward(), CornerMode::NinetyDeg);
        assert_eq!(
            mode.cycle_forward().cycle_forward().cycle_forward(),
            CornerMode::AnyAngle
        );
        assert_eq!(
            mode.cycle_forward()
                .cycle_forward()
                .cycle_forward()
                .cycle_forward(),
            CornerMode::FortyFiveDeg
        );
    }

    #[test]
    fn test_ninety_deg_waypoints() {
        let start = Point2D::new(0, 0);
        let target = Point2D::new(100, 50);
        let pts_h = generate_corner_waypoints(start, target, CornerMode::NinetyDeg, true);
        assert_eq!(pts_h.len(), 3);
        assert_eq!(pts_h[1], Point2D::new(100, 0));

        let pts_v = generate_corner_waypoints(start, target, CornerMode::NinetyDeg, false);
        assert_eq!(pts_v.len(), 3);
        assert_eq!(pts_v[1], Point2D::new(0, 50));
    }

    #[test]
    fn test_forty_five_deg_waypoints() {
        let start = Point2D::new(0, 0);
        let target = Point2D::new(100, 40);
        let pts = generate_corner_waypoints(start, target, CornerMode::FortyFiveDeg, true);
        assert_eq!(pts.len(), 3);
        assert_eq!(pts[0], start);
        assert_eq!(*pts.last().unwrap(), target);
        // Slope of second segment is 45 degrees
        let d = Point2D::new(target.x - pts[1].x, target.y - pts[1].y);
        assert_eq!(d.x.abs(), d.y.abs());
    }
}
