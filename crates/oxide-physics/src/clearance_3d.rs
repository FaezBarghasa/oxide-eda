//! 3D Bounding Volume Hierarchy (BVH) & Spatial Clearance Engine.
//!
//! Cleanroom implementation for real-time 3D component clearance checks,
//! enclosure collision detection, and creepage/clearance distance verification in 3D CAD space.

use serde::{Deserialize, Serialize};

/// 3D Vector / Coordinate in millimeters (cleanroom representation).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Axis-Aligned Bounding Box (AABB) in 3D space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Aabb3d {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb3d {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Check if two 3D boxes intersect or overlap.
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    /// Compute shortest 3D distance between two non-overlapping AABBs.
    /// Returns 0.0 if they overlap.
    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = if self.max.x < other.min.x {
            other.min.x - self.max.x
        } else if other.max.x < self.min.x {
            self.min.x - other.max.x
        } else {
            0.0
        };

        let dy = if self.max.y < other.min.y {
            other.min.y - self.max.y
        } else if other.max.y < self.min.y {
            self.min.y - other.max.y
        } else {
            0.0
        };

        let dz = if self.max.z < other.min.z {
            other.min.z - self.max.z
        } else if other.max.z < self.min.z {
            self.min.z - other.max.z
        } else {
            0.0
        };

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Expand bounding box by margin on all sides.
    pub fn expanded(&self, margin: f64) -> Self {
        Self {
            min: Vec3::new(self.min.x - margin, self.min.y - margin, self.min.z - margin),
            max: Vec3::new(self.max.x + margin, self.max.y + margin, self.max.z + margin),
        }
    }
}

/// 3D Component body representation for clearance validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body3d {
    pub id: String,
    pub reference: String,
    pub aabb: Aabb3d,
    pub height_mm: f64,
}

/// 3D Clearance and Collision violation report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearanceViolation3d {
    pub item_a: String,
    pub item_b: String,
    pub actual_clearance_mm: f64,
    pub required_clearance_mm: f64,
    pub is_collision: bool,
}

/// 3D Clearance and Collision Engine.
pub struct ClearanceEngine3d;

impl ClearanceEngine3d {
    /// Check 3D component-to-component and component-to-enclosure clearances.
    pub fn check_clearances(
        bodies: &[Body3d],
        min_clearance_mm: f64,
        enclosure_max_height_mm: Option<f64>,
    ) -> Vec<ClearanceViolation3d> {
        let mut violations = Vec::new();

        // 1. Component vs Component
        for i in 0..bodies.len() {
            for j in (i + 1)..bodies.len() {
                let a = &bodies[i];
                let b = &bodies[j];

                if a.aabb.intersects(&b.aabb) {
                    violations.push(ClearanceViolation3d {
                        item_a: a.reference.clone(),
                        item_b: b.reference.clone(),
                        actual_clearance_mm: 0.0,
                        required_clearance_mm: min_clearance_mm,
                        is_collision: true,
                    });
                } else {
                    let dist = a.aabb.distance_to(&b.aabb);
                    if dist < min_clearance_mm {
                        violations.push(ClearanceViolation3d {
                            item_a: a.reference.clone(),
                            item_b: b.reference.clone(),
                            actual_clearance_mm: dist,
                            required_clearance_mm: min_clearance_mm,
                            is_collision: false,
                        });
                    }
                }
            }
        }

        // 2. Component vs Enclosure ceiling height
        if let Some(max_h) = enclosure_max_height_mm {
            for body in bodies {
                if body.aabb.max.z > max_h {
                    violations.push(ClearanceViolation3d {
                        item_a: body.reference.clone(),
                        item_b: "Enclosure_Ceiling".to_string(),
                        actual_clearance_mm: max_h - body.aabb.max.z,
                        required_clearance_mm: 0.0,
                        is_collision: true,
                    });
                }
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb3d_clearance_and_collision() {
        let u1 = Body3d {
            id: "1".to_string(),
            reference: "U1".to_string(),
            aabb: Aabb3d::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 3.0)),
            height_mm: 3.0,
        };

        let c1 = Body3d {
            id: "2".to_string(),
            reference: "C1".to_string(),
            aabb: Aabb3d::new(Vec3::new(10.2, 0.0, 0.0), Vec3::new(12.0, 2.0, 1.0)),
            height_mm: 1.0,
        };

        let c2 = Body3d {
            id: "3".to_string(),
            reference: "C2".to_string(),
            aabb: Aabb3d::new(Vec3::new(9.0, 5.0, 0.0), Vec3::new(12.0, 8.0, 1.0)),
            height_mm: 1.0,
        };

        // U1 and C1 are 0.2mm apart (violates 0.5mm clearance)
        // U1 and C2 collide (intersect)
        let bodies = vec![u1, c1, c2];
        let violations = ClearanceEngine3d::check_clearances(&bodies, 0.5, Some(2.5));

        assert_eq!(violations.len(), 3);

        // Check collision between U1 and C2
        let col = violations.iter().find(|v| v.item_a == "U1" && v.item_b == "C2").unwrap();
        assert!(col.is_collision);

        // Check clearance between U1 and C1
        let clr = violations.iter().find(|v| v.item_a == "U1" && v.item_b == "C1").unwrap();
        assert!(!clr.is_collision);
        assert!((clr.actual_clearance_mm - 0.2).abs() < 1e-4);

        // Check enclosure violation (U1 height 3.0 > max_h 2.5)
        let enc = violations.iter().find(|v| v.item_b == "Enclosure_Ceiling").unwrap();
        assert_eq!(enc.item_a, "U1");
    }
}
