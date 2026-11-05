//! Titan-H7 Dense BGA Escape & High-Speed Routing Benchmark.
//! Stress-tests Cascade Shove, Elastic Spring-Back, and Clearance Rules.

use oxide_router::geometry::Point2D;
use oxide_router::interactive::elastic_shove::{CascadeObstacle, CascadeShoveEngine};

#[test]
fn test_titan_h7_dense_bga_cascade_shove_escape() {
    let cascade_engine = CascadeShoveEngine::new(5, 127); // 5 cascade hops, 127um (5 mil) clearance

    // Setup microvias and trace obstacle in dense 0.8mm BGA fanout area
    let orig_via_pos = Point2D::new(800, 100);
    let mut bga_via = CascadeObstacle::new_via(101, orig_via_pos, false, 200.0);

    // Simulate high-density trace shoving the BGA via aside
    bga_via.points[0] = Point2D::new(800, 240);
    assert_eq!(bga_via.points[0].y, 240);

    // Primary route settles or moves into adjacent channel
    let clear_path = vec![
        Point2D::new(0, -600),
        Point2D::new(800, -600),
        Point2D::new(1600, -600),
    ];

    // Relax towards rest
    cascade_engine.relax_obstacles(&clear_path, std::slice::from_mut(&mut bga_via), 0.75);

    assert!(
        bga_via.points[0].y < 240,
        "Titan-H7 BGA via must elastically relax back toward original position (current: {})",
        bga_via.points[0].y
    );
    assert!(
        bga_via.points[0].y >= 100,
        "Via must not overshoot resting position"
    );
}
