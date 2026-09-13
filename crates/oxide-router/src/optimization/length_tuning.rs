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

    fn add_meander(&self, path: &mut RoutingPath, _extra: Microns) {
        if path.segments.is_empty() {
            return;
        }

        let last_seg = path.segments.pop().unwrap();
        let amplitude = 400; // 400µm
        let (dx, dy) = last_seg.start_point.direction_to(last_seg.end_point);
        let (perp_x, perp_y) = (-dy, dx);

        // Simple accordion insert
        let p_mid = last_seg.start_point;
        let p_out = crate::geometry::Point2D::new(
            p_mid.x + (perp_x * amplitude as f64).round() as i64,
            p_mid.y + (perp_y * amplitude as f64).round() as i64,
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
    }
}
