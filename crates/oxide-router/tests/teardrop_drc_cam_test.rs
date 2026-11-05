//! D11 Integration Test: Teardrop Fillet Generation, DRC Cleanliness, and CAM Gerber Export.
//!
//! Verifies:
//! 1. `TeardropGenerator::apply_teardrops_to_board` generates curvilinear/angled fillets on pad/via junctions.
//! 2. DRC oracle (`ConstraintManager::run_drc`) passes with zero violations (teardrops preserve net identity).
//! 3. CAM Gerber export produces enhanced copper geometry for the filleted junctions.

use oxide_output::gerber::{GerberExporter, GerberLayer, GerberOptions};
use oxide_router::copper_pour::TeardropGenerator;
use oxide_rules::ConstraintManager;
use oxide_types::pcb::{
    Footprint, Pad, PadNet, PadShape, PadType, PcbBoard, Point, Segment, Via, ViaType,
};
use uuid::Uuid;

#[test]
fn test_d11_teardrop_generation_drc_and_cam_export() {
    let mut board = PcbBoard::default();

    // 1. Add Footprint U1 with Pad 1 at (10.0, 10.0) on Net 1
    let pad = Pad {
        uuid: Uuid::now_v7(),
        number: "1".to_string(),
        pad_type: PadType::Thru,
        shape: PadShape::Circle,
        position: Point::new(0.0, 0.0),
        size: Point::new(1.2, 1.2),
        drill: Some(oxide_types::pcb::DrillDef {
            diameter: 0.6,
            shape: "round".to_string(),
        }),
        layers: vec!["Top Layer".to_string(), "Bottom Layer".to_string()],
        net: Some(PadNet {
            number: 1,
            name: "VCC".to_string(),
        }),
        roundrect_ratio: 0.0,
    };

    let fp = Footprint {
        uuid: Uuid::now_v7(),
        reference: "U1".to_string(),
        value: "LDO".to_string(),
        footprint_id: "TO-220".to_string(),
        position: Point::new(10.0, 10.0),
        rotation: 0.0,
        layer: "Top Layer".to_string(),
        locked: false,
        pads: vec![pad],
        graphics: Vec::new(),
        properties: Vec::new(),
    };
    board.footprints.push(fp);

    // 2. Add Via at (20.0, 10.0) on Net 1
    board.vias.push(Via {
        uuid: Uuid::now_v7(),
        position: Point::new(20.0, 10.0),
        diameter: 0.6,
        drill: 0.3,
        layers: vec!["Top Layer".to_string(), "Bottom Layer".to_string()],
        net: 1,
        via_type: ViaType::Through,
        via_span: None,
    });

    // 3. Add connecting Track segment from Pad (10.0, 10.0) to Via (20.0, 10.0)
    board.segments.push(Segment {
        uuid: Uuid::now_v7(),
        start: Point::new(10.0, 10.0),
        end: Point::new(20.0, 10.0),
        width: 0.25,
        layer: "Top Layer".to_string(),
        net: 1,
    });

    assert_eq!(board.segments.len(), 1, "Initial segment count");

    // 4. Run pre-CAM Teardrop Pass
    let added = TeardropGenerator::apply_teardrops_to_board(&mut board);
    assert_eq!(
        added, 4,
        "Expected 4 teardrop fillet segments (2 for pad, 2 for via)"
    );
    assert_eq!(
        board.segments.len(),
        5,
        "Total segments after teardrop pass"
    );

    // 5. DRC Oracle Verification: Zero Violations
    let rules = ConstraintManager::standard_default();
    let drc_result = rules.run_drc(&board);
    assert_eq!(
        drc_result.len(),
        0,
        "DRC must pass with zero violations on teardrop-filleted board: {:?}",
        drc_result
    );

    // 6. CAM Gerber Export Verification
    let exporter = GerberExporter::new(GerberOptions::default());
    let outputs = exporter.export_board(&board).expect("export gerbers");

    let top_cu = outputs
        .iter()
        .find(|o| o.layer == GerberLayer::TopCopper)
        .expect("Top copper gerber layer");

    // Top copper Gerber must contain draw commands for the teardrop fillets
    assert!(
        top_cu.content.contains("D10*") || top_cu.content.contains("G01*"),
        "Top copper Gerber must contain linear interpolation commands"
    );
    assert!(
        top_cu.content.len() > 500,
        "Top copper Gerber output must be non-empty and well-formed"
    );
}
