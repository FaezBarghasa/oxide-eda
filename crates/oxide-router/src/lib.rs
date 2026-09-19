//! Complete routing engine for Oxide EDA: Topological Autorouting, Interactive Routing,
//! and High-Speed Optimization.

use std::sync::Arc;

use oxide_physics::Microns;
use oxide_rules::ConstraintManager;
use oxide_types::pcb::{PcbBoard, ViaType};

pub mod copper_pour;
pub mod geometry;
pub mod interactive;
pub mod optimization;
pub mod topology;
pub mod workflow;

pub use copper_pour::{
    CopperPourEngine, CopperZoneConfig, TeardropGenerator, ThermalReliefStyle, ThermalSpoke,
};
pub use geometry::rtree::{NetId, ObjectId, SpatialIndex, SpatialObject, SpatialObjectType};
pub use geometry::{BoundingBox, Point2D};
pub use interactive::{InteractiveRouter, RoutingMode};
pub use optimization::OptimizationEngine;
pub use topology::TopologicalAutorouter;
pub use workflow::{RoutingWorkflow, WorkflowResult};

pub type LayerId = u8;

/// Main routing engine coordinating topological autorouting, interactive gestures, and post-route optimization.
#[derive(Debug, Clone)]
pub struct RoutingEngine {
    pub topology_engine: TopologicalAutorouter,
    pub interactive_engine: InteractiveRouter,
    pub optimization_engine: OptimizationEngine,
    pub rules: Arc<ConstraintManager>,
    pub spatial_index: Arc<SpatialIndex>,
}

impl RoutingEngine {
    pub fn new(rules: Arc<ConstraintManager>, board: &PcbBoard) -> Self {
        let spatial_index = Arc::new(SpatialIndex::build(board));
        let topology_engine =
            TopologicalAutorouter::new(Arc::clone(&rules), Arc::clone(&spatial_index));
        let interactive_engine =
            InteractiveRouter::new(Arc::clone(&rules), Arc::clone(&spatial_index));
        let optimization_engine = OptimizationEngine::new(Arc::clone(&rules));

        Self {
            topology_engine,
            interactive_engine,
            optimization_engine,
            rules,
            spatial_index,
        }
    }
}

/// Status and outcome of a routing operation.
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingResult {
    Success(RoutingPath),
    PartialSuccess {
        path: RoutingPath,
        unconnected: Vec<NetId>,
    },
    Failed(RoutingError),
}

/// Routing error taxonomy.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum RoutingError {
    #[error("No valid path could be found between endpoints")]
    NoPathFound,
    #[error("Clearance violation at ({location:?}) - actual {actual:?}, required {required:?}")]
    ClearanceViolation {
        location: Point2D,
        actual: Microns,
        required: Microns,
    },
    #[error("Layer constraint violation on net {net:?} for layer {layer:?}")]
    LayerConstraintViolation { net: NetId, layer: LayerId },
    #[error("Impedance mismatch on net {net:?} - actual {actual:.2} Ω, target {target:.2} Ω")]
    ImpedanceViolation {
        net: NetId,
        actual: f64,
        target: f64,
    },
    #[error("Routing congestion exceeded in region ({region:?}) with density {density:.2}")]
    CongestionExceeded { region: BoundingBox, density: f64 },
}

/// Complete routing path for a routed net.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RoutingPath {
    pub net_id: NetId,
    pub segments: Vec<RouteSegment>,
    pub vias: Vec<ViaPlacement>,
    pub total_length: Microns,
    pub layer_transitions: Vec<LayerTransition>,
}

/// A single segment in a routed path.
#[derive(Debug, Clone, PartialEq)]
pub struct RouteSegment {
    pub start_point: Point2D,
    pub end_point: Point2D,
    pub width: Microns,
    pub layer: LayerId,
    pub net_id: NetId,
    pub segment_type: SegmentType,
}

impl RouteSegment {
    pub fn length(&self) -> Microns {
        self.start_point.distance_to(self.end_point)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SegmentType {
    #[default]
    Straight,
    Arc {
        center: Point2D,
        radius: Microns,
    },
}

/// Placed through/blind/micro via in a routed path.
#[derive(Debug, Clone, PartialEq)]
pub struct ViaPlacement {
    pub position: Point2D,
    pub via_type: ViaType,
    pub drill_size: Microns,
    pub pad_diameter: Microns,
    pub start_layer: LayerId,
    pub end_layer: LayerId,
    pub net_id: NetId,
}

/// Transition between two physical copper layers.
#[derive(Debug, Clone, PartialEq)]
pub struct LayerTransition {
    pub from_layer: LayerId,
    pub to_layer: LayerId,
    pub via_position: Point2D,
}
