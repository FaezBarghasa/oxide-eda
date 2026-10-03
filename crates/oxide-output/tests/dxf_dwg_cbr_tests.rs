use oxide_output::{
    CbrExporter, CbrOptions, CdrExporter, CdrOptions, CdrVersion, DwgExporter, DwgOptions,
    DwgVersion, DxfExporter, DxfOptions,
};
use oxide_types::pcb::{
    DrillDef, Footprint, FpGraphic, Pad, PadShape, PadType, PcbBoard, Point, Segment, Via, ViaType,
    Zone,
};
use uuid::Uuid;

fn create_sample_board() -> PcbBoard {
    let mut board = PcbBoard {
        uuid: Uuid::now_v7(),
        ..Default::default()
    };

    // 1. Trace on Top Copper and Bottom Copper
    board.segments.push(Segment {
        uuid: Uuid::now_v7(),
        start: Point::new(10.0, 10.0),
        end: Point::new(30.0, 10.0),
        width: 0.25,
        layer: "F.Cu".to_string(),
        net: 1,
    });
    board.segments.push(Segment {
        uuid: Uuid::now_v7(),
        start: Point::new(10.0, 20.0),
        end: Point::new(40.0, 20.0),
        width: 0.35,
        layer: "B.Cu".to_string(),
        net: 2,
    });

    // 2. Via
    board.vias.push(Via {
        uuid: Uuid::now_v7(),
        position: Point::new(30.0, 10.0),
        diameter: 0.6,
        drill: 0.3,
        layers: vec!["F.Cu".to_string(), "B.Cu".to_string()],
        net: 1,
        via_type: ViaType::Through,
        via_span: None,
    });

    // 3. Footprint with SMD and Thru pads
    let mut fp = Footprint {
        uuid: Uuid::now_v7(),
        reference: "U1".to_string(),
        value: "STM32F4".to_string(),
        footprint_id: "LQFP-48".to_string(),
        position: Point::new(50.0, 50.0),
        rotation: 0.0,
        layer: "F.Cu".to_string(),
        locked: false,
        pads: Vec::new(),
        graphics: Vec::new(),
        properties: Vec::new(),
    };

    // SMD Pad
    fp.pads.push(Pad {
        uuid: Uuid::now_v7(),
        number: "1".to_string(),
        pad_type: PadType::Smd,
        shape: PadShape::Rect,
        position: Point::new(-5.0, 0.0),
        size: Point::new(1.5, 0.5),
        drill: None,
        layers: vec!["F.Cu".to_string()],
        net: None,
        roundrect_ratio: 0.0,
    });

    // Through-hole Pad
    fp.pads.push(Pad {
        uuid: Uuid::now_v7(),
        number: "2".to_string(),
        pad_type: PadType::Thru,
        shape: PadShape::Circle,
        position: Point::new(5.0, 0.0),
        size: Point::new(1.6, 1.6),
        drill: Some(DrillDef {
            diameter: 0.8,
            shape: "circle".to_string(),
        }),
        layers: vec!["F.Cu".to_string(), "B.Cu".to_string()],
        net: None,
        roundrect_ratio: 0.0,
    });

    // Silkscreen outline on footprint
    fp.graphics.push(FpGraphic {
        graphic_type: "line".to_string(),
        layer: "F.SilkS".to_string(),
        width: 0.15,
        start: Some(Point::new(-6.0, -6.0)),
        end: Some(Point::new(6.0, -6.0)),
        center: None,
        mid: None,
        radius: 0.0,
        points: Vec::new(),
        text: String::new(),
        font_size: 1.0,
        position: None,
        rotation: 0.0,
        fill: String::new(),
    });

    board.footprints.push(fp);

    // 4. Bottom Copper Zone
    board.zones.push(Zone {
        uuid: Uuid::now_v7(),
        net: 2,
        net_name: "GND".to_string(),
        layer: "B.Cu".to_string(),
        outline: vec![
            Point::new(0.0, 0.0),
            Point::new(80.0, 0.0),
            Point::new(80.0, 80.0),
            Point::new(0.0, 80.0),
        ],
        priority: 0,
        fill_type: "solid".to_string(),
        thermal_relief: true,
        thermal_gap: 0.5,
        thermal_width: 0.25,
        clearance: 0.2,
        min_thickness: 0.25,
    });

    board
}

