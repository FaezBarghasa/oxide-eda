---
okf_version: "0.2"
type: Function
title: apply_sketch_edit_with_warnings
description: "Same as [`apply_sketch_edit`] but captures any returned"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings
language: rust
---

# apply_sketch_edit_with_warnings

Same as [`apply_sketch_edit`] but captures any returned

## Signature

```rust
pub fn apply_sketch_edit_with_warnings(
    state: &mut FootprintEditorState,
    footprint: &mut Footprint,
    edit: SketchEdit,
)
```

## Visibility

- `pub`

## Docstring

Same as [`apply_sketch_edit`] but captures any returned
[`SketchError`] into `state.solve_warnings` instead of dropping it.
Used at app-dispatch call sites where there is no caller to
propagate the error to — the inspector strip surfaces the warning
list to the user.

## Source
Lines 49–57 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch.md) |
| calls | [apply_sketch_edit](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit.md) |
| called_by | [fp_editor_delete_array](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_delete_array.md) |
| called_by | [fp_editor_edit_array_param](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_edit_array_param.md) |
| called_by | [fp_editor_set_array_numbering_scheme](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_array_numbering_scheme.md) |
| called_by | [fp_editor_set_bga_skip_letters](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_skip_letters.md) |
| called_by | [fp_editor_set_bga_start_col](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_start_col.md) |
| called_by | [fp_editor_set_bga_start_row](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_start_row.md) |
| called_by | [fp_editor_set_cutout_edge_radius](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_cutout_edge_radius.md) |
| called_by | [fp_editor_set_cutout_through](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_cutout_through.md) |
| called_by | [fp_editor_set_keepout_kind](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_keepout_kind.md) |
| called_by | [fp_editor_set_pour_fill_type](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_fill_type.md) |
| called_by | [fp_editor_set_pour_net](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_net.md) |
| called_by | [fp_editor_set_pour_priority](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_priority.md) |
| called_by | [fp_editor_toggle_array_instance](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_toggle_array_instance.md) |
| called_by | [solver_errors_surface_in_solve_warnings_not_silently_swallowed](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/solver_errors_surface_in_solve_warnings_not_silently_swallowed.md) |
| called_by | [warning_wrapper_captures_parse_error_into_solve_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/warning_wrapper_captures_parse_error_into_solve_warnings.md) |
| called_by | [delete_selected](/crates/oxide-app/src/library/editor/footprint/updates/selection/delete_selected.md) |
| called_by | [add_constraint_for_selection](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/add_constraint_for_selection.md) |
| called_by | [edit_parameter](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/edit_parameter.md) |
| called_by | [move_line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_line.md) |
| called_by | [move_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_point.md) |
| called_by | [place_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/place_point.md) |
| called_by | [make_pad_from_profile](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/make_pad_from_profile.md) |
| called_by | [unlink_corner_radius](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/unlink_corner_radius.md) |
| called_by | [arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/arc.md) |
| called_by | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| called_by | [edge_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/edge_arc.md) |
| called_by | [line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/line.md) |
| called_by | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| called_by | [rounded_rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rounded_rectangle.md) |
| called_by | [tangent_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/tangent_arc.md) |
| called_by | [break_track](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/break_track.md) |
| called_by | [fillet_second_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_second_click.md) |
| called_by | [trim](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/trim.md) |
| called_by | [resolve_click_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_click_point.md) |
| called_by | [try_consume_repick_polar_center](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/try_consume_repick_polar_center.md) |
| called_by | [circular_pattern](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/circular_pattern.md) |
| called_by | [mirror](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/mirror.md) |
| called_by | [offset_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_arc.md) |
| called_by | [offset_circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_circle.md) |
| called_by | [offset_line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_line.md) |
| called_by | [rect_pattern](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/rect_pattern.md) |
| called_by | [set_mode](/crates/oxide-app/src/library/editor/footprint/updates/view/set_mode.md) |
