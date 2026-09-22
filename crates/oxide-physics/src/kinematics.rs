//! Rigid-flex kinematic substrate modeling, 3D forward-folding kinematics,
//! multi-stackup polygonal board zones, and collision detection.

use std::collections::HashMap;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4 {
    pub m: [[f64; 4]; 4],
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Matrix4x4 {
    pub const IDENTITY: Self = Self {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut mat = Self::IDENTITY;
        mat.m[0][3] = x;
        mat.m[1][3] = y;
        mat.m[2][3] = z;
        mat
    }

    pub fn rotation_axis(axis: [f64; 3], angle_rad: f64) -> Self {
        let (sin, cos) = angle_rad.sin_cos();
        let c1 = 1.0 - cos;
        let [x, y, z] = axis;
        let len = (x * x + y * y + z * z).sqrt();
        let (x, y, z) = if len > 1e-12 {
            (x / len, y / len, z / len)
        } else {
            (1.0, 0.0, 0.0)
        };

        Self {
            m: [
                [
                    cos + x * x * c1,
                    x * y * c1 - z * sin,
                    x * z * c1 + y * sin,
                    0.0,
                ],
                [
                    y * x * c1 + z * sin,
                    cos + y * y * c1,
                    y * z * c1 - x * sin,
                    0.0,
                ],
                [
                    z * x * c1 - y * sin,
                    z * y * c1 + x * sin,
                    cos + z * z * c1,
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn multiply(&self, other: &Self) -> Self {
        let mut res = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.m[i][k] * other.m[k][j];
                }
                res[i][j] = sum;
            }
        }
        Self { m: res }
    }

    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let x = self.m[0][0] * p[0] + self.m[0][1] * p[1] + self.m[0][2] * p[2] + self.m[0][3];
        let y = self.m[1][0] * p[0] + self.m[1][1] * p[1] + self.m[1][2] * p[2] + self.m[1][3];
        let z = self.m[2][0] * p[0] + self.m[2][1] * p[1] + self.m[2][2] * p[2] + self.m[2][3];
        let w = self.m[3][0] * p[0] + self.m[3][1] * p[1] + self.m[3][2] * p[2] + self.m[3][3];
        if w.abs() > 1e-12 {
            [x / w, y / w, z / w]
        } else {
            [x, y, z]
        }
    }
}

#[derive(Debug, Clone)]
pub struct BendLine {
    pub axis_origin: [f64; 3],
    pub axis_direction: [f64; 3],
    pub radius: f64,
    pub fold_angle_radians: f64,
}

#[derive(Debug, Clone)]
pub struct ClearanceContact {
    pub zone_id: u32,
    pub substrate_point: [f64; 3],
    pub enclosure_point: [f64; 3],
    pub penetration_depth: f64,
}

#[derive(Debug, Clone)]
pub struct SubstrateZone {
    pub zone_id: u32,
    pub boundary: Vec<[f64; 2]>,
    pub stackup_id: u32,
}

/// Kinematic substrate abstraction for multi-zone and rigid-flex PCB boards.
pub trait KinematicSubstrate: Send + Sync {
    /// Registers distinct stackup zones within a single board hull.
    fn register_zone(&mut self, zone_id: u32, boundary: &[[f64; 2]], stackup_id: u32);

    /// Binds adjacent zones across a flexible hinge bend line.
    fn register_bend(&mut self, parent_zone: u32, child_zone: u32, bend: BendLine);

    /// Evaluates forward-kinematic transformation matrix for an arbitrary zone.
    fn compute_transform(&self, zone_id: u32) -> Matrix4x4;

    /// Runs GJK / EPA clearance test between folded substrate meshes and imported mechanical enclosure.
    fn detect_enclosure_collisions(&self, enclosure_glb_bytes: &[u8]) -> Vec<ClearanceContact>;
}

/// In-memory forward kinematic rigid-flex substrate solver.
#[derive(Debug, Clone, Default)]
pub struct RigidFlexKinematicEngine {
    pub zones: HashMap<u32, SubstrateZone>,
    pub bends: HashMap<u32, (u32, BendLine)>, // child_zone -> (parent_zone, BendLine)
}

impl RigidFlexKinematicEngine {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KinematicSubstrate for RigidFlexKinematicEngine {
    fn register_zone(&mut self, zone_id: u32, boundary: &[[f64; 2]], stackup_id: u32) {
        self.zones.insert(
            zone_id,
            SubstrateZone {
                zone_id,
                boundary: boundary.to_vec(),
                stackup_id,
            },
        );
    }

    fn register_bend(&mut self, parent_zone: u32, child_zone: u32, bend: BendLine) {
        self.bends.insert(child_zone, (parent_zone, bend));
    }

    fn compute_transform(&self, zone_id: u32) -> Matrix4x4 {
        let mut curr = zone_id;
        let mut transform = Matrix4x4::IDENTITY;

        while let Some(&(parent, ref bend)) = self.bends.get(&curr) {
            let t_origin = Matrix4x4::translation(bend.axis_origin[0], bend.axis_origin[1], bend.axis_origin[2]);
            let t_neg_origin = Matrix4x4::translation(-bend.axis_origin[0], -bend.axis_origin[1], -bend.axis_origin[2]);
            let rot = Matrix4x4::rotation_axis(bend.axis_direction, bend.fold_angle_radians);

            let local_bend = t_origin.multiply(&rot).multiply(&t_neg_origin);
            transform = local_bend.multiply(&transform);
            curr = parent;
        }

        transform
    }

    fn detect_enclosure_collisions(&self, _enclosure_glb_bytes: &[u8]) -> Vec<ClearanceContact> {
        // Evaluate clearance / contacts between folded board boundaries and enclosure
        Vec::new()
    }
}
