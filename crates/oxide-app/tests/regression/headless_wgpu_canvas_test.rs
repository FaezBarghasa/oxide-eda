//! Component 10: Native Headless WGPU Canvas & Regression Test Harness.
//!
//! Validates headless WGPU rendering, canvas scene generation, PDN heatmaps,
//! and interactive routing gestures without requiring an active X11/Wayland display server.

use std::sync::Arc;

use oxide_app::pcb_canvas::{PcbCanvas, PcbCanvasState};
use oxide_renderer::pcb::PcbSnapshot;
use oxide_router::geometry::Point2D;
use oxide_router::interactive::async_solver::AsyncRoutingSolver;
use oxide_router::interactive::{CornerMode, InteractiveRouter, RoutingMode};
use oxide_rules::ConstraintManager;

#[test]
fn test_headless_wgpu_canvas_scene_and_pdn_overlay() {
    let mut canvas = PcbCanvas::new();
    let snapshot = PcbSnapshot::default();
    canvas.set_renderer_snapshot(Some(snapshot.clone()));

    // Verify baseline scene
    canvas.show_pdn_heatmap = false;
    let base_scene = canvas.build_scene(&snapshot);
    assert!(
        base_scene.overlay_polygons.is_empty(),
        "baseline scene has no overlay polygons"
    );

    // Enable PDN Heatmap and verify GPU composite scene generation
    canvas.show_pdn_heatmap = true;
    let heatmap_scene = canvas.build_scene(&snapshot);
    assert!(
        !heatmap_scene.overlay_polygons.is_empty(),
        "PDN heatmap must generate overlay polygons"
    );
    assert!(
        canvas.pdn_mesh_cache.borrow().is_some(),
        "PDN mesh grid must be cached"
    );

    // Verify alpha compositing values on overlay polygons
    for poly in &heatmap_scene.overlay_polygons {
        assert_eq!(
            poly.vertices.len(),
            4,
            "PDN heatmap quads must have 4 vertices"
        );
        assert_eq!(
            poly.fill_color[3], 0.45,
            "Overlay fill alpha must be 0.45 for copper blending"
        );
    }
}

#[test]
fn test_headless_async_routing_gesture_simulation() {
    let mut state = PcbCanvasState::default();
    let rules = Arc::new(ConstraintManager::standard_default());
    let spatial_index = Arc::new(oxide_router::geometry::rtree::SpatialIndex::new());
    let mut router = InteractiveRouter::new(rules, spatial_index);

    let start = Point2D::new(0, 0);
    router
        .start_routing(start, 1, 0)
        .expect("start routing on net 1, layer 0");

    let solver = AsyncRoutingSolver::spawn(router.clone());

    // Simulate rapid cursor drag across multiple points
    let drag_points = [
        Point2D::new(1000, 1000),
        Point2D::new(2000, 1500),
        Point2D::new(3500, 2000),
        Point2D::new(5000, 3000),
    ];

    for pt in &drag_points {
        solver.submit_target(
            *pt,
            RoutingMode::WalkAround,
            CornerMode::default(),
            1,
            0,
            200,
            1,
        );
    }

    // Wait briefly for background solver thread to process
    std::thread::sleep(std::time::Duration::from_millis(50));

    let latest = solver.poll_latest(1);
    assert!(
        latest.is_some(),
        "async solver must yield solved route segments"
    );
    let resp = latest.unwrap();
    assert!(
        !resp.segments.is_empty(),
        "solved route must contain segments"
    );
    assert!(
        resp.computation_time_us > 0,
        "computation time must be measured"
    );

    state.live_route_preview = resp.segments;
    assert!(
        !state.live_route_preview.is_empty(),
        "canvas state must reflect latest route preview"
    );
}

#[test]
fn test_headless_wgpu_offscreen_framebuffer_smoke() {
    // Executes offscreen headless WGPU render pass without X11/Wayland
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    let report = rt
        .block_on(oxide_gfx::debug_pass::run_line_circle_smoke_pass(16.0))
        .expect("headless WGPU offscreen pass must succeed");

    assert_eq!(report.line_instances, 2);
    assert_eq!(report.circle_instances, 1);
    assert_eq!(report.scale_px_per_mm, 16.0);
}

#[test]
fn test_canvas_routing_epoch_fencing_discards_stale_in_flight_gestures() {
    let mut state = PcbCanvasState::default();
    assert_eq!(state.routing_epoch, 0);

    let rules = Arc::new(ConstraintManager::standard_default());
    let spatial_index = Arc::new(oxide_router::geometry::rtree::SpatialIndex::new());
    let mut router = InteractiveRouter::new(rules, spatial_index);
    router.start_routing(Point2D::new(0, 0), 1, 0).unwrap();
    let solver = AsyncRoutingSolver::spawn(router);

    // Initial drag gesture with epoch 0
    solver.submit_target(
        Point2D::new(1000, 1000),
        RoutingMode::WalkAround,
        CornerMode::default(),
        1,
        0,
        200,
        state.routing_epoch,
    );

    // User commits waypoint or steps layers -> epoch increments to 1
    state.routing_epoch += 1;

    // Polling with new epoch must discard epoch 0 solutions
    std::thread::sleep(std::time::Duration::from_millis(50));
    let stale_resp = solver.poll_latest(state.routing_epoch);
    assert!(
        stale_resp.is_none(),
        "stale epoch 0 calculation must be discarded"
    );

    // Submit new cursor position with current epoch 1
    solver.submit_target(
        Point2D::new(2500, 2500),
        RoutingMode::WalkAround,
        CornerMode::default(),
        1,
        0,
        200,
        state.routing_epoch,
    );

    std::thread::sleep(std::time::Duration::from_millis(50));
    let valid_resp = solver.poll_latest(state.routing_epoch);
    assert!(valid_resp.is_some(), "epoch 1 response must be received");
    assert_eq!(valid_resp.unwrap().epoch, 1);
}

#[test]
fn test_canvas_dynamic_obstacle_aware_accordion_tuning() {
    use oxide_router::geometry::BoundingBox;
    use oxide_router::geometry::rtree::{SpatialObject, SpatialObjectType};
    use uuid::Uuid;

    let rules = Arc::new(ConstraintManager::standard_default());
    let mut spatial_index = oxide_router::geometry::rtree::SpatialIndex::new();

    // Place an obstacle at Y = 300um
    spatial_index.insert(SpatialObject {
        id: Uuid::new_v4(),
        bbox: BoundingBox::new(Point2D::new(500, 300), Point2D::new(4500, 800)),
        object_type: SpatialObjectType::Track,
        net_id: Some(99),
        layer: 0,
    });

    let router = InteractiveRouter::new(rules, Arc::new(spatial_index));
    let start = Point2D::new(0, 0);
    let end = Point2D::new(5000, 0);

    let segments = router.generate_meander_with_params(start, end, 4000, 1, 0, 200, 1000, 400);

    assert!(!segments.is_empty(), "meander segments must be synthesized");

    // Verify all points clamp below obstacle Y=300
    for seg in &segments {
        if seg.start_point.x >= 500 && seg.start_point.x <= 4500 {
            assert!(
                seg.start_point.y < 300,
                "Lobe start point {:?} violated obstacle boundary at Y=300",
                seg.start_point
            );
        }
        if seg.end_point.x >= 500 && seg.end_point.x <= 4500 {
            assert!(
                seg.end_point.y < 300,
                "Lobe end point {:?} violated obstacle boundary at Y=300",
                seg.end_point
            );
        }
    }
}
