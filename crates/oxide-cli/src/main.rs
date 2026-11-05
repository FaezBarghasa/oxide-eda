//! Headless CLI binary for Oxide EDA CI/CD and automation.

use clap::{Parser, Subcommand};
use oxide_output::{
    ExcellonExporter, GerberExporter, GerberOptions, OutputJobConfig, OutputJobRunner,
    PickAndPlaceExporter, PickAndPlaceOptions,
};
use oxide_rules::ConstraintManager;
use oxide_types::pcb::PcbBoard;
use std::path::{Path, PathBuf};

fn load_board(path: &Path) -> Result<PcbBoard, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    if let Ok(snx) = oxide_types::format::SnxPcb::parse(&content) {
        Ok(snx.board)
    } else {
        let pcb: PcbBoard = serde_json::from_str(&content)?;
        Ok(pcb)
    }
}

fn load_schematic(
    path: &Path,
) -> Result<oxide_types::schematic::SchematicSheet, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    if let Ok(snx) = oxide_types::format::SnxSchematic::parse(&content) {
        Ok(snx.sheet)
    } else {
        let sheet = serde_json::from_str(&content)?;
        Ok(sheet)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct GoldenBomItem {
    pub refdes: String,
    pub value: String,
    pub footprint: String,
}

#[derive(Parser)]
#[command(name = "oxide")]
#[command(about = "Headless CLI tool for Oxide EDA: DRC, ERC, CAM and Release automation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Design Rule Check (DRC) against PCB board and rules
    Drc {
        #[arg(short, long)]
        board: PathBuf,
        #[arg(short, long)]
        rules: Option<PathBuf>,
    },
    /// Generate CAM manufacturing outputs (Gerber, Drill, Pick-and-Place)
    Cam {
        #[arg(short, long)]
        board: PathBuf,
        #[arg(short, long)]
        out_dir: PathBuf,
    },
    /// Run an automated Output Job file (.snxoutjob)
    Outjob {
        #[arg(short, long)]
        job: PathBuf,
        #[arg(short, long)]
        board: PathBuf,
        #[arg(short, long)]
        out_dir: PathBuf,
    },
    /// Generate IPC-2581C intelligent manufacturing package
    Ipc2581 {
        #[arg(short, long)]
        board: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
    /// Generate ODB++ v8.1 manufacturing package
    Odbpp {
        #[arg(short, long)]
        board: PathBuf,
        #[arg(short, long)]
        out_dir: PathBuf,
    },
    /// Import an Altium Designer file (.SchDoc, .PcbDoc, .SchLib, .PcbLib, .IntLib) to native Oxide format
    ImportAltium {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Run holistic CI/CD quality gate checking ERC, DRC, stackup impedance, and BOM drift
    Check {
        #[arg(short, long)]
        board: PathBuf,
        #[arg(short, long)]
        schematic: Option<PathBuf>,
        #[arg(short, long)]
        rules: Option<PathBuf>,
        #[arg(long)]
        golden_bom: Option<PathBuf>,
        #[arg(long)]
        target_z0: Option<f64>,
        #[arg(long, default_value_t = 5.0)]
        max_z0_error_pct: f64,
    },
    /// Automated panelization with rails, tooling holes, fiducials, and V-scores
    Panel {
        #[arg(short, long)]
        board: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
        #[arg(long, default_value_t = 2)]
        cols: u32,
        #[arg(long, default_value_t = 2)]
        rows: u32,
        #[arg(long, default_value_t = 2.0)]
        spacing_x: f64,
        #[arg(long, default_value_t = 2.0)]
        spacing_y: f64,
        #[arg(long, default_value_t = 5.0)]
        rail_width: f64,
    },
    /// Generate Pick-and-Place (CPL / Centroid) XY placement file
    PickAndPlace {
        #[arg(short, long)]
        board: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
    /// Generate Bill of Materials (BOM) and assert zero drift against golden specification
    Bom {
        #[arg(short, long)]
        board: PathBuf,
        #[arg(short, long)]
        out: Option<PathBuf>,
        #[arg(long)]
        golden: Option<PathBuf>,
    },
    /// Start Oxide EDA Model Context Protocol (MCP) server over stdio for agent interaction
    Mcp,
    /// Print Universal Tool Call Protocol (UTCP) discovery manifest
    Utcp,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Drc { board, rules } => {
            println!("Oxide EDA DRC Validator");
            println!("Loading PCB: {}", board.display());
            let pcb = load_board(&board)?;

            let cm = match rules {
                Some(r_path) => {
                    println!("Loading rules: {}", r_path.display());
                    ConstraintManager::from_file(r_path)
                        .map_err(|e| format!("Failed to read rules: {e}"))?
                }
                None => {
                    if pcb.layers.len() > 6 || pcb.vias.iter().any(|v| v.drill < 0.25) {
                        println!(
                            "Using HDI advanced default rules (board has {} layers, microvias)",
                            pcb.layers.len()
                        );
                        ConstraintManager::hdi_advanced_default()
                    } else {
                        println!("Using standard default rules");
                        ConstraintManager::standard_default()
                    }
                }
            };

            println!(
                "Loaded PCB with {} footprints, {} segments, {} vias",
                pcb.footprints.len(),
                pcb.segments.len(),
                pcb.vias.len()
            );

            println!("Running DRC validation across {} rules...", cm.rules.len());
            let violations = cm.run_drc(&pcb);

            if violations.is_empty() {
                println!("DRC Check Passed: 0 violations found.");
            } else {
                eprintln!("\n=======================================================");
                eprintln!("DRC FAILED: {} violation(s) detected", violations.len());
                eprintln!("=======================================================");
                for (idx, v) in violations.iter().enumerate() {
                    eprintln!(
                        "[{}] {:?}: {} (Required: {}, Actual: {})",
                        idx + 1,
                        v.violation_type,
                        v.message,
                        v.required_value,
                        v.actual_value
                    );
                    if let Some((x, y)) = v.location {
                        eprintln!("    Location: ({:.3} mm, {:.3} mm)", x, y);
                    }
                    if !v.object_ids.is_empty() {
                        eprintln!("    Objects: {}", v.object_ids.join(", "));
                    }
                }
                std::process::exit(1);
            }
        }
        Commands::Cam { board, out_dir } => {
            println!("Oxide EDA CAM Release Exporter");
            std::fs::create_dir_all(&out_dir)?;

            let pcb = load_board(&board)?;

            // 1. Gerbers
            let gerber_exp = GerberExporter::new(GerberOptions::default());
            let g_outputs = gerber_exp.export_board(&pcb)?;
            for g in g_outputs {
                let p = out_dir.join(&g.filename);
                std::fs::write(&p, &g.content)?;
                println!("Exported Gerber: {}", p.display());
            }

            // 2. Excellon Drill
            let drill_exp = ExcellonExporter::new();
            let d_outputs = drill_exp.export_board(&pcb)?;
            for d in d_outputs {
                let p = out_dir.join(&d.filename);
                std::fs::write(&p, &d.content)?;
                println!("Exported Drill: {}", p.display());
            }

            // 3. Pick and Place
            let pnp_exp = PickAndPlaceExporter::new(PickAndPlaceOptions::default());
            let pnp_csv = pnp_exp.export(&pcb)?;
            let pnp_path = out_dir.join("pick_and_place.csv");
            std::fs::write(&pnp_path, &pnp_csv)?;
            println!("Exported Pick and Place: {}", pnp_path.display());

            println!("CAM Export Completed Successfully to {}", out_dir.display());
        }
        Commands::Outjob {
            job,
            board,
            out_dir,
        } => {
            println!("Oxide EDA Output Job Runner: {}", job.display());
            std::fs::create_dir_all(&out_dir)?;

            let pcb = load_board(&board)?;

            let job_content = std::fs::read_to_string(&job)?;
            let config: OutputJobConfig = serde_json::from_str(&job_content)?;

            let empty_ctx = oxide_output::ExportContext {
                sheets: Vec::new(),
                metadata: oxide_output::ProjectMetadata::default(),
                netlist: None,
            };
            let pkg = OutputJobRunner::run(&config, &empty_ctx, &pcb)?;
            pkg.write_to_disk(&out_dir)?;

            println!(
                "Output Job Completed: Package written to {}",
                out_dir.display()
            );
        }
        Commands::Ipc2581 { board, out } => {
            println!("Oxide EDA IPC-2581C Exporter: {}", board.display());
            let pcb = load_board(&board)?;

            let opts = oxide_output::Ipc2581Options::default();
            let res = oxide_output::export_ipc2581(&pcb, &opts)?;
            std::fs::write(&out, &res.xml_content)?;
            println!("IPC-2581C Export written to {}", out.display());
        }
        Commands::Odbpp { board, out_dir } => {
            println!("Oxide EDA ODB++ v8.1 Exporter: {}", board.display());
            std::fs::create_dir_all(&out_dir)?;

            let pcb = load_board(&board)?;

            let pkg = oxide_output::export_odbpp_package(&pcb)?;
            for file in &pkg.files {
                let full_path = out_dir.join(&file.relative_path);
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&full_path, &file.content)?;
            }
            println!(
                "ODB++ Export Completed: {} files written to {}",
                pkg.files.len(),
                out_dir.display()
            );
        }
        Commands::ImportAltium { input, out } => {
            println!("Oxide EDA Altium Importer: {}", input.display());
            let result = oxide_altium_importer::import_altium_file(&input)
                .map_err(|e| format!("Altium import failed: {e}"))?;

            match &result {
                oxide_altium_importer::AltiumImportResult::Schematic(sheet) => {
                    println!(
                        "Successfully parsed Schematic Sheet: {} symbols, {} wires, {} labels",
                        sheet.symbols.len(),
                        sheet.wires.len(),
                        sheet.labels.len()
                    );
                    if let Some(out_path) = out {
                        let json = serde_json::to_string_pretty(sheet)?;
                        std::fs::write(&out_path, json)?;
                        println!("Wrote native schematic to {}", out_path.display());
                    }
                }
                oxide_altium_importer::AltiumImportResult::Pcb(board) => {
                    println!(
                        "Successfully parsed PCB Board: {} footprints, {} tracks, {} vias, {} zones",
                        board.footprints.len(),
                        board.segments.len(),
                        board.vias.len(),
                        board.zones.len()
                    );
                    if let Some(out_path) = out {
                        let json = serde_json::to_string_pretty(board)?;
                        std::fs::write(&out_path, json)?;
                        println!("Wrote native PCB board to {}", out_path.display());
                    }
                }
                oxide_altium_importer::AltiumImportResult::SymbolLibrary(symbols) => {
                    println!(
                        "Successfully parsed Schematic Symbol Library: {} symbols",
                        symbols.len()
                    );
                    if let Some(out_path) = out {
                        let json = serde_json::to_string_pretty(symbols)?;
                        std::fs::write(&out_path, json)?;
                        println!("Wrote native symbol library to {}", out_path.display());
                    }
                }
                oxide_altium_importer::AltiumImportResult::FootprintLibrary(fps) => {
                    println!(
                        "Successfully parsed PCB Footprint Library: {} footprints",
                        fps.len()
                    );
                    if let Some(out_path) = out {
                        let json = serde_json::to_string_pretty(fps)?;
                        std::fs::write(&out_path, json)?;
                        println!("Wrote native footprint library to {}", out_path.display());
                    }
                }
                oxide_altium_importer::AltiumImportResult::IntegratedLibrary(intlib) => {
                    println!(
                        "Successfully parsed Integrated Library: {} symbols, {} footprints",
                        intlib.symbols.len(),
                        intlib.footprints.len()
                    );
                    if let Some(out_path) = out {
                        let json = serde_json::to_string_pretty(intlib)?;
                        std::fs::write(&out_path, json)?;
                        println!("Wrote native integrated library to {}", out_path.display());
                    }
                }
            }
        }
        Commands::Check {
            board,
            schematic,
            rules,
            golden_bom,
            target_z0,
            max_z0_error_pct,
        } => {
            println!("=======================================================");
            println!("Oxide EDA Headless CI/CD Holistic Quality Gate");
            println!("=======================================================");

            // 1. ERC (Electrical Rules Check)
            if let Some(sch_path) = schematic {
                println!(
                    "--> [Gate 1/4] Running Electrical Rules Check (ERC): {}",
                    sch_path.display()
                );
                let sheet = load_schematic(&sch_path)?;
                let erc_violations = oxide_erc::run(&sheet);
                let err_count = erc_violations
                    .iter()
                    .filter(|v| v.severity == oxide_erc::Severity::Error)
                    .count();

                if err_count > 0 {
                    eprintln!("ERC GATE FAILED: {} error(s) detected", err_count);
                    for v in erc_violations
                        .iter()
                        .filter(|v| v.severity == oxide_erc::Severity::Error)
                    {
                        eprintln!("  [ERC ERROR] {:?}: {}", v.rule, v.message);
                    }
                    std::process::exit(2);
                } else {
                    println!(
                        "    ERC Gate: PASSED (0 errors, {} warnings)",
                        erc_violations.len()
                    );
                }
            } else {
                println!("--> [Gate 1/4] ERC: Skipped (no schematic provided)");
            }

            // 2. DRC (Design Rule Check)
            println!(
                "--> [Gate 2/4] Running Design Rule Check (DRC): {}",
                board.display()
            );
            let pcb = load_board(&board)?;
            let cm = match rules {
                Some(r_path) => ConstraintManager::from_file(r_path)
                    .map_err(|e| format!("Failed to read rules: {e}"))?,
                None => {
                    if pcb.layers.len() > 6 || pcb.vias.iter().any(|v| v.drill < 0.25) {
                        println!(
                            "    Using HDI advanced default rules (board has {} layers, microvias)",
                            pcb.layers.len()
                        );
                        ConstraintManager::hdi_advanced_default()
                    } else {
                        println!("    Using standard default rules");
                        ConstraintManager::standard_default()
                    }
                }
            };
            let drc_violations = cm.run_drc_parallel(&pcb);
            if !drc_violations.is_empty() {
                eprintln!(
                    "DRC GATE FAILED: {} violation(s) detected",
                    drc_violations.len()
                );
                for (idx, v) in drc_violations.iter().enumerate() {
                    eprintln!(
                        "  [{}] {:?}: {} (Required: {}, Actual: {})",
                        idx + 1,
                        v.violation_type,
                        v.message,
                        v.required_value,
                        v.actual_value
                    );
                }
                std::process::exit(1);
            } else {
                println!("    DRC Gate: PASSED (0 violations)");
            }

            // 3. Impedance Check
            if let Some(target) = target_z0 {
                println!(
                    "--> [Gate 3/4] Running Stackup Impedance Check (Target: {:.1} Ω)",
                    target
                );
                let default_trace_w = pcb.segments.first().map(|s| s.width).unwrap_or(0.15);
                let width_um = (default_trace_w * 1000.0).round() as i64;
                let calc_z0 = oxide_physics::ImpedanceCalculator::calculate_microstrip(
                    width_um, 100, // 100µm (0.10mm dielectric height above plane)
                    35,  // 35µm (1oz copper)
                    4.3, // Standard FR-4 permittivity
                );
                let error_pct = ((calc_z0 - target).abs() / target) * 100.0;
                println!(
                    "    Calculated Single-Ended Z0: {:.2} Ω (Deviation: {:.2}%)",
                    calc_z0, error_pct
                );
                if error_pct > max_z0_error_pct {
                    eprintln!(
                        "IMPEDANCE GATE FAILED: Z0 = {:.2} Ω deviates by {:.2}% (exceeds max {:.2}%)",
                        calc_z0, error_pct, max_z0_error_pct
                    );
                    std::process::exit(3);
                } else {
                    println!(
                        "    Impedance Gate: PASSED (Within {:.1}% tolerance)",
                        max_z0_error_pct
                    );
                }
            } else {
                println!("--> [Gate 3/4] Impedance Check: Skipped (no target specified)");
            }

            // 4. BOM Drift Gate
            if let Some(bom_path) = golden_bom {
                println!(
                    "--> [Gate 4/4] Running BOM Drift Gate: {}",
                    bom_path.display()
                );
                let golden_content = std::fs::read_to_string(&bom_path)?;
                let golden_items: Vec<GoldenBomItem> = serde_json::from_str(&golden_content)?;

                let mut golden_map: std::collections::HashMap<String, &GoldenBomItem> =
                    golden_items
                        .iter()
                        .map(|item| (item.refdes.clone(), item))
                        .collect();

                let mut drift_errors = Vec::new();
                for fp in &pcb.footprints {
                    if let Some(golden) = golden_map.remove(&fp.reference) {
                        if !golden.value.is_empty()
                            && !fp.value.is_empty()
                            && golden.value != fp.value
                        {
                            drift_errors.push(format!(
                                "Value mismatch on '{}': board has '{}', golden requires '{}'",
                                fp.reference, fp.value, golden.value
                            ));
                        }
                        if !golden.footprint.is_empty()
                            && !fp.footprint_id.is_empty()
                            && golden.footprint != fp.footprint_id
                        {
                            drift_errors.push(format!(
                                "Footprint mismatch on '{}': board has '{}', golden requires '{}'",
                                fp.reference, fp.footprint_id, golden.footprint
                            ));
                        }
                    } else {
                        drift_errors.push(format!(
                            "Untracked component on board: '{}' (Value: '{}', Footprint: '{}')",
                            fp.reference, fp.value, fp.footprint_id
                        ));
                    }
                }

                for (missing_ref, _) in golden_map {
                    drift_errors.push(format!("Missing component from board: '{missing_ref}'"));
                }

                if !drift_errors.is_empty() {
                    eprintln!(
                        "BOM DRIFT GATE FAILED: {} discrepancy(ies) detected",
                        drift_errors.len()
                    );
                    for err in &drift_errors {
                        eprintln!("  [BOM DRIFT] {err}");
                    }
                    std::process::exit(4);
                } else {
                    println!("    BOM Drift Gate: PASSED (0 drift against golden spec)");
                }
            } else {
                println!("--> [Gate 4/4] BOM Drift Gate: Skipped (no golden BOM provided)");
            }

            println!("\n=======================================================");
            println!("ALL CI/CD QUALITY GATES PASSED (Exit Code: 0)");
            println!("=======================================================");
        }
        Commands::Panel {
            board,
            out,
            cols,
            rows,
            spacing_x,
            spacing_y,
            rail_width,
        } => {
            println!("Oxide EDA Panelization Engine");
            let pcb = load_board(&board)?;
            let b_width = pcb
                .setup
                .as_ref()
                .map(|s| s.grid_size)
                .unwrap_or(50.0)
                .max(30.0);
            let b_height = pcb
                .setup
                .as_ref()
                .map(|s| s.grid_size)
                .unwrap_or(50.0)
                .max(30.0);

            let panel = oxide_output::PanelLayout::create_step_and_repeat(
                b_width, b_height, cols, rows, spacing_x, spacing_y, rail_width,
            );
            let violations = panel.validate();
            if !violations.is_empty() {
                eprintln!("Warning: Panel DFM violations detected: {:?}", violations);
            }

            let json = serde_json::to_string_pretty(&panel)?;
            std::fs::write(&out, json)?;
            println!(
                "Successfully generated panel ({} boards) to {}",
                panel.boards.len(),
                out.display()
            );
        }
        Commands::PickAndPlace { board, out } => {
            println!("Oxide EDA Pick-and-Place (CPL) Exporter");
            let pcb = load_board(&board)?;
            let exp = PickAndPlaceExporter::new(PickAndPlaceOptions::default());
            let csv = exp.export(&pcb)?;
            std::fs::write(&out, csv)?;
            println!("Exported Centroid Pick-and-Place file to {}", out.display());
        }
        Commands::Bom { board, out, golden } => {
            println!("Oxide EDA BOM Exporter & Verification");
            let pcb = load_board(&board)?;
            let items: Vec<GoldenBomItem> = pcb
                .footprints
                .iter()
                .map(|fp| GoldenBomItem {
                    refdes: fp.reference.clone(),
                    value: fp.value.clone(),
                    footprint: fp.footprint_id.clone(),
                })
                .collect();

            if let Some(golden_path) = golden {
                let golden_content = std::fs::read_to_string(&golden_path)?;
                let golden_items: Vec<GoldenBomItem> = serde_json::from_str(&golden_content)?;
                let mut golden_map: std::collections::HashMap<String, &GoldenBomItem> =
                    golden_items
                        .iter()
                        .map(|item| (item.refdes.clone(), item))
                        .collect();

                let mut diffs = Vec::new();
                for item in &items {
                    if let Some(g) = golden_map.remove(&item.refdes) {
                        if g.value != item.value || g.footprint != item.footprint {
                            diffs.push(format!(
                                "Mismatch on '{}': Board({}/{}) vs Golden({}/{})",
                                item.refdes, item.value, item.footprint, g.value, g.footprint
                            ));
                        }
                    } else {
                        diffs.push(format!("Untracked component on board: '{}'", item.refdes));
                    }
                }
                for (rem, _) in golden_map {
                    diffs.push(format!("Missing component from board: '{rem}'"));
                }

                if !diffs.is_empty() {
                    eprintln!("BOM DRIFT DETECTED: {} discrepancy(ies)", diffs.len());
                    for d in &diffs {
                        eprintln!("  {d}");
                    }
                    std::process::exit(4);
                } else {
                    println!(
                        "BOM verification PASSED: Zero drift against {}",
                        golden_path.display()
                    );
                }
            }

            if let Some(out_path) = out {
                let json = serde_json::to_string_pretty(&items)?;
                std::fs::write(&out_path, json)?;
                println!("Wrote BOM to {}", out_path.display());
            }
        }
        Commands::Mcp => {
            use tokio::io::AsyncBufReadExt;
            eprintln!("[oxide-mcp] Oxide EDA MCP stdio server listening...");
            let stdin = tokio::io::BufReader::new(tokio::io::stdin());
            let mut lines = stdin.lines();
            while let Some(line) = lines.next_line().await? {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                match serde_json::from_str::<oxide_ai::McpRequest>(trimmed) {
                    Ok(req) => {
                        let resp = oxide_ai::McpServer::handle_request(&req, None, None);
                        println!("{}", serde_json::to_string(&resp)?);
                    }
                    Err(e) => {
                        let err_resp = oxide_ai::McpResponse {
                            id: "null".to_string(),
                            result: None,
                            error: Some(oxide_ai::McpError {
                                code: -32700,
                                message: format!("Parse error: {e}"),
                            }),
                        };
                        println!("{}", serde_json::to_string(&err_resp)?);
                    }
                }
            }
        }
        Commands::Utcp => {
            let manifest = oxide_ai::utcp_manifest();
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
    }

    Ok(())
}
