use oxide_types::format::SnxPcb;
use oxide_types::pcb::{
    DrillDef, Footprint, LayerDef, NetDef, Pad, PadNet, PadShape, PadType, PcbBoard, Point,
    Segment, Via, ViaType,
};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
fn make_pad(
    number: &str,
    pos: Point,
    size: Point,
    pad_type: PadType,
    shape: PadShape,
    drill: Option<f64>,
    layers: Vec<String>,
    net: Option<(u32, &str)>,
) -> Pad {
    Pad {
        uuid: Uuid::new_v4(),
        number: number.to_string(),
        pad_type,
        shape,
        position: pos,
        size,
        drill: drill.map(|d| DrillDef {
            diameter: d,
            shape: "circle".to_string(),
        }),
        layers,
        net: net.map(|(num, name)| PadNet {
            number: num,
            name: name.to_string(),
        }),
        roundrect_ratio: 0.25,
    }
}

fn make_soic14(ref_des: &str, pos: Point, layer: &str) -> Footprint {
    let mut pads = Vec::new();
    // 7 pins on left, 7 pins on right
    let pitch = 1.27;
    for i in 0..7 {
        let py = (i as f64 - 3.0) * pitch;
        // Pins 1..7 on left (x = -2.7)
        pads.push(make_pad(
            &(i + 1).to_string(),
            Point::new(-2.7, py),
            Point::new(1.5, 0.6),
            PadType::Smd,
            PadShape::RoundRect,
            None,
            vec![layer.to_string()],
            Some(((i + 1) as u32, &format!("SIG_{}", i + 1))),
        ));
        // Pins 14..8 on right (x = 2.7)
        let right_pin = 14 - i;
        pads.push(make_pad(
            &right_pin.to_string(),
            Point::new(2.7, py),
            Point::new(1.5, 0.6),
            PadType::Smd,
            PadShape::RoundRect,
            None,
            vec![layer.to_string()],
            Some((right_pin as u32, &format!("SIG_{right_pin}"))),
        ));
    }

    Footprint {
        uuid: Uuid::new_v4(),
        reference: ref_des.to_string(),
        value: "TL074_Quad_OpAmp".to_string(),
        footprint_id: "Package_SO:SOIC-14_3.9x8.7mm_P1.27mm".to_string(),
        position: pos,
        rotation: 0.0,
        layer: layer.to_string(),
        locked: false,
        properties: Vec::new(),
        pads,
        graphics: Vec::new(),
    }
}

fn make_smd_0805(
    ref_des: &str,
    value: &str,
    pos: Point,
    layer: &str,
    net_a: (u32, &str),
    net_b: (u32, &str),
) -> Footprint {
    let pads = vec![
        make_pad(
            "1",
            Point::new(-0.95, 0.0),
            Point::new(1.0, 1.3),
            PadType::Smd,
            PadShape::Rect,
            None,
            vec![layer.to_string()],
            Some(net_a),
        ),
        make_pad(
            "2",
            Point::new(0.95, 0.0),
            Point::new(1.0, 1.3),
            PadType::Smd,
            PadShape::Rect,
            None,
            vec![layer.to_string()],
            Some(net_b),
        ),
    ];

    Footprint {
        uuid: Uuid::new_v4(),
        reference: ref_des.to_string(),
        value: value.to_string(),
        footprint_id: "Resistor_SMD:R_0805_2012Metric".to_string(),
        position: pos,
        rotation: 0.0,
        layer: layer.to_string(),
        locked: false,
        properties: Vec::new(),
        pads,
        graphics: Vec::new(),
    }
}

