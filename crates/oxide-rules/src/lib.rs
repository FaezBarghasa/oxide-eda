//! Hierarchical constraint manager and deterministic design rule check (DRC) engine for Oxide EDA.
//!
//! # Core Philosophy
//! "AI proposes, Deterministic Rules validate."
//!
//! All geometric constraints, electrical clearances, routing widths, and impedance matching rules
//! are stored hierarchically and evaluated strictly:
//! `Net` (highest) > `NetClass` > `Room` > `Global` (lowest).

pub mod manager;
pub mod query_dsl;
pub mod rules;
pub mod scope;
pub mod violation;

pub use manager::{ConstraintManager, RuleConfigFile};
pub use oxide_physics::Microns;
pub use query_dsl::{
    ComparisonOp, PrimitiveEvaluationContext, QueryParser, QueryPredicate, QueryTargetType,
};
pub use rules::{
    ClearanceRule, ComponentClearanceRule, DesignRule, DiffPairPhaseRule, HighSpeedRule,
    NetAntennaRule, ObjectType, PolygonConnectRule, ReturnPathRule, RoutingLayerRule,
    SilkscreenRule, SolderMaskRule, ViaStyleRule, WidthRule,
};
pub use scope::RuleScope;
pub use violation::{RuleViolation, RuleViolationType};
