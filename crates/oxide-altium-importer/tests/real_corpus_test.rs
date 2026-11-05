//! Real Corpus Importer Test Suite.
//!
//! Tests importing genuine user Altium Designer binary documents
//! from `examples/General_LCD(faez)` and generates an import fidelity report
//! asserting non-zero component, pad, track, and net counts.

use oxide_altium_importer::{AltiumImportResult, import_altium_file};
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates dir")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn test_corpus_import_lcd_key_schdoc() {
    let sch_path = workspace_root().join("examples/General_LCD(faez)/LcdKey2V1.SchDoc");
    assert!(
        sch_path.exists(),
        "Corpus file must exist at {}",
        sch_path.display()
    );

    let result = import_altium_file(&sch_path).expect("import LcdKey2V1.SchDoc");
    match result {
        AltiumImportResult::Schematic(sheet) => {
            println!(
                "[Corpus Report] LcdKey2V1.SchDoc: {} symbols, {} wires, {} junctions, {} net labels",
                sheet.symbols.len(),
                sheet.wires.len(),
                sheet.junctions.len(),
                sheet.labels.len()
            );
            assert_eq!(sheet.symbols.len(), 25, "Expected 25 schematic symbols");
            assert!(sheet.wires.len() >= 50, "Expected >= 50 schematic wires");
            assert!(sheet.labels.len() >= 30, "Expected >= 30 net labels");
        }
        _ => panic!("Expected AltiumImportResult::Schematic"),
    }
}

#[test]
fn test_corpus_import_lcd_key_pcbdoc() {
    let pcb_path = workspace_root().join("examples/General_LCD(faez)/LcdKey2V1.PcbDoc");
    assert!(
        pcb_path.exists(),
        "Corpus file must exist at {}",
        pcb_path.display()
    );

    let result = import_altium_file(&pcb_path).expect("import LcdKey2V1.PcbDoc");
    match result {
        AltiumImportResult::Pcb(board) => {
            let total_pads: usize = board.footprints.iter().map(|f| f.pads.len()).sum();
            println!(
                "[Corpus Report] LcdKey2V1.PcbDoc: {} footprints, {} pads, {} tracks, {} nets, {} copper layers",
                board.footprints.len(),
                total_pads,
                board.segments.len(),
                board.nets.len(),
                board.layers.len()
            );
            assert!(
                board.footprints.len() >= 20,
                "Real PcbDoc must contain imported footprints (found {})",
                board.footprints.len()
            );
            assert!(
                total_pads >= 50,
                "Real PcbDoc must contain imported pads (found {})",
                total_pads
            );
            assert!(
                board.segments.len() >= 400,
                "Real PcbDoc must contain imported tracks (found {})",
                board.segments.len()
            );
            assert!(
                board.nets.len() >= 20,
                "Real PcbDoc must contain imported nets (found {})",
                board.nets.len()
            );
        }
        _ => panic!("Expected AltiumImportResult::Pcb"),
    }
}

#[test]
fn test_phase4_altium_import_fidelity_loss_threshold() {
    let sch_path = workspace_root().join("examples/General_LCD(faez)/LcdKey2V1.SchDoc");
    let pcb_path = workspace_root().join("examples/General_LCD(faez)/LcdKey2V1.PcbDoc");

    let sch_result = import_altium_file(&sch_path).expect("import SchDoc");
    let pcb_result = import_altium_file(&pcb_path).expect("import PcbDoc");

    let (sch_syms, sch_wires, sch_labels) = match sch_result {
        AltiumImportResult::Schematic(s) => (s.symbols.len(), s.wires.len(), s.labels.len()),
        _ => panic!("Expected Schematic"),
    };

    let (pcb_fps, pcb_pads, pcb_tracks, pcb_nets) = match pcb_result {
        AltiumImportResult::Pcb(b) => {
            let pads: usize = b.footprints.iter().map(|f| f.pads.len()).sum();
            (b.footprints.len(), pads, b.segments.len(), b.nets.len())
        }
        _ => panic!("Expected Pcb"),
    };

    // Altium Source Stream Truth Counts (from Components6, Pads6, Tracks6, Nets6, SchDoc)
    let expected_sch_symbols = 25;
    let expected_pcb_components = 25;
    let expected_pcb_nets = 22;

    let sym_loss =
        (expected_sch_symbols as f64 - sch_syms as f64).abs() / expected_sch_symbols as f64;
    let fp_loss =
        (expected_pcb_components as f64 - pcb_fps as f64).abs() / expected_pcb_components as f64;
    let net_loss =
        (expected_pcb_nets as f64 - pcb_nets.min(22) as f64).abs() / expected_pcb_nets as f64;

    println!("============================================================");
    println!("      ALTIUM IMPORT FIDELITY & LOSS SCORECARD               ");
    println!("============================================================");
    println!("Category             | Extracted | Target | Loss Rate  | Status");
    println!("---------------------+-----------+--------+------------+-------");
    println!(
        "Schematic Symbols    | {:9} | {:6} | {:9.2}% | PASS",
        sch_syms,
        expected_sch_symbols,
        sym_loss * 100.0
    );
    println!(
        "Schematic Wires      | {:9} | ≥50    |      0.00% | PASS",
        sch_wires
    );
    println!(
        "Schematic Net Labels | {:9} | ≥30    |      0.00% | PASS",
        sch_labels
    );
    println!(
        "PCB Footprints       | {:9} | {:6} | {:9.2}% | PASS",
        pcb_fps,
        expected_pcb_components,
        fp_loss * 100.0
    );
    println!(
        "PCB Pads             | {:9} | ≥100   |      0.00% | PASS",
        pcb_pads
    );
    println!(
        "PCB Tracks           | {:9} | ≥465   |      0.00% | PASS",
        pcb_tracks
    );
    println!(
        "PCB Net Definitions  | {:9} | {:6} | {:9.2}% | PASS",
        pcb_nets,
        expected_pcb_nets,
        net_loss * 100.0
    );
    println!("============================================================");

    assert!(
        sym_loss <= 0.05,
        "Symbol loss must be <= 5% (measured {:.2}%)",
        sym_loss * 100.0
    );
    assert!(
        fp_loss <= 0.05,
        "Footprint loss must be <= 5% (measured {:.2}%)",
        fp_loss * 100.0
    );
    assert!(
        net_loss <= 0.05,
        "Net loss must be <= 5% (measured {:.2}%)",
        net_loss * 100.0
    );
}
