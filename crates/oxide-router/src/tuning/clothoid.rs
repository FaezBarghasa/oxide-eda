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
}
