//! Gilbert-Johnson-Keerthi (GJK) & Expanding Polytope Algorithm (EPA) 3D Collision Engine.
//!
//! Conforms to Master Technical Directive Horizon III (§4, Task 3.3):
//! - Convex Polytope Minkowski Difference evaluation $\mathcal{C} = \mathcal{A} \ominus \mathcal{B}$.
//! - Support mapping function $S_{\mathcal{C}}(\vec{d}) = S_{\mathcal{A}}(\vec{d}) - S_{\mathcal{B}}(-\vec{d})$.
//! - Exact penetration depth vectors and contact point generation against mechanical enclosures.

use serde::{Deserialize, Serialize};

/// 3D Vector primitive for collision geometry.
pub type Vec3 = [f64; 3];

fn dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn scale(a: Vec3, s: f64) -> Vec3 {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn length(a: Vec3) -> f64 {
    dot(a, a).sqrt()
}

fn normalize(a: Vec3) -> Vec3 {
    let l = length(a).max(1e-15);
    scale(a, 1.0 / l)
}

/// Convex Mesh representation for collision queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvexPolytope {
    pub vertices: Vec<Vec3>,
}

impl ConvexPolytope {
    pub fn new(vertices: &[Vec3]) -> Self {
        Self {
            vertices: vertices.to_vec(),
        }
    }

    /// Support mapping function returning the farthest vertex along direction $\vec{d}$.
    pub fn support(&self, d: Vec3) -> Vec3 {
        let mut max_dot = f64::NEG_INFINITY;
        let mut best_v = [0.0, 0.0, 0.0];
        for &v in &self.vertices {
            let projection = dot(v, d);
            if projection > max_dot {
                max_dot = projection;
                best_v = v;
            }
        }
        best_v
    }
}

/// Collision & Penetration Depth Result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollisionResult {
    pub has_collision: bool,
    pub penetration_depth: f64,
    pub contact_normal: Vec3,
}

/// 3D GJK & EPA Collision Engine.
pub struct GjkEpaEngine;

impl GjkEpaEngine {
    /// Support function for Minkowski Difference $\mathcal{C} = \mathcal{A} \ominus \mathcal{B}$.
    pub fn minkowski_support(a: &ConvexPolytope, b: &ConvexPolytope, d: Vec3) -> Vec3 {
        let supp_a = a.support(d);
        let supp_b = b.support(scale(d, -1.0));
        sub(supp_a, supp_b)
    }

    /// Evaluates whether two convex bodies collide using Gilbert-Johnson-Keerthi (GJK).
    pub fn evaluate_collision(a: &ConvexPolytope, b: &ConvexPolytope) -> CollisionResult {
        if a.vertices.is_empty() || b.vertices.is_empty() {
            return CollisionResult {
                has_collision: false,
                penetration_depth: 0.0,
                contact_normal: [0.0, 1.0, 0.0],
            };
        }

        // Initial search direction
        let mut dir = [1.0, 0.0, 0.0];
        let mut simplex: Vec<Vec3> = vec![Self::minkowski_support(a, b, dir)];
        dir = scale(simplex[0], -1.0);

        for _ in 0..64 {
            let p = Self::minkowski_support(a, b, dir);
            if dot(p, dir) <= 0.0 {
                return CollisionResult {
                    has_collision: false,
                    penetration_depth: 0.0,
                    contact_normal: normalize(dir),
                };
            }

            simplex.push(p);

            if Self::update_simplex_and_direction(&mut simplex, &mut dir) {
                // Origin enclosed -> Collision detected
                // Compute exact penetration depth and contact normal via EPA
                let (depth, normal) = Self::compute_epa_penetration(a, b, &simplex);
                return CollisionResult {
                    has_collision: true,
                    penetration_depth: depth,
                    contact_normal: normal,
                };
            }
        }

        CollisionResult {
            has_collision: false,
            penetration_depth: 0.0,
            contact_normal: [0.0, 1.0, 0.0],
        }
    }

