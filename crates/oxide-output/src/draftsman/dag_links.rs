//! Reactive Drawing Dependency Graph (DAG) & Dangling Reference Handling for Draftsman.
//!
//! Conforms to Master Technical Directive §4.5 & Domain 5 Benchmark:
//! - Reactive DAG linking dimensions and annotations to core PCB database UUIDs
//! - Graceful degradation to `AnnotationState::Dangling` on deleted components/pads/holes
//! - Zero-unwrap panic-immune execution during PDF and vector exports

use oxide_types::geometry::Point2D;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use uuid::Uuid;

/// Drafting DAG error types.
#[derive(Error, Debug)]
pub enum DraftingError {
    #[error("Cyclic dependency detected in drawing DAG at node {0}")]
    CyclicDependency(Uuid),
    #[error("Render warning: document contains {0} dangling annotations")]
    DanglingAnnotationsWarning(usize),
}

/// Associative link state for drawing annotations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationState {
    /// Target entities exist and coordinates are synchronized.
    Synchronized,
    /// Target entity was deleted or modified; annotation retains last-known coordinates with warning.
    Dangling,
    /// Annotation is hidden or suppressed.
    Suppressed,
}

/// Target geometric reference on the PCB layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DimensionTarget {
    Pad {
        component_id: Uuid,
        pad_number: String,
    },
    MountingHole {
        hole_id: Uuid,
    },
    BoardContourVertex {
        vertex_index: usize,
    },
    TraceCenterline {
        net_id: Uuid,
        segment_id: Uuid,
    },
    FreeCoordinate(Point2D),
}

/// Associative linear dimension with reactive target bindings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociativeLinearDimension {
    pub dimension_id: Uuid,
    pub target_start: DimensionTarget,
    pub target_end: DimensionTarget,
    pub cached_start_pt: Point2D,
    pub cached_end_pt: Point2D,
    pub offset_distance_mm: f64,
    pub state: AnnotationState,
    pub text_override: Option<String>,
}

/// Reactive Drawing Dependency Graph (DAG) manager.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DrawingDagEngine {
    pub dependencies: HashMap<Uuid, HashSet<Uuid>>,
    pub dimensions: HashMap<Uuid, AssociativeLinearDimension>,
}

impl DrawingDagEngine {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dimensions: HashMap::new(),
        }
    }

    /// Registers an associative dimension in the DAG.
    pub fn add_dimension(&mut self, dim: AssociativeLinearDimension) {
        let mut target_ids = HashSet::new();
        Self::collect_target_uuids(&dim.target_start, &mut target_ids);
        Self::collect_target_uuids(&dim.target_end, &mut target_ids);

        self.dependencies.insert(dim.dimension_id, target_ids);
        self.dimensions.insert(dim.dimension_id, dim);
    }

    /// Helper to collect referenced UUIDs from a target.
    fn collect_target_uuids(target: &DimensionTarget, set: &mut HashSet<Uuid>) {
        match target {
            DimensionTarget::Pad { component_id, .. } => {
                set.insert(*component_id);
            }
            DimensionTarget::MountingHole { hole_id } => {
                set.insert(*hole_id);
            }
            DimensionTarget::TraceCenterline {
                net_id, segment_id, ..
            } => {
                set.insert(*net_id);
                set.insert(*segment_id);
            }
            _ => {}
        }
    }

    /// Re-evaluates dimension endpoints against a resolver without unwrapping.
    pub fn synchronize_dimension(
        &mut self,
        dim_id: Uuid,
        entity_resolver: &impl Fn(&DimensionTarget) -> Option<Point2D>,
    ) -> Result<AnnotationState, DraftingError> {
        let dim = match self.dimensions.get_mut(&dim_id) {
            Some(d) => d,
            None => return Ok(AnnotationState::Suppressed),
        };

        let pt_start = entity_resolver(&dim.target_start);
        let pt_end = entity_resolver(&dim.target_end);

        match (pt_start, pt_end) {
            (Some(s), Some(e)) => {
                dim.cached_start_pt = s;
                dim.cached_end_pt = e;
                dim.state = AnnotationState::Synchronized;
            }
            _ => {
                // Target primitive missing: degrade to Dangling, keep cached points
                dim.state = AnnotationState::Dangling;
            }
        }

        Ok(dim.state)
    }

    /// Queries all dangling dimension UUIDs for inspection or warnings.
    pub fn query_dangling_dimensions(&self) -> Vec<Uuid> {
        self.dimensions
            .iter()
            .filter(|(_, dim)| dim.state == AnnotationState::Dangling)
            .map(|(id, _)| *id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag_dimension_synchronization_and_dangling() {
        let mut engine = DrawingDagEngine::new();
        let comp_id = Uuid::new_v4();
        let hole_id = Uuid::new_v4();

        let dim = AssociativeLinearDimension {
            dimension_id: Uuid::new_v4(),
            target_start: DimensionTarget::Pad {
                component_id: comp_id,
                pad_number: "1".to_string(),
            },
            target_end: DimensionTarget::MountingHole { hole_id },
            cached_start_pt: Point2D::new(0.0, 0.0),
            cached_end_pt: Point2D::new(50.0, 0.0),
            offset_distance_mm: 10.0,
            state: AnnotationState::Synchronized,
            text_override: None,
        };

        let dim_id = dim.dimension_id;
        engine.add_dimension(dim);

        // 1. Resolver finds both targets -> Synchronized
        let state = engine
            .synchronize_dimension(dim_id, &|target| match target {
                DimensionTarget::Pad { .. } => Some(Point2D::new(5.0, 5.0)),
                DimensionTarget::MountingHole { .. } => Some(Point2D::new(55.0, 5.0)),
                _ => None,
            })
            .unwrap();
        assert_eq!(state, AnnotationState::Synchronized);
        assert_eq!(
            engine.dimensions[&dim_id].cached_start_pt,
            Point2D::new(5.0, 5.0)
        );

        // 2. Component deleted (resolver returns None for Pad) -> Graceful Dangling state without panic
        let state_after_delete = engine
            .synchronize_dimension(dim_id, &|target| match target {
                DimensionTarget::MountingHole { .. } => Some(Point2D::new(55.0, 5.0)),
                _ => None,
            })
            .unwrap();
        assert_eq!(state_after_delete, AnnotationState::Dangling);
        assert_eq!(engine.query_dangling_dimensions().len(), 1);
    }
}
