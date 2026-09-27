---
okf_version: "0.2"
type: Class
title: FootprintEditorState
description: "Live, in-memory state of the Footprint canvas — drives interaction"
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/FootprintEditorState
language: rust
---

# FootprintEditorState

Live, in-memory state of the Footprint canvas — drives interaction

## Signature

```rust
pub struct FootprintEditorState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Live, in-memory state of the Footprint canvas — drives interaction
and rendering. The authoritative pad list lives on
`ComponentEditorState.footprint.pads`; this struct mirrors it for
the canvas's hit-test + draw layer.

`PartialEq` is intentionally NOT derived: Phase 5.3 added
`sketch_solver` / `last_solve` whose underlying types in
`oxide-sketch` don't implement `PartialEq`.
[derive(Debug, Clone)]

## Methods

- `pads`
- `layer_visibility`
- `selected_pad`
- `selected_pads_extra`
- `lasso_mode_active`
- `lasso_vertices`
- `touching_line_active`
- `touching_line_first`
- `last_click_world_mm`
- `auto_fit_courtyard`
- `courtyard_mm`
- `courtyard_outline_mm`
- `cursor_mm`
- `mode`
- `sketch_solver`
- `last_solve`
- `solve_warnings`
- `conflicts_row_hovered`
- `active_tool`
- `tool_pending`
- `selected_sketch`
- `selected_sketch_secondary`
- `selected_sketch_extra`
- `dimension_input`
- `move_by_modal`
- `align_modal`
- `pads_tool`
- `construction_mode`
- `centerline_mode`
- `placement_paused`
- `next_pad_defaults`
- `snap_options`
- `track_first`
- `place_arc_pending`
- `place_polygon_vertices`
- `selected_silk_f`
- `guides`
- `grids`
- `active_grid_idx`
- `global_snap_disabled`
- `selection_filter`
- `selection_mode_2d`
- `snap_subtab`
- `snapping_mode`
- `active_bar_menu`
- `pad_stack_tab`
- `placement_input`
- `placement_input_others`
- `numeric_buffers`
- `context_menu`
- `fit_pending`

## Source
Lines 109–259 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
