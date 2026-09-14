//! Integration tests for oxide-router: Spatial Index, A*, Triangulation, Glossing, and Workflow.

use std::sync::Arc;
use uuid::Uuid;

use oxide_rules::ConstraintManager;
use oxide_types::pcb::{PcbBoard, Segment};
use oxide_types::schematic::Point;

use oxide_router::geometry::{BoundingBox, Point2D};
use oxide_router::interactive::InteractiveRouter;
use oxide_router::optimization::OptimizationEngine;
use oxide_router::topology::TopologicalAutorouter;
use oxide_router::{
    RouteSegment, RoutingMode, RoutingPath, RoutingResult, RoutingWorkflow, SegmentType,
    SpatialIndex, SpatialObjectType,
};

fn mock_board() -> PcbBoard {
    let mut board = PcbBoard {
        uuid: Uuid::new_v4(),
        version: 1,
        generator: "oxide".to_string(),
        thickness: 1.6,
        outline: vec![
            Point::new(0.0, 0.0),
            Point::new(100.0, 0.0),
            Point::new(100.0, 100.0),
            Point::new(0.0, 100.0),
        ],
        layers: Vec::new(),
        setup: None,
        nets: Vec::new(),
        footprints: Vec::new(),
        segments: Vec::new(),
        vias: Vec::new(),
        zones: Vec::new(),
        graphics: Vec::new(),
        texts: Vec::new(),
    };

    // Add track obstacle
    board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        start: Point::new(15.0, 10.0),
        end: Point::new(15.0, 20.0),
        width: 0.25,
        layer: "F.Cu".to_string(),
        net: 99,
    });

    board
}

#[test]
fn test_spatial_index_queries() {
    let board = mock_board();
    let index = SpatialIndex::build(&board);

    assert!(index.count() >= 1);

    let query_box = BoundingBox::new(Point2D::from_mm(14.0, 9.0), Point2D::from_mm(16.0, 21.0));
    let results = index.query_bbox(&query_box);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].object_type, SpatialObjectType::Track);

    // Collision check excluding net 99
    let collisions_excluded = index.check_collision(&query_box, &[99]);
    assert_eq!(collisions_excluded.len(), 0);

    // Collision check without excluding net 99
    let collisions = index.check_collision(&query_box, &[]);
    assert_eq!(collisions.len(), 1);
}

#[test]
fn test_interactive_astar_routing() {
    let board = mock_board();
    let rules = Arc::new(ConstraintManager::standard_default());
    let mut router =
        InteractiveRouter::new(Arc::clone(&rules), Arc::new(SpatialIndex::build(&board)));

    let start = Point2D::from_mm(10.0, 15.0);
    let target = Point2D::from_mm(20.0, 15.0);

    let result = router.route_net(1, start, target);
    match result {
        RoutingResult::Success(path) => {
            assert_eq!(path.net_id, 1);
            assert!(!path.segments.is_empty());
            assert!(path.total_length > 9_000);
        }
        _ => panic!("Expected successful route"),
    }
}

#[test]
fn test_differential_pair_and_meander() {
    let board = mock_board();
    let rules = Arc::new(ConstraintManager::standard_default());
    let mut router =
        InteractiveRouter::new(Arc::clone(&rules), Arc::new(SpatialIndex::build(&board)));

    router.mode = RoutingMode::DifferentialPair;
    let _ = router.start_routing(Point2D::from_mm(10.0, 10.0), 1, 0);
    let diff_segments = router.on_mouse_move(Point2D::from_mm(20.0, 10.0));

    assert!(!diff_segments.is_empty());
    // In diff pair mode, there should be pairs of segments (positive and negative tracks)
    assert_eq!(diff_segments.len() % 2, 0);

    // Meander / length tuning test
    router.mode = RoutingMode::LengthTuning;
    let meander_segments = router.on_mouse_move(Point2D::from_mm(20.0, 10.0));
    assert!(meander_segments.len() >= 3);
}

#[test]
fn test_cascade_push_and_shove() {
    let board = mock_board();
    let rules = Arc::new(ConstraintManager::standard_default());
    let router =
        InteractiveRouter::new(Arc::clone(&rules), Arc::new(SpatialIndex::build(&board)));

    let start = Point2D::from_mm(10.0, 15.0);
    let end = Point2D::from_mm(20.0, 15.0);

    let (segments, pushes) = router.push_and_shove_with_displacements(start, end, 1, 0, 200);

    assert_eq!(segments.len(), 1);
    // Obstacle at (15.0, 10..20) should be detected and pushed
    assert!(!pushes.is_empty());
    assert!(pushes[0].displacement > 0);
}

#[test]
fn test_topological_triangulation_and_autoroute() {
    let mut board = mock_board();
    let rules = Arc::new(ConstraintManager::standard_default());
    let spatial = Arc::new(SpatialIndex::build(&board));
    let mut autorouter = TopologicalAutorouter::new(rules, spatial);

    let _ = autorouter.build_topological_map(&board);
    assert!(!autorouter.topological_map.triangulation.triangles.is_empty());

    let results = autorouter.route_board(&mut board, &[1, 2]);
    assert_eq!(results.len(), 2);
}

#[test]
fn test_optimization_glossing_and_loop_removal() {
    let rules = Arc::new(ConstraintManager::standard_default());
    let optimizer = OptimizationEngine::new(rules);
    let mut path = RoutingPath {
        net_id: 1,
        segments: vec![
            RouteSegment {
                start_point: Point2D::from_mm(0.0, 0.0),
                end_point: Point2D::from_mm(5.0, 0.0),
                width: 200,
                layer: 0,
                net_id: 1,
                segment_type: SegmentType::Straight,
            },
            RouteSegment {
                start_point: Point2D::from_mm(5.0, 0.0),
                end_point: Point2D::from_mm(10.0, 0.0),
                width: 200,
                layer: 0,
                net_id: 1,
                segment_type: SegmentType::Straight,
            },
        ],
        vias: Vec::new(),
        total_length: 10_000,
        layer_transitions: Vec::new(),
    };

    let gloss_res = optimizer.glossing.gloss_path(&mut path);
    assert_eq!(gloss_res.corners_removed, 1);
    assert_eq!(path.segments.len(), 1);
    assert_eq!(path.segments[0].start_point, Point2D::from_mm(0.0, 0.0));
    assert_eq!(path.segments[0].end_point, Point2D::from_mm(10.0, 0.0));
}

#[test]
fn test_end_to_end_routing_workflow() {
    let board = mock_board();
    let rules = Arc::new(ConstraintManager::standard_default());

    let mut workflow = RoutingWorkflow::new(rules, board, vec![1, 2, 3]);
    let result = workflow.execute();

    assert_eq!(result.total_nets, 3);
    assert_eq!(result.routed_nets, 3);
    assert_eq!(result.failed_nets, 0);
    assert!(result.total_length > 0);
}

#[test]
fn test_ml_guided_astar_routing() {
    let board = mock_board();
    let spatial = SpatialIndex::build(&board);
    let start = Point2D::from_mm(10.0, 15.0);
    let target = Point2D::from_mm(20.0, 15.0);

    let ml_engine = oxide_ml::MlEngine::new(oxide_ml::MlConfig::default());
    let mut advisor = ml_engine.create_routing_advisor(4);

    let path = oxide_router::interactive::astar::find_astar_path_with_ml(
        &spatial,
        start,
        target,
        1,
        200,
        Some(&mut advisor),
    );

    assert!(path.len() >= 2);
    assert_eq!(path[0], start);
    assert_eq!(*path.last().unwrap(), target);
}