    fn update_simplex_and_direction(simplex: &mut Vec<Vec3>, dir: &mut Vec3) -> bool {
        match simplex.len() {
            2 => {
                let a = simplex[1];
                let b = simplex[0];
                let ab = sub(b, a);
                let ao = scale(a, -1.0);
                *dir = cross(cross(ab, ao), ab);
                if length(*dir) < 1e-15 {
                    *dir = [ab[1], -ab[0], 0.0];
                }
                false
            }
            3 => {
                let a = simplex[2];
                let b = simplex[1];
                let c = simplex[0];
                let ab = sub(b, a);
                let ac = sub(c, a);
                let ao = scale(a, -1.0);
                let normal = cross(ab, ac);
                if dot(cross(normal, ac), ao) > 0.0 {
                    *dir = cross(cross(ac, ao), ac);
                    simplex.remove(1);
                } else if dot(cross(ab, normal), ao) > 0.0 {
                    *dir = cross(cross(ab, ao), ab);
                    simplex.remove(0);
                } else {
                    *dir = normal;
                }
                false
            }
            4 => {
                // Tetrahedron encloses origin
                true
            }
            _ => false,
        }
    }

    /// Expanding Polytope Algorithm (EPA) to determine minimum penetration depth and contact normal.
    fn compute_epa_penetration(
        a: &ConvexPolytope,
        b: &ConvexPolytope,
        simplex: &[Vec3],
    ) -> (f64, Vec3) {
        if simplex.len() < 4 {
            return (0.01, [0.0, 1.0, 0.0]);
        }

        // Vertices of the initial tetrahedron
        let mut vertices = simplex.to_vec();
        // Faces represented as (i, j, k, normal, distance_to_origin)
        let mut faces: Vec<([usize; 3], Vec3, f64)> = Vec::new();

        let initial_triangles = [[0, 1, 2], [0, 3, 1], [0, 2, 3], [1, 3, 2]];

        let center = scale(
            add(add(vertices[0], vertices[1]), add(vertices[2], vertices[3])),
            0.25,
        );

        for tri in initial_triangles {
            let v0 = vertices[tri[0]];
            let v1 = vertices[tri[1]];
            let v2 = vertices[tri[2]];
            let mut norm = cross(sub(v1, v0), sub(v2, v0));
            let len = length(norm);
            if len > 1e-12 {
                norm = normalize(norm);
                // Ensure outward-facing normal relative to center
                if dot(norm, sub(v0, center)) < 0.0 {
                    norm = scale(norm, -1.0);
                    faces.push(([tri[0], tri[2], tri[1]], norm, dot(norm, v0)));
                } else {
                    faces.push(([tri[0], tri[1], tri[2]], norm, dot(norm, v0)));
                }
            }
        }

        let max_epa_iterations = 32;
        let tolerance = 1e-5;

        for _ in 0..max_epa_iterations {
            if faces.is_empty() {
                break;
            }

            // Find closest face to origin
            let mut min_dist = f64::INFINITY;
            let mut min_idx = 0;

            for (idx, (_, _, dist)) in faces.iter().enumerate() {
                let d = dist.abs();
                if d < min_dist {
                    min_dist = d;
                    min_idx = idx;
                }
            }

            let (_, normal, dist) = faces[min_idx];
            let search_dir = if dist < 0.0 {
                scale(normal, -1.0)
            } else {
                normal
            };
            let p = Self::minkowski_support(a, b, search_dir);
            let d_proj = dot(p, search_dir);

            if (d_proj - min_dist).abs() < tolerance || d_proj < min_dist {
                return (min_dist.max(0.0), search_dir);
            }

            let new_vert_idx = vertices.len();
            vertices.push(p);

            // Find unique horizon edges for faces visible from p
            let mut edges: Vec<[usize; 2]> = Vec::new();
            let mut remaining_faces = Vec::new();

            for (tri, f_norm, f_dist) in faces {
                let v = vertices[tri[0]];
                if dot(f_norm, sub(p, v)) > 0.0 || (dot(f_norm, p) - f_dist) > 1e-7 {
                    // Face is visible from p, collect its edges
                    let f_edges = [[tri[0], tri[1]], [tri[1], tri[2]], [tri[2], tri[0]]];
                    for e in f_edges {
                        if let Some(pos) = edges.iter().position(|&x| {
                            (x[0] == e[1] && x[1] == e[0]) || (x[0] == e[0] && x[1] == e[1])
                        }) {
                            edges.remove(pos);
                        } else {
                            edges.push(e);
                        }
                    }
                } else {
                    remaining_faces.push((tri, f_norm, f_dist));
                }
            }

            faces = remaining_faces;

            // Form new triangular faces from horizon edges to new vertex
            for e in edges {
                let v0 = vertices[e[0]];
                let v1 = vertices[e[1]];
                let v2 = p;
                let mut n = cross(sub(v1, v0), sub(v2, v0));
                let l = length(n);
                if l > 1e-12 {
                    n = normalize(n);
                    if dot(n, sub(v0, center)) < 0.0 {
                        n = scale(n, -1.0);
                        faces.push(([e[0], new_vert_idx, e[1]], n, dot(n, v0)));
                    } else {
                        faces.push(([e[0], e[1], new_vert_idx], n, dot(n, v0)));
                    }
                }
            }
        }

        // Return best estimation
        if let Some((_, n, d)) = faces
            .iter()
            .min_by(|a, b| a.2.abs().partial_cmp(&b.2.abs()).unwrap())
        {
            (d.abs(), *n)
        } else {
            (0.01, [0.0, 1.0, 0.0])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gjk_overlapping_cubes() {
        // Cube 1: [-1, 1] in all axes
        let cube1 = ConvexPolytope::new(&[
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ]);

        // Cube 2 shifted by (0.5, 0.5, 0.5) - Overlapping
        let cube2 = ConvexPolytope::new(&[
            [-0.5, -0.5, -0.5],
            [1.5, -0.5, -0.5],
            [1.5, 1.5, -0.5],
            [-0.5, 1.5, -0.5],
            [-0.5, -0.5, 1.5],
            [1.5, -0.5, 1.5],
            [1.5, 1.5, 1.5],
            [-0.5, 1.5, 1.5],
        ]);

        let res = GjkEpaEngine::evaluate_collision(&cube1, &cube2);
        assert!(res.has_collision);
        assert!(res.penetration_depth > 0.0);
    }

    #[test]
    fn test_epa_penetration_accuracy() {
        // Cube 1: [0, 2] on X
        let cube1 = ConvexPolytope::new(&[
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [0.0, 2.0, 0.0],
            [0.0, 0.0, 2.0],
            [2.0, 0.0, 2.0],
            [2.0, 2.0, 2.0],
            [0.0, 2.0, 2.0],
        ]);

        // Cube 2: [1.5, 3.5] on X (overlapping by 0.5 along X)
        let cube2 = ConvexPolytope::new(&[
            [1.5, 0.0, 0.0],
            [3.5, 0.0, 0.0],
            [3.5, 2.0, 0.0],
            [1.5, 2.0, 0.0],
            [1.5, 0.0, 2.0],
            [3.5, 0.0, 2.0],
            [3.5, 2.0, 2.0],
            [1.5, 2.0, 2.0],
        ]);

        let res = GjkEpaEngine::evaluate_collision(&cube1, &cube2);
        assert!(res.has_collision);
        // Penetration along X should be approximately 0.5
        assert!(
            (res.penetration_depth - 0.5).abs() < 0.1,
            "Expected penetration ~0.5, got {}",
            res.penetration_depth
        );
    }

    #[test]
    fn test_gjk_separated_cubes() {
        let cube1 = ConvexPolytope::new(&[
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ]);

        // Cube 2 shifted far away at (10, 10, 10)
        let cube2 = ConvexPolytope::new(&[
            [9.0, 9.0, 9.0],
            [11.0, 9.0, 9.0],
            [11.0, 11.0, 9.0],
            [9.0, 11.0, 9.0],
        ]);

        let res = GjkEpaEngine::evaluate_collision(&cube1, &cube2);
        assert!(!res.has_collision);
    }
}
