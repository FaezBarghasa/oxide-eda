---
okf_version: "0.2"
type: Module
title: tests
description: v0.18.25.1 — regression tests for the silk hit-test edge cases
resource: crates/oxide-app/src/library/editor/footprint/canvas/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/tests
language: rust
---

# tests

v0.18.25.1 — regression tests for the silk hit-test edge cases

## Docstring

v0.18.25.1 — regression tests for the silk hit-test edge cases
flagged by the v0.18.25 code review (H1 zero-sweep arc, M1
polygon near-horizontal edges).

## Relationships

| Type | Target |
|------|--------|
| related | [line](/crates/oxide-app/src/library/editor/footprint/canvas/tests/line.md) |
| related | [arc](/crates/oxide-app/src/library/editor/footprint/canvas/tests/arc.md) |
| related | [line_hit_on_segment](/crates/oxide-app/src/library/editor/footprint/canvas/tests/line_hit_on_segment.md) |
| related | [line_miss_above_aabb_below_segment_distance](/crates/oxide-app/src/library/editor/footprint/canvas/tests/line_miss_above_aabb_below_segment_distance.md) |
| related | [arc_zero_sweep_is_no_hit](/crates/oxide-app/src/library/editor/footprint/canvas/tests/arc_zero_sweep_is_no_hit.md) |
| related | [arc_full_circle_via_360_sweep](/crates/oxide-app/src/library/editor/footprint/canvas/tests/arc_full_circle_via_360_sweep.md) |
| related | [arc_seam_crossing_includes_zero_degrees](/crates/oxide-app/src/library/editor/footprint/canvas/tests/arc_seam_crossing_includes_zero_degrees.md) |
| related | [arc_excludes_outside_sweep](/crates/oxide-app/src/library/editor/footprint/canvas/tests/arc_excludes_outside_sweep.md) |
| related | [polygon_horizontal_edge_no_nan_propagation](/crates/oxide-app/src/library/editor/footprint/canvas/tests/polygon_horizontal_edge_no_nan_propagation.md) |
| related | [polygon_outline_hit_on_edge](/crates/oxide-app/src/library/editor/footprint/canvas/tests/polygon_outline_hit_on_edge.md) |
| related | [point_to_segment_dist_zero_length](/crates/oxide-app/src/library/editor/footprint/canvas/tests/point_to_segment_dist_zero_length.md) |
| related | [arc_sketch](/crates/oxide-app/src/library/editor/footprint/canvas/tests/arc_sketch.md) |
| related | [arc_hit_test_grabs_the_stroke_not_the_centre](/crates/oxide-app/src/library/editor/footprint/canvas/tests/arc_hit_test_grabs_the_stroke_not_the_centre.md) |
| related | [polygon_filled_silk_uses_even_odd](/crates/oxide-app/src/library/editor/footprint/canvas/tests/polygon_filled_silk_uses_even_odd.md) |
| related | [line_sketch](/crates/oxide-app/src/library/editor/footprint/canvas/tests/line_sketch.md) |
| related | [state_with_tool](/crates/oxide-app/src/library/editor/footprint/canvas/tests/state_with_tool.md) |
| related | [make_canvas](/crates/oxide-app/src/library/editor/footprint/canvas/tests/make_canvas.md) |
| related | [press_near_line_end_arms_nearer_endpoint_point_drag](/crates/oxide-app/src/library/editor/footprint/canvas/tests/press_near_line_end_arms_nearer_endpoint_point_drag.md) |
| related | [press_equidistant_resolves_to_start_deterministically](/crates/oxide-app/src/library/editor/footprint/canvas/tests/press_equidistant_resolves_to_start_deterministically.md) |
| related | [seated_cstate](/crates/oxide-app/src/library/editor/footprint/canvas/tests/seated_cstate.md) |
| related | [left_press_at](/crates/oxide-app/src/library/editor/footprint/canvas/tests/left_press_at.md) |
| related | [disarmed_select_tool_still_drags_whole_line_via_dispatcher](/crates/oxide-app/src/library/editor/footprint/canvas/tests/disarmed_select_tool_still_drags_whole_line_via_dispatcher.md) |
| related | [armed_drag_track_end_wins_walk_order_via_dispatcher](/crates/oxide-app/src/library/editor/footprint/canvas/tests/armed_drag_track_end_wins_walk_order_via_dispatcher.md) |
| related | [release_cleanly_ends_a_sketch_point_drag](/crates/oxide-app/src/library/editor/footprint/canvas/tests/release_cleanly_ends_a_sketch_point_drag.md) |
| related | [hit_test_uses_solved_positions_not_stale_authored_coords](/crates/oxide-app/src/library/editor/footprint/canvas/tests/hit_test_uses_solved_positions_not_stale_authored_coords.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
