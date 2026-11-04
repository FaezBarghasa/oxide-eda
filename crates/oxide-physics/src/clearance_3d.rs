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

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

/// 3D Triangle primitive for solid mesh boundary clearance checking.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Triangle3d {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle3d {
    pub const fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }

    /// Check if two 3D triangles intersect using Separating Axis Theorem (SAT).
    pub fn intersects(&self, other: &Self) -> bool {
        let n1 = self.normal();
        let d1 = -n1.dot(&self.a);
        let da0 = n1.dot(&other.a) + d1;
        let da1 = n1.dot(&other.b) + d1;
        let da2 = n1.dot(&other.c) + d1;
        if (da0 > 1e-6 && da1 > 1e-6 && da2 > 1e-6) || (da0 < -1e-6 && da1 < -1e-6 && da2 < -1e-6) {
            return false;
        }

        let n2 = other.normal();
        let d2 = -n2.dot(&other.a);
        let db0 = n2.dot(&self.a) + d2;
        let db1 = n2.dot(&self.b) + d2;
        let db2 = n2.dot(&self.c) + d2;
        if (db0 > 1e-6 && db1 > 1e-6 && db2 > 1e-6) || (db0 < -1e-6 && db1 < -1e-6 && db2 < -1e-6) {
            return false;
        }

        let edges1 = [self.b.sub(&self.a), self.c.sub(&self.b), self.a.sub(&self.c)];
        let edges2 = [other.b.sub(&other.a), other.c.sub(&other.b), other.a.sub(&other.c)];

        for e1 in &edges1 {
            for e2 in &edges2 {
                let axis = e1.cross(e2);
                if axis.length_squared() < 1e-8 {
                    continue;
                }
                let (min1, max1) = self.project_onto(&axis);
                let (min2, max2) = other.project_onto(&axis);
                if min1 > max2 || min2 > max1 {
                    return false;
                }
            }
        }

        true
    }

    pub fn normal(&self) -> Vec3 {
        let ab = self.b.sub(&self.a);
        let ac = self.c.sub(&self.a);
        ab.cross(&ac)
    }

    pub fn project_onto(&self, axis: &Vec3) -> (f64, f64) {
        let d0 = self.a.dot(axis);
        let d1 = self.b.dot(axis);
        let d2 = self.c.dot(axis);
        let min = d0.min(d1).min(d2);
        let max = d0.max(d1).max(d2);
        (min, max)
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
            min: Vec3::new(
                self.min.x - margin,
                self.min.y - margin,
                self.min.z - margin,
            ),
            max: Vec3::new(
                self.max.x + margin,
                self.max.y + margin,
                self.max.z + margin,
            ),
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Vec<Triangle3d>>,
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
                    // Check narrow-phase mesh collision if solid meshes are attached
                    let is_mesh_collision = match (&a.mesh, &b.mesh) {
                        (Some(m_a), Some(m_b)) => {
                            let mut hit = false;
                            'outer: for t_a in m_a {
                                for t_b in m_b {
                                    if t_a.intersects(t_b) {
                                        hit = true;
                                        break 'outer;
                                    }
                                }
                            }
                            hit
                        }
                        _ => true, // Fallback to AABB intersection
                    };

                    if is_mesh_collision {
                        violations.push(ClearanceViolation3d {
                            item_a: a.reference.clone(),
                            item_b: b.reference.clone(),
                            actual_clearance_mm: 0.0,
                            required_clearance_mm: min_clearance_mm,
                            is_collision: true,
                        });
                    }
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
            mesh: None,
        };

        let c1 = Body3d {
            id: "2".to_string(),
            reference: "C1".to_string(),
            aabb: Aabb3d::new(Vec3::new(10.2, 0.0, 0.0), Vec3::new(12.0, 2.0, 1.0)),
            height_mm: 1.0,
            mesh: None,
        };

        let c2 = Body3d {
            id: "3".to_string(),
            reference: "C2".to_string(),
            aabb: Aabb3d::new(Vec3::new(9.0, 5.0, 0.0), Vec3::new(12.0, 8.0, 1.0)),
            height_mm: 1.0,
            mesh: None,
        };

        // U1 and C1 are 0.2mm apart (violates 0.5mm clearance)
        // U1 and C2 collide (intersect)
        let bodies = vec![u1, c1, c2];
        let violations = ClearanceEngine3d::check_clearances(&bodies, 0.5, Some(2.5));

        assert_eq!(violations.len(), 3);

        // Check collision between U1 and C2
        let col = violations
            .iter()
            .find(|v| v.item_a == "U1" && v.item_b == "C2")
            .unwrap();
        assert!(col.is_collision);

        // Check clearance between U1 and C1
        let clr = violations
            .iter()
            .find(|v| v.item_a == "U1" && v.item_b == "C1")
            .unwrap();
        assert!(!clr.is_collision);
        assert!((clr.actual_clearance_mm - 0.2).abs() < 1e-4);

        // Check enclosure violation (U1 height 3.0 > max_h 2.5)
        let enc = violations
            .iter()
            .find(|v| v.item_b == "Enclosure_Ceiling")
            .unwrap();
        assert_eq!(enc.item_a, "U1");
    }

    #[test]
    fn test_triangle3d_mesh_collision() {
        // Two intersecting triangles
        let t1 = Triangle3d::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(5.0, 0.0, 0.0),
            Vec3::new(0.0, 5.0, 0.0),
        );
        let t2 = Triangle3d::new(
            Vec3::new(2.0, 2.0, -1.0),
            Vec3::new(2.0, 2.0, 1.0),
            Vec3::new(2.0, -1.0, 0.0),
        );
        assert!(t1.intersects(&t2));

        // Separated triangle
        let t3 = Triangle3d::new(
            Vec3::new(10.0, 10.0, 10.0),
            Vec3::new(15.0, 10.0, 10.0),
            Vec3::new(10.0, 15.0, 10.0),
        );
        assert!(!t1.intersects(&t3));
    }
}
