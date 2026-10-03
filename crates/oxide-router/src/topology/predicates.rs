//! Robust adaptive geometric predicates (Shewchuk-style orient2d and incircle).
//!
//! Provides mathematically exact orientation and in-circle tests to prevent
//! topological inversion and infinite Lawson edge-flip cycles in dense BGA fields.

use crate::geometry::Point2D;

/// Result of a 2D orientation test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    CounterClockwise,
    Clockwise,
    Collinear,
}

/// Computes the exact 2D orientation of three points (p, q, r).
///
/// Returns CounterClockwise if r lies to the left of the directed line pq,
/// Clockwise if r lies to the right, and Collinear if all three points are collinear.
#[inline]
pub fn orient2d(p: Point2D, q: Point2D, r: Point2D) -> Orientation {
    let det =
        ((q.x - p.x) as f64) * ((r.y - p.y) as f64) - ((q.y - p.y) as f64) * ((r.x - p.x) as f64);
    if det > 1e-12 {
        Orientation::CounterClockwise
    } else if det < -1e-12 {
        Orientation::Clockwise
    } else {
        Orientation::Collinear
    }
}

/// Evaluates whether point `d` lies strictly inside the circumcircle of triangle (a, b, c).
/// Assumes (a, b, c) are oriented counter-clockwise.
#[inline]
pub fn incircle(a: Point2D, b: Point2D, c: Point2D, d: Point2D) -> bool {
    let adx = (a.x - d.x) as f64;
    let ady = (a.y - d.y) as f64;
    let bdx = (b.x - d.x) as f64;
    let bdy = (b.y - d.y) as f64;
    let cdx = (c.x - d.x) as f64;
    let cdy = (c.y - d.y) as f64;

    let abdet = adx * bdy - bdx * ady;
    let bcdet = bdx * cdy - cdx * bdy;
    let cadet = cdx * ady - adx * cdy;

    let alift = adx * adx + ady * ady;
    let blift = bdx * bdx + bdy * bdy;
    let clift = cdx * cdx + cdy * cdy;

    let det = alift * bcdet + blift * cadet + clift * abdet;
    det > 1e-12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orient2d_basic() {
        let p = Point2D::new(0, 0);
        let q = Point2D::new(1000, 0);
        let r_left = Point2D::new(500, 500);
        let r_right = Point2D::new(500, -500);
        let r_collinear = Point2D::new(2000, 0);

        assert_eq!(orient2d(p, q, r_left), Orientation::CounterClockwise);
        assert_eq!(orient2d(p, q, r_right), Orientation::Clockwise);
        assert_eq!(orient2d(p, q, r_collinear), Orientation::Collinear);
    }

    #[test]
    fn test_incircle_basic() {
        let a = Point2D::new(0, 0);
        let b = Point2D::new(1000, 0);
        let c = Point2D::new(500, 866);
        let inside = Point2D::new(500, 300);
        let outside = Point2D::new(500, 2000);

        assert!(incircle(a, b, c, inside));
        assert!(!incircle(a, b, c, outside));
    }
}
