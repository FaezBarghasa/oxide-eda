//! Geometric primitives and spatial algorithms for PCB routing.

use serde::{Deserialize, Serialize};

use oxide_physics::Microns;

pub mod collision;
pub mod rtree;

/// High-precision 2D Point in routing space (coordinates in Microns i64).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Point2D {
    pub x: Microns,
    pub y: Microns,
}

impl Point2D {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const fn new(x: Microns, y: Microns) -> Self {
        Self { x, y }
    }

    pub fn from_mm(x_mm: f64, y_mm: f64) -> Self {
        Self {
            x: (x_mm * 1000.0).round() as i64,
            y: (y_mm * 1000.0).round() as i64,
        }
    }

    pub fn to_mm(&self) -> (f64, f64) {
        (self.x as f64 / 1000.0, self.y as f64 / 1000.0)
    }

    /// Euclidean distance to another point in micrometers.
    pub fn distance_to(&self, other: Point2D) -> Microns {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt().round() as i64
    }

    /// Squared euclidean distance to avoid square root.
    pub fn distance_squared(&self, other: Point2D) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Normalize vector to unit direction (f64).
    pub fn direction_to(&self, other: Point2D) -> (f64, f64) {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1e-9 {
            (0.0, 0.0)
        } else {
            (dx / len, dy / len)
        }
    }
}

/// Axis-Aligned Bounding Box (AABB) in routing space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Point2D,
    pub max: Point2D,
}

impl BoundingBox {
    pub fn new(min: Point2D, max: Point2D) -> Self {
        Self {
            min: Point2D::new(min.x.min(max.x), min.y.min(max.y)),
            max: Point2D::new(min.x.max(max.x), min.y.max(max.y)),
        }
    }

    pub fn from_center_radius(center: Point2D, radius: Microns) -> Self {
        Self {
            min: Point2D::new(center.x - radius, center.y - radius),
            max: Point2D::new(center.x + radius, center.y + radius),
        }
    }

    pub fn from_points(points: &[Point2D]) -> Self {
        if points.is_empty() {
            return Self::default();
        }
        let mut min_x = points[0].x;
        let mut min_y = points[0].y;
        let mut max_x = points[0].x;
        let mut max_y = points[0].y;

        for p in points.iter().skip(1) {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        Self {
            min: Point2D::new(min_x, min_y),
            max: Point2D::new(max_x, max_y),
        }
    }

    pub fn center(&self) -> Point2D {
        Point2D::new((self.min.x + self.max.x) / 2, (self.min.y + self.max.y) / 2)
    }

    pub fn width(&self) -> Microns {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> Microns {
        self.max.y - self.min.y
    }

    pub fn contains_point(&self, point: Point2D) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Minimum distance from this bounding box to another bounding box.
    pub fn distance_to(&self, other: &BoundingBox) -> Microns {
        let dx = if self.max.x < other.min.x {
            other.min.x - self.max.x
        } else if other.max.x < self.min.x {
            self.min.x - other.max.x
        } else {
            0
        };

        let dy = if self.max.y < other.min.y {
            other.min.y - self.max.y
        } else if other.max.y < self.min.y {
            self.min.y - other.max.y
        } else {
            0
        };

        if dx == 0 && dy == 0 {
            0
        } else if dx == 0 {
            dy
        } else if dy == 0 {
            dx
        } else {
            ((dx * dx + dy * dy) as f64).sqrt().round() as i64
        }
    }

    /// Expand bounding box by margin in all directions.
    pub fn expand(&self, margin: Microns) -> Self {
        Self {
            min: Point2D::new(self.min.x - margin, self.min.y - margin),
            max: Point2D::new(self.max.x + margin, self.max.y + margin),
        }
    }
}