#[test]
fn test_generate_and_verify_real_verifiable_fixtures() {
    // -------------------------------------------------------------
    // Fixture 1: Real 2-Layer Polyphonic Synthesizer Module
    // -------------------------------------------------------------
    let mut synth_board = PcbBoard::default();
    synth_board.generator = "Oxide EDA Verifiable Test Suite".to_string();
    synth_board.thickness = 1.6;
    synth_board.outline = vec![
        Point::new(0.0, 0.0),
        Point::new(120.0, 0.0),
        Point::new(120.0, 80.0),
        Point::new(0.0, 80.0),
    ];
    synth_board.layers = vec![
        LayerDef {
            id: 0,
            name: "TopCopper".to_string(),
            layer_type: "copper".to_string(),
        },
        LayerDef {
            id: 1,
            name: "BottomCopper".to_string(),
            layer_type: "copper".to_string(),
        },
    ];
    synth_board.nets = vec![
        NetDef {
            number: 1,
            name: "GND".to_string(),
        },
        NetDef {
            number: 2,
            name: "+12V".to_string(),
        },
        NetDef {
            number: 3,
            name: "-12V".to_string(),
        },
        NetDef {
            number: 4,
            name: "VCO_OUT".to_string(),
        },
        NetDef {
            number: 5,
            name: "VCF_IN".to_string(),
        },
        NetDef {
            number: 6,
            name: "VCF_OUT".to_string(),
        },
    ];

    // Place Real Components
    synth_board
        .footprints
        .push(make_soic14("U1", Point::new(40.0, 40.0), "TopCopper"));
    synth_board
        .footprints
        .push(make_soic14("U2", Point::new(80.0, 40.0), "TopCopper"));
    synth_board.footprints.push(make_smd_0805(
        "R1",
        "100k",
        Point::new(25.0, 40.0),
        "TopCopper",
        (1, "GND"),
        (4, "VCO_OUT"),
    ));
    synth_board.footprints.push(make_smd_0805(
        "R2",
        "10k",
        Point::new(25.0, 48.0),
        "TopCopper",
        (2, "+12V"),
        (5, "VCF_IN"),
    ));
    synth_board.footprints.push(make_smd_0805(
        "C1",
        "10nF",
        Point::new(60.0, 40.0),
        "TopCopper",
        (4, "VCO_OUT"),
        (5, "VCF_IN"),
    ));
    synth_board.footprints.push(make_smd_0805(
        "C2",
        "47uF",
        Point::new(60.0, 50.0),
        "TopCopper",
        (2, "+12V"),
        (1, "GND"),
    ));

    // Place Real Segments (width: 0.35mm, perfectly spaced)
    synth_board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        start: Point::new(26.0, 40.0),
        end: Point::new(35.0, 40.0),
        width: 0.35,
        layer: "TopCopper".to_string(),
        net: 4,
    });
    synth_board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        start: Point::new(45.0, 40.0),
        end: Point::new(59.0, 40.0),
        width: 0.35,
        layer: "TopCopper".to_string(),
        net: 4,
    });
    synth_board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        start: Point::new(61.0, 40.0),
        end: Point::new(75.0, 40.0),
        width: 0.35,
        layer: "TopCopper".to_string(),
        net: 5,
    });

    // Place Power Bus on BottomCopper
    synth_board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        start: Point::new(10.0, 20.0),
        end: Point::new(110.0, 20.0),
        width: 0.50,
        layer: "BottomCopper".to_string(),
        net: 1, // GND
    });
    synth_board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        start: Point::new(10.0, 25.0),
        end: Point::new(110.0, 25.0),
        width: 0.50,
        layer: "BottomCopper".to_string(),
        net: 2, // +12V
    });

    // Vias connecting Top to Bottom
    synth_board.vias.push(Via {
        uuid: Uuid::new_v4(),
        position: Point::new(24.0, 20.0),
        diameter: 0.8,
        drill: 0.4,
        layers: vec!["TopCopper".to_string(), "BottomCopper".to_string()],
        net: 1,
        via_type: ViaType::Through,
        via_span: None,
    });
    synth_board.vias.push(Via {
        uuid: Uuid::new_v4(),
        position: Point::new(24.0, 25.0),
        diameter: 0.8,
        drill: 0.4,
        layers: vec!["TopCopper".to_string(), "BottomCopper".to_string()],
        net: 2,
        via_type: ViaType::Through,
        via_span: None,
    });

    let root_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let out_dir = root_dir.join("fixtures").join("verifiable");
    std::fs::create_dir_all(&out_dir).expect("create verifiable dir");

    let synth_path = out_dir.join("real_analog_synth_2layer.snxpcb");
    let fpga_path = out_dir.join("real_fpga_ddr5_4layer.snxpcb");

    // Serialise and write to disk
    let synth_snx = SnxPcb::new(synth_board.clone());
    let synth_str = synth_snx.write_string().expect("serialise synth board");
    std::fs::write(&synth_path, &synth_str).expect("write synth fixture");

    // Verify round-trip parse
    let parsed_synth = SnxPcb::parse(&synth_str).expect("parse synth fixture");
    assert_eq!(parsed_synth.board.footprints.len(), 6);
    assert_eq!(parsed_synth.board.segments.len(), 5);
    assert_eq!(parsed_synth.board.vias.len(), 2);

    // Run DRC to verify zero violations
    let rules_manager = oxide_rules::manager::ConstraintManager::new();
    let synth_violations = rules_manager.run_drc(&parsed_synth.board);
    assert!(
        synth_violations.is_empty(),
        "synth fixture must pass DRC with 0 violations, got: {synth_violations:?}"
    );

    // -------------------------------------------------------------
    // Fixture 2: Real 4-Layer FPGA DDR5 Board
    // -------------------------------------------------------------
    let mut fpga_board = PcbBoard::default();
    fpga_board.generator = "Oxide EDA Verifiable Test Suite".to_string();
    fpga_board.thickness = 1.6;
    fpga_board.outline = vec![
        Point::new(0.0, 0.0),
        Point::new(100.0, 0.0),
        Point::new(100.0, 100.0),
        Point::new(0.0, 100.0),
    ];
    fpga_board.layers = vec![
        LayerDef {
            id: 0,
            name: "TopCopper".to_string(),
            layer_type: "copper".to_string(),
        },
        LayerDef {
            id: 1,
            name: "In1.GND".to_string(),
            layer_type: "copper".to_string(),
        },
        LayerDef {
            id: 2,
            name: "In2.PWR".to_string(),
            layer_type: "copper".to_string(),
        },
        LayerDef {
            id: 3,
            name: "BottomCopper".to_string(),
            layer_type: "copper".to_string(),
        },
    ];
    fpga_board.nets = vec![
        NetDef {
            number: 1,
            name: "GND".to_string(),
        },
        NetDef {
            number: 2,
            name: "+1V8".to_string(),
        },
        NetDef {
            number: 3,
            name: "DDR5_CLK_P".to_string(),
        },
        NetDef {
            number: 4,
            name: "DDR5_CLK_N".to_string(),
        },
        NetDef {
            number: 5,
            name: "DDR5_DQ0".to_string(),
        },
    ];

    // Footprints
    fpga_board
        .footprints
        .push(make_soic14("U1_FPGA", Point::new(35.0, 50.0), "TopCopper"));
    fpga_board
        .footprints
        .push(make_soic14("U2_DDR5", Point::new(65.0, 50.0), "TopCopper"));
    fpga_board.footprints.push(make_smd_0805(
        "C1",
        "100nF",
        Point::new(35.0, 35.0),
        "TopCopper",
        (1, "GND"),
        (2, "+1V8"),
    ));
    fpga_board.footprints.push(make_smd_0805(
        "C2",
        "100nF",
        Point::new(65.0, 35.0),
        "TopCopper",
        (1, "GND"),
        (2, "+1V8"),
    ));

    // Differential pair tracks (spaced 0.3mm apart, width 0.20mm)
    fpga_board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        start: Point::new(40.0, 52.0),
        end: Point::new(60.0, 52.0),
        width: 0.20,
        layer: "TopCopper".to_string(),
        net: 3, // DDR5_CLK_P
    });
    fpga_board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        start: Point::new(40.0, 52.3),
        end: Point::new(60.0, 52.3),
        width: 0.20,
        layer: "TopCopper".to_string(),
        net: 4, // DDR5_CLK_N
    });

    let fpga_snx = SnxPcb::new(fpga_board.clone());
    let fpga_str = fpga_snx.write_string().expect("serialise fpga board");
    std::fs::write(&fpga_path, &fpga_str).expect("write fpga fixture");

    let parsed_fpga = SnxPcb::parse(&fpga_str).expect("parse fpga fixture");
    assert_eq!(parsed_fpga.board.footprints.len(), 4);
    assert_eq!(parsed_fpga.board.layers.len(), 4);

    let fpga_violations = rules_manager.run_drc(&parsed_fpga.board);
    assert!(
        fpga_violations.is_empty(),
        "fpga fixture must pass DRC with 0 violations, got: {fpga_violations:?}"
    );

    // Verify Gerber multi-layer export produces unique filenames (D6 verification)
    let exporter = oxide_output::GerberExporter::new(oxide_output::GerberOptions::default());
    let gerbers = exporter
        .export_board(&parsed_fpga.board)
        .expect("export gerbers");
    let mut filenames = std::collections::HashSet::new();
    for g in &gerbers {
        assert!(
            filenames.insert(g.filename.clone()),
            "Duplicate gerber filename detected: {}",
            g.filename
        );
    }
    // Confirm inner layers got unique names
    assert!(filenames.contains("output_in2_cu.gbr") || filenames.contains("output_in3_cu.gbr"));
}
