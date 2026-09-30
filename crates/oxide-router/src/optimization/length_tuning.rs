//! Length tuning optimizer for differential pair phase skew matching and bus delay tuning.

use std::sync::Arc;

use oxide_physics::Microns;
use oxide_rules::ConstraintManager;

use crate::{RouteSegment, RoutingPath};

#[derive(Debug, Clone, PartialEq)]
pub struct TuningResult {
    pub positive_length: Microns,
    pub negative_length: Microns,
    pub length_difference: Microns,
}

#[derive(Debug, Clone)]
pub struct LengthTuningOptimizer {
    pub rules: Arc<ConstraintManager>,
}

impl LengthTuningOptimizer {
    pub fn new(rules: Arc<ConstraintManager>) -> Self {
        Self { rules }
    }

    /// Match lengths between positive and negative traces of a differential pair.
    pub fn tune_differential_pair(
        &self,
        pos_path: &mut RoutingPath,
        neg_path: &mut RoutingPath,
    ) -> TuningResult {
        let len_pos = pos_path.total_length;
        let len_neg = neg_path.total_length;

        if len_pos > len_neg {
            let delta = len_pos - len_neg;
            self.add_meander(neg_path, delta);
        } else if len_neg > len_pos {
            let delta = len_neg - len_pos;
            self.add_meander(pos_path, delta);
        }

        TuningResult {
            positive_length: pos_path.total_length,
            negative_length: neg_path.total_length,
            length_difference: (pos_path.total_length - neg_path.total_length).abs(),
        }
    }

    fn add_meander(&self, path: &mut RoutingPath, extra: Microns) {
        if path.segments.is_empty() || extra <= 0 {
            return;
        }

        let last_seg = path.segments.pop().unwrap();
        let amplitude: Microns = 400; // 400µm amplitude
        let pitch: Microns = 300;     // 300µm pitch per bump
        let seg_len = last_seg.length();

        if seg_len < pitch {
            // Segment too short for meander, insert single detour
            let (dx, dy) = last_seg.start_point.direction_to(last_seg.end_point);
            let (perp_x, perp_y) = (-dy, dx);
            let p_out = crate::geometry::Point2D::new(
                last_seg.start_point.x + (perp_x * amplitude as f64).round() as i64,
                last_seg.start_point.y + (perp_y * amplitude as f64).round() as i64,
            );
            path.segments.push(RouteSegment {
                start_point: last_seg.start_point,
                end_point: p_out,
                width: last_seg.width,
                layer: last_seg.layer,
                net_id: last_seg.net_id,
                segment_type: last_seg.segment_type,
            });
            path.segments.push(RouteSegment {
                start_point: p_out,
                end_point: last_seg.end_point,
                width: last_seg.width,
                layer: last_seg.layer,
                net_id: last_seg.net_id,
                segment_type: last_seg.segment_type,
            });
            path.total_length = path.segments.iter().map(|s| s.length()).sum();
            return;
        }

        // Multi-cycle accordion along the segment direction
        let (dx, dy) = last_seg.start_point.direction_to(last_seg.end_point);
        let (perp_x, perp_y) = (-dy, dx);

        let mut current = last_seg.start_point;
        let mut remaining = extra;
        let mut flip = true;

        while current.distance_to(last_seg.end_point) > pitch && remaining > 0 {
            let next_base = crate::geometry::Point2D::new(
                current.x + (dx * pitch as f64).round() as i64,
                current.y + (dy * pitch as f64).round() as i64,
            );
            let sign = if flip { 1.0 } else { -1.0 };
            let p1 = crate::geometry::Point2D::new(
                current.x + (perp_x * amplitude as f64 * sign).round() as i64,
                current.y + (perp_y * amplitude as f64 * sign).round() as i64,
            );
            let p2 = crate::geometry::Point2D::new(
                next_base.x + (perp_x * amplitude as f64 * sign).round() as i64,
                next_base.y + (perp_y * amplitude as f64 * sign).round() as i64,
            );

            let bump_len = current.distance_to(p1) + p1.distance_to(p2) + p2.distance_to(next_base);
            let added = bump_len.saturating_sub(pitch);

            path.segments.push(RouteSegment {
                start_point: current,
                end_point: p1,
                width: last_seg.width,
                layer: last_seg.layer,
                net_id: last_seg.net_id,
                segment_type: last_seg.segment_type,
            });
            path.segments.push(RouteSegment {
                start_point: p1,
                end_point: p2,
                width: last_seg.width,
                layer: last_seg.layer,
                net_id: last_seg.net_id,
                segment_type: last_seg.segment_type,
            });
            path.segments.push(RouteSegment {
                start_point: p2,
                end_point: next_base,
                width: last_seg.width,
                layer: last_seg.layer,
                net_id: last_seg.net_id,
                segment_type: last_seg.segment_type,
            });

            current = next_base;
            remaining = remaining.saturating_sub(added);
            flip = !flip;
        }

        if current != last_seg.end_point {
            path.segments.push(RouteSegment {
                start_point: current,
                end_point: last_seg.end_point,
                width: last_seg.width,
                layer: last_seg.layer,
                net_id: last_seg.net_id,
                segment_type: last_seg.segment_type,
            });
        }

        path.total_length = path.segments.iter().map(|s| s.length()).sum();
    }
}
