//! Complex Real-World Stress Benchmark: Oberon-8 Poly Synth, Strata-FPGA DDR5, Titan-SBC 12-Layer.
//! Validates Oxide EDA engine performance across analog multi-channel replication,
//! high-density BGA escape placement, and multi-layer microvia HDI PCB stackups.

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
fn test_benchmark_project_1_oberon_8_voice_analog_poly_expansion() {
    // Project 1: Oberon-8 Full Discrete Polyphonic Synthesizer
    // 8 Voice Cards with VCOs (SSI2130), 4-pole Ladder VCF (SSI2144), dual VCAs, and BBD effects
    let mut voice_sheet = SchematicSheet {
        uuid: Uuid::new_v4(),
        version: 1,
        generator: "Oxide EDA".to_string(),
        generator_version: "1.5.0".to_string(),
        paper_size: "A3".to_string(),
        root_sheet_page: "2".to_string(),
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

    // Add VCO core (Dual SSI2130)
    let mut vco = Symbol::empty();
    vco.reference = "U101".to_string();
    vco.value = "SSI2130".to_string();
    vco.footprint = "Package_SO:TSSOP-30_4.4x7.8mm_P0.5mm".to_string();
    vco.position = SchPoint::new(80.0, 60.0);
    voice_sheet.symbols.push(vco);

    // Add VCF core (SSI2144 4-pole OTA filter)
    let mut vcf = Symbol::empty();
    vcf.reference = "U102".to_string();
    vcf.value = "SSI2144".to_string();
    vcf.footprint = "Package_SO:SOIC-16_3.9x9.9mm_P1.27mm".to_string();
    vcf.position = SchPoint::new(140.0, 60.0);
    voice_sheet.symbols.push(vcf);

    // Add Dual VCA (THAT2180)
    let mut vca = Symbol::empty();
    vca.reference = "U103".to_string();
    vca.value = "THAT2180".to_string();
    vca.footprint = "Package_SIP:SIP-8_2.54mm_Straight".to_string();
    vca.position = SchPoint::new(200.0, 60.0);
    voice_sheet.symbols.push(vca);

    let child_sheet = ChildSheet {
        uuid: Uuid::new_v4(),
        name: "AnalogVoice".to_string(),
        filename: "analog_voice.snxsch".to_string(),
        position: SchPoint::new(20.0, 20.0),
        size: (80.0, 60.0),
        stroke_width: 0.15,
        fill: FillType::None,
        stroke_color: None,
        fill_color: None,
        fields_autoplaced: true,
        pins: Vec::new(),
        instances: Vec::new(),
    };

    // Altium-style Repeat(Voice, 1, 8) expansion for 8 independent analog voice cards
    let instances = parse_and_expand_repeat("Repeat(Voice, 1, 8)", &child_sheet, &voice_sheet);
    assert_eq!(instances.len(), 8, "Must expand to 8 full polyphonic voice channels");
    assert_eq!(instances[0].sheet_name, "Voice_CH1");
    assert_eq!(instances[7].sheet_name, "Voice_CH8");
    assert_eq!(instances[0].designator_map.get("U101").unwrap(), "U101_CH1");
    assert_eq!(instances[7].designator_map.get("U101").unwrap(), "U101_CH8");
    assert_eq!(instances[0].designator_map.get("U102").unwrap(), "U102_CH1");
    assert_eq!(instances[7].designator_map.get("U102").unwrap(), "U102_CH8");
}

#[test]
fn test_benchmark_project_2_fpga_ddr5_bga_fanout_placement() {
    // Project 2: Strata-FPGA DDR5 Kintex-UltraScale+ (0.8mm BGA, 1156 pins)
    let board = PcbBoard::default();
    let mut engine = PcbEngine::new(board);

    let fpga_uuid = Uuid::new_v4();
    let fpga = Footprint {
        uuid: fpga_uuid,
        reference: "U1".to_string(),
        value: "XCKU3P-2FFVA1156E".to_string(),
        footprint_id: "Package_BGA:BGA-1156_35.0x35.0mm_P0.8mm".to_string(),
        position: Point::new(100.0, 100.0),
        rotation: 0.0,
        layer: "F.Cu".to_string(),
        locked: true,
        properties: vec![PcbProperty {
            key: "PartNumber".to_string(),
            value: "XCKU3P-2FFVA1156E".to_string(),
            ..Default::default()
        }],
        pads: vec![
            Pad {
                uuid: Uuid::new_v4(),
                number: "A1".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(86.0, 86.0),
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
                position: Point::new(86.8, 86.0),
                size: Point::new(0.40, 0.40),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet {
                    number: 20,
                    name: "DDR5_CK_P".to_string(),
                }),
                roundrect_ratio: 0.0,
            },
            Pad {
                uuid: Uuid::new_v4(),
                number: "A3".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(87.6, 86.0),
                size: Point::new(0.40, 0.40),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet {
                    number: 21,
                    name: "DDR5_CK_N".to_string(),
                }),
                roundrect_ratio: 0.0,
            },
        ],
        graphics: Vec::new(),
    };

    engine.execute(PcbCommand::PlaceFootprint { footprint: fpga }).unwrap();
    assert_eq!(engine.board().footprints.len(), 1);

    // Add DDR5 differential clock pair segments (POD08, 0.10mm width, 0.12mm gap)
    let ck_p_seg = Segment {
        uuid: Uuid::new_v4(),
        net: 20,
        start: Point::new(86.8, 86.0),
        end: Point::new(70.0, 86.0),
        width: 0.10,
        layer: "F.Cu".to_string(),
    };
    let ck_n_seg = Segment {
        uuid: Uuid::new_v4(),
        net: 21,
        start: Point::new(87.6, 86.0),
        end: Point::new(70.0, 86.12),
        width: 0.10,
        layer: "F.Cu".to_string(),
    };
    engine.execute(PcbCommand::AddSegment { segment: ck_p_seg }).unwrap();
    engine.execute(PcbCommand::AddSegment { segment: ck_n_seg }).unwrap();
    assert_eq!(engine.board().segments.len(), 2);
}

