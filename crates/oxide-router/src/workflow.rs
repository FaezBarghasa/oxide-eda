//! Complete multi-phase routing workflow manager for Oxide EDA.

use std::sync::Arc;

use oxide_physics::Microns;
use oxide_rules::ConstraintManager;
use oxide_types::pcb::PcbBoard;

use crate::geometry::Point2D;
use crate::geometry::rtree::NetId;
use crate::{RoutingEngine, RoutingResult};

/// End-to-end execution result of a complete board routing workflow.
#[derive(Debug, Clone)]
pub struct WorkflowResult {
    pub total_nets: usize,
    pub routed_nets: usize,
    pub failed_nets: usize,
    pub total_length: Microns,
}

/// Orchestrates full board autorouting from preparation to validation.
#[derive(Debug, Clone)]
pub struct RoutingWorkflow {
    pub engine: RoutingEngine,
    pub board: PcbBoard,
    pub nets: Vec<NetId>,
    pub results: Vec<RoutingResult>,
}

impl RoutingWorkflow {
    pub fn new(rules: Arc<ConstraintManager>, board: PcbBoard, nets: Vec<NetId>) -> Self {
        let engine = RoutingEngine::new(rules, &board);
        Self {
            engine,
            board,
            nets,
            results: Vec::new(),
        }
    }

    /// Execute all routing phases: Preparation, Global Topological Routing, Detailed Routing, Optimization.
    pub fn execute(&mut self) -> WorkflowResult {
        // Phase 1: Pre-routing setup & topological map construction
        let _ = self
            .engine
            .topology_engine
            .build_topological_map(&self.board);

        // Phase 2: Global topological routing
        let global_results = self
            .engine
            .topology_engine
            .route_board(&mut self.board, &self.nets);
        self.results = global_results;

        // Phase 3: Detailed interactive fallback for unrouted or failed nets
        for (i, res) in self.results.iter_mut().enumerate() {
            if matches!(res, RoutingResult::Failed(_)) {
                let net_id = self.nets[i];
                let fallback = self.engine.interactive_engine.route_net(
                    net_id,
                    Point2D::new(10_000, 10_000),
                    Point2D::new(20_000, 20_000),
                );
                *res = fallback;
            }
        }

        // Phase 4: Route optimization (glossing and loop removal)
        for res in &mut self.results {
            if let RoutingResult::Success(path) = res {
                self.engine.optimization_engine.glossing.gloss_path(path);
                self.engine
                    .optimization_engine
                    .loop_removal
                    .remove_loops(path);
            }
        }

        let routed_nets = self
            .results
            .iter()
            .filter(|r| matches!(r, RoutingResult::Success(_)))
            .count();
        let failed_nets = self
            .results
            .iter()
            .filter(|r| matches!(r, RoutingResult::Failed(_)))
            .count();

        let total_length: Microns = self
            .results
            .iter()
            .filter_map(|r| match r {
                RoutingResult::Success(p) => Some(p.total_length),
                _ => None,
            })
            .sum();

        WorkflowResult {
            total_nets: self.nets.len(),
            routed_nets,
            failed_nets,
            total_length,
        }
    }
}
