//! Co-Simulation Session Lifecycle and Synchronization State.

use serde::{Deserialize, Serialize};

/// State of the Co-Simulation Session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoSimStatus {
    Idle,
    Running,
    Paused,
    Finished,
    Error,
}

/// Statistics and timeline metrics for the active co-simulation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoSimStats {
    pub sim_time_s: f64,
    pub wall_clock_elapsed_s: f64,
    pub speed_ratio: f64,
    pub total_steps: u64,
    pub mcu_cycles: u64,
    pub packets_transferred: u64,
}

impl Default for CoSimStats {
    fn default() -> Self {
        Self {
            sim_time_s: 0.0,
            wall_clock_elapsed_s: 0.0,
            speed_ratio: 1.0,
            total_steps: 0,
            mcu_cycles: 0,
            packets_transferred: 0,
        }
    }
}
