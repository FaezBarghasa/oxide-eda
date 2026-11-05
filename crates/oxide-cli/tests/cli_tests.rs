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
