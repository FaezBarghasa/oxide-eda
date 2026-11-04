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

        let edges1 = [
            self.b.sub(&self.a),
            self.c.sub(&self.b),
            self.a.sub(&self.c),
        ];
        let edges2 = [
            other.b.sub(&other.a),
            other.c.sub(&other.b),
            other.a.sub(&other.c),
        ];

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

    pub fn aabb(&self) -> Aabb3d {
        let min = Vec3::new(
            self.a.x.min(self.b.x).min(self.c.x),
            self.a.y.min(self.b.y).min(self.c.y),
            self.a.z.min(self.b.z).min(self.c.z),
        );
        let max = Vec3::new(
            self.a.x.max(self.b.x).max(self.c.x),
            self.a.y.max(self.b.y).max(self.c.y),
            self.a.z.max(self.b.z).max(self.c.z),
        );
        Aabb3d::new(min, max)
    }

    pub fn centroid(&self) -> Vec3 {
        Vec3::new(
            (self.a.x + self.b.x + self.c.x) / 3.0,
            (self.a.y + self.b.y + self.c.y) / 3.0,
            (self.a.z + self.b.z + self.c.z) / 3.0,
        )
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

    /// Compute union of two 3D AABBs.
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
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

/// Node in a hierarchical 3D Bounding Volume Hierarchy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriangleBvhNode {
    Leaf {
        aabb: Aabb3d,
        triangles: Vec<Triangle3d>,
    },
    Internal {
        aabb: Aabb3d,
        left: Box<TriangleBvhNode>,
        right: Box<TriangleBvhNode>,
    },
}

impl TriangleBvhNode {
    pub fn aabb(&self) -> &Aabb3d {
        match self {
            Self::Leaf { aabb, .. } => aabb,
            Self::Internal { aabb, .. } => aabb,
        }
    }

    pub fn build(mut triangles: Vec<Triangle3d>) -> Self {
        if triangles.is_empty() {
            return Self::Leaf {
                aabb: Aabb3d::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
                triangles,
            };
        }

        if triangles.len() <= 4 {
            let mut aabb = triangles[0].aabb();
            for t in &triangles[1..] {
                aabb = aabb.union(&t.aabb());
            }
            return Self::Leaf { aabb, triangles };
        }

        let mut total_aabb = triangles[0].aabb();
        for t in &triangles[1..] {
            total_aabb = total_aabb.union(&t.aabb());
        }

        let extent_x = total_aabb.max.x - total_aabb.min.x;
        let extent_y = total_aabb.max.y - total_aabb.min.y;
        let extent_z = total_aabb.max.z - total_aabb.min.z;

        let axis = if extent_x >= extent_y && extent_x >= extent_z {
            0
        } else if extent_y >= extent_x && extent_y >= extent_z {
            1
        } else {
            2
        };

        triangles.sort_by(|t1, t2| {
            let c1 = t1.centroid();
            let c2 = t2.centroid();
            let v1 = match axis {
                0 => c1.x,
                1 => c1.y,
                _ => c1.z,
            };
            let v2 = match axis {
                0 => c2.x,
                1 => c2.y,
                _ => c2.z,
            };
            v1.partial_cmp(&v2).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mid = triangles.len() / 2;
        let right_tris = triangles.split_off(mid);
        let left_tris = triangles;

        let left = Box::new(Self::build(left_tris));
        let right = Box::new(Self::build(right_tris));
        let aabb = left.aabb().union(right.aabb());

        Self::Internal { aabb, left, right }
    }
}

/// 3D Bounding Volume Hierarchy for accelerating triangle mesh interference checks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriangleBvh {
    pub root: TriangleBvhNode,
}

impl TriangleBvh {
    /// Construct a top-down BVH tree from a collection of 3D triangles.
    pub fn build(triangles: Vec<Triangle3d>) -> Self {
        let root = TriangleBvhNode::build(triangles);
        Self { root }
    }

    /// Check if two BVH trees have any intersecting triangles using recursive dual-tree traversal.
    pub fn intersects(&self, other: &Self) -> bool {
        Self::collide_nodes(&self.root, &other.root)
    }

