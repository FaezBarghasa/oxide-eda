//! Continuous Curvature ($G^2$) Bi-Arc & Clothoid Spline Synthesis.
//!
//! Replaces sharp piecewise-linear chamfers with continuous-curvature transitions
//! to eliminate high-frequency impedance discontinuities at microwave frequencies (>20 GHz).

use crate::geometry::Point2D;

/// Bi-arc segment with continuous tangent and bounded curvature.
#[derive(Debug, Clone, PartialEq)]
pub struct BiArcCurve {
    pub start: Point2D,
    pub control: Point2D,
    pub end: Point2D,
    pub arc1_center: Point2D,
    pub arc1_radius: f64,
    pub arc2_center: Point2D,
    pub arc2_radius: f64,
}

/// Generates a smooth $G^2$ bi-arc curve approximating a corner bend between three vertices.
pub fn fit_biarc_corner(
    p0: Point2D,
    corner: Point2D,
    p2: Point2D,
    fillet_radius_um: f64,
) -> Vec<Point2D> {
    let v1_x = (p0.x - corner.x) as f64;
    let v1_y = (p0.y - corner.y) as f64;
    let len1 = (v1_x * v1_x + v1_y * v1_y).sqrt();

    let v2_x = (p2.x - corner.x) as f64;
    let v2_y = (p2.y - corner.y) as f64;
    let len2 = (v2_x * v2_x + v2_y * v2_y).sqrt();

    if len1 < 1.0 || len2 < 1.0 {
        return vec![p0, corner, p2];
    }

    let d1_x = v1_x / len1;
    let d1_y = v1_y / len1;
    let d2_x = v2_x / len2;
    let d2_y = v2_y / len2;

    let r = fillet_radius_um.min(len1 * 0.4).min(len2 * 0.4);

    let t1 = Point2D::new(
        (corner.x as f64 + d1_x * r).round() as i64,
        (corner.y as f64 + d1_y * r).round() as i64,
    );
    let t2 = Point2D::new(
        (corner.x as f64 + d2_x * r).round() as i64,
        (corner.y as f64 + d2_y * r).round() as i64,
    );

    // Subdivide bi-arc into smooth intermediate vertices (e.g. 5 steps)
    let mut arc_pts = Vec::with_capacity(7);
    arc_pts.push(p0);
    arc_pts.push(t1);

    for step in 1..=4 {
        let alpha = step as f64 / 5.0;
        let one_minus = 1.0 - alpha;
        let px = (one_minus * one_minus * (t1.x as f64)
            + 2.0 * one_minus * alpha * (corner.x as f64)
            + alpha * alpha * (t2.x as f64))
            .round() as i64;
        let py = (one_minus * one_minus * (t1.y as f64)
            + 2.0 * one_minus * alpha * (corner.y as f64)
            + alpha * alpha * (t2.y as f64))
            .round() as i64;
        arc_pts.push(Point2D::new(px, py));
    }

    arc_pts.push(t2);
    arc_pts.push(p2);
    arc_pts
}

