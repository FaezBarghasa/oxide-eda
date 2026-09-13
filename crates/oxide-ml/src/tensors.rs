use ndarray::{Array, ArrayD, IxDyn};

use crate::geom::Point2D;
use crate::schema::{CellFeature, RoutingAdvisorInputConfig};

/// Extracts local board window features and constructs ndarray input tensors
pub struct BoardTensorBuilder {
    config: RoutingAdvisorInputConfig,
}

impl BoardTensorBuilder {
    pub fn new(config: RoutingAdvisorInputConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &RoutingAdvisorInputConfig {
        &self.config
    }

    /// Construct a 4D tensor `[num_layers, grid_h, grid_w, num_features]` around `center`
    pub fn build_window_tensor(
        &self,
        center: Point2D,
        _current_layer: u32,
        target: Point2D,
        _net_id: u32,
        obstacles: &[(Point2D, u32)], // (location, layer)
    ) -> ArrayD<f32> {
        let nl = self.config.num_layers;
        let gh = self.config.grid_height;
        let gw = self.config.grid_width;
        let nf = self.config.num_features;

        // Grid cell pitch = 0.1 mm (100,000 nm)
        let cell_pitch_nm = 100_000i64;
        let half_w = (gw / 2) as i64;
        let half_h = (gh / 2) as i64;

        let mut tensor = Array::zeros(IxDyn(&[nl, gh, gw, nf]));

        let target_dist_base = center.distance_to(target).max(1.0);

        for layer_idx in 0..nl {
            let is_signal = layer_idx == 0 || layer_idx == nl.saturating_sub(1);
            let layer_u32 = layer_idx as u32;

            for gy in 0..gh {
                let wy = center.y + (gy as i64 - half_h) * cell_pitch_nm;
                for gx in 0..gw {
                    let wx = center.x + (gx as i64 - half_w) * cell_pitch_nm;
                    let cell_pos = Point2D::new(wx, wy);

                    // Feature 0: Obstacle presence
                    let has_obs = obstacles
                        .iter()
                        .any(|&(op, ol)| ol == layer_u32 && op.distance_to(cell_pos) < (cell_pitch_nm as f64));
                    if has_obs {
                        tensor[[layer_idx, gy, gx, CellFeature::Obstacle as usize]] = 1.0;
                    }

                    // Feature 2: Is target
                    let dist_to_target = cell_pos.distance_to(target);
                    if dist_to_target < (cell_pitch_nm as f64) {
                        tensor[[layer_idx, gy, gx, CellFeature::IsTarget as usize]] = 1.0;
                    }

                    // Feature 3: Is start
                    if gx == (gw / 2) && gy == (gh / 2) {
                        tensor[[layer_idx, gy, gx, CellFeature::IsStart as usize]] = 1.0;
                    }

                    // Feature 8: Normalized distance to target
                    tensor[[layer_idx, gy, gx, CellFeature::DistanceToTarget as usize]] =
                        ((dist_to_target / target_dist_base) as f32).min(1.0);

                    // Feature 9: Via allowed (allowed everywhere except near boundary)
                    let via_allowed = gx > 1 && gx < gw - 2 && gy > 1 && gy < gh - 2 && !has_obs;
                    if via_allowed {
                        tensor[[layer_idx, gy, gx, CellFeature::ViaAllowed as usize]] = 1.0;
                    }

                    // Feature 10: Signal layer
                    if is_signal {
                        tensor[[layer_idx, gy, gx, CellFeature::IsSignalLayer as usize]] = 1.0;
                    }
                }
            }
        }

        tensor
    }
}
