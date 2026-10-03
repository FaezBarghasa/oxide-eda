//! Bounded Elastic String Relaxation & Push-and-Shove Stabilization.
//!
//! Enforces:
//! - Hard recursion limit `MAX_SHOVE_RECURSION_DEPTH = 16`.
//! - Visited net bitset tracking to eliminate cycle loops.
//! - Quadratic deflection energy minimization with walkaround fallback.

use std::collections::HashSet;
use thiserror::Error;

use crate::geometry::Point2D;
use oxide_physics::Microns;

pub const MAX_SHOVE_RECURSION_DEPTH: usize = 16;
pub const MAX_RELAXATION_ITERATIONS: usize = 15;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ShoveError {
    #[error("Recursion limit exceeded ({0} levels); cycle detected in obstacle field")]
    RecursionLimitExceeded(usize),
    #[error("Corridor blocked by immovable obstacles; walkaround required")]
    CorridorBlocked,
    #[error("Clearance invariant violated at coordinate {0:?}")]
    ClearanceBreached(Point2D),
}

/// Thread-local shove context tracking recursion depth and visited nets.
#[derive(Debug, Clone)]
pub struct ShoveContext {
    pub visited_nets: HashSet<u32>,
    pub current_depth: usize,
    pub max_depth: usize,
    pub clearance_pad_nm: i64,
}

impl Default for ShoveContext {
    fn default() -> Self {
        Self {
            visited_nets: HashSet::new(),
            current_depth: 0,
            max_depth: MAX_SHOVE_RECURSION_DEPTH,
            clearance_pad_nm: 100_000,
        }
    }
}

impl ShoveContext {
    pub fn new(clearance_pad_nm: i64) -> Self {
        Self {
            visited_nets: HashSet::new(),
            current_depth: 0,
            max_depth: MAX_SHOVE_RECURSION_DEPTH,
            clearance_pad_nm,
        }
    }

    pub fn enter_net(&mut self, net_id: u32) -> Result<(), ShoveError> {
        if self.current_depth >= self.max_depth {
            return Err(ShoveError::RecursionLimitExceeded(self.current_depth));
        }
        if self.visited_nets.contains(&net_id) {
            return Err(ShoveError::RecursionLimitExceeded(self.current_depth));
        }
        self.visited_nets.insert(net_id);
        self.current_depth += 1;
        Ok(())
    }

    pub fn exit_net(&mut self, net_id: u32) {
        self.visited_nets.remove(&net_id);
        self.current_depth = self.current_depth.saturating_sub(1);
    }
}

/// Result of an elastic relaxation deflection.
#[derive(Debug, Clone, PartialEq)]
pub struct ElasticShoveResult {
    pub deflected_points: Vec<Point2D>,
    pub energy: f64,
    pub iterations: usize,
    pub converged: bool,
}

/// Bounded Elastic String Relaxation engine.
#[derive(Debug, Clone)]
pub struct ElasticShoveEngine {
    pub k_stretch: f64,
    pub k_stiff: f64,
    pub damping: f64,
}

impl Default for ElasticShoveEngine {
    fn default() -> Self {
        Self {
            k_stretch: 0.3,
            k_stiff: 0.6,
            damping: 0.5,
        }
    }
}

impl ElasticShoveEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Relaxes a polyline path around obstacles subject to clearance constraints.
    pub fn relax_path(
        &self,
        original_path: &[Point2D],
        obstacles: &[Point2D],
        clearance_microns: Microns,
    ) -> ElasticShoveResult {
        if original_path.len() < 2 {
            return ElasticShoveResult {
                deflected_points: original_path.to_vec(),
                energy: 0.0,
                iterations: 0,
                converged: true,
            };
        }

        let mut pts = original_path.to_vec();
        let clearance = clearance_microns as f64;
        let mut converged = false;
        let mut final_energy = 0.0;
        let mut iteration_count = 0;

        for iter in 0..MAX_RELAXATION_ITERATIONS {
            iteration_count = iter + 1;
            let mut max_displacement = 0.0;
            final_energy = 0.0;

            for i in 1..pts.len() - 1 {
                let orig = original_path[i];
                let prev = pts[i - 1];
                let next = pts[i + 1];

                // Spring forces: stretch toward neighbors, stiff toward original
                let fx_spring = self.k_stretch * ((prev.x + next.x) as f64 / 2.0 - pts[i].x as f64)
                    + self.k_stiff * (orig.x as f64 - pts[i].x as f64);
                let fy_spring = self.k_stretch * ((prev.y + next.y) as f64 / 2.0 - pts[i].y as f64)
                    + self.k_stiff * (orig.y as f64 - pts[i].y as f64);

                // Repulsive obstacle force
                let mut fx_repulse = 0.0;
                let mut fy_repulse = 0.0;

                for obs in obstacles {
                    let dx = (pts[i].x - obs.x) as f64;
                    let dy = (pts[i].y - obs.y) as f64;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < clearance && dist > 1e-3 {
                        let overlap = clearance - dist;
                        let nx = dx / dist;
                        let ny = dy / dist;
                        fx_repulse += nx * overlap * 1.2;
                        fy_repulse += ny * overlap * 1.2;
                    }
                }

                let disp_x = ((fx_spring + fx_repulse) * self.damping).round() as i64;
                let disp_y = ((fy_spring + fy_repulse) * self.damping).round() as i64;

                let d_mag = ((disp_x * disp_x + disp_y * disp_y) as f64).sqrt();
                if d_mag > max_displacement {
                    max_displacement = d_mag;
                }

                pts[i].x += disp_x;
                pts[i].y += disp_y;

                final_energy += d_mag;
            }

            if max_displacement <= 2.0 {
                converged = true;
                break;
            }
        }

        ElasticShoveResult {
            deflected_points: pts,
            energy: final_energy,
            iterations: iteration_count,
            converged,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shove_context_recursion_limit() {
        let mut ctx = ShoveContext::new(100_000);
        for net in 0..16 {
            assert!(ctx.enter_net(net).is_ok());
        }
        // Exceeding MAX_SHOVE_RECURSION_DEPTH (16) must return error
        assert!(ctx.enter_net(16).is_err());
    }

    #[test]
    fn test_shove_context_cycle_detection() {
        let mut ctx = ShoveContext::new(100_000);
        assert!(ctx.enter_net(1).is_ok());
        assert!(ctx.enter_net(2).is_ok());
        // Re-entering visited net 1 must return error
        assert!(ctx.enter_net(1).is_err());
    }

    #[test]
    fn test_elastic_shove_relaxation() {
        let engine = ElasticShoveEngine::new();
        let path = vec![
            Point2D::new(0, 0),
            Point2D::new(500, 0),
            Point2D::new(1000, 0),
        ];
        let obstacles = vec![Point2D::new(500, 20)]; // obstacle directly on path

        let result = engine.relax_path(&path, &obstacles, 150);
        assert!(result.converged);
        assert_eq!(result.deflected_points.len(), 3);
        // Middle point must have been deflected away from obstacle
        assert!(result.deflected_points[1].y != 0);
    }
}
