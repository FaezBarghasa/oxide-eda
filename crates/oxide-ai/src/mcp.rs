//! Oxide Model Context Protocol (MCP) Server Surface.
//!
//! Exposes structured EDA queries, ERC/DRC validation, routing, ECO, Altium import,
//! manufacturing CAM export, physics calculations, and the 3,004-command Altium UI
//! catalog over standard JSON-RPC 2.0 / MCP for automated agent and CLI interaction.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use oxide_altium_importer::{AltiumImportResult, import_altium_file};
use oxide_net::{EcoEngine, build_netlist, prune_wire_redundancies};
use oxide_output::{
    ExcellonExporter, GerberExporter, GerberOptions, PickAndPlaceExporter, PickAndPlaceOptions,
    export_ipc2581, export_odbpp_package,
};
use oxide_physics::ImpedanceCalculator;
use oxide_router::{SpatialIndex, TeardropGenerator, TopologicalAutorouter};
use oxide_rules::ConstraintManager;
use oxide_types::atomic_io::atomic_write;
use oxide_types::format::{SnxPcb, SnxSchematic};
use oxide_types::pcb::{Footprint, PcbBoard, Point};
use oxide_types::schematic::SchematicSheet;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::altium_catalog::AltiumCatalog;

/// Standard MCP Request Envelope (compatible with JSON-RPC 2.0).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    #[serde(default = "default_req_id")]
    pub id: Value,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

fn default_req_id() -> Value {
    Value::String("0".to_string())
}

/// Standard MCP Response Envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

impl McpResponse {
    pub fn ok(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn err(id: Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(McpError {
                code,
                message: message.into(),
            }),
        }
    }
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
    pub category: String,
    pub input_schema: Value,
}

/// Mutable session context shared across MCP/UTCP tool invocations.
#[derive(Default)]
pub struct McpContext {
    pub schematic: Option<SchematicSheet>,
    pub schematic_path: Option<PathBuf>,
    pub board: Option<PcbBoard>,
    pub board_path: Option<PathBuf>,
    pub rules: Option<ConstraintManager>,
    pub dirty_schematic: bool,
    pub dirty_board: bool,
}

impl McpContext {
    /// Load board, schematic, and optional rules from disk into a live MCP context.
    pub fn load(
        board_path: Option<&Path>,
        schematic_path: Option<&Path>,
        rules_path: Option<&Path>,
    ) -> Result<Self, String> {
        let mut ctx = Self::default();
        if let Some(p) = board_path {
            let board = load_board_file(p)?;
            ctx.board = Some(board);
            ctx.board_path = Some(p.to_path_buf());
        }
        if let Some(p) = schematic_path {
            let sheet = load_schematic_file(p)?;
            ctx.schematic = Some(sheet);
            ctx.schematic_path = Some(p.to_path_buf());
        }
        if let Some(p) = rules_path {
            let cm = ConstraintManager::from_file(p)
                .map_err(|e| format!("Failed to read rules '{}': {e}", p.display()))?;
            ctx.rules = Some(cm);
        }
        Ok(ctx)
    }

    /// Resolve effective DRC constraint manager for the loaded board.
    pub fn effective_rules(&self, board: &PcbBoard) -> ConstraintManager {
        if let Some(cm) = &self.rules {
            return cm.clone();
        }
        if board.layers.len() > 6 || board.vias.iter().any(|v| v.drill < 0.25) {
            ConstraintManager::hdi_advanced_default()
        } else {
            ConstraintManager::standard_default()
        }
    }

    /// Save dirty or explicitly requested board/schematic outputs atomically.
    pub fn save_targets(
        &mut self,
        board_out: Option<&Path>,
        schematic_out: Option<&Path>,
    ) -> Result<Vec<String>, String> {
        let mut saved = Vec::new();
        if let Some(board) = &self.board
            && let Some(target) = board_out.or(self.board_path.as_deref())
        {
            write_board_file(target, board)?;
            self.dirty_board = false;
            saved.push(target.display().to_string());
        }
        if let Some(sheet) = &self.schematic
            && let Some(target) = schematic_out.or(self.schematic_path.as_deref())
        {
            write_schematic_file(target, sheet)?;
            self.dirty_schematic = false;
            saved.push(target.display().to_string());
        }
        Ok(saved)
    }
}

pub fn load_board_file(path: &Path) -> Result<PcbBoard, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read board '{}': {e}", path.display()))?;
    if let Ok(snx) = SnxPcb::parse(&content) {
        Ok(snx.board)
    } else {
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse board '{}': {e}", path.display()))
    }
}

pub fn load_schematic_file(path: &Path) -> Result<SchematicSheet, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read schematic '{}': {e}", path.display()))?;
    if let Ok(snx) = SnxSchematic::parse(&content) {
        Ok(snx.sheet)
    } else {
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse schematic '{}': {e}", path.display()))
    }
}

pub fn write_board_file(path: &Path, board: &PcbBoard) -> Result<(), String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let payload = if ext == "json" {
        serde_json::to_string_pretty(board)
            .map_err(|e| format!("Failed to serialize PCB JSON: {e}"))?
    } else {
        SnxPcb::new(board.clone())
            .write_string()
            .map_err(|e| format!("Failed to serialize SnxPcb: {e}"))?
    };
    atomic_write(path, payload.as_bytes())
        .map_err(|e| format!("Failed to write board '{}': {e}", path.display()))
}

pub fn write_schematic_file(path: &Path, sheet: &SchematicSheet) -> Result<(), String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let payload = if ext == "json" {
        serde_json::to_string_pretty(sheet)
            .map_err(|e| format!("Failed to serialize Schematic JSON: {e}"))?
    } else {
        SnxSchematic::new(sheet.clone())
            .write_string()
            .map_err(|e| format!("Failed to serialize SnxSchematic: {e}"))?
    };
    atomic_write(path, payload.as_bytes())
        .map_err(|e| format!("Failed to write schematic '{}': {e}", path.display()))
}

/// Oxide MCP Handler dispatching agent requests to core document engines.
pub struct McpServer;

