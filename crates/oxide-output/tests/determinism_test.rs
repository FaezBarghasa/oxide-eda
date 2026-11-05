//! Determinism Verification Test (Phase 1.4).
//!
//! Verifies that multiple successive runs of CAM export (Gerber RS-274X, Drill, IPC-2581, ODB++)
//! on the exact same board geometry produce byte-identical outputs.
//! Neither KiCad nor Altium guarantee deterministic byte-for-byte outputs across runs due to timestamps.
//! Oxide EDA guarantees deterministic reproducibility for Hardware-as-Code CI/CD.

use oxide_output::gerber::{GerberExporter, GerberOptions};
use oxide_output::{Ipc2581Options, export_ipc2581, export_odbpp_package};
use oxide_types::pcb::{
    Footprint, LayerDef, NetDef, Pad, PadNet, PadShape, PadType, PcbBoard, Point, Segment, Via,
    ViaType,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

fn create_deterministic_test_board() -> PcbBoard {
    let mut board = PcbBoard {
        uuid: Uuid::from_u128(0x018f401a_1234_7000_8000_000000000001),
        version: 1,
        generator: "Oxide Determinism Oracle Test".to_string(),
        thickness: 1.6,
        outline: vec![
            Point::new(0.0, 0.0),
            Point::new(100.0, 0.0),
            Point::new(100.0, 80.0),
            Point::new(0.0, 80.0),
            Point::new(0.0, 0.0),
        ],
        layers: vec![
            LayerDef {
                id: 0,
                name: "Top Layer".to_string(),
                layer_type: "signal".to_string(),
            },
            LayerDef {
                id: 1,
                name: "In1.GND".to_string(),
                layer_type: "plane".to_string(),
            },
            LayerDef {
                id: 2,
                name: "Bottom Layer".to_string(),
                layer_type: "signal".to_string(),
            },
        ],
        setup: None,
        nets: vec![
            NetDef {
                number: 1,
                name: "GND".to_string(),
            },
            NetDef {
                number: 2,
                name: "+3V3".to_string(),
            },
            NetDef {
                number: 3,
                name: "SPI_SCK".to_string(),
            },
        ],
        footprints: Vec::new(),
        segments: Vec::new(),
        vias: Vec::new(),
        graphics: Vec::new(),
        texts: Vec::new(),
        zones: Vec::new(),
    };

    // Deterministic U1 Footprint with static UUIDs
    let pad1 = Pad {
        uuid: Uuid::from_u128(0x018f401a_1234_7000_8000_000000000010),
        number: "1".to_string(),
        pad_type: PadType::Smd,
        shape: PadShape::Rect,
        position: Point::new(-1.0, 0.0),
        size: Point::new(0.8, 1.2),
        drill: None,
        layers: vec!["Top Layer".to_string()],
        net: Some(PadNet {
            number: 1,
            name: "GND".to_string(),
        }),
        roundrect_ratio: 0.0,
    };
    let pad2 = Pad {
        uuid: Uuid::from_u128(0x018f401a_1234_7000_8000_000000000011),
        number: "2".to_string(),
        pad_type: PadType::Smd,
        shape: PadShape::Rect,
        position: Point::new(1.0, 0.0),
        size: Point::new(0.8, 1.2),
        drill: None,
        layers: vec!["Top Layer".to_string()],
        net: Some(PadNet {
            number: 2,
            name: "+3V3".to_string(),
        }),
        roundrect_ratio: 0.0,
    };

    board.footprints.push(Footprint {
        uuid: Uuid::from_u128(0x018f401a_1234_7000_8000_000000000020),
        reference: "C1".to_string(),
        value: "100nF".to_string(),
        footprint_id: "Capacitor_SMD:C_0603".to_string(),
        position: Point::new(25.0, 25.0),
        rotation: 0.0,
        layer: "Top Layer".to_string(),
        locked: false,
        pads: vec![pad1, pad2],
        graphics: Vec::new(),
        properties: Vec::new(),
    });

    // Deterministic segments
    board.segments.push(Segment {
        uuid: Uuid::from_u128(0x018f401a_1234_7000_8000_000000000030),
        start: Point::new(26.0, 25.0),
        end: Point::new(50.0, 25.0),
        width: 0.25,
        layer: "Top Layer".to_string(),
        net: 2,
    });
    board.segments.push(Segment {
        uuid: Uuid::from_u128(0x018f401a_1234_7000_8000_000000000031),
        start: Point::new(10.0, 40.0),
        end: Point::new(80.0, 40.0),
        width: 0.15,
        layer: "Bottom Layer".to_string(),
        net: 3,
    });

    // Deterministic via
    board.vias.push(Via {
        uuid: Uuid::from_u128(0x018f401a_1234_7000_8000_000000000040),
        position: Point::new(50.0, 25.0),
        diameter: 0.60,
        drill: 0.30,
        layers: vec!["Top Layer".to_string(), "In1.GND".to_string()],
        net: 1,
        via_type: ViaType::Through,
        via_span: None,
    });

    board
}

fn content_hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn test_gerber_export_determinism_identical_output() {
    let board = create_deterministic_test_board();
    let exporter = GerberExporter::new(GerberOptions::default());

    let run1 = exporter.export_board(&board).expect("run 1 export");
    let run2 = exporter.export_board(&board).expect("run 2 export");

    assert_eq!(run1.len(), run2.len(), "Layer count must match");
    assert!(!run1.is_empty(), "Must export layers");

    for (layer1, layer2) in run1.iter().zip(run2.iter()) {
        assert_eq!(layer1.layer, layer2.layer);
        let hash1 = content_hash(&layer1.content);
        let hash2 = content_hash(&layer2.content);
        assert_eq!(
            hash1, hash2,
            "Gerber layer {:?} must have identical hash across runs",
            layer1.layer
        );
        assert_eq!(
            layer1.content, layer2.content,
            "Gerber layer {:?} byte content must be identical",
            layer1.layer
        );
    }
}

#[test]
fn test_ipc2581_export_determinism_identical_output() {
    let board = create_deterministic_test_board();
    let options = Ipc2581Options::default();

    let out1 = export_ipc2581(&board, &options).expect("run 1 ipc2581 export");
    let out2 = export_ipc2581(&board, &options).expect("run 2 ipc2581 export");

    assert_eq!(out1.filename, out2.filename);
    assert_eq!(
        content_hash(&out1.xml_content),
        content_hash(&out2.xml_content),
        "IPC-2581 XML must be byte-identical with matching hash across runs"
    );
    assert_eq!(out1.xml_content, out2.xml_content);
}

#[test]
fn test_odbpp_export_determinism_identical_output() {
    let board = create_deterministic_test_board();

    let odb1 = export_odbpp_package(&board).expect("run 1 odbpp export");
    let odb2 = export_odbpp_package(&board).expect("run 2 odbpp export");

    assert_eq!(odb1.files.len(), odb2.files.len());
    for (f1, f2) in odb1.files.iter().zip(odb2.files.iter()) {
        assert_eq!(f1.relative_path, f2.relative_path);
        assert_eq!(
            content_hash(&f1.content),
            content_hash(&f2.content),
            "ODB++ file {} must be identical across runs",
            f1.relative_path
        );
        assert_eq!(f1.content, f2.content);
    }
}
