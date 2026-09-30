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

#[test]
fn test_altium_pcb_pad_and_custom_shape_parsing() {
    use oxide_altium_importer::pcb_importer::parse_pads;
    use oxide_types::pcb::{Footprint, PadShape, PadType, Point};

    let mut board = PcbBoard {
        uuid: Uuid::now_v7(),
        version: 1,
        generator: "Test".to_string(),
        thickness: 1.6,
        outline: Vec::new(),
        layers: Vec::new(),
        setup: None,
        nets: Vec::new(),
        footprints: vec![Footprint {
            uuid: Uuid::now_v7(),
            reference: "U1".to_string(),
            value: "STM32".to_string(),
            footprint_id: "QFN-32".to_string(),
            position: Point::new(10.0, 10.0),
            rotation: 0.0,
            layer: "Top Layer".to_string(),
            locked: false,
            pads: Vec::new(),
            graphics: Vec::new(),
            properties: Vec::new(),
        }],
        segments: Vec::new(),
        vias: Vec::new(),
        zones: Vec::new(),
        graphics: Vec::new(),
        texts: Vec::new(),
    };

    // Pad 1: RoundRect SMD pad with 25% corner radius
    let mut rec1 = AltiumRecord::new();
    rec1.properties.insert("RECORD".to_string(), "Pad".to_string());
    rec1.properties.insert("NAME".to_string(), "1".to_string());
    rec1.properties.insert("LOCATION.X".to_string(), "10.5mm".to_string());
    rec1.properties.insert("LOCATION.Y".to_string(), "10.5mm".to_string());
    rec1.properties.insert("TOPXSIZE".to_string(), "0.30mm".to_string());
    rec1.properties.insert("TOPYSIZE".to_string(), "0.80mm".to_string());
    rec1.properties.insert("TOPSHAPE".to_string(), "RoundedRectangle".to_string());
    rec1.properties.insert("ROUNDRECTANGULARRADIUS".to_string(), "25".to_string());
    rec1.properties.insert("LAYER".to_string(), "Top Layer".to_string());
    rec1.properties.insert("NET".to_string(), "5".to_string());

    // Pad 2: Through-hole circular pad with drill
    let mut rec2 = AltiumRecord::new();
    rec2.properties.insert("RECORD".to_string(), "Pad".to_string());
    rec2.properties.insert("NAME".to_string(), "EP".to_string());
    rec2.properties.insert("LOCATION.X".to_string(), "10.0mm".to_string());
    rec2.properties.insert("LOCATION.Y".to_string(), "10.0mm".to_string());
    rec2.properties.insert("TOPXSIZE".to_string(), "3.5mm".to_string());
    rec2.properties.insert("TOPYSIZE".to_string(), "3.5mm".to_string());
    rec2.properties.insert("HOLESIZE".to_string(), "0.3mm".to_string());
    rec2.properties.insert("TOPSHAPE".to_string(), "Round".to_string());
    rec2.properties.insert("LAYER".to_string(), "Multi-Layer".to_string());

    parse_pads(&[rec1, rec2], &mut board);

    let fp = &board.footprints[0];
    assert_eq!(fp.pads.len(), 2);

    let pad1 = &fp.pads[0];
    assert_eq!(pad1.number, "1");
    assert_eq!(pad1.pad_type, PadType::Smd);
    assert_eq!(pad1.shape, PadShape::RoundRect);
    assert_eq!(pad1.roundrect_ratio, 0.25);
    assert_eq!(pad1.net.as_ref().unwrap().number, 5);

    let pad2 = &fp.pads[1];
    assert_eq!(pad2.number, "EP");
    assert_eq!(pad2.pad_type, PadType::Thru);
    assert_eq!(pad2.shape, PadShape::Circle);
    assert_eq!(pad2.drill.as_ref().unwrap().diameter, 0.3);
}

