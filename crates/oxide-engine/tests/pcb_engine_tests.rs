use oxide_engine::pcb::{PcbCommand, PcbEngine};
use oxide_types::pcb::{Footprint, PcbBoard, Point, Segment, Via, ViaType};
use uuid::Uuid;

#[test]
fn test_pcb_engine_footprint_lifecycle_and_undo() {
    let board = PcbBoard::default();
    let mut engine = PcbEngine::new(board);
    assert_eq!(engine.generation(), 1);
    assert!(!engine.can_undo());

    let fp_id = Uuid::new_v4();
    let fp = Footprint {
        uuid: fp_id,
        reference: "R1".to_string(),
        value: "10k".to_string(),
        footprint_id: "Resistor_SMD:R_0805".to_string(),
        position: Point::new(10.0, 20.0),
        rotation: 0.0,
        layer: "F.Cu".to_string(),
        locked: false,
        properties: Vec::new(),
        pads: Vec::new(),
        graphics: Vec::new(),
    };

    // 1. Place footprint
    engine
        .execute(PcbCommand::PlaceFootprint { footprint: fp })
        .unwrap();
    assert_eq!(engine.board().footprints.len(), 1);
    assert!(engine.can_undo());

    // 2. Move footprint
    engine
        .execute(PcbCommand::MoveFootprint {
            uuid: fp_id,
            dx: 5.0,
            dy: -2.0,
        })
        .unwrap();
    let moved_fp = &engine.board().footprints[0];
    assert_eq!(moved_fp.position.x, 15.0);
    assert_eq!(moved_fp.position.y, 18.0);

    // 3. Rotate footprint
    engine
        .execute(PcbCommand::RotateFootprint {
            uuid: fp_id,
            delta_deg: 90.0,
        })
        .unwrap();
    assert_eq!(engine.board().footprints[0].rotation, 90.0);

    // 4. Undo Rotate
    assert!(engine.undo().unwrap());
    assert_eq!(engine.board().footprints[0].rotation, 0.0);

    // 5. Undo Move
    assert!(engine.undo().unwrap());
    assert_eq!(engine.board().footprints[0].position.x, 10.0);
    assert_eq!(engine.board().footprints[0].position.y, 20.0);

    // 6. Undo Place
    assert!(engine.undo().unwrap());
    assert_eq!(engine.board().footprints.len(), 0);
    assert!(!engine.can_undo());
    assert!(engine.can_redo());

    // 7. Redo Place
    assert!(engine.redo().unwrap());
    assert_eq!(engine.board().footprints.len(), 1);
}

#[test]
fn test_pcb_engine_routing_segments_and_vias() {
    let board = PcbBoard::default();
    let mut engine = PcbEngine::new(board);

    let seg_id = Uuid::new_v4();
    let seg = Segment {
        uuid: seg_id,
        start: Point::new(0.0, 0.0),
        end: Point::new(10.0, 10.0),
        width: 0.25,
        layer: "F.Cu".to_string(),
        net: 1,
    };

    let via_id = Uuid::new_v4();
    let via = Via {
        uuid: via_id,
        position: Point::new(10.0, 10.0),
        diameter: 0.6,
        drill: 0.3,
        layers: vec!["F.Cu".to_string(), "B.Cu".to_string()],
        net: 1,
        via_type: ViaType::Through,
        via_span: None,
    };

    engine
        .execute(PcbCommand::AddSegment { segment: seg })
        .unwrap();
    engine.execute(PcbCommand::AddVia { via }).unwrap();

    assert_eq!(engine.board().segments.len(), 1);
    assert_eq!(engine.board().vias.len(), 1);

    // Delete segment
    engine
        .execute(PcbCommand::DeleteSegment { uuid: seg_id })
        .unwrap();
    assert_eq!(engine.board().segments.len(), 0);

    // Undo delete segment
    engine.undo().unwrap();
    assert_eq!(engine.board().segments.len(), 1);
    assert_eq!(engine.board().segments[0].uuid, seg_id);
}

#[test]
fn test_pcb_engine_hit_test_and_move_selection() {
    use oxide_engine::pcb::SelectedPcbKind;

    let board = PcbBoard::default();
    let mut engine = PcbEngine::new(board);

    let fp_id = Uuid::new_v4();
    let fp = Footprint {
        uuid: fp_id,
        reference: "U1".to_string(),
        value: "MCU".to_string(),
        footprint_id: "QFP:LQFP-48".to_string(),
        position: Point::new(50.0, 50.0),
        rotation: 0.0,
        layer: "F.Cu".to_string(),
        locked: false,
        properties: Vec::new(),
        pads: Vec::new(),
        graphics: Vec::new(),
    };
    engine
        .execute(PcbCommand::PlaceFootprint { footprint: fp })
        .unwrap();

    // Hit test footprint
    let hit = engine.hit_test(50.5, 49.8).expect("should hit footprint");
    assert_eq!(hit.uuid, fp_id);
    assert_eq!(hit.kind, SelectedPcbKind::Footprint);

    // Hit test empty space
    assert!(engine.hit_test(100.0, 100.0).is_none());

    // Selection tracking
    engine.set_selection(vec![hit.clone()]);
    assert_eq!(engine.selected_items().len(), 1);

    // Move selection
    engine
        .execute(PcbCommand::MoveSelection {
            items: vec![hit],
            dx: 10.0,
            dy: -5.0,
        })
        .unwrap();

    let moved_fp = &engine.board().footprints[0];
    assert_eq!(moved_fp.position.x, 60.0);
    assert_eq!(moved_fp.position.y, 45.0);

    // Undo move selection
    engine.undo().unwrap();
    let undone_fp = &engine.board().footprints[0];
    assert_eq!(undone_fp.position.x, 50.0);
    assert_eq!(undone_fp.position.y, 50.0);
}
