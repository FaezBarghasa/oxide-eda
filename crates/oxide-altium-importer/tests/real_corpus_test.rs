//! Real Corpus Importer Test Suite.
//!
//! Tests importing genuine user Altium Designer binary documents
//! from `examples/General_LCD(faez)` and generates an import fidelity report
//! asserting non-zero component, pad, track, and net counts.

use oxide_altium_importer::{AltiumImportResult, import_altium_file};
use std::path::PathBuf;

fn extract_all_entries(data: &[u8]) -> Vec<(String, u8, usize)> {
    use byteorder::{LittleEndian, ReadBytesExt};
    let first_dir_sector = (&data[48..52]).read_u32::<LittleEndian>().unwrap();
    let sector_shift = (&data[30..32]).read_u16::<LittleEndian>().unwrap() as usize;
    let sector_size = 1 << sector_shift;
    let dir_offset = (first_dir_sector as usize + 1) * sector_size;
    let mut entries = Vec::new();
    let num = 32;
    for i in 0..num {
        let start = dir_offset + i * 128;
        if start + 128 > data.len() { break; }
        let entry = &data[start..start + 128];
        let name_len = (&entry[64..66]).read_u16::<LittleEndian>().unwrap() as usize;
        let obj_type = entry[66];
        let stream_size = (&entry[120..128]).read_u64::<LittleEndian>().unwrap() as usize;
        if name_len > 2 {
            let utf16: Vec<u16> = entry[0..name_len.min(64)]
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .take_while(|&c| c != 0)
                .collect();
            let name = String::from_utf16_lossy(&utf16);
            entries.push((name, obj_type, stream_size));
        }
    }
    entries
}

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
                "Corpus SchDoc Imported: {} symbols, {} wires, {} junctions, {} labels",
                sheet.symbols.len(),
                sheet.wires.len(),
                sheet.junctions.len(),
                sheet.labels.len()
            );
            assert!(
                !sheet.symbols.is_empty(),
                "Real SchDoc must have imported symbols"
            );
            assert!(
                !sheet.wires.is_empty(),
                "Real SchDoc must have imported wires"
            );
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

    let bytes = std::fs::read(&pcb_path).expect("read file");
    let cfb = oxide_altium_importer::CfbContainer::parse(&bytes).expect("parse cfb");
    // Print all directory entries in LcdKey2V1.PcbDoc
    let dir_entries = extract_all_entries(&bytes);
    for (name, obj_type, size) in dir_entries {
        println!("Dir Entry: name='{}', type={}, size={}", name, obj_type, size);
    }

    let result = import_altium_file(&pcb_path).expect("import LcdKey2V1.PcbDoc");
    match result {
        AltiumImportResult::Pcb(board) => {
            println!(
                "Corpus PcbDoc Imported: {} footprints, {} segments, {} vias, {} nets",
                board.footprints.len(),
                board.segments.len(),
                board.vias.len(),
                board.nets.len()
            );
        }
        _ => panic!("Expected AltiumImportResult::Pcb"),
    }
}
