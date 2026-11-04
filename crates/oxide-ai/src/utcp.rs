//! Universal Tool Call Protocol (UTCP) discovery and manifest definition.

use serde_json::Value;
use crate::mcp::McpServer;

/// Produce the UTCP manifest describing the Oxide EDA tool capabilities.
pub fn utcp_manifest() -> Value {
    let tools = McpServer::list_tools();
    serde_json::json!({
        "protocol": "utcp/1.0",
        "name": "oxide-eda",
        "description": "Oxide EDA - AI-first electronic design automation suite",
        "version": env!("CARGO_PKG_VERSION"),
        "transport": {
            "type": "stdio",
            "command": "oxide",
            "args": ["mcp"]
        },
        "tools": tools.into_iter().map(|t| {
            serde_json::json!({
                "name": t.name,
                "description": t.description,
                "parameters": t.input_schema
            })
        }).collect::<Vec<_>>()
    })
}
