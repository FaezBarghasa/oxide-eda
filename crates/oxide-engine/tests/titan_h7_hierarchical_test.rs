//! Titan-H7: Industrial 6-Layer Multi-Axis Motion & Mixed-Signal Core
//! Hierarchical & Multi-Channel Test Suite.

use oxide_engine::multi_channel::parse_and_expand_repeat;
use oxide_engine::pcb::{PcbCommand, PcbEngine};
use oxide_types::pcb::{
    Footprint, Pad, PadNet, PadShape, PadType, PcbBoard, Point, Segment, Via, ViaSpan, ViaType,
};
use oxide_types::property::PcbProperty;
use oxide_types::schematic::{ChildSheet, FillType, Point as SchPoint, SchematicSheet, Symbol};
use std::collections::HashMap;
use uuid::Uuid;

#[test]
fn test_titan_h7_hierarchical_multi_channel_expansion() {
    let mut stepper_child = SchematicSheet {
        uuid: Uuid::new_v4(),
        version: 1,
        generator: "Oxide EDA".to_string(),
        generator_version: "1.5.0".to_string(),
        paper_size: "A3".to_string(),
        root_sheet_page: "6".to_string(),
        symbols: Vec::new(),
        wires: Vec::new(),
        junctions: Vec::new(),
        labels: Vec::new(),
        child_sheets: Vec::new(),
        no_connects: Vec::new(),
        text_notes: Vec::new(),
        buses: Vec::new(),
        bus_entries: Vec::new(),
        drawings: Vec::new(),
        no_erc_directives: Vec::new(),
        title_block: HashMap::new(),
        lib_symbols: HashMap::new(),
    };

    let mut u1 = Symbol::empty();
    u1.reference = "U1".to_string();
    u1.value = "TMC5160-TA".to_string();
    u1.footprint = "Package_QFP:TQFP-48-1EP_7x7mm_P0.5mm_EP5x5mm".to_string();
    u1.position = SchPoint::new(100.0, 100.0);
    stepper_child.symbols.push(u1);

    let mut r1 = Symbol::empty();
    r1.reference = "R_SENSE1".to_string();
    r1.value = "0.075R 1% 2W".to_string();
    r1.footprint = "Resistor_SMD:R_2512_6332Metric".to_string();
    stepper_child.symbols.push(r1);

    let child_sheet = ChildSheet {
        uuid: Uuid::new_v4(),
        name: "StepperChannel".to_string(),
        filename: "stepper_channel.snxsch".to_string(),
        position: SchPoint::new(50.0, 50.0),
        size: (60.0, 40.0),
        stroke_width: 0.15,
        fill: FillType::None,
        stroke_color: None,
        fill_color: None,
        fields_autoplaced: true,
        pins: Vec::new(),
        instances: Vec::new(),
    };

    let instances = parse_and_expand_repeat("Repeat(Axis, 1, 4)", &child_sheet, &stepper_child);
    assert_eq!(
        instances.len(),
        4,
        "Must expand to exactly 4 physical axis channels"
    );
    assert_eq!(instances[0].sheet_name, "Axis_CH1");
    assert_eq!(instances[3].sheet_name, "Axis_CH4");
    assert_eq!(instances[0].designator_map.get("U1").unwrap(), "U1_CH1");
    assert_eq!(instances[3].designator_map.get("U1").unwrap(), "U1_CH4");
    assert_eq!(
        instances[0].designator_map.get("R_SENSE1").unwrap(),
        "R_SENSE1_CH1"
    );
    assert_eq!(
        instances[3].designator_map.get("R_SENSE1").unwrap(),
        "R_SENSE1_CH4"
    );
}

#[test]
fn test_titan_h7_6_layer_pcb_stackup_and_placement() {
    let board = PcbBoard::default();
    let mut engine = PcbEngine::new(board);

    let mcu_uuid = Uuid::new_v4();
    let mcu = Footprint {
        uuid: mcu_uuid,
        reference: "U101".to_string(),
        value: "STM32H750IBK6".to_string(),
        footprint_id: "Package_BGA:BGA-176_10.0x10.0mm_P0.8mm".to_string(),
        position: Point::new(100.0, 100.0),
        rotation: 0.0,
        layer: "F.Cu".to_string(),
        locked: true,
        properties: vec![PcbProperty {
            key: "PartNumber".to_string(),
            value: "STM32H750IBK6TR".to_string(),
            ..Default::default()
        }],
        pads: vec![
            Pad {
                uuid: Uuid::new_v4(),
                number: "A1".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(96.0, 96.0),
                size: Point::new(0.40, 0.40),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet {
                    number: 1,
                    name: "GND".to_string(),
                }),
                roundrect_ratio: 0.0,
            },
            Pad {
                uuid: Uuid::new_v4(),
                number: "A2".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(96.8, 96.0),
                size: Point::new(0.40, 0.40),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet {
                    number: 2,
                    name: "VDD_1V2".to_string(),
                }),
                roundrect_ratio: 0.0,
            },
        ],
        graphics: Vec::new(),
    };

    engine
        .execute(PcbCommand::PlaceFootprint { footprint: mcu })
        .unwrap();
    assert_eq!(engine.board().footprints.len(), 1);

    // Place High-Speed USB 2.0 Differential Pair Traces (0.18mm width)
    let usb_dp_seg = Segment {
        uuid: Uuid::new_v4(),
        net: 3,
        start: Point::new(96.0, 96.8),
        end: Point::new(80.0, 96.8),
        width: 0.18,
        layer: "F.Cu".to_string(),
    };
    engine
        .execute(PcbCommand::AddSegment {
            segment: usb_dp_seg,
        })
        .unwrap();
    assert_eq!(engine.board().segments.len(), 1);

    // Add High-Current Power Microvia for VDD_1V2 rail
    let pwr_via = Via {
        uuid: Uuid::new_v4(),
        net: 2,
        position: Point::new(96.8, 95.2),
        diameter: 0.45,
        drill: 0.20,
        layers: vec!["F.Cu".to_string(), "In1_SigPower".to_string()],
        via_type: ViaType::Micro,
        via_span: Some(ViaSpan::new(1, 2)),
    };
    engine.execute(PcbCommand::AddVia { via: pwr_via }).unwrap();
    assert_eq!(engine.board().vias.len(), 1);
}