    fn collide_nodes(node_a: &TriangleBvhNode, node_b: &TriangleBvhNode) -> bool {
        if !node_a.aabb().intersects(node_b.aabb()) {
            return false;
        }

        match (node_a, node_b) {
            (
                TriangleBvhNode::Leaf {
                    triangles: tris_a, ..
                },
                TriangleBvhNode::Leaf {
                    triangles: tris_b, ..
                },
            ) => {
                for ta in tris_a {
                    for tb in tris_b {
                        if ta.intersects(tb) {
                            return true;
                        }
                    }
                }
                false
            }
            (TriangleBvhNode::Internal { left, right, .. }, TriangleBvhNode::Leaf { .. }) => {
                Self::collide_nodes(left, node_b) || Self::collide_nodes(right, node_b)
            }
            (TriangleBvhNode::Leaf { .. }, TriangleBvhNode::Internal { left, right, .. }) => {
                Self::collide_nodes(node_a, left) || Self::collide_nodes(node_a, right)
            }
            (
                TriangleBvhNode::Internal {
                    left: la,
                    right: ra,
                    ..
                },
                TriangleBvhNode::Internal {
                    left: lb,
                    right: rb,
                    ..
                },
            ) => {
                let vol_a = (node_a.aabb().max.x - node_a.aabb().min.x)
                    * (node_a.aabb().max.y - node_a.aabb().min.y)
                    * (node_a.aabb().max.z - node_a.aabb().min.z);
                let vol_b = (node_b.aabb().max.x - node_b.aabb().min.x)
                    * (node_b.aabb().max.y - node_b.aabb().min.y)
                    * (node_b.aabb().max.z - node_b.aabb().min.z);
                if vol_a >= vol_b {
                    Self::collide_nodes(la, node_b) || Self::collide_nodes(ra, node_b)
                } else {
                    Self::collide_nodes(node_a, lb) || Self::collide_nodes(node_a, rb)
                }
            }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bvh: Option<TriangleBvh>,
}

impl Body3d {
    pub fn with_mesh(mut self, mesh: Vec<Triangle3d>) -> Self {
        self.bvh = Some(TriangleBvh::build(mesh.clone()));
        self.mesh = Some(mesh);
        self
    }
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
                    // Check narrow-phase mesh collision with BVH acceleration if solid meshes are attached
                    let is_mesh_collision = match (&a.mesh, &b.mesh) {
                        (Some(m_a), Some(m_b)) => {
                            if let (Some(bvh_a), Some(bvh_b)) = (&a.bvh, &b.bvh) {
                                bvh_a.intersects(bvh_b)
                            } else if m_a.len() > 8 || m_b.len() > 8 {
                                let bvh_a = a
                                    .bvh
                                    .clone()
                                    .unwrap_or_else(|| TriangleBvh::build(m_a.clone()));
                                let bvh_b = b
                                    .bvh
                                    .clone()
                                    .unwrap_or_else(|| TriangleBvh::build(m_b.clone()));
                                bvh_a.intersects(&bvh_b)
                            } else {
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
            bvh: None,
        };

        let c1 = Body3d {
            id: "2".to_string(),
            reference: "C1".to_string(),
            aabb: Aabb3d::new(Vec3::new(10.2, 0.0, 0.0), Vec3::new(12.0, 2.0, 1.0)),
            height_mm: 1.0,
            mesh: None,
            bvh: None,
        };

        let c2 = Body3d {
            id: "3".to_string(),
            reference: "C2".to_string(),
            aabb: Aabb3d::new(Vec3::new(9.0, 5.0, 0.0), Vec3::new(12.0, 8.0, 1.0)),
            height_mm: 1.0,
            mesh: None,
            bvh: None,
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

    #[test]
    fn test_bvh_accelerated_mesh_collision_performance() {
        // Build two complex meshes (500 triangles each)
        let mut mesh_a = Vec::with_capacity(500);
        let mut mesh_b_far = Vec::with_capacity(500);
        let mut mesh_b_hit = Vec::with_capacity(500);

        for i in 0..500 {
            let offset = i as f64 * 0.1;
            mesh_a.push(Triangle3d::new(
                Vec3::new(offset, 0.0, 0.0),
                Vec3::new(offset + 0.1, 1.0, 0.0),
                Vec3::new(offset, 1.0, 1.0),
            ));

            mesh_b_far.push(Triangle3d::new(
                Vec3::new(offset + 100.0, 0.0, 0.0),
                Vec3::new(offset + 100.1, 1.0, 0.0),
                Vec3::new(offset + 100.0, 1.0, 1.0),
            ));

            mesh_b_hit.push(Triangle3d::new(
                Vec3::new(offset + 25.0, 0.5, -0.5),
                Vec3::new(offset + 25.0, 0.5, 1.5),
                Vec3::new(offset + 25.1, -0.5, 0.5),
            ));
        }

        let bvh_a = TriangleBvh::build(mesh_a.clone());
        let bvh_far = TriangleBvh::build(mesh_b_far.clone());
        let bvh_hit = TriangleBvh::build(mesh_b_hit.clone());

        // Performance timing: BVH disjoint test should finish in microseconds (< 500us vs 500x500 = 250,000 SAT tests)
        let t0 = std::time::Instant::now();
        assert!(!bvh_a.intersects(&bvh_far));
        let bvh_disjoint_us = t0.elapsed().as_micros();

        assert!(bvh_a.intersects(&bvh_hit));

        // Ensure clearance engine uses BVH properly
        let body_a = Body3d {
            id: "1".into(),
            reference: "U1".into(),
            aabb: bvh_a.root.aabb().clone(),
            height_mm: 5.0,
            mesh: Some(mesh_a),
            bvh: Some(bvh_a),
        };

        let body_b = Body3d {
            id: "2".into(),
            reference: "U2".into(),
            aabb: bvh_hit.root.aabb().clone(),
            height_mm: 5.0,
            mesh: Some(mesh_b_hit),
            bvh: Some(bvh_hit),
        };

        let violations = ClearanceEngine3d::check_clearances(&[body_a, body_b], 1.0, None);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].is_collision);
        assert!(
            bvh_disjoint_us < 2000,
            "BVH disjoint query took too long: {} us",
            bvh_disjoint_us
        );
    }
}
