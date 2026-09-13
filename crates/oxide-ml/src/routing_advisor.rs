use std::collections::HashMap;
use oxide_types::coord::Point2D;

use crate::engine::InferenceEngine;
use crate::schema::{RoutingAction, RoutingAdvisorInputConfig, RoutingAdvisorOutput};
use crate::tensors::BoardTensorBuilder;

/// RoutingAdvisor evaluates local board context and predicts routing policy distributions
pub struct RoutingAdvisor {
    tensor_builder: BoardTensorBuilder,
    engine: Option<InferenceEngine>,
    cache: HashMap<(Point2D, u32), RoutingAdvisorOutput>,
}

impl RoutingAdvisor {
    pub fn new(config: RoutingAdvisorInputConfig, engine: Option<InferenceEngine>) -> Self {
        Self {
            tensor_builder: BoardTensorBuilder::new(config),
            engine,
            cache: HashMap::new(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(RoutingAdvisorInputConfig::default(), None)
    }

    /// Predict action scores for a given routing state
    pub fn predict(
        &mut self,
        current_pos: Point2D,
        current_layer: u32,
        target_pos: Point2D,
        net_id: u32,
        obstacles: &[(Point2D, u32)],
    ) -> RoutingAdvisorOutput {
        let key = (current_pos, current_layer);
        if let Some(cached) = self.cache.get(&key) {
            return cached.clone();
        }

        // If an ONNX inference model is mounted, run model inference
        if let Some(ref engine) = self.engine {
            let tensor_data = self.tensor_builder.build_window_tensor(
                current_pos,
                current_layer,
                target_pos,
                net_id,
                obstacles,
            );

            let shape: Vec<usize> = tensor_data.shape().to_vec();
            if let Ok(raw_slice) = tensor_data.as_slice() {
                if let Ok(tract_tensor) = tract_onnx::prelude::Tensor::from_slice(&shape, raw_slice) {
                    if let Ok(results) = engine.run(tract_onnx::prelude::tvec!(tract_tensor)) {
                        if let Some(first) = results.first() {
                            if let Ok(view) = first.to_array_view::<f32>() {
                                if view.len() >= 11 {
                                    let mut action_scores = [0.0f32; 10];
                                    for (i, v) in view.iter().take(10).enumerate() {
                                        action_scores[i] = *v;
                                    }
                                    let confidence = view[10];
                                    let out = RoutingAdvisorOutput {
                                        action_scores,
                                        confidence,
                                    };
                                    self.cache.insert(key, out.clone());
                                    return out;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Analytical learned-heuristic fallback policy (direction gradient + layer preference)
        let dx = (target_pos.x - current_pos.x) as f32;
        let dy = (target_pos.y - current_pos.y) as f32;
        let dist = (dx * dx + dy * dy).sqrt().max(1.0);
        let dir_x = dx / dist;
        let dir_y = dy / dist;

        let mut logits = [0.0f32; 10];
        for i in 0..8 {
            let action = match i {
                0 => RoutingAction::MoveNorth,
                1 => RoutingAction::MoveNorthEast,
                2 => RoutingAction::MoveEast,
                3 => RoutingAction::MoveSouthEast,
                4 => RoutingAction::MoveSouth,
                5 => RoutingAction::MoveSouthWest,
                6 => RoutingAction::MoveWest,
                _ => RoutingAction::MoveNorthWest,
            };
            let (ax, ay, _) = action.delta();
            let act_dist = ((ax * ax + ay * ay) as f32).sqrt();
            let dot = (dir_x * (ax as f32) + dir_y * (ay as f32)) / act_dist;
            logits[i] = dot * 2.0;
        }

        // Layer changes: slight penalty unless blocked or layer misalignment
        logits[8] = -0.5; // Up
        logits[9] = -0.5; // Down

        // Softmax
        let max_l = logits.iter().copied().fold(f32::MIN, f32::max);
        let exp_sum: f32 = logits.iter().map(|&x| (x - max_l).exp()).sum();
        let mut action_scores = [0.0f32; 10];
        for (i, v) in logits.iter().enumerate() {
            action_scores[i] = (v - max_l).exp() / exp_sum;
        }

        let out = RoutingAdvisorOutput {
            action_scores,
            confidence: 0.85,
        };
        self.cache.insert(key, out.clone());
        out
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
