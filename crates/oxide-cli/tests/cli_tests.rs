//! Integration tests for oxide-cli CI/CD commands: check, panel, pick-and-place, and bom.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
fn test_cli_help_and_subcommands_display() {
    let output = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .arg("--help")
        .output()
        .expect("Failed to execute oxide-cli");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("check"));
    assert!(stdout.contains("panel"));
    assert!(stdout.contains("pick-and-place"));
    assert!(stdout.contains("bom"));
    assert!(stdout.contains("ipc2581"));
    assert!(stdout.contains("odbpp"));
}

#[test]
fn test_cli_check_success_on_flagship_fixture() {
    let ws = get_workspace_root();
    let board_path = ws.join("fixtures/high_stress_suite/titan_sbc_12layer_flagship.snxpcb");
    let sch_path = ws.join("fixtures/high_stress_suite/titan_sbc_12layer_flagship.snxsch");

    let output = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args([
            "check",
            "--board",
            board_path.to_str().unwrap(),
            "--schematic",
            sch_path.to_str().unwrap(),
            "--target-z0",
            "60.0",
            "--max-z0-error-pct",
            "5.0",
        ])
        .output()
        .expect("Failed to execute oxide-cli check");

    assert!(
        output.status.success(),
        "oxide-cli check must exit with 0 on clean flagship fixture. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ALL CI/CD QUALITY GATES PASSED (Exit Code: 0)"));
}

