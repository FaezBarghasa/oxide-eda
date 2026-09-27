---
okf_version: "0.2"
type: Function
title: polygon_symbol
description: "--- Polygon graphic coverage --------------------------------------------"
resource: crates/oxide-app/src/library/editor/symbol/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/tests/polygon_symbol
language: rust
---

# polygon_symbol

--- Polygon graphic coverage --------------------------------------------

## Signature

```rust
fn polygon_symbol(vertices: Vec<[f64; 2]>, fill: Option<[u8; 4]>) -> Symbol
```

## Docstring

--- Polygon graphic coverage --------------------------------------------

## Source
Lines 572–581 in `crates/oxide-app/src/library/editor/symbol/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/symbol/state/tests.md) |
| called_by | [graphic_handle_position_returns_polygon_vertex](/crates/oxide-app/src/library/editor/symbol/state/tests/graphic_handle_position_returns_polygon_vertex.md) |
| called_by | [graphic_handles_returns_one_per_polygon_vertex](/crates/oxide-app/src/library/editor/symbol/state/tests/graphic_handles_returns_one_per_polygon_vertex.md) |
| called_by | [hit_test_filled_concave_polygon_excludes_the_notch](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_filled_concave_polygon_excludes_the_notch.md) |
| called_by | [hit_test_filled_polygon_hits_interior_and_edge](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_filled_polygon_hits_interior_and_edge.md) |
| called_by | [hit_test_graphic_handle_finds_polygon_vertex_when_selected](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_finds_polygon_vertex_when_selected.md) |
| called_by | [hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_ignores_polygon_vertex_when_not_selected.md) |
| called_by | [hit_test_outlined_polygon_hits_edge_band_not_interior](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_outlined_polygon_hits_edge_band_not_interior.md) |
| called_by | [move_graphic_handle_moves_polygon_vertex](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_moves_polygon_vertex.md) |
| called_by | [move_selected_translates_polygon_by_centroid_delta](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_translates_polygon_by_centroid_delta.md) |
| called_by | [rotate_selected_about_geometry_center_rotates_polygon_vertices](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_rotates_polygon_vertices.md) |
| called_by | [select_in_box_crossing_touches_polygon_bbox](/crates/oxide-app/src/library/editor/symbol/state/tests/select_in_box_crossing_touches_polygon_bbox.md) |
| called_by | [select_in_box_window_includes_polygon_by_bbox](/crates/oxide-app/src/library/editor/symbol/state/tests/select_in_box_window_includes_polygon_by_bbox.md) |
