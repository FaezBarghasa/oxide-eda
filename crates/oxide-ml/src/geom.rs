use serde::{Deserialize, Serialize};

use oxide_physics::Microns;

/// High-precision 2D Point for ML spatial tensors and routing geometry
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

    pub fn distance_to(&self, other: Point2D) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}
