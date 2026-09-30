use oxide_altium_importer::{import_rules_from_records, verify_pcb_board, verify_schematic_sheet, AltiumRecord};
use oxide_types::pcb::PcbBoard;
use oxide_types::schematic::SchematicSheet;
use uuid::Uuid;

#[test]
fn test_altium_rule_importer_clearance_and_width() {
    let mut rec1 = AltiumRecord::new();
    rec1.insert("RECORD", "Rule");
    rec1.insert("RULEKIND", "Clearance");
    rec1.insert("GAP", "0.200mm");
    rec1.insert("SCOPE1", "InNetClass('HIGH_SPEED')");

    let mut rec2 = AltiumRecord::new();
    rec2.insert("RECORD", "Rule");
    rec2.insert("RULEKIND", "Width");
    rec2.insert("MINWIDTH", "0.125mm");
    rec2.insert("FAVOREDWIDTH", "0.254mm");
    rec2.insert("MAXWIDTH", "0.500mm");
    rec2.insert("SCOPE1", "All");

    let manager = import_rules_from_records(&[rec1, rec2]);
    assert_eq!(manager.rules.len(), 2);
}

#[test]
fn test_conversion_verification_engine() {
    let board = PcbBoard {
        uuid: Uuid::now_v7(),
        version: 1,
        generator: "Test".to_string(),
        thickness: 1.6,
        outline: Vec::new(),
        layers: Vec::new(),
        setup: None,
        nets: Vec::new(),
        footprints: Vec::new(),
        segments: Vec::new(),
        vias: Vec::new(),
        zones: Vec::new(),
        graphics: Vec::new(),
        texts: Vec::new(),
    };

    let report = verify_pcb_board(&board, 0, 0).expect("verify");
    assert!(report.passed);
}
