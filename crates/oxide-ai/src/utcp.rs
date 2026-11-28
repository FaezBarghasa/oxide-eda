//! Universal Tool Call Protocol (UTCP) discovery, manifest, validation, and execution.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::mcp::{McpContext, McpServer};

/// Single UTCP invocation descriptor (used for CLI and batch workflows).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtcpCallRequest {
    pub tool: String,
    #[serde(default)]
    pub arguments: Value,
}

/// Structured UTCP invocation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtcpCallResponse {
    pub tool: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Summary produced by [`execute_utcp_batch`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtcpBatchReport {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub dirty_board: bool,
    pub dirty_schematic: bool,
    pub results: Vec<UtcpCallResponse>,
}

/// Result of [`validate_utcp_manifest`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtcpValidationSummary {
    pub valid: bool,
    pub protocol: String,
    pub tool_count: usize,
}

/// Produce the default UTCP 1.0 manifest describing all Oxide EDA tool capabilities.
pub fn utcp_manifest() -> Value {
    utcp_manifest_for_transport("stdio")
}

/// Produce a UTCP 1.0 manifest customized for `"stdio"` (MCP) or `"cli"` call templates.
pub fn utcp_manifest_for_transport(transport: &str) -> Value {
    let tools = McpServer::list_tools();
    let transport_block = if transport.eq_ignore_ascii_case("cli") {
        json!({
            "type": "cli",
            "command": "oxide-cli",
            "args_template": ["utcp", "call", "{tool}", "--args", "{arguments_json}"]
        })
    } else {
        json!({
            "type": "stdio",
            "command": "oxide-cli",
            "args": ["mcp", "serve"]
        })
    };

    json!({
        "protocol": "utcp/1.0",
        "manual_version": "1.0.0",
        "name": "oxide-eda",
        "description": "Oxide EDA - Altium-compatible open-source EDA suite with full MCP/UTCP automation",
        "version": env!("CARGO_PKG_VERSION"),
        "transport": transport_block,
        "tools": tools.into_iter().map(|t| {
            json!({
                "name": t.name,
                "category": t.category,
                "tags": ["eda", t.category, "oxide"],
                "description": t.description,
                "parameters": t.input_schema,
                "cli_example": format!("oxide-cli utcp call {} --args '{{}}'", t.name)
            })
        }).collect::<Vec<_>>()
    })
}

/// Execute a single UTCP tool call against a live [`McpContext`].
pub fn execute_utcp_call(tool: &str, arguments: Value, ctx: &mut McpContext) -> UtcpCallResponse {
    let args = if arguments.is_null() {
        json!({})
    } else {
        arguments
    };
    match McpServer::call_tool(tool, &args, ctx) {
        Ok(result) => UtcpCallResponse {
            tool: tool.to_string(),
            ok: true,
            result: Some(result),
            error: None,
        },
        Err((code, msg)) => UtcpCallResponse {
            tool: tool.to_string(),
            ok: false,
            result: None,
            error: Some(format!("[{code}] {msg}")),
        },
    }
}

/// Execute an ordered batch of UTCP calls against a shared [`McpContext`].
pub fn execute_utcp_batch(
    calls: &[UtcpCallRequest],
    ctx: &mut McpContext,
    stop_on_error: bool,
) -> UtcpBatchReport {
    let mut results = Vec::with_capacity(calls.len());
    let mut succeeded = 0usize;
    let mut failed = 0usize;

    for call in calls {
        let resp = execute_utcp_call(&call.tool, call.arguments.clone(), ctx);
        let ok = resp.ok;
        results.push(resp);
        if ok {
            succeeded += 1;
        } else {
            failed += 1;
            if stop_on_error {
                break;
            }
        }
    }

    UtcpBatchReport {
        total: calls.len(),
        succeeded,
        failed,
        dirty_board: ctx.dirty_board,
        dirty_schematic: ctx.dirty_schematic,
        results,
    }
}

/// Validate that a JSON value conforms to the Oxide UTCP 1.0 manifest contract.
pub fn validate_utcp_manifest(manifest: &Value) -> Result<UtcpValidationSummary, Vec<String>> {
    let mut errors = Vec::new();
    let protocol = manifest
        .get("protocol")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if !protocol.starts_with("utcp/") {
        errors.push("missing or invalid 'protocol' field (expected 'utcp/1.0')".to_string());
    }
    if manifest
        .get("name")
        .and_then(|v| v.as_str())
        .is_none_or(|s| s.is_empty())
    {
        errors.push("missing non-empty 'name' field".to_string());
    }
    if manifest.get("transport").is_none_or(|v| !v.is_object()) {
        errors.push("missing 'transport' object".to_string());
    }
    let tools = manifest.get("tools").and_then(|v| v.as_array());
    let tool_count = tools.map_or(0, Vec::len);
    if let Some(arr) = tools {
        if arr.is_empty() {
            errors.push("'tools' array must not be empty".to_string());
        }
        for (idx, item) in arr.iter().enumerate() {
            if item
                .get("name")
                .and_then(|v| v.as_str())
                .is_none_or(|s| s.is_empty())
            {
                errors.push(format!("tools[{idx}] missing 'name'"));
            }
            if item.get("parameters").is_none_or(|v| !v.is_object()) {
                errors.push(format!("tools[{idx}] missing 'parameters' schema object"));
            }
        }
    } else {
        errors.push("missing 'tools' array".to_string());
    }

    if errors.is_empty() {
        Ok(UtcpValidationSummary {
            valid: true,
            protocol,
            tool_count,
        })
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_manifest_validates() {
        let m = utcp_manifest();
        let summary = validate_utcp_manifest(&m).expect("manifest valid");
        assert!(summary.valid);
        assert!(summary.tool_count >= 18);
    }

    #[test]
    fn utcp_batch_runs_impedance_and_catalog() {
        let mut ctx = McpContext::default();
        let calls = vec![
            UtcpCallRequest {
                tool: "calculate_impedance".to_string(),
                arguments: json!({ "mode": "microstrip", "trace_width_um": 180, "dielectric_height_um": 100 }),
            },
            UtcpCallRequest {
                tool: "altium_catalog_search".to_string(),
                arguments: json!({ "query": "PCB:Undo", "limit": 3 }),
            },
        ];
        let report = execute_utcp_batch(&calls, &mut ctx, true);
        assert_eq!(report.succeeded, 2);
        assert_eq!(report.failed, 0);
    }
}
