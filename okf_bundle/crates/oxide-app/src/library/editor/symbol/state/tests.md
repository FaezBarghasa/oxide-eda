---
okf_version: "0.2"
type: Module
title: tests
description: Tests for symbol-editor interaction state.
resource: crates/oxide-app/src/library/editor/symbol/state/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/tests
language: rust
---

# tests

Tests for symbol-editor interaction state.

## Docstring

Tests for symbol-editor interaction state.

## Relationships

| Type | Target |
|------|--------|
| related | [add_pin_assigns_next_number](/crates/oxide-app/src/library/editor/symbol/state/tests/add_pin_assigns_next_number.md) |
| related | [add_pin_records_active_part](/crates/oxide-app/src/library/editor/symbol/state/tests/add_pin_records_active_part.md) |
| related | [max_part_number_ignores_part_zero](/crates/oxide-app/src/library/editor/symbol/state/tests/max_part_number_ignores_part_zero.md) |
| related | [max_part_number_defaults_to_one](/crates/oxide-app/src/library/editor/symbol/state/tests/max_part_number_defaults_to_one.md) |
| related | [delete_unit_removes_and_renumbers](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_removes_and_renumbers.md) |
| related | [delete_unit_out_of_range_leaves_count_unchanged](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_out_of_range_leaves_count_unchanged.md) |
| related | [delete_pin_clears_selection_via_return](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_pin_clears_selection_via_return.md) |
| related | [move_selected_updates_position](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_updates_position.md) |
| related | [hit_test_returns_pin](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_returns_pin.md) |
| related | [graphic_handle_position_returns_rectangle_corners](/crates/oxide-app/src/library/editor/symbol/state/tests/graphic_handle_position_returns_rectangle_corners.md) |
| related | [hit_test_graphic_handle_finds_rectangle_corner](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_finds_rectangle_corner.md) |
| related | [move_graphic_handle_moves_line_endpoint](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_moves_line_endpoint.md) |
| related | [move_graphic_handle_resizes_circle_radius](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_resizes_circle_radius.md) |
| related | [hit_test_returns_graphic_inside_rectangle](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_returns_graphic_inside_rectangle.md) |
| related | [move_selected_translates_rectangle_by_anchor_delta](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_translates_rectangle_by_anchor_delta.md) |
| related | [rotate_selected_rotates_rectangle_clockwise_around_origin](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_rotates_rectangle_clockwise_around_origin.md) |
| related | [rotate_selected_rotates_pin_orientation_in_place](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_rotates_pin_orientation_in_place.md) |
| related | [rotate_selected_about_geometry_center_keeps_rectangle_center](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_keeps_rectangle_center.md) |
| related | [rotate_selected_about_geometry_center_keeps_text_anchor_fixed](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_keeps_text_anchor_fixed.md) |
| related | [delete_selected_removes_graphic](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_selected_removes_graphic.md) |
| related | [move_graphic_handle_no_op_for_mismatched_variant](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_no_op_for_mismatched_variant.md) |
| related | [graphic_on_part_shared_and_scoped](/crates/oxide-app/src/library/editor/symbol/state/tests/graphic_on_part_shared_and_scoped.md) |
| related | [hit_test_respects_active_part](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_respects_active_part.md) |
| related | [delete_unit_prunes_and_renumbers_graphics](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_prunes_and_renumbers_graphics.md) |
| related | [hit_test_ignores_other_unit_pin](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_ignores_other_unit_pin.md) |
| related | [select_in_box_all_uses_visible_counts](/crates/oxide-app/src/library/editor/symbol/state/tests/select_in_box_all_uses_visible_counts.md) |
| related | [polygon_symbol](/crates/oxide-app/src/library/editor/symbol/state/tests/polygon_symbol.md) |
| related | [polygon_centroid_averages_vertices](/crates/oxide-app/src/library/editor/symbol/state/tests/polygon_centroid_averages_vertices.md) |
| related | [polygon_centroid_is_area_weighted_not_skewed_by_a_densely_subdivided_side](/crates/oxide-app/src/library/editor/symbol/state/tests/polygon_centroid_is_area_weighted_not_skewed_by_a_densely_subdivided_side.md) |
| related | [polygon_centroid_falls_back_to_vertex_mean_for_a_bowtie](/crates/oxide-app/src/library/editor/symbol/state/tests/polygon_centroid_falls_back_to_vertex_mean_for_a_bowtie.md) |
| related | [hit_test_outlined_polygon_hits_edge_band_not_interior](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_outlined_polygon_hits_edge_band_not_interior.md) |
| related | [hit_test_filled_polygon_hits_interior_and_edge](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_filled_polygon_hits_interior_and_edge.md) |
| related | [hit_test_filled_concave_polygon_excludes_the_notch](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_filled_concave_polygon_excludes_the_notch.md) |
| related | [graphic_handle_position_returns_polygon_vertex](/crates/oxide-app/src/library/editor/symbol/state/tests/graphic_handle_position_returns_polygon_vertex.md) |
| related | [graphic_handles_returns_one_per_polygon_vertex](/crates/oxide-app/src/library/editor/symbol/state/tests/graphic_handles_returns_one_per_polygon_vertex.md) |
| related | [hit_test_graphic_handle_finds_polygon_vertex_when_selected](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_finds_polygon_vertex_when_selected.md) |
| related | [hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected.md) |
| related | [move_graphic_handle_moves_polygon_vertex](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_moves_polygon_vertex.md) |
| related | [move_selected_translates_polygon_by_centroid_delta](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_translates_polygon_by_centroid_delta.md) |
| related | [rotate_selected_about_geometry_center_rotates_polygon_vertices](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_rotates_polygon_vertices.md) |
| related | [select_in_box_window_includes_polygon_by_bbox](/crates/oxide-app/src/library/editor/symbol/state/tests/select_in_box_window_includes_polygon_by_bbox.md) |
| related | [select_in_box_crossing_touches_polygon_bbox](/crates/oxide-app/src/library/editor/symbol/state/tests/select_in_box_crossing_touches_polygon_bbox.md) |
| related | [arc_symbol](/crates/oxide-app/src/library/editor/symbol/state/tests/arc_symbol.md) |
| related | [rotated_wraparound_arc_hit_test_and_draw_sweep_agree](/crates/oxide-app/src/library/editor/symbol/state/tests/rotated_wraparound_arc_hit_test_and_draw_sweep_agree.md) |
| related | [arc_endpoint_handle_drag_survives_save_reload](/crates/oxide-app/src/library/editor/symbol/state/tests/arc_endpoint_handle_drag_survives_save_reload.md) |