#[test]
fn test_dxf_r12_export_structure() {
    let board = create_sample_board();
    let exporter = DxfExporter::new(DxfOptions::default());
    let dxf_content = exporter.export_board(&board).expect("export dxf");

    // Verify AutoCAD R12 ASCII DXF standard sections
    assert!(dxf_content.contains("$ACADVER\n  1\nAC1009"));
    assert!(dxf_content.contains("$INSUNITS\n 70\n4")); // Metric mm
    assert!(dxf_content.contains("SECTION\n  2\nTABLES"));
    assert!(dxf_content.contains("LAYER\n  2\nF_CU"));
    assert!(dxf_content.contains("LAYER\n  2\nB_CU"));
    assert!(dxf_content.contains("LAYER\n  2\nDRILL"));
    assert!(dxf_content.contains("SECTION\n  2\nENTITIES"));
    assert!(dxf_content.contains("LINE\n  8\nF_CU"));
    assert!(dxf_content.contains("LINE\n  8\nB_CU"));
    assert!(dxf_content.contains("CIRCLE\n  8\nDRILL"));
    assert!(dxf_content.contains("SOLID\n  8\nF_CU"));
    assert!(dxf_content.contains("POLYLINE\n  8\nB_CU"));
    assert!(dxf_content.contains("TEXT\n  8\nF_SILK\n 10\n"));
    assert!(dxf_content.contains("U1"));
    assert!(dxf_content.ends_with("  0\nEOF\n"));
}

#[test]
fn test_dwg_r12_binary_export() {
    let board = create_sample_board();
    let exporter = DwgExporter::new(DwgOptions {
        version: DwgVersion::R12Ac1009,
        ..Default::default()
    });
    let dwg_bytes = exporter.export_dwg(&board).expect("export dwg");

    // Verify AC1009 magic header bytes
    assert_eq!(&dwg_bytes[0..6], b"AC1009");
    assert_eq!(&dwg_bytes[6..8], &[0x00, 0x00]);
    assert!(dwg_bytes.len() > 100);
}

#[test]
fn test_dwt_drawing_template_export() {
    let board = create_sample_board();
    let exporter = DwgExporter::new(DwgOptions {
        version: DwgVersion::R12Ac1009,
        is_template: true,
        title: "Test Motor Driver PCB".to_string(),
        revision: "2.1".to_string(),
        author: "Oxide Engineering".to_string(),
        ..Default::default()
    });
    let dwt_bytes = exporter.export_dwt(&board).expect("export dwt");

    // Verify Drawing Template header and title block presence
    assert_eq!(&dwt_bytes[0..6], b"AC1009");
    assert!(dwt_bytes.len() > 200);
}

#[test]
fn test_cbr_copper_bottom_routing_export() {
    let board = create_sample_board();
    let exporter = CbrExporter::new(CbrOptions {
        title: "Motor Driver Bottom Routing".to_string(),
        ..Default::default()
    });
    let cbr_content = exporter.export_bottom_copper(&board).expect("export cbr");

    // Verify RS-274X Photoplotter syntax
    assert!(cbr_content.contains("G04 File: CBR (Copper Bottom Routing)"));
    assert!(cbr_content.contains("%FSLAX24Y24*%"));
    assert!(cbr_content.contains("%MOMM*%"));
    assert!(cbr_content.contains("%LPD*%"));
    assert!(cbr_content.contains("%ADD10C,")); // Aperture definition
    assert!(cbr_content.contains("D02*")); // Move
    assert!(cbr_content.contains("D01*")); // Draw
    assert!(cbr_content.contains("D03*")); // Flash pad/via
    assert!(cbr_content.contains("G36*")); // Start polygon fill
    assert!(cbr_content.contains("G37*")); // End polygon fill
    assert!(cbr_content.ends_with("M02*\n"));
}

#[test]
fn test_cdr_coreldraw_export() {
    let board = create_sample_board();
    let exporter = CdrExporter::new(CdrOptions {
        version: CdrVersion::V3_0,
        title: "Motor Driver Silkscreen & Layers".to_string(),
        ..Default::default()
    });
    let cdr_bytes = exporter.export_board(&board).expect("export cdr");

    // Verify RIFF CDR binary container structure
    assert_eq!(&cdr_bytes[0..4], b"RIFF");
    assert_eq!(&cdr_bytes[8..12], b"CDR ");
    assert!(cdr_bytes.windows(4).any(|w| w == b"vrsn"));
    assert!(cdr_bytes.windows(4).any(|w| w == b"info"));
    assert!(cdr_bytes.windows(4).any(|w| w == b"layr"));
    assert!(cdr_bytes.windows(4).any(|w| w == b"oblt"));
    assert!(cdr_bytes.len() > 200);
}
