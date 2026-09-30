use oxide_output::{export_ipc2581, export_odbpp_package, Ipc2581Options};
use oxide_types::pcb::{Footprint, LayerDef, NetDef, PcbBoard, Point, Segment, Via, ViaType};
use uuid::Uuid;

fn sample_test_board() -> PcbBoard {
    PcbBoard {
        uuid: Uuid::now_v7(),
        version: 1,
        generator: "Oxide Output Test".to_string(),
        thickness: 1.6,
        outline: Vec::new(),
        layers: vec![
            LayerDef {
                id: 0,
                name: "Top Layer".to_string(),
                layer_type: "signal".to_string(),
            },
            LayerDef {
                id: 1,
                name: "Bottom Layer".to_string(),
                layer_type: "signal".to_string(),
            },
        ],
        setup: None,
        nets: vec![NetDef {
            number: 1,
            name: "GND".to_string(),
        }],
        footprints: vec![Footprint {
            uuid: Uuid::now_v7(),
            reference: "U1".to_string(),
            value: "STM32F401".to_string(),
            footprint_id: "Package_QFP:LQFP-64".to_string(),
            position: Point::new(50.0, 50.0),
            rotation: 0.0,
            layer: "Top Layer".to_string(),
            locked: false,
            pads: Vec::new(),
            graphics: Vec::new(),
            properties: Vec::new(),
        }],
        segments: vec![Segment {
            uuid: Uuid::now_v7(),
            start: Point::new(10.0, 10.0),
            end: Point::new(20.0, 10.0),
            width: 0.25,
            layer: "Top Layer".to_string(),
            net: 1,
        }],
        vias: vec![Via {
            uuid: Uuid::now_v7(),
            position: Point::new(20.0, 10.0),
            diameter: 0.6,
            drill: 0.3,
            layers: vec!["Top Layer".to_string(), "Bottom Layer".to_string()],
            net: 1,
            via_type: ViaType::Through,
            via_span: None,
        }],
        zones: Vec::new(),
        graphics: Vec::new(),
        texts: Vec::new(),
    }
}

#[test]
fn test_ipc2581c_export_generation() {
    let board = sample_test_board();
    let opts = Ipc2581Options::default();
    let res = export_ipc2581(&board, &opts).expect("ipc2581 export");

    assert!(res.xml_content.contains("IPC-2581"));
    assert!(res.xml_content.contains("STM32F401"));
    assert!(res.xml_content.contains("GND"));
}

#[test]
fn test_odbpp_export_generation() {
    let board = sample_test_board();
    let pkg = export_odbpp_package(&board).expect("odbpp export");

    assert!(!pkg.files.is_empty());
    assert!(pkg.files.iter().any(|f| f.relative_path == "matrix/matrix"));
    assert!(pkg.files.iter().any(|f| f.relative_path == "steps/step/netlists/cadnet/netlist"));
    assert!(pkg.files.iter().any(|f| f.relative_path == "steps/step/layers/comp_+_top/components"));
}
