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
    assert!(base_scene.overlay_polygons.is_empty(), "baseline scene has no overlay polygons");

    // Enable PDN Heatmap and verify GPU composite scene generation
    canvas.show_pdn_heatmap = true;
    let heatmap_scene = canvas.build_scene(&snapshot);
    assert!(!heatmap_scene.overlay_polygons.is_empty(), "PDN heatmap must generate overlay polygons");
    assert!(canvas.pdn_mesh_cache.borrow().is_some(), "PDN mesh grid must be cached");

    // Verify alpha compositing values on overlay polygons
    for poly in &heatmap_scene.overlay_polygons {
        assert_eq!(poly.vertices.len(), 4, "PDN heatmap quads must have 4 vertices");
        assert_eq!(poly.fill_color[3], 0.45, "Overlay fill alpha must be 0.45 for copper blending");
    }
}

#[test]
fn test_headless_async_routing_gesture_simulation() {
    let mut state = PcbCanvasState::default();
    let rules = Arc::new(ConstraintManager::standard_default());
    let spatial_index = Arc::new(oxide_router::geometry::rtree::SpatialIndex::new());
    let mut router = InteractiveRouter::new(rules, spatial_index);

    let start = Point2D::new(0, 0);
    router.start_routing(start, 1, 0).expect("start routing on net 1, layer 0");

    let solver = AsyncRoutingSolver::spawn(router.clone());

    // Simulate rapid cursor drag across multiple points
    let drag_points = [
        Point2D::new(1000, 1000),
        Point2D::new(2000, 1500),
        Point2D::new(3500, 2000),
        Point2D::new(5000, 3000),
    ];

    for pt in &drag_points {
        solver.submit_target(*pt, RoutingMode::WalkAround, CornerMode::default(), 1, 0, 200, 1);
    }

    // Wait briefly for background solver thread to process
    std::thread::sleep(std::time::Duration::from_millis(50));

    let latest = solver.poll_latest(1);
    assert!(latest.is_some(), "async solver must yield solved route segments");
    let resp = latest.unwrap();
    assert!(!resp.segments.is_empty(), "solved route must contain segments");
    assert!(resp.computation_time_us > 0, "computation time must be measured");

    state.live_route_preview = resp.segments;
    assert!(!state.live_route_preview.is_empty(), "canvas state must reflect latest route preview");
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
