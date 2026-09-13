//! Delaunay Triangulation using the Bowyer-Watson algorithm for planar obstacle meshing.

use uuid::Uuid;

use super::Triangle;
use crate::geometry::Point2D;

/// Perform 2D Delaunay Triangulation on a set of vertices using Bowyer-Watson.
pub fn bowyer_watson_delaunay(vertices: &[Point2D]) -> Vec<Triangle> {
    if vertices.len() < 3 {
        return Vec::new();
    }

    // 1. Determine bounding box for super-triangle
    let mut min_x = vertices[0].x;
    let mut min_y = vertices[0].y;
    let mut max_x = vertices[0].x;
    let mut max_y = vertices[0].y;

    for v in vertices.iter().skip(1) {
        min_x = min_x.min(v.x);
        min_y = min_y.min(v.y);
        max_x = max_x.max(v.x);
        max_y = max_y.max(v.y);
    }

    let dx = (max_x - min_x) as f64;
    let dy = (max_y - min_y) as f64;
    let delta_max = dx.max(dy).max(10_000.0);
    let mid_x = (min_x + max_x) as f64 / 2.0;
    let mid_y = (min_y + max_y) as f64 / 2.0;

    // Super-triangle vertices encompassing all input points
    let p1 = Point2D::new(
        (mid_x - 20.0 * delta_max) as i64,
        (mid_y - delta_max) as i64,
    );
    let p2 = Point2D::new(mid_x as i64, (mid_y + 20.0 * delta_max) as i64);
    let p3 = Point2D::new(
        (mid_x + 20.0 * delta_max) as i64,
        (mid_y - delta_max) as i64,
    );

    let super_triangle = [p1, p2, p3];
    let mut triangles = vec![super_triangle];

    // 2. Incrementally insert each vertex
    for vertex in vertices {
        let mut bad_triangles = Vec::new();
        let mut polygon_edges: Vec<[Point2D; 2]> = Vec::new();

        for tri in &triangles {
            if in_circumcircle(vertex, tri) {
                bad_triangles.push(*tri);
            }
        }

        // Find the boundary of the polygonal hole
        for tri in &bad_triangles {
            for i in 0..3 {
                let edge = [tri[i], tri[(i + 1) % 3]];
                // Check if edge is shared with any other bad triangle
                let mut shared = false;
                for other in &bad_triangles {
                    if other == tri {
                        continue;
                    }
                    if has_edge(other, edge) {
                        shared = true;
                        break;
                    }
                }
                if !shared {
                    polygon_edges.push(edge);
                }
            }
        }

        // Remove bad triangles
        triangles.retain(|t| !bad_triangles.contains(t));

        // Re-triangulate the polygonal hole with the new vertex
        for edge in polygon_edges {
            triangles.push([edge[0], edge[1], *vertex]);
        }
    }

    // 3. Remove triangles containing vertices of the super-triangle
    triangles.retain(|tri| !tri.iter().any(|v| super_triangle.contains(v)));

    triangles
        .into_iter()
        .map(|v| Triangle {
            id: Uuid::new_v4(),
            vertices: v,
            adjacent_triangles: Vec::new(),
            is_free: true,
        })
        .collect()
}

/// Test if point is inside circumcircle of triangle.
fn in_circumcircle(point: &Point2D, triangle: &[Point2D; 3]) -> bool {
    let ax = (triangle[0].x - point.x) as f64;
    let ay = (triangle[0].y - point.y) as f64;
    let bx = (triangle[1].x - point.x) as f64;
    let by = (triangle[1].y - point.y) as f64;
    let cx = (triangle[2].x - point.x) as f64;
    let cy = (triangle[2].y - point.y) as f64;

    let det = (ax * ax + ay * ay) * (bx * cy - cx * by) - (bx * bx + by * by) * (ax * cy - cx * ay)
        + (cx * cx + cy * cy) * (ax * by - bx * ay);

    // Orientation check (counter-clockwise)
    let ccw = (triangle[1].x - triangle[0].x) as f64 * (triangle[2].y - triangle[0].y) as f64
        - (triangle[1].y - triangle[0].y) as f64 * (triangle[2].x - triangle[0].x) as f64;

    if ccw > 0.0 { det > 0.0 } else { det < 0.0 }
}

fn has_edge(tri: &[Point2D; 3], edge: [Point2D; 2]) -> bool {
    for i in 0..3 {
        let e1 = tri[i];
        let e2 = tri[(i + 1) % 3];
        if (e1 == edge[0] && e2 == edge[1]) || (e1 == edge[1] && e2 == edge[0]) {
            return true;
        }
    }
    false
}
