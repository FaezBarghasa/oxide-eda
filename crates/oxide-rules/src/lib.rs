//! Hierarchical constraint manager and deterministic design rule check (DRC) engine for Oxide EDA.
//!
//! # Core Philosophy
//! "AI proposes, Deterministic Rules validate."
//!
//! All geometric constraints, electrical clearances, routing widths, and impedance matching rules
//! are stored hierarchically and evaluated strictly:
//! `Net` (highest) > `NetClass` > `Room` > `Global` (lowest).

pub mod manager;
pub mod rules;
pub mod scope;
pub mod violation;

pub use manager::ConstraintManager;
pub use oxide_physics::Microns;
pub use rules::{
    ClearanceRule, DesignRule, HighSpeedRule, ObjectType, PolygonConnectRule, ViaStyleRule,
    WidthRule,
};
pub use scope::RuleScope;
pub use violation::{RuleViolation, RuleViolationType};
