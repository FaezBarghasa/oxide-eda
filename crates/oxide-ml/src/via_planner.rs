use oxide_types::coord::Point2D;

#[derive(Debug, Clone)]
pub struct ViaPrediction {
    pub should_place: bool,
    pub confidence: f32,
    pub target_layer: u32,
}

pub struct ViaPlanner;

impl Default for ViaPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl ViaPlanner {
    pub fn new() -> Self {
        Self
    }

    /// Predict optimal layer transition / via placement
    pub fn should_place_via(
        &self,
        current_pos: Point2D,
        current_layer: u32,
        target_layer: u32,
        blocked_ahead: bool,
    ) -> ViaPrediction {
        if current_layer == target_layer && !blocked_ahead {
            return ViaPrediction {
                should_place: false,
                confidence: 0.9,
                target_layer,
            };
        }

        if blocked_ahead {
            // Suggest switching to alternative routing layer
            let alt_layer = if current_layer == 0 { 1 } else { 0 };
            return ViaPrediction {
                should_place: true,
                confidence: 0.8,
                target_layer: alt_layer,
            };
        }

        ViaPrediction {
            should_place: current_pos.distance_to(Point2D::origin()) > 0.0,
            confidence: 0.6,
            target_layer,
        }
    }
}