#[test]
fn test_cli_panel_generation_and_validation() {
    let ws = get_workspace_root();
    let board_path = ws.join("fixtures/high_stress_suite/titan_sbc_12layer_flagship.snxpcb");

    let temp_dir = std::env::temp_dir().join("oxide_cli_panel_test");
    fs::create_dir_all(&temp_dir).expect("create temp dir");
    let panel_out = temp_dir.join("titan_panel.json");

    let output = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args([
            "panel",
            "--board",
            board_path.to_str().unwrap(),
            "--out",
            panel_out.to_str().unwrap(),
            "--cols",
            "2",
            "--rows",
            "2",
            "--spacing-x",
            "2.0",
            "--spacing-y",
            "2.0",
            "--rail-width",
            "5.0",
        ])
        .output()
        .expect("Failed to execute oxide-cli panel");

    assert!(
        output.status.success(),
        "oxide-cli panel failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(panel_out.exists());
    let content = fs::read_to_string(&panel_out).unwrap();
    assert!(content.contains("\"boards\""));
    assert!(content.contains("\"fiducials\""));
    assert!(content.contains("\"tooling_holes\""));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_cli_pick_and_place_export() {
    let ws = get_workspace_root();
    let board_path = ws.join("fixtures/high_stress_suite/titan_sbc_12layer_flagship.snxpcb");

    let temp_dir = std::env::temp_dir().join("oxide_cli_pnp_test");
    fs::create_dir_all(&temp_dir).expect("create temp dir");
    let pnp_out = temp_dir.join("cpl.csv");

    let output = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args([
            "pick-and-place",
            "--board",
            board_path.to_str().unwrap(),
            "--out",
            pnp_out.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute oxide-cli pick-and-place");

    assert!(
        output.status.success(),
        "oxide-cli pick-and-place failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(pnp_out.exists());
    let csv = fs::read_to_string(&pnp_out).unwrap();
    assert!(csv.contains("Designator") || csv.contains("Reference"));
    assert!(csv.contains("U1") || csv.contains("Footprint"));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_cli_bom_drift_detection_gate() {
    let ws = get_workspace_root();
    let board_path = ws.join("fixtures/high_stress_suite/titan_sbc_12layer_flagship.snxpcb");

    let temp_dir = std::env::temp_dir().join("oxide_cli_bom_test");
    fs::create_dir_all(&temp_dir).expect("create temp dir");

    // 1. Export actual BOM to golden
    let golden_bom = temp_dir.join("golden_bom.json");
    let exp_out = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args([
            "bom",
            "--board",
            board_path.to_str().unwrap(),
            "--out",
            golden_bom.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to export BOM");
    assert!(
        exp_out.status.success(),
        "BOM export failed: {}",
        String::from_utf8_lossy(&exp_out.stderr)
    );
    assert!(golden_bom.exists());

    // 2. Check against identical golden -> must exit with 0
    let check_ok = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args([
            "check",
            "--board",
            board_path.to_str().unwrap(),
            "--golden-bom",
            golden_bom.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute check");
    assert!(
        check_ok.status.success(),
        "Check with matching golden BOM must succeed. Output: {}",
        String::from_utf8_lossy(&check_ok.stderr)
    );

    // 3. Mutate golden BOM (change refdes) -> must trigger drift error and exit with code 4
    let mut golden_content: Vec<serde_json::Value> =
        serde_json::from_str(&fs::read_to_string(&golden_bom).unwrap()).unwrap();
    if let Some(first) = golden_content.first_mut() {
        first["value"] = serde_json::Value::String("TAMPERED_INCORRECT_VALUE".to_string());
    }
    let tampered_bom = temp_dir.join("tampered_bom.json");
    fs::write(
        &tampered_bom,
        serde_json::to_string(&golden_content).unwrap(),
    )
    .unwrap();

    let check_drift = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args([
            "check",
            "--board",
            board_path.to_str().unwrap(),
            "--golden-bom",
            tampered_bom.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute check");

    assert_eq!(
        check_drift.status.code(),
        Some(4),
        "Tampered golden BOM must cause exit code 4 (BOM drift)"
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_cli_mcp_and_utcp_rich_surfaces() {
    let ws = get_workspace_root();
    let board_path = ws.join("fixtures/high_stress_suite/titan_sbc_12layer_flagship.snxpcb");
    let sch_path = ws.join("fixtures/high_stress_suite/titan_sbc_12layer_flagship.snxsch");

    // 1. UTCP manifest default + validation
    let utcp_manifest_out = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args(["utcp"])
        .output()
        .expect("run utcp");
    assert!(utcp_manifest_out.status.success());
    let manifest_json: serde_json::Value =
        serde_json::from_slice(&utcp_manifest_out.stdout).expect("valid UTCP JSON");
    assert_eq!(manifest_json["protocol"], "utcp/1.0");
    assert!(manifest_json["tools"].as_array().unwrap().len() >= 18);

    let utcp_validate = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args(["utcp", "validate"])
        .output()
        .expect("run utcp validate");
    assert!(utcp_validate.status.success());
    let val_json: serde_json::Value = serde_json::from_slice(&utcp_validate.stdout).unwrap();
    assert_eq!(val_json["valid"], true);

    // 2. UTCP call with --arg key=value coercion on impedance calculator
    let utcp_call = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args([
            "utcp",
            "call",
            "calculate_impedance",
            "--arg",
            "mode=microstrip",
            "--arg",
            "trace_width_um=150",
            "--arg",
            "dielectric_height_um=100",
            "--arg",
            "er=4.3",
        ])
        .output()
        .expect("run utcp call");
    assert!(utcp_call.status.success());
    let call_json: serde_json::Value = serde_json::from_slice(&utcp_call.stdout).unwrap();
    assert_eq!(call_json["ok"], true);
    assert!(call_json["result"]["z0_ohms"].as_f64().unwrap() > 40.0);

    // 3. MCP list, schema, and call against flagship board + schematic
    let mcp_list = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args(["mcp", "list", "--format", "json"])
        .output()
        .expect("run mcp list");
    assert!(mcp_list.status.success());
    let tools: Vec<serde_json::Value> = serde_json::from_slice(&mcp_list.stdout).unwrap();
    assert!(tools.len() >= 18);

    let mcp_call = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args([
            "mcp",
            "call",
            "read_pcb",
            "--board",
            board_path.to_str().unwrap(),
            "--arg",
            "include_details=true",
        ])
        .output()
        .expect("run mcp call read_pcb");
    assert!(mcp_call.status.success());
    let pcb_resp: serde_json::Value = serde_json::from_slice(&mcp_call.stdout).unwrap();
    assert!(pcb_resp["error"].is_null());
    assert!(pcb_resp["result"]["footprints_count"].as_u64().unwrap() > 0);

    let mcp_erc = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args([
            "mcp",
            "call",
            "read_schematic",
            "--schematic",
            sch_path.to_str().unwrap(),
        ])
        .output()
        .expect("run mcp call read_schematic");
    assert!(mcp_erc.status.success());
    let sch_resp: serde_json::Value = serde_json::from_slice(&mcp_erc.stdout).unwrap();
    assert_eq!(sch_resp["result"]["loaded"], true);

    // 4. UTCP batch pipeline via stdin
    use std::io::Write;
    let mut batch_child = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args(["utcp", "batch", "--board", board_path.to_str().unwrap()])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn utcp batch");
    {
        let stdin = batch_child.stdin.as_mut().unwrap();
        stdin
            .write_all(
                br#"[
                    {"tool": "run_drc", "arguments": {"limit": 10}},
                    {"tool": "altium_catalog_search", "arguments": {"query": "PCB:Undo", "limit": 5}}
                ]"#,
            )
            .unwrap();
    }
    let batch_out = batch_child.wait_with_output().unwrap();
    assert!(batch_out.status.success());
    let batch_json: serde_json::Value = serde_json::from_slice(&batch_out.stdout).unwrap();
    assert_eq!(batch_json["succeeded"], 2);
    assert_eq!(batch_json["failed"], 0);

    // 5. Altium catalog CLI stats + search + inspect
    let cat_stats = Command::new(env!("CARGO_BIN_EXE_oxide-cli"))
        .args(["altium-catalog", "stats", "--format", "json"])
        .output()
        .expect("run altium-catalog stats");
    assert!(cat_stats.status.success());
    let stats_json: serde_json::Value = serde_json::from_slice(&cat_stats.stdout).unwrap();
    assert_eq!(stats_json["total_entries"], 3004);
    assert_eq!(stats_json["unique_commands"], 1190);
}
