//! AI Orchestration Layer for Oxide EDA: Prompt-to-Intent, real-part validation,
//! schematic drafting IR, MCP server, UTCP runtime, and Altium catalog index.
//!
//! # Core Tenet
//! "AI proposes, Deterministic Rules validate."

pub mod altium_catalog;
pub mod intent;
pub mod mcp;
pub mod prompt;
pub mod utcp;
pub mod validator;

pub use altium_catalog::{AltiumCatalog, AltiumCatalogEntry, AltiumCatalogStats, AltiumUiCommand};
pub use intent::{CircuitIntent, ComponentRequest, ConnectionIntent, Distributor};
pub use mcp::{McpContext, McpError, McpRequest, McpResponse, McpServer, McpToolInfo};
pub use prompt::generate_circuit_generation_prompt;
pub use utcp::{
    UtcpBatchReport, UtcpCallRequest, UtcpCallResponse, UtcpValidationSummary, execute_utcp_batch,
    execute_utcp_call, utcp_manifest, utcp_manifest_for_transport, validate_utcp_manifest,
};
pub use validator::{
    PartLookupService, StandardCatalogService, ValidationError, validate_and_enrich,
};
