//! AI Orchestration Layer for Oxide EDA: Prompt-to-Intent, real-part validation, and schematic drafting IR.
//!
//! # Core Tenet
//! "AI proposes, Deterministic Rules validate."

pub mod intent;
pub mod prompt;
pub mod validator;

pub use intent::{CircuitIntent, ComponentRequest, ConnectionIntent, Distributor};
pub use prompt::generate_circuit_generation_prompt;
pub use validator::{
    validate_and_enrich, PartLookupService, StandardCatalogService, ValidationError,
};
