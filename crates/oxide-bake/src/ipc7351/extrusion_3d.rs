use oxide_library::harvester::PackageDimensions;
use oxide_library::primitive::footprint::{Body3D, BodyShape, Polygon};

/// Procedural 3D B-Rep Generator for electronic packages.
pub struct Package3DExtruder;

impl Package3DExtruder {
    /// Synthesizes an embedded `Body3D` primitive for procedural viewport rendering.
    pub fn synthesize_body_3d(dimensions: &PackageDimensions) -> Body3D {
        let length_x = dimensions.body_length_mm[1] as f32; // nom
        let width_y = dimensions.body_width_mm[1] as f32;   // nom
        let height_z = dimensions.seated_height_mm[1] as f32; // nom

        let half_x = length_x / 2.0;
        let half_y = width_y / 2.0;
        let chamfer = (half_x * 0.15).min(0.5);

        // Outline with Pin 1 chamfer on top-left (-half_x, half_y)
        let outline = Polygon::new(vec![
            [(-half_x + chamfer) as f64, half_y as f64],
            [half_x as f64, half_y as f64],
            [half_x as f64, -half_y as f64],
            [-half_x as f64, -half_y as f64],
            [-half_x as f64, (half_y - chamfer) as f64],
        ]);

        Body3D {
            shape: BodyShape::Extrude,
            height_mm: height_z.max(0.5),
            offset_z_mm: 0.0,
            top_color: [0.18, 0.18, 0.18, 1.0],  // Dark IC epoxy black
            side_color: [0.22, 0.22, 0.22, 1.0], // Dark charcoal sides
            outline: Some(outline),
        }
    }

    /// Procedurally extrudes a 3D boundary-representation mesh (Wavefront OBJ bytes).
    /// Centroid is strictly anchored to (0, 0, 0) matching footprint origin.
    pub fn synthesize_mesh_obj(dimensions: &PackageDimensions) -> Vec<u8> {
        let length_x = dimensions.body_length_mm[1];
        let width_y = dimensions.body_width_mm[1];
        let height_z = dimensions.seated_height_mm[1].max(0.5);

        let hx = length_x / 2.0;
        let hy = width_y / 2.0;
        let hz = height_z;

        let mut obj = String::new();
        obj.push_str("# Oxide EDA Procedural Package B-Rep\n");
        obj.push_str(&format!("o {}\n", dimensions.package_class));

        // Bottom vertices (z = 0)
        obj.push_str(&format!("v {:.4} {:.4} {:.4}\n", -hx, -hy, 0.0));
        obj.push_str(&format!("v {:.4} {:.4} {:.4}\n", hx, -hy, 0.0));
        obj.push_str(&format!("v {:.4} {:.4} {:.4}\n", hx, hy, 0.0));
        obj.push_str(&format!("v {:.4} {:.4} {:.4}\n", -hx, hy, 0.0));

        // Top vertices (z = hz)
        obj.push_str(&format!("v {:.4} {:.4} {:.4}\n", -hx, -hy, hz));
        obj.push_str(&format!("v {:.4} {:.4} {:.4}\n", hx, -hy, hz));
        obj.push_str(&format!("v {:.4} {:.4} {:.4}\n", hx, hy, hz));
        obj.push_str(&format!("v {:.4} {:.4} {:.4}\n", -hx, hy, hz));

        // Normals
        obj.push_str("vn 0.0 0.0 -1.0\n"); // 1: bottom
        obj.push_str("vn 0.0 0.0 1.0\n");  // 2: top
        obj.push_str("vn 0.0 -1.0 0.0\n"); // 3: south
        obj.push_str("vn 1.0 0.0 0.0\n");  // 4: east
        obj.push_str("vn 0.0 1.0 0.0\n");  // 5: north
        obj.push_str("vn -1.0 0.0 0.0\n"); // 6: west

        // Faces (1-indexed)
        // Bottom: 4 3 2 1
        obj.push_str("f 4//1 3//1 2//1 1//1\n");
        // Top: 5 6 7 8
        obj.push_str("f 5//2 6//2 7//2 8//2\n");
        // South: 1 2 6 5
        obj.push_str("f 1//3 2//3 6//3 5//3\n");
        // East: 2 3 7 6
        obj.push_str("f 2//4 3//4 7//4 6//4\n");
        // North: 3 4 8 7
        obj.push_str("f 3//5 4//5 8//5 7//5\n");
        // West: 4 1 5 8
        obj.push_str("f 4//6 1//6 5//6 8//6\n");

        obj.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural_body_3d_generation() {
        let dims = PackageDimensions::standard_soic(8);
        let body = Package3DExtruder::synthesize_body_3d(&dims);
        assert!(body.height_mm > 1.0);
        assert_eq!(body.offset_z_mm, 0.0);
        assert!(body.outline.is_some());
    }

    #[test]
    fn procedural_mesh_obj_contains_vertices() {
        let dims = PackageDimensions::standard_soic(8);
        let obj_bytes = Package3DExtruder::synthesize_mesh_obj(&dims);
        let obj_str = String::from_utf8(obj_bytes).expect("valid utf8");
        assert!(obj_str.contains("v -"));
        assert!(obj_str.contains("f 5//2 6//2 7//2 8//2"));
    }
}
