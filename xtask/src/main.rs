use std::process::{Command, exit};
use std::time::Instant;

struct Check {
    name: &'static str,
    desc: &'static str,
    cmd: &'static str,
    args: &'static [&'static str],
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let subcommand = args.get(1).map(|s| s.as_str()).unwrap_or("verify");

    match subcommand {
        "verify" => run_verify(),
        "help" | "--help" | "-h" => {
            println!("Oxide EDA xtask harness");
            println!();
            println!("Usage: cargo xtask <SUBCOMMAND>");
            println!();
            println!("Subcommands:");
            println!("  verify    Run all Phase 1 verifiable checks and benchmarks");
        }
        unknown => {
            eprintln!("Unknown subcommand: {unknown}");
            eprintln!("Run `cargo xtask help` for usage");
            exit(1);
        }
    }
}

fn run_verify() {
    println!("============================================================");
    println!("     OXIDE EDA — VERIFIABLE TEST & PARITY HARNESS           ");
    println!("============================================================");

    let checks = [
        Check {
            name: "D2/T1+T2+T3: Verifiable Fixtures & DRC",
            desc: "2-layer synth + 4-layer FPGA + 6-layer BGA fixtures, SnxPcb roundtrip, DRC zero violations",
            cmd: "cargo",
            args: &[
                "test",
                "-p",
                "oxide-engine",
                "--test",
                "verifiable_fixtures_test",
                "--",
                "--nocapture",
            ],
        },
        Check {
            name: "D3: 12 Altium Shortcut Scenarios",
            desc: "Ctrl+Shift+P, F1, F8, F9, P W, Esc, Ctrl+C/V/D, Q, Alt+A, Shift+Space",
            cmd: "cargo",
            args: &[
                "test",
                "-p",
                "oxide-app",
                "--lib",
                "--",
                "test_shortcut_scenario",
                "--nocapture",
            ],
        },
        Check {
            name: "D4: Brand Consistency Gate",
            desc: "Application title and metadata strictly Oxide, zero Signex regressions",
            cmd: "cargo",
            args: &[
                "test",
                "-p",
                "oxide-app",
                "--test",
                "brand_consistency_test",
                "--",
                "--nocapture",
            ],
        },
        Check {
            name: "D7: DRC Real Net Names",
            desc: "DRC reports actual net names from board netlist, no generic fallbacks",
            cmd: "cargo",
            args: &[
                "test",
                "-p",
                "oxide-rules",
                "--test",
                "rules_tests",
                "--",
                "test_drc_reports_real_net_names",
                "--nocapture",
            ],
        },
        Check {
            name: "D9: Net-Tie DRC Exemption",
            desc: "Star-ground Net-Tie components exempt tied nets from clearance violations",
            cmd: "cargo",
            args: &[
                "test",
                "-p",
                "oxide-rules",
                "--test",
                "rules_tests",
                "--",
                "test_d9_net_tie",
                "--nocapture",
            ],
        },
        Check {
            name: "D2: Blank-Guard Scenario Runner",
            desc: "Headless scenario runner verifies T1, T2, and T3 load with non-empty canvas bounds",
            cmd: "cargo",
            args: &[
                "test",
                "-p",
                "oxide-app",
                "--test",
                "scenario_runner_test",
                "--",
                "--nocapture",
            ],
        },
        Check {
            name: "Phase 2: Altium Real Corpus Ingest",
            desc: "Import real Altium binary SchDoc/PcbDoc corpus files with 100% object extraction",
            cmd: "cargo",
            args: &[
                "test",
                "-p",
                "oxide-altium-importer",
                "--test",
                "real_corpus_test",
                "--",
                "--nocapture",
            ],
        },
        Check {
            name: "D6: Multi-layer CAM Gerber Output",
            desc: "Inner copper layers export with unique filenames (output_in{idx}_cu.gbr)",
            cmd: "cargo",
            args: &[
                "test",
                "-p",
                "oxide-output",
                "--lib",
                "--",
                "gerber",
                "--nocapture",
            ],
        },
    ];

    let mut all_passed = true;
    let mut results = Vec::new();
    let overall_start = Instant::now();

    for check in &checks {
        println!();
        println!(">>> Running check: {} ({})", check.name, check.desc);
        let start = Instant::now();
        let status = Command::new(check.cmd).args(check.args).status();

        let duration = start.elapsed();
        match status {
            Ok(s) if s.success() => {
                results.push((check.name, true, duration));
                println!(">>> Check passed: {} in {:.2?}", check.name, duration);
            }
            Ok(s) => {
                all_passed = false;
                results.push((check.name, false, duration));
                eprintln!(
                    ">>> Check FAILED (exit code {:?}): {}",
                    s.code(),
                    check.name
                );
            }
            Err(e) => {
                all_passed = false;
                results.push((check.name, false, duration));
                eprintln!(">>> Failed to execute command for {}: {}", check.name, e);
            }
        }
    }

    let overall_duration = overall_start.elapsed();

    println!();
    println!("============================================================");
    println!("                  VERIFICATION SUMMARY TABLE                ");
    println!("============================================================");
    println!("{:<45} | {:<8} | {:<10}", "Check", "Status", "Duration");
    println!("----------------------------------------------+----------+----------");
    for (name, passed, duration) in &results {
        let status_str = if *passed { "PASS" } else { "FAIL" };
        println!("{:<45} | {:<8} | {:.2?}", name, status_str, duration);
    }
    println!("----------------------------------------------+----------+----------");
    println!("Total time: {:.2?}", overall_duration);
    println!("============================================================");

    if all_passed {
        println!(">>> ALL PHASE 1 CHECKS PASSED SUCCESSFULLY. VERIFIED GREEN.");
    } else {
        eprintln!(">>> ONE OR MORE VERIFICATION CHECKS FAILED.");
        exit(1);
    }
}