impl McpServer {
    /// List all supported MCP tools exposed by Oxide EDA.
    pub fn list_tools() -> Vec<McpToolInfo> {
        vec![
            McpToolInfo {
                name: "read_schematic".to_string(),
                description: "Read active schematic hierarchy, symbols, wires, labels, and derived netlist".to_string(),
                category: "schematic".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Optional filter matching symbol designator, value, or net name" },
                        "include_netlist": { "type": "boolean", "description": "Include derived electrical nets", "default": true }
                    }
                }),
            },
            McpToolInfo {
                name: "read_pcb".to_string(),
                description: "Query PCB board stackup, layers, footprints, routed segments, vias, and zones".to_string(),
                category: "pcb".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "net": { "type": "string", "description": "Filter primitives by net name" },
                        "include_details": { "type": "boolean", "description": "Include footprint and net summary arrays", "default": false }
                    }
                }),
            },
            McpToolInfo {
                name: "query_design".to_string(),
                description: "Execute structured search across loaded schematic and PCB layout objects".to_string(),
                category: "design".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "nl_query": { "type": "string", "description": "Search term or object pattern (e.g. U1, GND, 10k)" },
                        "kind": { "type": "string", "enum": ["all", "symbol", "footprint", "net", "label"], "default": "all" }
                    },
                    "required": ["nl_query"]
                }),
            },
            McpToolInfo {
                name: "run_erc".to_string(),
                description: "Run Electrical Rules Check (ERC) on the active schematic sheet".to_string(),
                category: "validation".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "severity": { "type": "string", "enum": ["all", "error", "warning", "info"], "default": "all" }
                    }
                }),
            },
            McpToolInfo {
                name: "run_drc".to_string(),
                description: "Run parallel Design Rule Check (DRC) on the active PCB board".to_string(),
                category: "validation".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "limit": { "type": "integer", "description": "Max violations to return", "default": 100 }
                    }
                }),
            },
            McpToolInfo {
                name: "list_violations".to_string(),
                description: "List all active ERC and DRC violations with geometric coordinates and severity".to_string(),
                category: "validation".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "category": { "type": "string", "enum": ["All", "Electrical", "Clearance", "Manufacturing"], "default": "All" }
                    }
                }),
            },
            McpToolInfo {
                name: "place_component".to_string(),
                description: "Place a new footprint or move/rotate an existing component footprint on the PCB".to_string(),
                category: "pcb".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "reference": { "type": "string" },
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "rotation": { "type": "number", "default": 0.0 },
                        "value": { "type": "string" },
                        "footprint_id": { "type": "string" },
                        "layer": { "type": "string", "default": "F.Cu" }
                    },
                    "required": ["reference", "x", "y"]
                }),
            },
            McpToolInfo {
                name: "route_net".to_string(),
                description: "Execute topological autorouting for a net (or '*' for all nets) on the active PCB".to_string(),
                category: "routing".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "net_name": { "type": "string" },
                        "strategy": { "type": "string", "enum": ["WalkAround", "PushAndShove", "DiffPair"], "default": "PushAndShove" }
                    },
                    "required": ["net_name"]
                }),
            },
            McpToolInfo {
                name: "fix_violation".to_string(),
                description: "Apply automated repair strategies (teardrops, min_track_width, prune_wires, auto) to resolve violations".to_string(),
                category: "validation".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "violation_id": { "type": "string" },
                        "strategy": { "type": "string", "enum": ["auto", "teardrops", "min_track_width", "prune_wires"], "default": "auto" }
                    },
                    "required": ["strategy"]
                }),
            },
            McpToolInfo {
                name: "run_eco".to_string(),
                description: "Diff active schematic against active PCB and apply Forward Engineering Change Order (ECO)".to_string(),
                category: "eco".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "dry_run": { "type": "boolean", "default": false }
                    }
                }),
            },
            McpToolInfo {
                name: "import_altium".to_string(),
                description: "Import Altium Designer files (.SchDoc, .PcbDoc, .SchLib, .PcbLib, .IntLib) into Oxide types".to_string(),
                category: "altium".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "input_path": { "type": "string" },
                        "out_path": { "type": "string", "description": "Optional output file path (.snxsch, .snxpcb, or .json)" },
                        "load_into_session": { "type": "boolean", "default": true }
                    },
                    "required": ["input_path"]
                }),
            },
            McpToolInfo {
                name: "export_bom".to_string(),
                description: "Generate Bill of Materials (BOM) from active schematic or PCB".to_string(),
                category: "output".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "grouped": { "type": "boolean", "default": true },
                        "include_dnp": { "type": "boolean", "default": false },
                        "out_path": { "type": "string" }
                    }
                }),
            },
            McpToolInfo {
                name: "export_netlist".to_string(),
                description: "Derive and export authoritative electrical netlist from active schematic".to_string(),
                category: "output".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "out_path": { "type": "string" }
                    }
                }),
            },
            McpToolInfo {
                name: "export_cam".to_string(),
                description: "Export manufacturing files (gerber, drill, pnp, ipc2581, odbpp, all) for the active PCB".to_string(),
                category: "output".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "format": { "type": "string", "enum": ["gerber", "drill", "pnp", "ipc2581", "odbpp", "all"], "default": "all" },
                        "out_dir": { "type": "string" }
                    },
                    "required": ["out_dir"]
                }),
            },
            McpToolInfo {
                name: "calculate_impedance".to_string(),
                description: "Calculate PCB trace impedance (microstrip, embedded_microstrip, stripline, differential)".to_string(),
                category: "physics".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "mode": { "type": "string", "enum": ["microstrip", "embedded_microstrip", "stripline", "differential"], "default": "microstrip" },
                        "trace_width_um": { "type": "number", "default": 150.0 },
                        "dielectric_height_um": { "type": "number", "default": 100.0 },
                        "copper_thickness_um": { "type": "number", "default": 35.0 },
                        "er": { "type": "number", "default": 4.3 },
                        "spacing_um": { "type": "number", "default": 150.0 },
                        "coating_height_um": { "type": "number", "default": 25.0 },
                        "coating_er": { "type": "number", "default": 3.5 }
                    }
                }),
            },
            McpToolInfo {
                name: "altium_catalog_search".to_string(),
                description: "Search the 3,004-command Altium Designer .rcs UI catalog by keyword, namespace, or file".to_string(),
                category: "altium".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "default": "" },
                        "namespace": { "type": "string" },
                        "rcs_file": { "type": "string" },
                        "limit": { "type": "integer", "default": 25 }
                    }
                }),
            },
            McpToolInfo {
                name: "altium_command_describe".to_string(),
                description: "Inspect full metadata, shortcuts, params, and .rcs provenance for an Altium Designer command".to_string(),
                category: "altium".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string" }
                    },
                    "required": ["command"]
                }),
            },
            McpToolInfo {
                name: "save_design".to_string(),
                description: "Atomically save active PCB board and/or schematic sheet to disk (.snxpcb/.snxsch/.json)".to_string(),
                category: "io".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "board_out": { "type": "string" },
                        "schematic_out": { "type": "string" }
                    }
                }),
            },
        ]
    }

    /// Stateless convenience wrapper preserving the original `handle_request` signature.
    pub fn handle_request(
        req: &McpRequest,
        sheet: Option<&SchematicSheet>,
        board: Option<&PcbBoard>,
    ) -> McpResponse {
        let mut ctx = McpContext {
            schematic: sheet.cloned(),
            board: board.cloned(),
            ..McpContext::default()
        };
        Self::handle_request_with_context(req, &mut ctx)
    }

    /// Dispatch any MCP JSON-RPC request against a mutable [`McpContext`].
    pub fn handle_request_with_context(req: &McpRequest, ctx: &mut McpContext) -> McpResponse {
        match req.method.as_str() {
            "initialize" => McpResponse::ok(
                req.id.clone(),
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": { "listChanged": false },
                        "resources": { "subscribe": false, "listChanged": false },
                        "prompts": { "listChanged": false }
                    },
                    "serverInfo": {
                        "name": "oxide-eda-mcp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }),
            ),
            "notifications/initialized" | "initialized" | "ping" => {
                McpResponse::ok(req.id.clone(), json!({}))
            }
            "tools/list" => {
                let tools: Vec<Value> = Self::list_tools()
                    .into_iter()
                    .map(|t| {
                        json!({
                            "name": t.name,
                            "description": t.description,
                            "category": t.category,
                            "inputSchema": t.input_schema,
                            "input_schema": t.input_schema,
                        })
                    })
                    .collect();
                McpResponse::ok(req.id.clone(), json!({ "tools": tools }))
            }
            "resources/list" => {
                let mut resources = vec![json!({
                    "uri": "oxide://altium/catalog/stats",
                    "name": "Altium Designer UI Catalog Statistics",
                    "mimeType": "application/json"
                })];
                if ctx.schematic.is_some() {
                    resources.push(json!({
                        "uri": "oxide://schematic/active",
                        "name": "Active Schematic Sheet",
                        "mimeType": "application/json"
                    }));
                }
                if ctx.board.is_some() {
                    resources.push(json!({
                        "uri": "oxide://pcb/active",
                        "name": "Active PCB Board",
                        "mimeType": "application/json"
                    }));
                }
                McpResponse::ok(req.id.clone(), json!({ "resources": resources }))
            }
            "resources/read" => {
                let uri = req
                    .params
                    .get("uri")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                match uri {
                    "oxide://altium/catalog/stats" => {
                        let stats = AltiumCatalog::global().stats();
                        McpResponse::ok(
                            req.id.clone(),
                            json!({
                                "contents": [{
                                    "uri": uri,
                                    "mimeType": "application/json",
                                    "text": serde_json::to_string_pretty(&stats).unwrap_or_default()
                                }]
                            }),
                        )
                    }
                    "oxide://schematic/active" => match &ctx.schematic {
                        Some(sheet) => McpResponse::ok(
                            req.id.clone(),
                            json!({
                                "contents": [{
                                    "uri": uri,
                                    "mimeType": "application/json",
                                    "text": serde_json::to_string_pretty(sheet).unwrap_or_default()
                                }]
                            }),
                        ),
                        None => McpResponse::err(req.id.clone(), -32002, "No schematic loaded"),
                    },
                    "oxide://pcb/active" => match &ctx.board {
                        Some(board) => McpResponse::ok(
                            req.id.clone(),
                            json!({
                                "contents": [{
                                    "uri": uri,
                                    "mimeType": "application/json",
                                    "text": serde_json::to_string_pretty(board).unwrap_or_default()
                                }]
                            }),
                        ),
                        None => McpResponse::err(req.id.clone(), -32002, "No PCB board loaded"),
                    },
                    _ => McpResponse::err(
                        req.id.clone(),
                        -32602,
                        format!("Unknown resource URI: {uri}"),
                    ),
                }
            }
            "prompts/list" => McpResponse::ok(
                req.id.clone(),
                json!({
                    "prompts": [
                        {
                            "name": "drc_audit",
                            "description": "Audit active PCB design rules, clearances, and manufacturing readiness"
                        },
                        {
                            "name": "altium_migration",
                            "description": "Import Altium Designer binaries and verify schematic/PCB parity"
                        },
                        {
                            "name": "impedance_budget",
                            "description": "Compute controlled impedance stackup parameters for target Z0"
                        }
                    ]
                }),
            ),
            "prompts/get" => {
                let name = req
                    .params
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("drc_audit");
                let text = match name {
                    "drc_audit" => "1. Call read_pcb with include_details=true.\n2. Call run_drc and list_violations.\n3. Propose or apply fix_violation strategies and re-run run_drc.",
                    "altium_migration" => "1. Call import_altium with input_path.\n2. Run run_erc / run_drc.\n3. Verify BOM via export_bom and netlist via export_netlist.",
                    "impedance_budget" => "1. Inspect board layers via read_pcb.\n2. Compute microstrip/stripline/differential Z0 via calculate_impedance.",
                    _ => "Inspect design with read_schematic and read_pcb, then validate with run_erc and run_drc.",
                };
                McpResponse::ok(
                    req.id.clone(),
                    json!({
                        "description": format!("Oxide EDA workflow prompt: {name}"),
                        "messages": [{
                            "role": "user",
                            "content": { "type": "text", "text": text }
                        }]
                    }),
                )
            }
            "tools/call" => {
                let tool_name = req
                    .params
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let args = req
                    .params
                    .get("arguments")
                    .cloned()
                    .unwrap_or_else(|| json!({}));

                match Self::call_tool(tool_name, &args, ctx) {
                    Ok(value) => McpResponse::ok(req.id.clone(), value),
                    Err((code, msg)) => McpResponse::err(req.id.clone(), code, msg),
                }
            }
            _ => McpResponse::err(
                req.id.clone(),
                -32601,
                format!("Invalid MCP request method: {}", req.method),
            ),
        }
    }

    /// Execute a named MCP/UTCP tool against [`McpContext`].
    pub fn call_tool(
        tool_name: &str,
        args: &Value,
        ctx: &mut McpContext,
    ) -> Result<Value, (i32, String)> {
        match tool_name {
            "read_schematic" => Self::tool_read_schematic(args, ctx),
            "read_pcb" => Self::tool_read_pcb(args, ctx),
            "query_design" => Self::tool_query_design(args, ctx),
            "run_erc" => Self::tool_run_erc(args, ctx),
            "run_drc" => Self::tool_run_drc(args, ctx),
            "list_violations" => Self::tool_list_violations(args, ctx),
            "place_component" => Self::tool_place_component(args, ctx),
            "route_net" => Self::tool_route_net(args, ctx),
            "fix_violation" => Self::tool_fix_violation(args, ctx),
            "run_eco" => Self::tool_run_eco(args, ctx),
            "import_altium" => Self::tool_import_altium(args, ctx),
            "export_bom" => Self::tool_export_bom(args, ctx),
            "export_netlist" => Self::tool_export_netlist(args, ctx),
            "export_cam" => Self::tool_export_cam(args, ctx),
            "calculate_impedance" => Self::tool_calculate_impedance(args),
            "altium_catalog_search" => Self::tool_altium_catalog_search(args),
            "altium_command_describe" => Self::tool_altium_command_describe(args),
            "save_design" => Self::tool_save_design(args, ctx),
            _ => Err((-32601, format!("Unknown MCP tool: {tool_name}"))),
        }
    }

    fn tool_read_schematic(args: &Value, ctx: &McpContext) -> Result<Value, (i32, String)> {
        let Some(sheet) = &ctx.schematic else {
            return Ok(json!({
                "status": "ok",
                "loaded": false,
                "symbols_count": 0,
                "wires_count": 0,
                "labels_count": 0,
                "note": "No active schematic sheet loaded in context"
            }));
        };

        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        let include_netlist = args
            .get("include_netlist")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let symbols: Vec<Value> = sheet
            .symbols
            .iter()
            .filter(|s| {
                query.is_empty()
                    || s.reference.to_ascii_lowercase().contains(&query)
                    || s.value.to_ascii_lowercase().contains(&query)
                    || s.lib_id.to_ascii_lowercase().contains(&query)
            })
            .map(|s| {
                json!({
                    "reference": s.reference,
                    "value": s.value,
                    "footprint": s.footprint,
                    "lib_id": s.lib_id,
                    "x": s.position.x,
                    "y": s.position.y,
                    "dnp": s.dnp
                })
            })
            .collect();

        let nets = if include_netlist {
            let nl = build_netlist(sheet);
            nl.nets
                .iter()
                .filter(|n| query.is_empty() || n.name.to_ascii_lowercase().contains(&query))
                .map(|n| {
                    json!({
                        "name": n.name,
                        "terminals": n.terminals.len()
                    })
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        Ok(json!({
            "status": "ok",
            "loaded": true,
            "paper_size": sheet.paper_size,
            "symbols_count": sheet.symbols.len(),
            "wires_count": sheet.wires.len(),
            "labels_count": sheet.labels.len(),
            "buses_count": sheet.buses.len(),
            "child_sheets_count": sheet.child_sheets.len(),
            "symbols": symbols,
            "nets": nets
        }))
    }

    fn tool_read_pcb(args: &Value, ctx: &McpContext) -> Result<Value, (i32, String)> {
        let Some(board) = &ctx.board else {
            return Err((-32000, "No active PCB layout loaded".to_string()));
        };

        let net_filter = args
            .get("net")
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_ascii_lowercase());
        let include_details = args
            .get("include_details")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let matching_net_ids: Vec<u32> = if let Some(ref nf) = net_filter {
            board
                .nets
                .iter()
                .filter(|n| n.name.to_ascii_lowercase().contains(nf))
                .map(|n| n.number)
                .collect()
        } else {
            Vec::new()
        };

        let filtered_segments = if net_filter.is_some() {
            board
                .segments
                .iter()
                .filter(|s| matching_net_ids.contains(&s.net))
                .count()
        } else {
            board.segments.len()
        };

        let filtered_vias = if net_filter.is_some() {
            board
                .vias
                .iter()
                .filter(|v| matching_net_ids.contains(&v.net))
                .count()
        } else {
            board.vias.len()
        };

        let mut payload = json!({
            "status": "ok",
            "thickness": board.thickness,
            "layers_count": board.layers.len(),
            "nets_count": board.nets.len(),
            "footprints_count": board.footprints.len(),
            "segments_count": filtered_segments,
            "vias_count": filtered_vias,
            "zones_count": board.zones.len(),
        });

        if include_details {
            payload["layers"] = json!(
                board
                    .layers
                    .iter()
                    .map(|l| json!({"id": l.id, "name": l.name, "type": l.layer_type}))
                    .collect::<Vec<_>>()
            );
            payload["footprints"] = json!(
                board
                    .footprints
                    .iter()
                    .map(|fp| json!({
                        "reference": fp.reference,
                        "value": fp.value,
                        "footprint_id": fp.footprint_id,
                        "x": fp.position.x,
                        "y": fp.position.y,
                        "rotation": fp.rotation,
                        "layer": fp.layer,
                        "pads": fp.pads.len()
                    }))
                    .collect::<Vec<_>>()
            );
            payload["nets"] = json!(
                board
                    .nets
                    .iter()
                    .map(|n| json!({"number": n.number, "name": n.name}))
                    .collect::<Vec<_>>()
            );
        }

        Ok(payload)
    }

    fn tool_query_design(args: &Value, ctx: &McpContext) -> Result<Value, (i32, String)> {
        let query = args
            .get("nl_query")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        let kind = args
            .get("kind")
            .and_then(|v| v.as_str())
            .unwrap_or("all");

        let mut matches = Vec::new();
        if let Some(sheet) = &ctx.schematic {
            if kind == "all" || kind == "symbol" {
                for sym in &sheet.symbols {
                    if query.is_empty()
                        || sym.reference.to_ascii_lowercase().contains(&query)
                        || sym.value.to_ascii_lowercase().contains(&query)
                        || sym.footprint.to_ascii_lowercase().contains(&query)
                    {
                        matches.push(json!({
                            "domain": "schematic",
                            "kind": "symbol",
                            "reference": sym.reference,
                            "value": sym.value,
                            "footprint": sym.footprint,
                            "x": sym.position.x,
                            "y": sym.position.y
                        }));
                    }
                }
            }
            if kind == "all" || kind == "label" {
                for lbl in &sheet.labels {
                    if query.is_empty() || lbl.text.to_ascii_lowercase().contains(&query) {
                        matches.push(json!({
                            "domain": "schematic",
                            "kind": "label",
                            "text": lbl.text,
                            "x": lbl.position.x,
                            "y": lbl.position.y
                        }));
                    }
                }
            }
        }

        if let Some(board) = &ctx.board {
            if kind == "all" || kind == "footprint" {
                for fp in &board.footprints {
                    if query.is_empty()
                        || fp.reference.to_ascii_lowercase().contains(&query)
                        || fp.value.to_ascii_lowercase().contains(&query)
                        || fp.footprint_id.to_ascii_lowercase().contains(&query)
                    {
                        matches.push(json!({
                            "domain": "pcb",
                            "kind": "footprint",
                            "reference": fp.reference,
                            "value": fp.value,
                            "footprint_id": fp.footprint_id,
                            "layer": fp.layer,
                            "x": fp.position.x,
                            "y": fp.position.y
                        }));
                    }
                }
            }
            if kind == "all" || kind == "net" {
                for net in &board.nets {
                    if query.is_empty() || net.name.to_ascii_lowercase().contains(&query) {
                        matches.push(json!({
                            "domain": "pcb",
                            "kind": "net",
                            "number": net.number,
                            "name": net.name
                        }));
                    }
                }
            }
        }

        Ok(json!({
            "status": "ok",
            "query": query,
            "count": matches.len(),
            "matches": matches
        }))
    }

    fn tool_run_erc(args: &Value, ctx: &McpContext) -> Result<Value, (i32, String)> {
        let Some(sheet) = &ctx.schematic else {
            return Err((-32000, "No active schematic sheet loaded".to_string()));
        };
        let severity_filter = args
            .get("severity")
            .and_then(|v| v.as_str())
            .unwrap_or("all")
            .to_ascii_lowercase();

        let violations = oxide_erc::run(sheet);
        let serialized: Vec<Value> = violations
            .iter()
            .filter(|v| {
                severity_filter == "all"
                    || format!("{:?}", v.severity).to_ascii_lowercase() == severity_filter
            })
            .map(|v| {
                json!({
                    "rule": format!("{:?}", v.rule),
                    "severity": format!("{:?}", v.severity),
                    "message": v.message,
                    "location": { "x": v.location.x, "y": v.location.y }
                })
            })
            .collect();

        let errors = violations
            .iter()
            .filter(|v| v.severity == oxide_erc::Severity::Error)
            .count();

        Ok(json!({
            "status": if errors == 0 { "passed" } else { "failed" },
            "total": serialized.len(),
            "errors": errors,
            "violations": serialized
        }))
    }

    fn tool_run_drc(args: &Value, ctx: &McpContext) -> Result<Value, (i32, String)> {
        let Some(board) = &ctx.board else {
            return Err((-32000, "No active PCB layout loaded".to_string()));
        };
        let limit = args
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(100) as usize;
        let cm = ctx.effective_rules(board);
        let violations = cm.run_drc_parallel(board);
        let total = violations.len();

        let items: Vec<Value> = violations
            .into_iter()
            .take(limit)
            .enumerate()
            .map(|(idx, v)| {
                json!({
                    "id": format!("DRC-{}", idx + 1),
                    "type": format!("{:?}", v.violation_type),
                    "message": v.message,
                    "required": v.required_value,
                    "actual": v.actual_value,
                    "location": v.location.map(|(x, y)| json!({"x": x, "y": y})),
                    "object_ids": v.object_ids
                })
            })
            .collect();

        Ok(json!({
            "status": if total == 0 { "passed" } else { "failed" },
            "total": total,
            "returned": items.len(),
            "violations": items
        }))
    }

    fn tool_list_violations(args: &Value, ctx: &McpContext) -> Result<Value, (i32, String)> {
        let category = args
            .get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("All");
        let mut all = Vec::new();

        if (category == "All" || category == "Electrical")
            && let Some(sheet) = &ctx.schematic
        {
            for (i, v) in oxide_erc::run(sheet).into_iter().enumerate() {
                all.push(json!({
                    "id": format!("ERC-{}", i + 1),
                    "category": "Electrical",
                    "rule": format!("{:?}", v.rule),
                    "severity": format!("{:?}", v.severity),
                    "message": v.message,
                    "location": { "x": v.location.x, "y": v.location.y }
                }));
            }
        }

        if (category == "All" || category == "Clearance" || category == "Manufacturing")
            && let Some(board) = &ctx.board
        {
            let cm = ctx.effective_rules(board);
            for (i, v) in cm.run_drc_parallel(board).into_iter().enumerate() {
                let kind = format!("{:?}", v.violation_type);
                let cat = if kind.contains("Acid") || kind.contains("Sliver") || kind.contains("Annular") {
                    "Manufacturing"
                } else {
                    "Clearance"
                };
                if category == "All" || category == cat {
                    all.push(json!({
                        "id": format!("DRC-{}", i + 1),
                        "category": cat,
                        "rule": kind,
                        "severity": "Error",
                        "message": v.message,
                        "required": v.required_value,
                        "actual": v.actual_value,
                        "location": v.location.map(|(x, y)| json!({"x": x, "y": y})),
                        "object_ids": v.object_ids
                    }));
                }
            }
        }

        Ok(json!({
            "status": "ok",
            "category": category,
            "count": all.len(),
            "violations": all
        }))
    }

    fn tool_place_component(args: &Value, ctx: &mut McpContext) -> Result<Value, (i32, String)> {
        let refdes = args
            .get("reference")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        if refdes.is_empty() {
            return Err((-32602, "reference must not be empty".to_string()));
        }
        let x = args
            .get("x")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| (-32602, "numeric x coordinate required".to_string()))?;
        let y = args
            .get("y")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| (-32602, "numeric y coordinate required".to_string()))?;
        let rotation = args.get("rotation").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let value = args
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let footprint_id = args
            .get("footprint_id")
            .and_then(|v| v.as_str())
            .unwrap_or("Generic:SMD")
            .to_string();
        let layer = args
            .get("layer")
            .and_then(|v| v.as_str())
            .unwrap_or("F.Cu")
            .to_string();

        let action = if let Some(board) = ctx.board.as_mut() {
            if let Some(existing) = board.footprints.iter_mut().find(|f| f.reference == refdes) {
                existing.position = Point::new(x, y);
                existing.rotation = rotation;
                if !value.is_empty() {
                    existing.value = value.clone();
                }
                ctx.dirty_board = true;
                "moved"
            } else {
                board.footprints.push(Footprint {
                    uuid: Uuid::new_v4(),
                    reference: refdes.to_string(),
                    value,
                    footprint_id,
                    position: Point::new(x, y),
                    rotation,
                    layer,
                    locked: false,
                    pads: Vec::new(),
                    graphics: Vec::new(),
                    properties: Vec::new(),
                });
                ctx.dirty_board = true;
                "placed"
            }
        } else {
            "simulated"
        };

        Ok(json!({
            "status": "ok",
            "action": action,
            "placed": {
                "reference": refdes,
                "position": { "x": x, "y": y },
                "rotation": rotation
            }
        }))
    }

    fn tool_route_net(args: &Value, ctx: &mut McpContext) -> Result<Value, (i32, String)> {
        let net_name = args
            .get("net_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        let strategy = args
            .get("strategy")
            .and_then(|v| v.as_str())
            .unwrap_or("PushAndShove");

        let Some(board) = ctx.board.as_mut() else {
            return Err((-32000, "No active PCB layout loaded".to_string()));
        };

        let target_ids: Vec<u32> = if net_name == "*" || net_name.is_empty() {
            board.nets.iter().map(|n| n.number).collect()
        } else {
            board
                .nets
                .iter()
                .filter(|n| n.name.eq_ignore_ascii_case(net_name))
                .map(|n| n.number)
                .collect()
        };

        if target_ids.is_empty() {
            return Err((
                -32602,
                format!("Net '{net_name}' not found on active PCB board"),
            ));
        }

        let before_segments = board.segments.len();
        let cm = Arc::new(ConstraintManager::standard_default());
        let spatial = Arc::new(SpatialIndex::build(board));
        let mut autorouter = TopologicalAutorouter::new(cm, spatial);
        let _ = autorouter.build_topological_map(board);
        let results = autorouter.route_board(board, &target_ids);
        let added = board.segments.len().saturating_sub(before_segments);
        if added > 0 {
            ctx.dirty_board = true;
        }

        Ok(json!({
            "status": "ok",
            "net_name": net_name,
            "strategy": strategy,
            "nets_targeted": target_ids.len(),
            "routed_paths": results.len(),
            "routed_segments": added
        }))
    }

    fn tool_fix_violation(args: &Value, ctx: &mut McpContext) -> Result<Value, (i32, String)> {
        let violation_id = args
            .get("violation_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let strategy = args
            .get("strategy")
            .and_then(|v| v.as_str())
            .unwrap_or("auto");

        let mut changes = 0usize;
        let mut before_drc = 0usize;
        let mut after_drc = 0usize;

        if matches!(strategy, "prune_wires" | "auto")
            && let Some(sheet) = ctx.schematic.as_mut()
        {
            let removed = prune_wire_redundancies(&mut sheet.wires);
            if removed > 0 {
                ctx.dirty_schematic = true;
                changes += removed;
            }
        }

        if let Some(board) = ctx.board.take() {
            let mut board = board;
            let cm = ctx.effective_rules(&board);
            before_drc = cm.run_drc_parallel(&board).len();

            match strategy {
                "teardrops" | "auto" => {
                    let added = TeardropGenerator::apply_teardrops_to_board(&mut board);
                    if added > 0 {
                        ctx.dirty_board = true;
                        changes += added;
                    }
                }
                "min_track_width" => {
                    let min_w = board
                        .setup
                        .as_ref()
                        .map(|s| s.track_min_width)
                        .filter(|w| *w > 0.0)
                        .unwrap_or(0.15);
                    for seg in &mut board.segments {
                        if seg.width < min_w {
                            seg.width = min_w;
                            changes += 1;
                            ctx.dirty_board = true;
                        }
                    }
                }
                _ => {}
            }

            after_drc = cm.run_drc_parallel(&board).len();
            ctx.board = Some(board);
        }

        Ok(json!({
            "status": "ok",
            "violation_id": violation_id,
            "strategy": strategy,
            "changes_applied": changes,
            "drc_before": before_drc,
            "drc_after": after_drc,
            "resolved": after_drc <= before_drc
        }))
    }

    fn tool_run_eco(args: &Value, ctx: &mut McpContext) -> Result<Value, (i32, String)> {
        let dry_run = args
            .get("dry_run")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let Some(sheet) = &ctx.schematic else {
            return Err((-32000, "No active schematic sheet loaded".to_string()));
        };
        let board = ctx.board.get_or_insert_with(PcbBoard::default);
        let netlist = build_netlist(sheet);
        let components: Vec<(String, String, String)> = sheet
            .symbols
            .iter()
            .map(|s| (s.reference.clone(), s.value.clone(), s.footprint.clone()))
            .collect();

        let report = EcoEngine::diff_schematic_to_pcb(&netlist, board, &components);
        let actions_count = report.len();
        if !dry_run && actions_count > 0 {
            EcoEngine::apply_eco(board, &report);
            ctx.dirty_board = true;
        }

        Ok(json!({
            "status": "ok",
            "dry_run": dry_run,
            "actions_count": actions_count,
            "board_footprints": board.footprints.len(),
            "board_nets": board.nets.len()
        }))
    }

    fn tool_import_altium(args: &Value, ctx: &mut McpContext) -> Result<Value, (i32, String)> {
        let input_path = args
            .get("input_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| (-32602, "input_path is required".to_string()))?;
        let load_into_session = args
            .get("load_into_session")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let out_path = args.get("out_path").and_then(|v| v.as_str());

        let result = import_altium_file(Path::new(input_path))
            .map_err(|e| (-32001, format!("Altium import failed: {e}")))?;

        match result {
            AltiumImportResult::Schematic(sheet) => {
                let summary = json!({
                    "kind": "Schematic",
                    "symbols": sheet.symbols.len(),
                    "wires": sheet.wires.len(),
                    "labels": sheet.labels.len()
                });
                if let Some(out) = out_path {
                    write_schematic_file(Path::new(out), &sheet)
                        .map_err(|e| (-32001, e))?;
                }
                if load_into_session {
                    ctx.schematic = Some(sheet);
                    ctx.dirty_schematic = true;
                }
                Ok(json!({ "status": "ok", "imported": summary }))
            }
            AltiumImportResult::Pcb(board) => {
                let summary = json!({
                    "kind": "Pcb",
                    "footprints": board.footprints.len(),
                    "segments": board.segments.len(),
                    "vias": board.vias.len(),
                    "zones": board.zones.len()
                });
                if let Some(out) = out_path {
                    write_board_file(Path::new(out), &board).map_err(|e| (-32001, e))?;
                }
                if load_into_session {
                    ctx.board = Some(board);
                    ctx.dirty_board = true;
                }
                Ok(json!({ "status": "ok", "imported": summary }))
            }
            AltiumImportResult::SymbolLibrary(symbols) => {
                if let Some(out) = out_path {
                    let json = serde_json::to_string_pretty(&symbols)
                        .map_err(|e| (-32001, e.to_string()))?;
                    atomic_write(Path::new(out), json.as_bytes())
                        .map_err(|e| (-32001, e.to_string()))?;
                }
                Ok(json!({
                    "status": "ok",
                    "imported": { "kind": "SymbolLibrary", "symbols": symbols.len() }
                }))
            }
            AltiumImportResult::FootprintLibrary(fps) => {
                if let Some(out) = out_path {
                    let json = serde_json::to_string_pretty(&fps)
                        .map_err(|e| (-32001, e.to_string()))?;
                    atomic_write(Path::new(out), json.as_bytes())
                        .map_err(|e| (-32001, e.to_string()))?;
                }
                Ok(json!({
                    "status": "ok",
                    "imported": { "kind": "FootprintLibrary", "footprints": fps.len() }
                }))
            }
            AltiumImportResult::IntegratedLibrary(intlib) => {
                if let Some(out) = out_path {
                    let json = serde_json::to_string_pretty(&intlib)
                        .map_err(|e| (-32001, e.to_string()))?;
                    atomic_write(Path::new(out), json.as_bytes())
                        .map_err(|e| (-32001, e.to_string()))?;
                }
                Ok(json!({
                    "status": "ok",
                    "imported": {
                        "kind": "IntegratedLibrary",
                        "symbols": intlib.symbols.len(),
                        "footprints": intlib.footprints.len()
                    }
                }))
            }
        }
    }

    fn tool_export_bom(args: &Value, ctx: &McpContext) -> Result<Value, (i32, String)> {
        let grouped = args.get("grouped").and_then(|v| v.as_bool()).unwrap_or(true);
        let include_dnp = args
            .get("include_dnp")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let mut rows: Vec<(String, String, String, bool)> = Vec::new();
        if let Some(sheet) = &ctx.schematic {
            for s in &sheet.symbols {
                if !s.is_power && (include_dnp || !s.dnp) {
                    rows.push((
                        s.reference.clone(),
                        s.value.clone(),
                        s.footprint.clone(),
                        s.dnp,
                    ));
                }
            }
        } else if let Some(board) = &ctx.board {
            for fp in &board.footprints {
                rows.push((
                    fp.reference.clone(),
                    fp.value.clone(),
                    fp.footprint_id.clone(),
                    false,
                ));
            }
        } else {
            return Err((-32000, "No schematic or PCB loaded for BOM export".to_string()));
        }

        let bom_items: Vec<Value> = if grouped {
            let mut groups: HashMap<(String, String, bool), Vec<String>> = HashMap::new();
            for (r, v, fp, dnp) in rows {
                groups.entry((v, fp, dnp)).or_default().push(r);
            }
            let mut sorted: Vec<_> = groups.into_iter().collect();
            sorted.sort_by(|a, b| a.1.cmp(&b.1));
            sorted
                .into_iter()
                .map(|((val, fp, dnp), mut refs)| {
                    refs.sort();
                    json!({
                        "qty": refs.len(),
                        "designators": refs,
                        "value": val,
                        "footprint": fp,
                        "dnp": dnp
                    })
                })
                .collect()
        } else {
            rows.into_iter()
                .map(|(r, v, fp, dnp)| {
                    json!({
                        "qty": 1,
                        "designators": [r],
                        "value": v,
                        "footprint": fp,
                        "dnp": dnp
                    })
                })
                .collect()
        };

        if let Some(out_path) = args.get("out_path").and_then(|v| v.as_str()) {
            let payload = serde_json::to_string_pretty(&bom_items)
                .map_err(|e| (-32001, e.to_string()))?;
            atomic_write(Path::new(out_path), payload.as_bytes())
                .map_err(|e| (-32001, e.to_string()))?;
        }

        Ok(json!({
            "status": "ok",
            "grouped": grouped,
            "row_count": bom_items.len(),
            "items": bom_items
        }))
    }

    fn tool_export_netlist(args: &Value, ctx: &McpContext) -> Result<Value, (i32, String)> {
        let Some(sheet) = &ctx.schematic else {
            return Err((-32000, "No active schematic sheet loaded".to_string()));
        };
        let nl = build_netlist(sheet);
        let nets: Vec<Value> = nl
            .nets
            .iter()
            .map(|n| {
                json!({
                    "name": n.name,
                    "terminals": n.terminals.iter().map(|node| format!("{}.{}", node.reference, node.pin)).collect::<Vec<_>>()
                })
            })
            .collect();

        if let Some(out_path) = args.get("out_path").and_then(|v| v.as_str()) {
            let payload = serde_json::to_string_pretty(&nets)
                .map_err(|e| (-32001, e.to_string()))?;
            atomic_write(Path::new(out_path), payload.as_bytes())
                .map_err(|e| (-32001, e.to_string()))?;
        }

        Ok(json!({
            "status": "ok",
            "nets_count": nets.len(),
            "nets": nets
        }))
    }

    fn tool_export_cam(args: &Value, ctx: &McpContext) -> Result<Value, (i32, String)> {
        let Some(board) = &ctx.board else {
            return Err((-32000, "No active PCB layout loaded".to_string()));
        };
        let format = args
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("all");
        let out_dir_str = args
            .get("out_dir")
            .and_then(|v| v.as_str())
            .ok_or_else(|| (-32602, "out_dir is required".to_string()))?;
        let out_dir = Path::new(out_dir_str);
        std::fs::create_dir_all(out_dir).map_err(|e| (-32001, e.to_string()))?;

        let mut files_written = Vec::new();
        if format == "gerber" || format == "all" {
            let exp = GerberExporter::new(GerberOptions::default());
            let outputs = exp.export_board(board).map_err(|e| (-32001, e.to_string()))?;
            for g in outputs {
                let p = out_dir.join(&g.filename);
                std::fs::write(&p, &g.content).map_err(|e| (-32001, e.to_string()))?;
                files_written.push(p.display().to_string());
            }
        }
        if format == "drill" || format == "all" {
            let exp = ExcellonExporter::new();
            let outputs = exp.export_board(board).map_err(|e| (-32001, e.to_string()))?;
            for d in outputs {
                let p = out_dir.join(&d.filename);
                std::fs::write(&p, &d.content).map_err(|e| (-32001, e.to_string()))?;
                files_written.push(p.display().to_string());
            }
        }
        if format == "pnp" || format == "all" {
            let exp = PickAndPlaceExporter::new(PickAndPlaceOptions::default());
            let csv = exp.export(board).map_err(|e| (-32001, e.to_string()))?;
            let p = out_dir.join("pick_and_place.csv");
            std::fs::write(&p, &csv).map_err(|e| (-32001, e.to_string()))?;
            files_written.push(p.display().to_string());
        }
        if format == "ipc2581" {
            let res = export_ipc2581(board, &oxide_output::Ipc2581Options::default())
                .map_err(|e| (-32001, e.to_string()))?;
            let p = out_dir.join("board.ipc2581.xml");
            std::fs::write(&p, &res.xml_content).map_err(|e| (-32001, e.to_string()))?;
            files_written.push(p.display().to_string());
        }
        if format == "odbpp" {
            let pkg = export_odbpp_package(board).map_err(|e| (-32001, e.to_string()))?;
            for f in &pkg.files {
                let p = out_dir.join(&f.relative_path);
                if let Some(parent) = p.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| (-32001, e.to_string()))?;
                }
                std::fs::write(&p, &f.content).map_err(|e| (-32001, e.to_string()))?;
                files_written.push(p.display().to_string());
            }
        }

        Ok(json!({
            "status": "ok",
            "format": format,
            "files_count": files_written.len(),
            "files": files_written
        }))
    }

    fn tool_calculate_impedance(args: &Value) -> Result<Value, (i32, String)> {
        let mode = args
            .get("mode")
            .and_then(|v| v.as_str())
            .unwrap_or("microstrip");
        let w = args
            .get("trace_width_um")
            .and_then(|v| v.as_f64())
            .unwrap_or(150.0)
            .round() as i64;
        let h = args
            .get("dielectric_height_um")
            .and_then(|v| v.as_f64())
            .unwrap_or(100.0)
            .round() as i64;
        let t = args
            .get("copper_thickness_um")
            .and_then(|v| v.as_f64())
            .unwrap_or(35.0)
            .round() as i64;
        let er = args.get("er").and_then(|v| v.as_f64()).unwrap_or(4.3);
        let s = args
            .get("spacing_um")
            .and_then(|v| v.as_f64())
            .unwrap_or(150.0)
            .round() as i64;
        let c_h = args
            .get("coating_height_um")
            .and_then(|v| v.as_f64())
            .unwrap_or(25.0)
            .round() as i64;
        let c_er = args
            .get("coating_er")
            .and_then(|v| v.as_f64())
            .unwrap_or(3.5);

        let z0 = match mode {
            "microstrip" => ImpedanceCalculator::calculate_microstrip(w, h, t, er),
            "embedded_microstrip" => {
                ImpedanceCalculator::calculate_embedded_microstrip(w, h, c_h, t, er, c_er)
            }
            "stripline" => ImpedanceCalculator::calculate_stripline(w, h, t, er),
            "differential" => {
                let z0_se = ImpedanceCalculator::calculate_microstrip(w, h, t, er);
                ImpedanceCalculator::calculate_differential_microstrip(z0_se, s, h)
            }
            other => return Err((-32602, format!("Unsupported impedance mode: {other}"))),
        };

        Ok(json!({
            "status": "ok",
            "mode": mode,
            "z0_ohms": z0,
            "inputs": {
                "trace_width_um": w,
                "dielectric_height_um": h,
                "copper_thickness_um": t,
                "er": er
            }
        }))
    }

    fn tool_altium_catalog_search(args: &Value) -> Result<Value, (i32, String)> {
        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let namespace = args.get("namespace").and_then(|v| v.as_str());
        let rcs_file = args.get("rcs_file").and_then(|v| v.as_str());
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(25) as usize;

        let hits = AltiumCatalog::global().search(query, namespace, rcs_file, limit);
        let items: Vec<Value> = hits
            .into_iter()
            .map(|e| {
                json!({
                    "command": e.cmd.command,
                    "caption": e.cmd.plain_caption(),
                    "namespace": e.cmd.namespace(),
                    "shortcut1": e.cmd.shortcut1,
                    "shortcut2": e.cmd.shortcut2,
                    "params": e.cmd.params,
                    "description": e.cmd.description,
                    "source_file": e.source_file
                })
            })
            .collect();

        Ok(json!({
            "status": "ok",
            "count": items.len(),
            "commands": items
        }))
    }

    fn tool_altium_command_describe(args: &Value) -> Result<Value, (i32, String)> {
        let command = args
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| (-32602, "command is required".to_string()))?;

        let entries = AltiumCatalog::global().inspect(command);
        if entries.is_empty() {
            return Err((-32602, format!("Altium command '{command}' not found in catalog")));
        }
        let definitions: Vec<Value> = entries
            .into_iter()
            .map(|e| {
                json!({
                    "command": e.cmd.command,
                    "caption": e.cmd.plain_caption(),
                    "raw_caption": e.cmd.caption,
                    "namespace": e.cmd.namespace(),
                    "action": e.cmd.action(),
                    "image": e.cmd.image,
                    "shortcut1": e.cmd.shortcut1,
                    "shortcut2": e.cmd.shortcut2,
                    "params": e.cmd.params,
                    "description": e.cmd.description,
                    "object": e.cmd.object,
                    "source_file": e.source_file
                })
            })
            .collect();

        Ok(json!({
            "status": "ok",
            "command": command,
            "definitions": definitions
        }))
    }

    fn tool_save_design(args: &Value, ctx: &mut McpContext) -> Result<Value, (i32, String)> {
        let board_out = args.get("board_out").and_then(|v| v.as_str()).map(Path::new);
        let sch_out = args
            .get("schematic_out")
            .and_then(|v| v.as_str())
            .map(Path::new);
        let saved = ctx
            .save_targets(board_out, sch_out)
            .map_err(|e| (-32001, e))?;
        Ok(json!({
            "status": "ok",
            "saved_files": saved
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tools() {
        let tools = McpServer::list_tools();
        assert!(tools.len() >= 18);
        assert!(tools.iter().any(|t| t.name == "read_pcb"));
        assert!(tools.iter().any(|t| t.name == "place_component"));
        assert!(tools.iter().any(|t| t.name == "altium_catalog_search"));
    }

    #[test]
    fn test_handle_mcp_place_component() {
        let req = McpRequest {
            id: Value::String("msg-1".to_string()),
            method: "tools/call".to_string(),
            params: json!({
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
        let result = res.result.expect("result present");
        assert_eq!(result["status"], "ok");
        assert_eq!(result["placed"]["reference"], "U1");
    }

    #[test]
    fn test_mcp_initialize_and_altium_catalog_search() {
        let mut ctx = McpContext::default();
        let init_req = McpRequest {
            id: Value::Number(1.into()),
            method: "initialize".to_string(),
            params: json!({}),
        };
        let init_res = McpServer::handle_request_with_context(&init_req, &mut ctx);
        assert!(init_res.error.is_none());
        assert_eq!(
            init_res.result.as_ref().unwrap()["serverInfo"]["name"],
            "oxide-eda-mcp"
        );

        let search_req = McpRequest {
            id: Value::Number(2.into()),
            method: "tools/call".to_string(),
            params: json!({
                "name": "altium_catalog_search",
                "arguments": { "query": "PCB:Undo", "limit": 5 }
            }),
        };
        let search_res = McpServer::handle_request_with_context(&search_req, &mut ctx);
        assert!(search_res.error.is_none());
        assert!(search_res.result.unwrap()["count"].as_u64().unwrap() >= 1);
    }
}
