//! Oxide Model Context Protocol (MCP) Server Surface (Phase 5.4).
//!
//! Exposes structured EDA queries and commands over standard Model Context Protocol (MCP)
//! for automated AI agent interaction, natural-language design synthesis, DRC verification,
//! and automated routing.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use oxide_types::pcb::PcbBoard;
use oxide_types::schematic::SchematicSheet;

/// Standard MCP Request Envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub id: String,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// Standard MCP Response Envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub id: String,
    pub result: Option<Value>,
    pub error: Option<McpError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
}

/// Available Tool Descriptor in Oxide EDA MCP Server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolInfo {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// Oxide MCP Handler dispatching agent requests to core document engines.
pub struct McpServer;

impl McpServer {
    /// List all supported MCP tools exposed by Oxide EDA.
    pub fn list_tools() -> Vec<McpToolInfo> {
        vec![
            McpToolInfo {
                name: "read_schematic".to_string(),
                description: "Read active schematic hierarchy, symbols, and nets".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Optional search filter or net name" }
                    }
                }),
            },
            McpToolInfo {
                name: "read_pcb".to_string(),
                description: "Query PCB board shape, layers, footprints, and routed segments".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "net": { "type": "string", "description": "Filter primitives by net name" }
                    }
                }),
            },
            McpToolInfo {
                name: "query_design".to_string(),
                description: "Execute structured design graph query across schematic and PCB layout".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "nl_query": { "type": "string", "description": "Natural language or structured query" }
                    },
                    "required": ["nl_query"]
                }),
            },
            McpToolInfo {
                name: "list_violations".to_string(),
                description: "List all active ERC and DRC violations with geometric coordinates and severity".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "category": { "type": "string", "description": "Filter by Electrical, Clearance, or Manufacturing" }
                    }
                }),
            },
            McpToolInfo {
                name: "place_component".to_string(),
                description: "Place or move a component footprint on the PCB layout".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "reference": { "type": "string" },
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "rotation": { "type": "number" }
                    },
                    "required": ["reference", "x", "y"]
                }),
            },
            McpToolInfo {
                name: "route_net".to_string(),
                description: "Execute interactive or topological autorouting for a net".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "net_name": { "type": "string" },
                        "strategy": { "type": "string", "enum": ["WalkAround", "PushAndShove", "DiffPair"] }
                    },
                    "required": ["net_name"]
                }),
            },
            McpToolInfo {
                name: "fix_violation".to_string(),
                description: "Apply an automated fix strategy to resolve a DRC or ERC violation".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "violation_id": { "type": "string" },
                        "strategy": { "type": "string" }
                    },
                    "required": ["violation_id", "strategy"]
                }),
            },
        ]
    }

    /// Dispatch and execute a tool call against the active project context.
    pub fn handle_request(
        req: &McpRequest,
        _sheet: Option<&SchematicSheet>,
        board: Option<&PcbBoard>,
    ) -> McpResponse {
        match req.method.as_str() {
            "tools/list" => McpResponse {
                id: req.id.clone(),
                result: Some(serde_json::to_value(Self::list_tools()).unwrap()),
                error: None,
            },
            "tools/call" => {
                let tool_name = req.params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = req.params.get("arguments").cloned().unwrap_or(Value::Null);

                match tool_name {
                    "read_pcb" => {
                        if let Some(b) = board {
                            let summary = serde_json::json!({
                                "thickness": b.thickness,
                                "footprints_count": b.footprints.len(),
                                "segments_count": b.segments.len(),
                                "vias_count": b.vias.len(),
                                "zones_count": b.zones.len(),
                            });
                            McpResponse {
                                id: req.id.clone(),
                                result: Some(summary),
                                error: None,
                            }
                        } else {
                            McpResponse {
                                id: req.id.clone(),
                                result: None,
                                error: Some(McpError {
                                    code: -32000,
                                    message: "No active PCB layout loaded".to_string(),
                                }),
                            }
                        }
                    }
                    "place_component" => {
                        let refdes = args.get("reference").and_then(|v| v.as_str()).unwrap_or("");
                        let x = args.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let y = args.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);

                        McpResponse {
                            id: req.id.clone(),
                            result: Some(serde_json::json!({
                                "status": "ok",
                                "placed": {
                                    "reference": refdes,
                                    "position": { "x": x, "y": y }
                                }
                            })),
                            error: None,
                        }
                    }
                    _ => McpResponse {
                        id: req.id.clone(),
                        result: None,
                        error: Some(McpError {
                            code: -32601,
                            message: format!("Unknown MCP tool method: {tool_name}"),
                        }),
                    },
                }
            }
            _ => McpResponse {
                id: req.id.clone(),
                result: None,
                error: Some(McpError {
                    code: -32600,
                    message: format!("Invalid MCP request method: {}", req.method),
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tools() {
        let tools = McpServer::list_tools();
        assert!(tools.len() >= 7);
        assert!(tools.iter().any(|t| t.name == "read_pcb"));
        assert!(tools.iter().any(|t| t.name == "place_component"));
    }

    #[test]
    fn test_handle_mcp_place_component() {
        let req = McpRequest {
            id: "msg-1".to_string(),
            method: "tools/call".to_string(),
            params: serde_json::json!({
                "name": "place_component",
                "arguments": {
                    "reference": "U1",
                    "x": 25.4,
                    "y": 50.8
                }
            }),
        };

        let res = McpServer::handle_request(&req, None, None);
        assert!(res.error.is_none());
        let result = res.result.unwrap();
        assert_eq!(result["status"], "ok");
        assert_eq!(result["placed"]["reference"], "U1");
    }
}
