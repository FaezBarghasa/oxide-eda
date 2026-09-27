---
okf_version: "0.2"
type: Class
title: FootprintEditorPanelContext
description: "Context handed to the Properties panel when a `.snxfpt` editor"
resource: crates/oxide-app/src/panels/footprint_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_context/FootprintEditorPanelContext
language: rust
---

# FootprintEditorPanelContext

Context handed to the Properties panel when a `.snxfpt` editor

## Signature

```rust
pub struct FootprintEditorPanelContext
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Context handed to the Properties panel when a `.snxfpt` editor
tab is active. Mirrors a small read-only slice of the live
`FootprintEditorState` — the panel never mutates this, edits flow
back through `LibraryMessage::PrimitiveEditorEvent` like every
other primitive editor.
[derive(Debug, Clone)]

## Methods

- `path`
- `footprint_name`
- `version`
- `mode_kind`
- `pad_count`
- `sketch_entity_count`
- `sketch_constraint_count`
- `last_solve`
- `selected_pad`
- `selected_pad_count`
- `selected_sketch_entity`
- `auto_fit_courtyard`
- `library_siblings`
- `library_stem`
- `internal_footprints`
- `internal_selected_idx`
- `sketch_parameters`
- `solve_warnings`
- `selected_sketch_entity_id`
- `selected_sketch_role`
- `selected_sketch_is_point`
- `placement_active`
- `placement_paused`
- `next_pad_designator_override`
- `next_pad_size_x_mm`
- `next_pad_size_y_mm`
- `next_pad_side`
- `next_pad_rotation_deg`
- `next_pad_stack`
- `next_pad_shape`
- `next_pad_drill_diameter_mm`
- `next_pad_drill_slot_length_mm`
- `next_pad_template`
- `next_pad_template_library`
- `next_pad_feature_top`
- `next_pad_feature_bottom`
- `next_pad_testpoint`
- `pad_stack_tab`
- `next_pad_electrical_type`
- `next_pad_net`
- `next_pad_locked`
- `next_pad_kind`
- `footprint_description`
- `footprint_default_designator`
- `footprint_component_type`
- `footprint_height_mm`
- `next_pad_hole_tolerance_plus_mm`
- `next_pad_hole_tolerance_minus_mm`
- `next_pad_hole_rotation_deg`
- `next_pad_copper_offset_x_mm`
- `next_pad_copper_offset_y_mm`
- `selected_pour`
- `selected_keepout`
- `selected_cutout`
- `selected_sketch_pad`
- `snap_options`
- `selection_filter`
- `snap_subtab`
- `snapping_mode`
- `guides`
- `grids`
- `active_grid_idx`
- `selected_silk_summary`
- `numeric_buffers`
- `selected_array`
- `selected_pad_shape_params`

## Source
Lines 9–211 in `crates/oxide-app/src/panels/footprint_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_context](/crates/oxide-app/src/panels/footprint_context.md) |
