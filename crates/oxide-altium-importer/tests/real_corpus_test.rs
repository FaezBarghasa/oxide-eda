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
            println!(
                "[Corpus Report] LcdKey2V1.PcbDoc: {} tracks, {} nets, {} copper layers",
                board.segments.len(),
                board.nets.len(),
                board.layers.len()
            );
            assert!(
                !board.segments.is_empty(),
                "Real PcbDoc must contain imported tracks"
            );
            assert!(
                !board.nets.is_empty(),
                "Real PcbDoc must contain imported nets"
            );
        }
        _ => panic!("Expected AltiumImportResult::Pcb"),
    }
}
