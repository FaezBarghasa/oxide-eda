pub mod embed;
pub mod engine;
pub mod geom;
pub mod placement;
pub mod routing_advisor;
pub mod schema;
pub mod tensors;
pub mod via_planner;

pub use embed::{MlError, ModelKind};
pub use engine::InferenceEngine;
pub use geom::Point2D;
pub use placement::{PlacementAdvisor, PlacementSuggestion};
pub use routing_advisor::RoutingAdvisor;
pub use schema::{CellFeature, RoutingAction, RoutingAdvisorInputConfig, RoutingAdvisorOutput};
pub use tensors::BoardTensorBuilder;
pub use via_planner::{ViaPlanner, ViaPrediction};

#[derive(Debug, Clone)]
pub struct MlConfig {
    pub enabled: bool,
    pub confidence_threshold: f32,
}

impl Default for MlConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            confidence_threshold: 0.7,
        }
    }
}

pub struct MlEngine {
    config: MlConfig,
}

impl MlEngine {
    pub fn new(config: MlConfig) -> Self {
        Self { config }
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    pub fn create_routing_advisor(&self, num_layers: usize) -> RoutingAdvisor {
        let config = RoutingAdvisorInputConfig {
            num_layers,
            grid_height: 32,
            grid_width: 32,
            num_features: 12,
        };
        RoutingAdvisor::new(config, None)
    }

    pub fn via_planner(&self) -> ViaPlanner {
        ViaPlanner::new()
    }

    pub fn placement_advisor(&self) -> PlacementAdvisor {
        PlacementAdvisor::new()
    }
}
