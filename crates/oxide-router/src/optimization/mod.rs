//! Optimization algorithms for route refinement: ActiveRoute (River Route),
//! Glossing, Retrace, Length Tuning, and Loop Removal.

use std::sync::Arc;

use oxide_rules::ConstraintManager;

pub mod active_route;
pub mod glossing;
pub mod length_tuning;
pub mod loop_removal;
pub mod retrace;

/// Unified optimization engine coordinating all post-route cleanups.
#[derive(Debug, Clone)]
pub struct OptimizationEngine {
    pub active_route: active_route::ActiveRouteOptimizer,
    pub glossing: glossing::GlossingOptimizer,
    pub retrace: retrace::RetraceOptimizer,
    pub length_tuning: length_tuning::LengthTuningOptimizer,
    pub loop_removal: loop_removal::LoopRemovalOptimizer,
}

impl OptimizationEngine {
    pub fn new(rules: Arc<ConstraintManager>) -> Self {
        Self {
            active_route: active_route::ActiveRouteOptimizer::new(Arc::clone(&rules)),
            glossing: glossing::GlossingOptimizer::new(Arc::clone(&rules)),
            retrace: retrace::RetraceOptimizer::new(Arc::clone(&rules)),
            length_tuning: length_tuning::LengthTuningOptimizer::new(Arc::clone(&rules)),
            loop_removal: loop_removal::LoopRemovalOptimizer::new(Arc::clone(&rules)),
        }
    }
}
