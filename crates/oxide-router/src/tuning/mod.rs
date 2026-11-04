//! High-Speed Length & Delay Tuning, Intra-Pair Skew Balancing, and Continuous Curvature Splines.

pub mod clothoid;
pub mod coupled_meander;
pub mod skew_bump;

pub use clothoid::{fillet_meander_path, fit_biarc_corner, fit_clothoid_fillet};
pub use coupled_meander::{
    MeanderConstraint, TuningError, compute_analytical_safe_amplitude, compute_coupled_delay_ps,
    synthesize_coupled_accordion, synthesize_coupled_accordion_filleted,
};
pub use skew_bump::{BalancedPairResult, PhaseSkewBudget, balance_differential_phase};
