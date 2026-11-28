//! Rich CLI surfaces for MCP (Model Context Protocol), UTCP (Universal Tool
//! Call Protocol), and the 3,004-command Altium Designer `.rcs` UI catalog.

use std::path::{Path, PathBuf};

use clap::{Args, Subcommand, ValueEnum};
use oxide_ai::altium_catalog::AltiumCatalog;
use oxide_ai::{
    McpContext, McpRequest, McpServer, UtcpCallRequest, execute_utcp_batch, execute_utcp_call,
    utcp_manifest_for_transport, validate_utcp_manifest,
};
use serde_json::{Map, Value, json};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Json,
    Compact,
    Table,
}

#[derive(Debug, Clone, Args, Default)]
pub struct SessionArgs {
    /// PCB board file (.snxpcb or .json) to load into session context
    #[arg(long)]
    pub board: Option<PathBuf>,
    /// Schematic file (.snxsch or .json) to load into session context
    #[arg(long)]
    pub schematic: Option<PathBuf>,
    /// Design rules JSON file to apply to DRC checks
    #[arg(long)]
    pub rules: Option<PathBuf>,
    /// Optional output path to save mutated PCB board (or overwrite --board if omitted when --in-place is set)
    #[arg(long)]
    pub save_board: Option<PathBuf>,
    /// Optional output path to save mutated schematic
    #[arg(long)]
    pub save_schematic: Option<PathBuf>,
    /// Save mutations back in-place to the input --board / --schematic paths
    #[arg(long)]
    pub in_place: bool,
}

impl SessionArgs {
    pub fn load_context(&self) -> Result<McpContext, Box<dyn std::error::Error>> {
        let ctx = McpContext::load(
            self.board.as_deref(),
            self.schematic.as_deref(),
            self.rules.as_deref(),
        )?;
        Ok(ctx)
    }

    pub fn persist_if_needed(
        &self,
        ctx: &mut McpContext,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let board_target: Option<PathBuf> = if self.save_board.is_some() {
            self.save_board.clone()
        } else if self.in_place && ctx.dirty_board {
            ctx.board_path.clone()
        } else {
            None
        };

        let sch_target: Option<PathBuf> = if self.save_schematic.is_some() {
            self.save_schematic.clone()
        } else if self.in_place && ctx.dirty_schematic {
            ctx.schematic_path.clone()
        } else {
            None
        };

        if board_target.is_none() && sch_target.is_none() {
            return Ok(Vec::new());
        }
        let saved = ctx.save_targets(board_target.as_deref(), sch_target.as_deref())?;
        Ok(saved)
    }
}

