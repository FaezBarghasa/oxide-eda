//! Gerber X2/X3, IPC-2581C, and ODB++ Industrial Manufacturing Output Parity Tests.

use oxide_output::{
    export_ipc2581, export_odbpp_package, GerberExporter, GerberLayer, GerberOptions,
    Ipc2581Options,
};
use oxide_types::pcb::{
    LayerDef, NetDef, Pad, PadShape, PadType, PcbBoard, Point, Segment, Via, ViaType,
};
use uuid::Uuid;

fn build_manufacturing_test_board() -> PcbBoard {
    let mut board = PcbBoard {
        generator: "Titan-Flagship-Test".to_string(),
        layers: vec![
            LayerDef {
                id: 1,
                name: "Top Layer".into(),
                layer_type: "signal".into(),
            },
            LayerDef {
                id: 2,
                name: "Bottom Layer".into(),
                layer_type: "signal".into(),
            },
        ],
        nets: vec![
            NetDef {
                number: 1,
                name: "GND".into(),
            },
            NetDef {
                number: 2,
                name: "VCC_3V3".into(),
            },
            NetDef {
                number: 3,
                name: "DDR5_CLK_P".into(),
            },
        ],
        ..Default::default()
    };

    // Component U1 with multiple SMD pads
    let mut u1 = oxide_types::pcb::Footprint {
        uuid: Uuid::new_v4(),
        reference: "U1".into(),
        footprint_id: "QFN-16_3x3mm".into(),
        value: "STM32F401CEU6".into(),
        position: Point::new(25.0, 25.0),
        rotation: 0.0,
        layer: "Top Layer".into(),
        pads: Vec::new(),
        properties: Vec::new(),
        graphics: Vec::new(),
        locked: false,
    };

    for i in 1..=4 {
        u1.pads.push(Pad {
            uuid: Uuid::new_v4(),
            number: format!("{i}"),
            position: Point::new((i as f64) * 0.5 - 1.25, -1.5),
            size: Point::new(0.30, 0.80),
            shape: PadShape::Rect,
            pad_type: PadType::Smd,
            layers: vec!["Top Layer".into()],
            net: Some(oxide_types::pcb::PadNet {
                number: if i == 1 { 1 } else { 3 },
                name: if i == 1 { "GND".into() } else { "DDR5_CLK_P".into() },
            }),
            drill: None,
            roundrect_ratio: 0.0,
        });
    }
    board.footprints.push(u1);

    // Segment on net 3
    board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        start: Point::new(25.0, 25.0),
        end: Point::new(35.0, 25.0),
        width: 0.15,
        layer: "Top Layer".into(),
        net: 3,
    });

    // Via on net 1 (GND)
    board.vias.push(Via {
        uuid: Uuid::new_v4(),
        position: Point::new(20.0, 20.0),
        diameter: 0.45,
        drill: 0.20,
        layers: vec!["Top Layer".into(), "Bottom Layer".into()],
        net: 1,
        via_type: ViaType::Through,
        via_span: None,
    });

    board
}

#[test]
fn test_gerber_x2_and_x3_metadata_and_component_attributes() {
    let board = build_manufacturing_test_board();
    let opts = GerberOptions {
        use_gerber_x2_attributes: true,
        use_gerber_x3_attributes: true,
        include_netlist_attributes: true,
        coordinate_format_integer: 4,
        coordinate_format_decimal: 6,
    };
    let exporter = GerberExporter::new(opts);
    let top_cu = exporter
        .export_single_layer(&board, GerberLayer::TopCopper)
        .expect("Gerber export must succeed");

    // 1. Validate Gerber X2 File Attributes
    assert!(
        top_cu.contains("%TF.FileFunction,Copper,L1,Top*%"),
        "Top copper must have Gerber X2 FileFunction attribute"
    );
    assert!(
        top_cu.contains("%TF.GenerationSoftware,OxideEDA,Oxide"),
        "Must declare OxideEDA generation software attribute"
    );
    assert!(
        top_cu.contains("%TF.SameCoordinates,Original*%"),
        "Must specify standard coordinate alignment attribute"
    );

    // 2. Validate Gerber X3 Component Attributes (%TO)
    assert!(
        top_cu.contains("%TO.CN,U1*%"),
        "Must emit component name / refdes attribute for U1"
    );
    assert!(
        top_cu.contains("%TO.CF,QFN-16_3x3mm*%"),
        "Must emit footprint attribute for U1"
    );
    assert!(
        top_cu.contains("%TO.CV,STM32F401CEU6*%"),
        "Must emit component value attribute for U1"
    );

    // 3. Validate Gerber X3 Pin & Net Attributes (%TD)
    assert!(
        top_cu.contains("%TD.P,1*%"),
        "Must emit pin number attribute for pad 1"
    );
    assert!(
        top_cu.contains("%TD.N,GND*%"),
        "Must emit net GND attribute for pad 1"
    );
    assert!(
        top_cu.contains("%TD.N,DDR5_CLK_P*%"),
        "Must emit net DDR5_CLK_P attribute for trace"
    );
    assert!(
        top_cu.contains("%TD*%"),
        "Must emit %TD*% to clear object attributes after flashing / drawing"
    );
}

#[test]
fn test_ipc2581c_and_odbpp_full_layer_and_component_synthesis() {
    let board = build_manufacturing_test_board();

    // 1. IPC-2581C
    let ipc = export_ipc2581(&board, &Ipc2581Options::default())
        .expect("IPC-2581 export must succeed");
    assert!(
        ipc.xml_content.contains("<IPC-2581 version=\"C\""),
        "Must declare IPC-2581 version C"
    );
    assert!(
        ipc.xml_content.contains("<Component refDes=\"U1\""),
        "Must include U1 component centroid"
    );
    assert!(
        ipc.xml_content.contains("<Net name=\"DDR5_CLK_P\""),
        "Must include netlist metadata"
    );

    // 2. ODB++ Package
    let odb = export_odbpp_package(&board).expect("ODB++ export must succeed");
    assert!(
        odb.files.iter().any(|f| f.relative_path == "matrix/matrix"),
        "Must include matrix/matrix layer stackup definition"
    );
    assert!(
        odb.files.iter().any(|f| f.relative_path.contains("netlists")),
        "Must include netlists"
    );
    assert!(
        odb.files.iter().any(|f| f.relative_path.contains("components")),
        "Must include component placements"
    );
}
