use thiserror::Error;

#[derive(Error, Debug)]
pub enum MlError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Model size {size_mb} MB exceeds limit of {max_mb} MB for {kind:?}")]
    ModelTooLarge {
        kind: ModelKind,
        size_mb: usize,
        max_mb: usize,
    },

    #[error("Failed to parse ONNX model: {0}")]
    ModelLoadFailed(String),

    #[error("Model graph optimization failed: {0}")]
    OptimizationFailed(String),

    #[error("Execution plan creation failed: {0}")]
    PlanFailed(String),

    #[error("Inference execution failed: {0}")]
    InferenceFailed(String),

    #[error("Tensor conversion error: {0}")]
    TensorError(String),
}

/// Shipped embedded model kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelKind {
    /// Predicts routing congestion on a grid
    CongestionPredictor,
    /// Guides A* routing with learned heuristic policy
    RoutingAdvisor,
    /// Suggests optimal layer transitions / vias
    ViaPlanner,
    /// Suggests component placement regions
    PlacementAdvisor,
    /// Predicts high-speed differential pair patterns
    DiffPairRouter,
}

impl ModelKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::CongestionPredictor => "congestion_predictor",
            Self::RoutingAdvisor => "routing_advisor",
            Self::ViaPlanner => "via_planner",
            Self::PlacementAdvisor => "placement_advisor",
            Self::DiffPairRouter => "diff_pair_router",
        }
    }

    pub fn max_size_mb(&self) -> usize {
        match self {
            Self::CongestionPredictor => 15,
            Self::RoutingAdvisor => 30,
            Self::ViaPlanner => 10,
            Self::PlacementAdvisor => 20,
            Self::DiffPairRouter => 15,
        }
    }
}
