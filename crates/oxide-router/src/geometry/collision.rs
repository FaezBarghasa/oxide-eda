//! Detailed geometric collision and intersection detection routines.

use oxide_physics::Microns;

use super::{BoundingBox, Point2D};

/// Line segment in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSegment {
    pub start: Point2D,
    pub end: Point2D,
    pub width: Microns,
}

impl LineSegment {
    pub fn new(start: Point2D, end: Point2D, width: Microns) -> Self {
        Self { start, end, width }
    }

    pub fn length(&self) -> Microns {
        self.start.distance_to(self.end)
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let half_w = self.width / 2;
        BoundingBox::from_points(&[self.start, self.end]).expand(half_w)
    }

    /// Check if two line segments intersect (ignoring width).
    pub fn intersects_segment(&self, other: &LineSegment) -> bool {
        let (p1, q1) = (self.start, self.end);
        let (p2, q2) = (other.start, other.end);

        let o1 = orientation(p1, q1, p2);
        let o2 = orientation(p1, q1, q2);
        let o3 = orientation(p2, q2, p1);
        let o4 = orientation(p2, q2, q1);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        // Special Collinear Cases
        if o1 == 0 && on_segment(p1, p2, q1) {
            return true;
        }
        if o2 == 0 && on_segment(p1, q2, q1) {
            return true;
        }
        if o3 == 0 && on_segment(p2, p1, q2) {
            return true;
        }
        if o4 == 0 && on_segment(p2, q1, q2) {
            return true;
        }

        false
    }

    /// Shortest distance from a point to this line segment.
    pub fn distance_to_point(&self, p: Point2D) -> Microns {
        let l2 = self.start.distance_squared(self.end) as f64;
        if l2 < 1e-9 {
            return self.start.distance_to(p);
        }

        let px = p.x as f64;
        let py = p.y as f64;
        let x1 = self.start.x as f64;
        let y1 = self.start.y as f64;
        let x2 = self.end.x as f64;
        let y2 = self.end.y as f64;

        // Projection factor t
        let t = (((px - x1) * (x2 - x1) + (py - y1) * (y2 - y1)) / l2).clamp(0.0, 1.0);
        let proj_x = x1 + t * (x2 - x1);
        let proj_y = y1 + t * (y2 - y1);

        let dist = ((px - proj_x).powi(2) + (py - proj_y).powi(2)).sqrt();
        dist.round() as i64
    }
}

/// 0 -> collinear, 1 -> clockwise, 2 -> counterclockwise
fn orientation(p: Point2D, q: Point2D, r: Point2D) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val == 0 {
        0
    } else if val > 0 {
        1
    } else {
        2
    }
}

fn on_segment(p: Point2D, q: Point2D, r: Point2D) -> bool {
    q.x <= p.x.max(r.x)
        && q.x >= p.x.min(r.x)
        && q.y <= p.y.max(r.y)
        && q.y >= p.y.min(r.y)
}