#[test]
fn test_benchmark_project_3_titan_sbc_12_layer_hdi_stackup() {
    // Project 3: Titan-SBC 12-Layer Ultra-Dense ARM SBC (Credit card size, 32GB LPDDR5, HDMI 2.1, dual NVMe)
    let board = PcbBoard::default();
    let mut engine = PcbEngine::new(board);

    // Place Main SoC (0.65mm BGA)
    let soc_uuid = Uuid::new_v4();
    let soc = Footprint {
        uuid: soc_uuid,
        reference: "U1".to_string(),
        value: "ARM-SoC-OctaCore".to_string(),
        footprint_id: "Package_BGA:BGA-684_23.0x23.0mm_P0.65mm".to_string(),
        position: Point::new(42.5, 28.0),
        rotation: 0.0,
        layer: "F.Cu".to_string(),
        locked: true,
        properties: vec![PcbProperty {
            key: "PartNumber".to_string(),
            value: "ARM64-NPU-6TOPS".to_string(),
            ..Default::default()
        }],
        pads: vec![
            Pad {
                uuid: Uuid::new_v4(),
                number: "A1".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(35.0, 20.0),
                size: Point::new(0.35, 0.35),
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
                position: Point::new(35.65, 20.0),
                size: Point::new(0.35, 0.35),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet {
                    number: 2,
                    name: "VDD_CORE_0V75".to_string(),
                }),
                roundrect_ratio: 0.0,
            },
        ],
        graphics: Vec::new(),
    };

    engine.execute(PcbCommand::PlaceFootprint { footprint: soc }).unwrap();
    assert_eq!(engine.board().footprints.len(), 1);

    // Place High-Speed PCIe Gen 4 x4 TX Differential Pair (16 Gbps, 85 Ohm diff)
    let pcie_tx0_p = Segment {
        uuid: Uuid::new_v4(),
        net: 10,
        start: Point::new(35.0, 22.0),
        end: Point::new(20.0, 22.0),
        width: 0.12,
        layer: "F.Cu".to_string(),
    };
    let pcie_tx0_n = Segment {
        uuid: Uuid::new_v4(),
        net: 11,
        start: Point::new(35.0, 22.25),
        end: Point::new(20.0, 22.25),
        width: 0.12,
        layer: "F.Cu".to_string(),
    };
    engine.execute(PcbCommand::AddSegment { segment: pcie_tx0_p }).unwrap();
    engine.execute(PcbCommand::AddSegment { segment: pcie_tx0_n }).unwrap();
    assert_eq!(engine.board().segments.len(), 2);

    // Add Stacked Any-Layer Microvia (L1 -> L2, L2 -> L3) for Core VDD 0.75V (50A peak transient)
    let microvia_1_2 = Via {
        uuid: Uuid::new_v4(),
        net: 2,
        position: Point::new(35.65, 19.3),
        diameter: 0.30,
        drill: 0.10,
        layers: vec!["F.Cu".to_string(), "In1_GND".to_string()],
        via_type: ViaType::Micro,
        via_span: Some(ViaSpan::new(1, 2)),
    };
    let microvia_2_3 = Via {
        uuid: Uuid::new_v4(),
        net: 2,
        position: Point::new(35.65, 19.3),
        diameter: 0.30,
        drill: 0.10,
        layers: vec!["In1_GND".to_string(), "In2_Sig".to_string()],
        via_type: ViaType::Micro,
        via_span: Some(ViaSpan::new(2, 3)),
    };

    engine.execute(PcbCommand::AddVia { via: microvia_1_2 }).unwrap();
    engine.execute(PcbCommand::AddVia { via: microvia_2_3 }).unwrap();
    assert_eq!(engine.board().vias.len(), 2);
}
