use oxide_router::optimization::xsignals::{MatchedGroup, XSignal};

#[test]
fn test_xsignals_delay_and_matched_group_evaluation() {
    let mut sig1 = XSignal::new("DDR5_DQ0", vec![1, 2]);
    sig1.total_length = 45_000; // 45mm
    sig1.package_delay_ps = 5.0;

    let mut sig2 = XSignal::new("DDR5_DQ1", vec![3, 4]);
    sig2.total_length = 45_200; // 45.2mm
    sig2.package_delay_ps = 4.0;

    let mut group = MatchedGroup::new("BYTE_0", 500); // 500µm tolerance
    group.add_signal(sig1);
    group.add_signal(sig2);

    let status = group.evaluate_status(6.5); // 6.5 ps/mm in FR4
    assert!(status.within_tolerance);
    assert_eq!(status.delta, 200);
    assert!((status.skew_ps - 1.3).abs() < 0.01);
}

#[test]
fn test_via_shoving_and_multi_cycle_accordion_tuning() {
    use std::sync::Arc;
    use oxide_rules::ConstraintManager;
    use oxide_router::geometry::{Point2D, BoundingBox};
    use oxide_router::geometry::rtree::{SpatialObject, SpatialObjectType};
    use oxide_router::interactive::conflict::calculate_push;
    use oxide_router::optimization::length_tuning::LengthTuningOptimizer;
    use oxide_router::{RouteSegment, RoutingPath, SegmentType};
    use uuid::Uuid;

    // 1. Verify Via Shoving gives appropriate buffer vs track
    let via_obs = SpatialObject {
        id: Uuid::new_v4(),
        bbox: BoundingBox::new(Point2D::new(1000, 1000), Point2D::new(1600, 1600)),
        object_type: SpatialObjectType::Via,
        net_id: Some(1),
        layer: 0,
    };
    let start = Point2D::new(0, 1300);
    let end = Point2D::new(3000, 1300);

    let push = calculate_push(&via_obs, start, end, 150).expect("Via in path should be pushed");
    assert!(push.is_via);
    assert_eq!(push.displacement, 300); // 150 clearance + 150 via buffer

    // 2. Verify Multi-cycle Accordion Length Tuning
    let rules = Arc::new(ConstraintManager::standard_default());
    let optimizer = LengthTuningOptimizer::new(rules);

    let mut pos_path = RoutingPath {
        net_id: 10,
        segments: vec![RouteSegment {
            start_point: Point2D::new(0, 0),
            end_point: Point2D::new(5000, 0),
            width: 200,
            layer: 0,
            net_id: 10,
            segment_type: SegmentType::Straight,
        }],
        vias: Vec::new(),
        total_length: 5000,
        layer_transitions: Vec::new(),
    };

    let mut neg_path = RoutingPath {
        net_id: 11,
        segments: vec![RouteSegment {
            start_point: Point2D::new(0, 200),
            end_point: Point2D::new(3500, 200),
            width: 200,
            layer: 0,
            net_id: 11,
            segment_type: SegmentType::Straight,
        }],
        vias: Vec::new(),
        total_length: 3500,
        layer_transitions: Vec::new(),
    };

    let result = optimizer.tune_differential_pair(&mut pos_path, &mut neg_path);
    assert!(neg_path.total_length > 3500, "Negative path should have meanders inserted");
    assert!(neg_path.segments.len() > 1, "Accordion must generate multiple segments");
    assert!(result.length_difference < 1000, "Length difference should be significantly narrowed");
}

