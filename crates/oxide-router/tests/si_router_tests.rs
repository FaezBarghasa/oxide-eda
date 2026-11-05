//! Integration test for Phase 5.2: Field Solver in the Loop for Interactive Router.
//!
//! Validates that `InteractiveRouter` dynamically queries `oxide-physics` to synthesize:
//! 1. Single-ended track widths matching target characteristic impedance $Z_0$.
//! 2. Edge-coupled differential pair track spacing matching target differential impedance $Z_{\text{diff}}$.

use std::sync::Arc;
use uuid::Uuid;

use oxide_rules::ConstraintManager;
use oxide_types::pcb::PcbBoard;

use oxide_router::geometry::Point2D;
use oxide_router::interactive::InteractiveRouter;
use oxide_router::{RoutingMode, SpatialIndex};

fn create_test_board() -> PcbBoard {
    let mut board = PcbBoard::default();
    board.uuid = Uuid::new_v4();
    board.version = 1;
    board.generator = "oxide".to_string();
    board.thickness = 1.6;
    board
}

#[test]
fn test_router_single_ended_impedance_synthesis() {
    let board = create_test_board();
    let rules = Arc::new(ConstraintManager::standard_default());
    let mut router =
        InteractiveRouter::new(Arc::clone(&rules), Arc::new(SpatialIndex::build(&board)));

    let net_50 = 101;
    let net_60 = 102;
    let unconstrained_net = 999;

    // 1. Unconstrained net uses default track width (200 µm)
    assert_eq!(router.get_track_width_for_net(unconstrained_net), 200);

    // 2. Set target impedance to 50 Ohm (DDR5 / RF single-ended)
    // On 100 µm dielectric (FR-4 er=4.3, 35 µm copper), 50 Ohm evaluates to ~145-180 µm
    router.set_net_target_impedance(net_50, 50.0);
    let width_50 = router.get_track_width_for_net(net_50);

    // 3. Set target impedance to 60 Ohm
    router.set_net_target_impedance(net_60, 60.0);
    let width_60 = router.get_track_width_for_net(net_60);

    // Physical law: higher impedance -> narrower trace width
    assert!(
        width_60 < width_50,
        "60 Ohm trace ({width_60} µm) must be narrower than 50 Ohm trace ({width_50} µm)"
    );

    // Verify calculated impedance with physics engine
    let z0_calc = oxide_physics::ImpedanceCalculator::calculate_microstrip(width_50, 100, 35, 4.3);
    let diff = (z0_calc - 50.0).abs();
    assert!(
        diff < 1.0,
        "Synthesized width {width_50} µm should yield ~50 Ohm, got {z0_calc:.2} Ohm"
    );
}

#[test]
fn test_router_differential_impedance_pair_routing() {
    let board = create_test_board();
    let rules = Arc::new(ConstraintManager::standard_default());
    let mut router =
        InteractiveRouter::new(Arc::clone(&rules), Arc::new(SpatialIndex::build(&board)));

    let diff_net = 201;

    // Target 100 Ohm differential impedance (PCIe / Ethernet)
    router.set_net_target_diff_impedance(diff_net, 100.0);

    // Start differential pair routing
    router.mode = RoutingMode::DifferentialPair;
    let start = Point2D::from_mm(10.0, 10.0);
    let target = Point2D::from_mm(30.0, 10.0);

    router
        .start_routing(start, diff_net, 0)
        .expect("start differential routing");
    let diff_segments = router.on_mouse_move(target);

    assert!(
        !diff_segments.is_empty(),
        "Differential routing must yield segment pairs"
    );
    assert_eq!(
        diff_segments.len() % 2,
        0,
        "Segments must come in matched pairs"
    );

    // Verify distance between complementary positive and negative segments
    let seg_p = &diff_segments[0];
    let seg_n = &diff_segments[1];
    let center_to_center = (seg_p.start_point.y - seg_n.start_point.y).abs();
    let edge_to_edge_gap = center_to_center - router.get_track_width_for_net(diff_net);

    println!(
        "Differential Pair Routing: Target Z_diff=100 Ohm -> Synthesized Edge Gap = {} µm (Center-to-Center = {} µm)",
        edge_to_edge_gap, center_to_center
    );

    assert!(
        edge_to_edge_gap >= 50 && edge_to_edge_gap <= 400,
        "Synthesized differential edge gap {edge_to_edge_gap} µm out of expected range"
    );
}
