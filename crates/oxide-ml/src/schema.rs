use serde::{Deserialize, Serialize};

/// Layout configuration for Routing Advisor local 3D window input
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RoutingAdvisorInputConfig {
    pub num_layers: usize,
    pub grid_height: usize,
    pub grid_width: usize,
    pub num_features: usize,
}

impl Default for RoutingAdvisorInputConfig {
    fn default() -> Self {
        Self {
            num_layers: 4,
            grid_height: 32,
            grid_width: 32,
            num_features: 12,
        }
    }
}

/// Feature channels extracted per cell per layer
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellFeature {
    /// 0.0 = free space, 1.0 = obstacle present
    Obstacle = 0,
    /// Normalized Net ID
    NetId = 1,
    /// 1.0 if this cell contains target pin, else 0.0
    IsTarget = 2,
    /// 1.0 if this cell contains current start point, else 0.0
    IsStart = 3,
    /// Local routing congestion 0.0 - 1.0
    Congestion = 4,
    /// Keepout zone boundary 0.0 - 1.0
    IsKeepout = 5,
    /// Normalized clearance distance to nearest obstacle
    Clearance = 6,
    /// Preferred layer direction alignment 0.0 - 1.0
    PreferredLayer = 7,
    /// Normalized distance to target
    DistanceToTarget = 8,
    /// Via placement allowed indicator
    ViaAllowed = 9,
    /// 1.0 if signal layer, 0.0 for planes
    IsSignalLayer = 10,
    /// Thermal dissipation gradient / risk score
    ThermalRisk = 11,
}

/// The 10 discrete routing policy actions
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutingAction {
    MoveNorth = 0,
    MoveNorthEast = 1,
    MoveEast = 2,
    MoveSouthEast = 3,
    MoveSouth = 4,
    MoveSouthWest = 5,
    MoveWest = 6,
    MoveNorthWest = 7,
    ChangeLayerUp = 8,
    ChangeLayerDown = 9,
}

impl RoutingAction {
    pub const COUNT: usize = 10;

    pub fn delta(&self) -> (i64, i64, i32) {
        match self {
            Self::MoveNorth => (0, -1, 0),
            Self::MoveNorthEast => (1, -1, 0),
            Self::MoveEast => (1, 0, 0),
            Self::MoveSouthEast => (1, 1, 0),
            Self::MoveSouth => (0, 1, 0),
            Self::MoveSouthWest => (-1, 1, 0),
            Self::MoveWest => (-1, 0, 0),
            Self::MoveNorthWest => (-1, -1, 0),
            Self::ChangeLayerUp => (0, 0, 1),
            Self::ChangeLayerDown => (0, 0, -1),
        }
    }
}

/// Output predictions from Routing Advisor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingAdvisorOutput {
    /// Softmax probability distribution over the 10 actions
    pub action_scores: [f32; 10],
    /// Overall model certainty / confidence in this state (0.0 - 1.0)
    pub confidence: f32,
}

impl Default for RoutingAdvisorOutput {
    fn default() -> Self {
        Self {
            action_scores: [0.1; 10],
            confidence: 0.5,
        }
    }
}
