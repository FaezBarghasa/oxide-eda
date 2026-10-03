//! High-Speed Length & Delay Tuning, Intra-Pair Skew Balancing, and Continuous Curvature Splines.

pub mod clothoid;
pub mod coupled_meander;
pub mod skew_bump;

pub use clothoid::fit_biarc_corner;
pub use coupled_meander::{
    MeanderConstraint, TuningError, compute_coupled_delay_ps, synthesize_coupled_accordion,
};
pub use skew_bump::{BalancedPairResult, PhaseSkewBudget, balance_differential_phase};
