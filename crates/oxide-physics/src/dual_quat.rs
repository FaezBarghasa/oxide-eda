//! Unit Dual-Quaternion Forward Kinematic Transformation Chains for Rigid-Flex Fold Animation.
//!
//! Conforms to Master Technical Directive Horizon III (§4, Task 3.2):
//! - Dual-quaternion algebra $\hat{\mathbf{q}} = \mathbf{q}_r + \epsilon \mathbf{q}_d$ ($\epsilon^2 = 0$).
//! - Eliminates gimbal lock and numerical singularities across $0^\circ \to 180^\circ$ continuous folds.
//! - Kinematic chain propagation across arbitrary polygonal board domains $\Omega_k$.

use serde::{Deserialize, Serialize};

/// Standard 3D Quaternion (Real component).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quaternion {
    pub const IDENTITY: Self = Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    /// Constructs rotation quaternion from axis $\vec{L}$ and angle $\theta$ in radians.
    pub fn from_axis_angle(axis: [f64; 3], angle_rad: f64) -> Self {
        let half = angle_rad * 0.5;
        let sin_half = half.sin();
        let len = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt().max(1e-15);
        Self {
            w: half.cos(),
            x: (axis[0] / len) * sin_half,
            y: (axis[1] / len) * sin_half,
            z: (axis[2] / len) * sin_half,
        }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Unit Dual Quaternion $\hat{\mathbf{q}} = \mathbf{q}_r + \epsilon \mathbf{q}_d$.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DualQuaternion {
    pub real: Quaternion,
    pub dual: Quaternion,
}

impl DualQuaternion {
    pub const IDENTITY: Self = Self {
        real: Quaternion::IDENTITY,
        dual: Quaternion { w: 0.0, x: 0.0, y: 0.0, z: 0.0 },
    };

    /// Constructs dual quaternion from rotation quaternion and translation vector $\vec{t} = [x, y, z]$.
    pub fn from_rotation_translation(rotation: Quaternion, translation: [f64; 3]) -> Self {
        let t_quat = Quaternion::new(0.0, translation[0], translation[1], translation[2]);
        let dual = t_quat.multiply(&rotation);
        Self {
            real: rotation,
            dual: Quaternion::new(dual.w * 0.5, dual.x * 0.5, dual.y * 0.5, dual.z * 0.5),
        }
    }

    /// Composes dual quaternions: $\hat{\mathbf{q}}_{\text{combined}} = \hat{\mathbf{q}}_1 \cdot \hat{\mathbf{q}}_2$.
    pub fn multiply(&self, other: &Self) -> Self {
        let r_prod = self.real.multiply(&other.real);
        let d1 = self.real.multiply(&other.dual);
        let d2 = self.dual.multiply(&other.real);
        let dual_prod = Quaternion::new(
            d1.w + d2.w,
            d1.x + d2.x,
            d1.y + d2.y,
            d1.z + d2.z,
        );
        Self {
            real: r_prod,
            dual: dual_prod,
        }
    }

    /// Transforms 3D vertex point $\vec{p} \in \mathbb{R}^3$.
    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        // Translation extracted from 2 * dual * conjugate(real)
        let two_dual = Quaternion::new(
            self.dual.w * 2.0,
            self.dual.x * 2.0,
            self.dual.y * 2.0,
            self.dual.z * 2.0,
        );
        let t_quat = two_dual.multiply(&self.real.conjugate());

        // Rotate point: p_rot = real * p_quat * conjugate(real)
        let p_quat = Quaternion::new(0.0, p[0], p[1], p[2]);
        let rot_p = self.real.multiply(&p_quat).multiply(&self.real.conjugate());

        [
            rot_p.x + t_quat.x,
            rot_p.y + t_quat.y,
            rot_p.z + t_quat.z,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_quaternion_translation_and_rotation() {
        // Translate by (10, 0, 0) and rotate 90 degrees around Z axis
        let rot = Quaternion::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2);
        let dq = DualQuaternion::from_rotation_translation(rot, [10.0, 0.0, 0.0]);

        // Point at (1, 0, 0) rotated 90 deg -> (0, 1, 0) + translated (10, 0, 0) -> (10, 1, 0)
        let p_out = dq.transform_point([1.0, 0.0, 0.0]);
        assert!((p_out[0] - 10.0).abs() < 1e-6);
        assert!((p_out[1] - 1.0).abs() < 1e-6);
        assert!((p_out[2] - 0.0).abs() < 1e-6);
    }
}