#[derive(Debug, Clone, Args)]
pub struct McpArgs {
    #[command(subcommand)]
    pub action: Option<McpSubcommand>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum McpSubcommand {
    /// Start stateful JSON-RPC 2.0 MCP stdio server (default when no subcommand is given)
    Serve {
        #[command(flatten)]
        session: SessionArgs,
    },
    /// List all available MCP tools with category and description
    List {
        /// Filter tools by category (schematic, pcb, validation, routing, altium, output, physics, etc.)
        #[arg(long)]
        category: Option<String>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Print the JSON input schema for a specific MCP tool
    Schema { tool: String },
    /// Invoke an MCP tool directly from the CLI
    Call {
        /// Tool name (see `oxide-cli mcp list`)
        tool: String,
        /// JSON object string of arguments
        #[arg(long)]
        args: Option<String>,
        /// Path to a JSON file containing tool arguments
        #[arg(long)]
        args_file: Option<PathBuf>,
        /// Key=value argument pairs (auto-coerced to bool/number/json/string), repeatable
        #[arg(long = "arg", value_name = "KEY=VALUE")]
        kv_args: Vec<String>,
        #[command(flatten)]
        session: SessionArgs,
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
    /// Execute a JSONL batch of JSON-RPC 2.0 MCP requests from a file or stdin ('-')
    Batch {
        /// Input JSONL file path or '-' for stdin
        #[arg(short, long, default_value = "-")]
        input: String,
        #[command(flatten)]
        session: SessionArgs,
        #[arg(long)]
        stop_on_error: bool,
    },
}

#[derive(Debug, Clone, Args)]
pub struct UtcpArgs {
    #[command(subcommand)]
    pub action: Option<UtcpSubcommand>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum UtcpSubcommand {
    /// Emit UTCP 1.0 discovery manifest (default when no subcommand is given)
    Manifest {
        /// Transport template ('stdio' or 'cli')
        #[arg(long, default_value = "stdio")]
        transport: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
        /// Optional output path to write the manifest JSON
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// List all UTCP tools with tags and schemas
    List {
        /// Filter tools by tag or category
        #[arg(long)]
        tag: Option<String>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Describe a single UTCP tool with full parameters and CLI invocation template
    Describe {
        tool: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
    /// Execute a single UTCP tool call
    Call {
        tool: String,
        /// JSON object string of arguments
        #[arg(long)]
        args: Option<String>,
        /// Path to JSON file containing arguments
        #[arg(long)]
        args_file: Option<PathBuf>,
        /// Key=value argument pairs (auto-coerced to bool/number/json/string), repeatable
        #[arg(long = "arg", value_name = "KEY=VALUE")]
        kv_args: Vec<String>,
        #[command(flatten)]
        session: SessionArgs,
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
    /// Execute a sequential UTCP batch pipeline from a JSON array / JSONL file or stdin ('-')
    Batch {
        /// Input JSON array / JSONL path or '-' for stdin
        #[arg(short, long, default_value = "-")]
        input: String,
        #[command(flatten)]
        session: SessionArgs,
        #[arg(long)]
        stop_on_error: bool,
    },
    /// Validate the built-in UTCP manifest or an external manifest JSON file
    Validate {
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Args)]
pub struct AltiumCatalogArgs {
    #[command(subcommand)]
    pub action: AltiumCatalogSubcommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum AltiumCatalogSubcommand {
    /// Show catalog statistics (3,004 commands across 39 .rcs files and namespaces)
    Stats {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Search Altium Designer commands by keyword, namespace, or .rcs file
    Search {
        #[arg(default_value = "")]
        query: String,
        #[arg(short, long)]
        namespace: Option<String>,
        #[arg(short, long)]
        rcs: Option<String>,
        #[arg(short, long, default_value_t = 30)]
        limit: usize,
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Inspect full metadata for an Altium Designer command (e.g. PCB:Undo)
    Inspect {
        command: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
}

pub async fn run_mcp_command(args: McpArgs) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        None => run_mcp_stdio(SessionArgs::default()).await,
        Some(McpSubcommand::Serve { session }) => run_mcp_stdio(session).await,
        Some(McpSubcommand::List { category, format }) => {
            let mut tools = McpServer::list_tools();
            if let Some(ref cat) = category {
                let needle = cat.to_ascii_lowercase();
                tools.retain(|t| t.category.to_ascii_lowercase().contains(&needle));
            }
            match format {
                OutputFormat::Json | OutputFormat::Pretty => {
                    println!("{}", serde_json::to_string_pretty(&tools)?);
                }
                OutputFormat::Compact => {
                    println!("{}", serde_json::to_string(&tools)?);
                }
                OutputFormat::Table => {
                    println!("{:<26} {:<13} DESCRIPTION", "TOOL", "CATEGORY");
                    println!("{}", "-".repeat(86));
                    for t in &tools {
                        println!("{:<26} {:<13} {}", t.name, t.category, t.description);
                    }
                    println!("\nTotal: {} MCP tools", tools.len());
                }
            }
            Ok(())
        }
        Some(McpSubcommand::Schema { tool }) => {
            let tools = McpServer::list_tools();
            let Some(found) = tools.into_iter().find(|t| t.name == tool) else {
                return Err(format!("Unknown MCP tool: '{tool}'").into());
            };
            println!("{}", serde_json::to_string_pretty(&found)?);
            Ok(())
        }
        Some(McpSubcommand::Call {
            tool,
            args,
            args_file,
            kv_args,
            session,
            format,
        }) => {
            let parsed_args = parse_tool_arguments(args.as_deref(), args_file.as_deref(), &kv_args)?;
            let mut ctx = session.load_context()?;
            let req = McpRequest {
                id: Value::String("cli-call".to_string()),
                method: "tools/call".to_string(),
                params: json!({
                    "name": tool,
                    "arguments": parsed_args
                }),
            };
            let resp = McpServer::handle_request_with_context(&req, &mut ctx);
            let saved = session.persist_if_needed(&mut ctx)?;
            if !saved.is_empty() {
                eprintln!("[oxide-mcp] Saved: {}", saved.join(", "));
            }
            print_json_value(&serde_json::to_value(&resp)?, format)?;
            if resp.error.is_some() {
                std::process::exit(1);
            }
            Ok(())
        }
        Some(McpSubcommand::Batch {
            input,
            session,
            stop_on_error,
        }) => {
            let raw = read_input_source(&input)?;
            let mut ctx = session.load_context()?;
            let mut responses = Vec::new();
            let mut failed = 0usize;
            for (idx, line) in raw.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let req: McpRequest = serde_json::from_str(trimmed)
                    .map_err(|e| format!("Line {}: invalid JSON-RPC request: {e}", idx + 1))?;
                let resp = McpServer::handle_request_with_context(&req, &mut ctx);
                if resp.error.is_some() {
                    failed += 1;
                }
                responses.push(resp);
                if failed > 0 && stop_on_error {
                    break;
                }
            }
            let saved = session.persist_if_needed(&mut ctx)?;
            if !saved.is_empty() {
                eprintln!("[oxide-mcp] Saved: {}", saved.join(", "));
            }
            println!("{}", serde_json::to_string_pretty(&responses)?);
            if failed > 0 {
                std::process::exit(1);
            }
            Ok(())
        }
    }
}

async fn run_mcp_stdio(session: SessionArgs) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::io::AsyncBufReadExt;
    eprintln!("[oxide-mcp] Oxide EDA MCP stdio server listening...");
    let mut ctx = session.load_context()?;
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        match serde_json::from_str::<McpRequest>(trimmed) {
            Ok(req) => {
                let resp = McpServer::handle_request_with_context(&req, &mut ctx);
                let _ = session.persist_if_needed(&mut ctx)?;
                println!("{}", serde_json::to_string(&resp)?);
            }
            Err(e) => {
                let err_resp = oxide_ai::McpResponse::err(
                    Value::Null,
                    -32700,
                    format!("Parse error: {e}"),
                );
                println!("{}", serde_json::to_string(&err_resp)?);
            }
        }
    }
    Ok(())
}

pub fn run_utcp_command(args: UtcpArgs) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        None => {
            let manifest = utcp_manifest_for_transport("stdio");
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
        Some(UtcpSubcommand::Manifest {
            transport,
            format,
            out,
        }) => {
            let manifest = utcp_manifest_for_transport(&transport);
            if let Some(out_path) = out {
                let text = serde_json::to_string_pretty(&manifest)?;
                std::fs::write(&out_path, text)?;
                eprintln!("[oxide-utcp] Manifest written to {}", out_path.display());
            }
            print_json_value(&manifest, format)?;
            Ok(())
        }
        Some(UtcpSubcommand::List { tag, format }) => {
            let mut tools = McpServer::list_tools();
            if let Some(ref t) = tag {
                let needle = t.to_ascii_lowercase();
                tools.retain(|item| {
                    item.category.to_ascii_lowercase().contains(&needle)
                        || item.name.to_ascii_lowercase().contains(&needle)
                });
            }
            match format {
                OutputFormat::Json | OutputFormat::Pretty => {
                    println!("{}", serde_json::to_string_pretty(&tools)?);
                }
                OutputFormat::Compact => {
                    println!("{}", serde_json::to_string(&tools)?);
                }
                OutputFormat::Table => {
                    println!("{:<26} {:<13} DESCRIPTION", "UTCP TOOL", "TAG");
                    println!("{}", "-".repeat(86));
                    for t in &tools {
                        println!("{:<26} {:<13} {}", t.name, t.category, t.description);
                    }
                    println!("\nTotal: {} UTCP tools", tools.len());
                }
            }
            Ok(())
        }
        Some(UtcpSubcommand::Describe { tool, format }) => {
            let manifest = utcp_manifest_for_transport("cli");
            let found = manifest
                .get("tools")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.iter().find(|t| t.get("name").and_then(|n| n.as_str()) == Some(&tool)))
                .cloned();
            let Some(item) = found else {
                return Err(format!("Unknown UTCP tool: '{tool}'").into());
            };
            print_json_value(&item, format)?;
            Ok(())
        }
        Some(UtcpSubcommand::Call {
            tool,
            args,
            args_file,
            kv_args,
            session,
            format,
        }) => {
            let parsed_args = parse_tool_arguments(args.as_deref(), args_file.as_deref(), &kv_args)?;
            let mut ctx = session.load_context()?;
            let resp = execute_utcp_call(&tool, parsed_args, &mut ctx);
            let saved = session.persist_if_needed(&mut ctx)?;
            if !saved.is_empty() {
                eprintln!("[oxide-utcp] Saved: {}", saved.join(", "));
            }
            print_json_value(&serde_json::to_value(&resp)?, format)?;
            if !resp.ok {
                std::process::exit(1);
            }
            Ok(())
        }
        Some(UtcpSubcommand::Batch {
            input,
            session,
            stop_on_error,
        }) => {
            let raw = read_input_source(&input)?;
            let calls = parse_utcp_batch_input(&raw)?;
            let mut ctx = session.load_context()?;
            let report = execute_utcp_batch(&calls, &mut ctx, stop_on_error);
            let saved = session.persist_if_needed(&mut ctx)?;
            if !saved.is_empty() {
                eprintln!("[oxide-utcp] Saved: {}", saved.join(", "));
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            if report.failed > 0 {
                std::process::exit(1);
            }
            Ok(())
        }
        Some(UtcpSubcommand::Validate { file }) => {
            let manifest: Value = if let Some(path) = file {
                let raw = std::fs::read_to_string(&path)?;
                serde_json::from_str(&raw)?
            } else {
                utcp_manifest_for_transport("stdio")
            };
            match validate_utcp_manifest(&manifest) {
                Ok(summary) => {
                    println!("{}", serde_json::to_string_pretty(&summary)?);
                    Ok(())
                }
                Err(errs) => {
                    for e in errs {
                        eprintln!("UTCP validation error: {e}");
                    }
                    std::process::exit(1);
                }
            }
        }
    }
}

pub fn run_altium_catalog_command(args: AltiumCatalogArgs) -> Result<(), Box<dyn std::error::Error>> {
    let cat = AltiumCatalog::global();
    match args.action {
        AltiumCatalogSubcommand::Stats { format } => {
            let stats = cat.stats();
            match format {
                OutputFormat::Json | OutputFormat::Pretty => {
                    println!("{}", serde_json::to_string_pretty(&stats)?);
                }
                OutputFormat::Compact => {
                    println!("{}", serde_json::to_string(&stats)?);
                }
                OutputFormat::Table => {
                    println!("Altium Designer UI Catalog Statistics");
                    println!("  Total entries   : {}", stats.total_entries);
                    println!("  Unique commands : {}", stats.unique_commands);
                    println!("  .rcs files      : {}", stats.rcs_file_count);
                    println!("  Namespaces      : {}", stats.namespace_count);
                    println!("\nTop .rcs modules:");
                    for (file, count) in stats.top_rcs_files.iter().take(12) {
                        println!("  {:<44} {:>4}", file, count);
                    }
                    println!("\nTop namespaces:");
                    for (ns, count) in stats.top_namespaces.iter().take(15) {
                        println!("  {:<30} {:>4}", ns, count);
                    }
                }
            }
            Ok(())
        }
        AltiumCatalogSubcommand::Search {
            query,
            namespace,
            rcs,
            limit,
            format,
        } => {
            let hits = cat.search(&query, namespace.as_deref(), rcs.as_deref(), limit);
            match format {
                OutputFormat::Json | OutputFormat::Pretty => {
                    println!("{}", serde_json::to_string_pretty(&hits)?);
                }
                OutputFormat::Compact => {
                    println!("{}", serde_json::to_string(&hits)?);
                }
                OutputFormat::Table => {
                    println!(
                        "{:<36} {:<24} {:<14} SOURCE",
                        "COMMAND", "CAPTION", "SHORTCUT"
                    );
                    println!("{}", "-".repeat(96));
                    for e in &hits {
                        println!(
                            "{:<36} {:<24} {:<14} {}",
                            truncate_col(&e.cmd.command, 35),
                            truncate_col(&e.cmd.plain_caption(), 23),
                            truncate_col(&e.cmd.shortcut1, 13),
                            e.source_file
                        );
                    }
                    println!("\nShowing {} matching command(s)", hits.len());
                }
            }
            Ok(())
        }
        AltiumCatalogSubcommand::Inspect { command, format } => {
            let entries = cat.inspect(&command);
            if entries.is_empty() {
                return Err(format!("Altium command '{command}' not found in catalog").into());
            }
            print_json_value(&serde_json::to_value(&entries)?, format)?;
            Ok(())
        }
    }
}

fn parse_tool_arguments(
    args_json: Option<&str>,
    args_file: Option<&Path>,
    kv_args: &[String],
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut map: Map<String, Value> = if let Some(path) = args_file {
        let raw = std::fs::read_to_string(path)?;
        let val: Value = serde_json::from_str(&raw)?;
        val.as_object()
            .cloned()
            .ok_or("--args-file must contain a JSON object")?
    } else if let Some(raw) = args_json {
        let val: Value = serde_json::from_str(raw)?;
        val.as_object()
            .cloned()
            .ok_or("--args must be a JSON object")?
    } else {
        Map::new()
    };

    for pair in kv_args {
        let Some((k, v)) = pair.split_once('=') else {
            return Err(format!("Invalid --arg '{pair}': expected KEY=VALUE").into());
        };
        map.insert(k.trim().to_string(), coerce_cli_value(v.trim()));
    }

    Ok(Value::Object(map))
}

fn coerce_cli_value(raw: &str) -> Value {
    if raw.eq_ignore_ascii_case("true") {
        return Value::Bool(true);
    }
    if raw.eq_ignore_ascii_case("false") {
        return Value::Bool(false);
    }
    if raw.eq_ignore_ascii_case("null") {
        return Value::Null;
    }
    if let Ok(i) = raw.parse::<i64>() {
        return Value::Number(i.into());
    }
    if let Ok(f) = raw.parse::<f64>()
        && let Some(num) = serde_json::Number::from_f64(f)
    {
        return Value::Number(num);
    }
    if ((raw.starts_with('{') && raw.ends_with('}'))
        || (raw.starts_with('[') && raw.ends_with(']')))
        && let Ok(parsed) = serde_json::from_str::<Value>(raw)
    {
        return parsed;
    }
    Value::String(raw.to_string())
}

fn parse_utcp_batch_input(raw: &str) -> Result<Vec<UtcpCallRequest>, Box<dyn std::error::Error>> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    if trimmed.starts_with('[') {
        let calls: Vec<UtcpCallRequest> = serde_json::from_str(trimmed)?;
        return Ok(calls);
    }
    let mut calls = Vec::new();
    for (idx, line) in trimmed.lines().enumerate() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        let call: UtcpCallRequest = serde_json::from_str(l)
            .map_err(|e| format!("Line {}: invalid UTCP call object: {e}", idx + 1))?;
        calls.push(call);
    }
    Ok(calls)
}

fn read_input_source(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    if source == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        Ok(std::fs::read_to_string(source)?)
    }
}

fn print_json_value(val: &Value, format: OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Compact => println!("{}", serde_json::to_string(val)?),
        OutputFormat::Pretty | OutputFormat::Json | OutputFormat::Table => {
            println!("{}", serde_json::to_string_pretty(val)?);
        }
    }
    Ok(())
}

fn truncate_col(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}
