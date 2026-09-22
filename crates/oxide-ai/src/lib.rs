//! AI Orchestration Layer for Oxide EDA: Prompt-to-Intent, real-part validation, and schematic drafting IR.
//!
//! # Core Tenet
//! "AI proposes, Deterministic Rules validate."

pub mod intent;
pub mod mcp;
pub mod prompt;
pub mod validator;

pub use intent::{CircuitIntent, ComponentRequest, ConnectionIntent, Distributor};
pub use mcp::{McpError, McpRequest, McpResponse, McpServer, McpToolInfo};
pub use prompt::generate_circuit_generation_prompt;
pub use validator::{
    PartLookupService, StandardCatalogService, ValidationError, validate_and_enrich,
};
