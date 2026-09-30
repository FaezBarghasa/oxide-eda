use oxide_altium_importer::{import_rules_from_records, verify_pcb_board, AltiumRecord};
use oxide_types::pcb::PcbBoard;
use uuid::Uuid;

#[test]
fn test_altium_rule_importer_clearance_and_width() {
    let mut rec1 = AltiumRecord::new();
    rec1.properties.insert("RECORD".to_string(), "Rule".to_string());
    rec1.properties.insert("RULEKIND".to_string(), "Clearance".to_string());
    rec1.properties.insert("GAP".to_string(), "0.200mm".to_string());
    rec1.properties.insert("SCOPE1".to_string(), "InNetClass('HIGH_SPEED')".to_string());

    let mut rec2 = AltiumRecord::new();
    rec2.properties.insert("RECORD".to_string(), "Rule".to_string());
    rec2.properties.insert("RULEKIND".to_string(), "Width".to_string());
    rec2.properties.insert("MINWIDTH".to_string(), "0.125mm".to_string());
    rec2.properties.insert("FAVOREDWIDTH".to_string(), "0.254mm".to_string());
    rec2.properties.insert("MAXWIDTH".to_string(), "0.500mm".to_string());
    rec2.properties.insert("SCOPE1".to_string(), "All".to_string());

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