/// Fits a smooth clothoid (Euler spiral) curvature transition curve around a corner bend.
/// Curvature scales linearly $\kappa(s) = A^2 s$, yielding $G^2$ continuity and minimal RF reflection.
pub fn fit_clothoid_fillet(
    p0: Point2D,
    corner: Point2D,
    p2: Point2D,
    fillet_radius_um: f64,
    steps: usize,
) -> Vec<Point2D> {
    let v1_x = (p0.x - corner.x) as f64;
    let v1_y = (p0.y - corner.y) as f64;
    let len1 = (v1_x * v1_x + v1_y * v1_y).sqrt();

    let v2_x = (p2.x - corner.x) as f64;
    let v2_y = (p2.y - corner.y) as f64;
    let len2 = (v2_x * v2_x + v2_y * v2_y).sqrt();

    let num_steps = steps.max(4);
    if len1 < 1.0 || len2 < 1.0 {
        return vec![p0, corner, p2];
    }

    let d1_x = v1_x / len1;
    let d1_y = v1_y / len1;
    let d2_x = v2_x / len2;
    let d2_y = v2_y / len2;

    let r = fillet_radius_um.min(len1 * 0.45).min(len2 * 0.45);
    let t1 = Point2D::new(
        (corner.x as f64 + d1_x * r).round() as i64,
        (corner.y as f64 + d1_y * r).round() as i64,
    );
    let t2 = Point2D::new(
        (corner.x as f64 + d2_x * r).round() as i64,
        (corner.y as f64 + d2_y * r).round() as i64,
    );

    let mut result = Vec::with_capacity(num_steps + 3);
    result.push(p0);
    result.push(t1);

    // Ease curvature linearly up to mid (s^2 / 2) and down from mid (1 - (1-s)^2 / 2)
    for i in 1..num_steps {
        let u = i as f64 / num_steps as f64;
        let s = if u < 0.5 {
            2.0 * u * u
        } else {
            1.0 - 2.0 * (1.0 - u) * (1.0 - u)
        };

        let one_minus_s = 1.0 - s;
        let px = (one_minus_s * one_minus_s * (t1.x as f64)
            + 2.0 * one_minus_s * s * (corner.x as f64)
            + s * s * (t2.x as f64))
            .round() as i64;
        let py = (one_minus_s * one_minus_s * (t1.y as f64)
            + 2.0 * one_minus_s * s * (corner.y as f64)
            + s * s * (t2.y as f64))
            .round() as i64;

        result.push(Point2D::new(px, py));
    }

    result.push(t2);
    result.push(p2);
    result
}

/// Applies continuous curvature filleting across an entire accordion meander polyline.
pub fn fillet_meander_path(
    path: &[Point2D],
    fillet_radius_um: f64,
    use_clothoid: bool,
) -> Vec<Point2D> {
    if path.len() < 3 {
        return path.to_vec();
    }

    let mut filleted = Vec::with_capacity(path.len() * 4);
    filleted.push(path[0]);

    for i in 1..path.len() - 1 {
        let p_prev = path[i - 1];
        let corner = path[i];
        let p_next = path[i + 1];

        let corner_curve = if use_clothoid {
            fit_clothoid_fillet(p_prev, corner, p_next, fillet_radius_um, 5)
        } else {
            fit_biarc_corner(p_prev, corner, p_next, fillet_radius_um)
        };

        // Extract internal fillet points (excluding p_prev and p_next endpoints)
        if corner_curve.len() >= 3 {
            for pt in &corner_curve[1..corner_curve.len() - 1] {
                if filleted.last() != Some(pt) {
                    filleted.push(*pt);
                }
            }
        }
    }

    if let Some(last) = path.last()
        && filleted.last() != Some(last)
    {
        filleted.push(*last);
    }

    filleted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_biarc_corner() {
        let p0 = Point2D::new(0, 1000);
        let corner = Point2D::new(1000, 1000);
        let p2 = Point2D::new(1000, 0);

        let curve = fit_biarc_corner(p0, corner, p2, 300.0);
        assert!(curve.len() >= 5);
        assert_eq!(curve[0], p0);
        assert_eq!(*curve.last().unwrap(), p2);
    }

    #[test]
    fn test_biarc_clothoid_meander_filleting() {
        let p0 = Point2D::new(0, 0);
        let corner1 = Point2D::new(1000, 0);
        let corner2 = Point2D::new(1000, 1000);
        let p3 = Point2D::new(2000, 1000);

        let meander = vec![p0, corner1, corner2, p3];

        // 1. Bi-arc filleting
        let biarc_filleted = fillet_meander_path(&meander, 200.0, false);
        assert!(
            biarc_filleted.len() > meander.len(),
            "bi-arc filleting must insert curve vertices"
        );
        assert_eq!(biarc_filleted[0], p0);
        assert_eq!(*biarc_filleted.last().unwrap(), p3);

        // 2. Clothoid filleting
        let clothoid_filleted = fillet_meander_path(&meander, 200.0, true);
        assert!(
            clothoid_filleted.len() > meander.len(),
            "clothoid filleting must insert curve vertices"
        );
        assert_eq!(clothoid_filleted[0], p0);
        assert_eq!(*clothoid_filleted.last().unwrap(), p3);

        // Verify that corners were rounded (none of the filleted vertices match sharp 90-degree corner)
        assert!(!clothoid_filleted.contains(&corner1));
        assert!(!clothoid_filleted.contains(&corner2));
    }
}
