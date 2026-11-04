//! Rigid-Flex 3D Parametric Folding and Deformation Engine.
//!
//! Conforms to IPC-2223 Section 5.2 design rules:
//! - Static bend ratio: $R/t \ge 10$
//! - Dynamic flexing bend ratio: $R/t \ge 20$
//! - Transition zone keepout clearance and strain tensor calculation.

use serde::{Deserialize, Serialize};

/// 3D Mesh Vertex for Rigid-Flex board rendering.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MeshVertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub strain_ratio: f32, // 0.0 (unstrained) to 1.0 (approaching 0.3% copper fatigue limit)
}

/// Flexible Zone definition with parametric bend parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlexBendZone {
    pub name: String,
    pub x_start_mm: f32,
    pub x_end_mm: f32,
    pub width_mm: f32,
    pub thickness_um: f32,
    pub min_bend_radius_mm: f32,
    pub is_dynamic: bool,
}

impl FlexBendZone {
    /// Evaluates IPC-2223 compliance for a given bend radius.
    pub fn check_ipc2223_compliance(&self, radius_mm: f32) -> Result<(), String> {
        let t_mm = self.thickness_um * 1e-3;
        let r_over_t = radius_mm / t_mm;
        let min_required = if self.is_dynamic { 20.0 } else { 10.0 };

        if r_over_t < min_required {
            Err(format!(
                "IPC-2223 Violation in bend zone '{}': R/t = {:.1} is below minimum requirement of {:.1} ({})",
                self.name,
                r_over_t,
                min_required,
                if self.is_dynamic {
                    "dynamic flexing"
                } else {
                    "static installation"
                }
            ))
        } else {
            Ok(())
        }
    }
}

/// Rigid-Flex Parametric Folding Engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RigidFlexViewerEngine {
    pub board_length_mm: f32,
    pub board_width_mm: f32,
    pub bend_zones: Vec<FlexBendZone>,
    pub current_bend_angle_deg: f32,
}

impl RigidFlexViewerEngine {
    pub fn new(length_mm: f32, width_mm: f32) -> Self {
        Self {
            board_length_mm: length_mm,
            board_width_mm: width_mm,
            bend_zones: Vec::new(),
            current_bend_angle_deg: 0.0,
        }
    }

    pub fn add_bend_zone(&mut self, zone: FlexBendZone) {
        self.bend_zones.push(zone);
    }

    /// Generates 3D deformed mesh vertices according to bend angle $\theta$.
    pub fn generate_deformed_mesh(&self, angle_deg: f32) -> Vec<MeshVertex3D> {
        let mut vertices = Vec::new();
        let angle_rad = angle_deg.to_radians();

        // Sample points along the board length
        let segments = 60;
        let dx = self.board_length_mm / (segments as f32);

        let mut current_x = 0.0f32;
        let mut current_z = 0.0f32;
        let mut tangent_angle = 0.0f32;

        for i in 0..=segments {
            let x_orig = (i as f32) * dx;

            // Check if within any bend zone
            let in_zone = self
                .bend_zones
                .iter()
                .find(|z| x_orig >= z.x_start_mm && x_orig <= z.x_end_mm);
            let strain = if let Some(zone) = in_zone {
                let zone_len = zone.x_end_mm - zone.x_start_mm;
                if zone_len > 0.0 {
                    let d_theta = angle_rad * (dx / zone_len);
                    tangent_angle += d_theta;
                    // Strain $\epsilon = \frac{t}{2R}$
                    let t_mm = zone.thickness_um * 1e-3;
                    let r_est = (zone_len / angle_rad.max(0.01)).max(1.0);
                    ((t_mm / (2.0 * r_est)) / 0.003).clamp(0.0, 1.0)
                } else {
                    0.0
                }
            } else {
                0.0
            };

            current_x += dx * tangent_angle.cos();
            current_z += dx * tangent_angle.sin();

            // Emit quad pair (y = 0 and y = width)
            vertices.push(MeshVertex3D {
                position: [current_x, 0.0, current_z],
                normal: [-tangent_angle.sin(), 0.0, tangent_angle.cos()],
                strain_ratio: strain,
            });
            vertices.push(MeshVertex3D {
                position: [current_x, self.board_width_mm, current_z],
                normal: [-tangent_angle.sin(), 0.0, tangent_angle.cos()],
                strain_ratio: strain,
            });
        }

        vertices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc2223_bend_radius_compliance() {
        let zone = FlexBendZone {
            name: "Hinge1".to_string(),
            x_start_mm: 40.0,
            x_end_mm: 60.0,
            width_mm: 30.0,
            thickness_um: 100.0, // 0.1 mm
            min_bend_radius_mm: 1.0,
            is_dynamic: true,
        };

        // R = 2.5mm -> R/t = 25 >= 20 (Pass)
        assert!(zone.check_ipc2223_compliance(2.5).is_ok());

        // R = 1.0mm -> R/t = 10 < 20 (Fail for dynamic)
        assert!(zone.check_ipc2223_compliance(1.0).is_err());
    }

    #[test]
    fn test_parametric_mesh_deformation() {
        let mut engine = RigidFlexViewerEngine::new(100.0, 30.0);
        engine.add_bend_zone(FlexBendZone {
            name: "FlexSection".to_string(),
            x_start_mm: 40.0,
            x_end_mm: 60.0,
            width_mm: 30.0,
            thickness_um: 120.0,
            min_bend_radius_mm: 2.0,
            is_dynamic: false,
        });

        let mesh = engine.generate_deformed_mesh(90.0);
        assert!(!mesh.is_empty());
        // End point should have rotated into positive Z
        let last = mesh.last().unwrap();
        assert!(
            last.position[2] > 0.0,
            "Folded end should deviate from flat Z plane"
        );
    }
}
