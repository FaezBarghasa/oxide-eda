---
okf_version: "0.2"
type: Function
title: apply_sketch_edit
description: "Apply a single [`SketchEdit`] and (if the sketch is non-trivial"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit
language: rust
---

# apply_sketch_edit

Apply a single [`SketchEdit`] and (if the sketch is non-trivial

## Signature

```rust
pub fn apply_sketch_edit(
    state: &mut FootprintEditorState,
    footprint: &mut Footprint,
    edit: SketchEdit,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Docstring

Apply a single [`SketchEdit`] and (if the sketch is non-trivial
and live-solve is not paused) run a solve + bake.

`state` is the editor's in-memory state; `footprint` is the
authoritative library primitive whose `sketch` and `pads` fields
the dispatcher mutates.

## Source
Lines 35–42 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch.md) |
| calls | [apply_edit_inner](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_edit_inner.md) |
| calls | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [hit_test_uses_solved_positions_not_stale_authored_coords](/crates/oxide-app/src/library/editor/footprint/canvas/tests/hit_test_uses_solved_positions_not_stale_authored_coords.md) |
| called_by | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [a_failed_solve_clears_the_previous_solves_colours](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/a_failed_solve_clears_the_previous_solves_colours.md) |
| called_by | [add_constraint_solves_geometry](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/add_constraint_solves_geometry.md) |
| called_by | [add_entity_triggers_solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/add_entity_triggers_solve_and_bake.md) |
| called_by | [line_tool_two_clicks_creates_line_with_snap](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/line_tool_two_clicks_creates_line_with_snap.md) |
| called_by | [set_mode_initialises_sketch_field_and_preserves_literal_pads](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_mode_initialises_sketch_field_and_preserves_literal_pads.md) |
| called_by | [solver_runs_on_every_edit](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/solver_runs_on_every_edit.md) |
| called_by | [qfn16_row_bakes_at_05mm_pitch](/crates/oxide-app/tests/sketch_qfn16_smoke/qfn16_row_bakes_at_05mm_pitch.md) |
| called_by | [qfn16_row_regenerates_when_pad_pitch_changes](/crates/oxide-app/tests/sketch_qfn16_smoke/qfn16_row_regenerates_when_pad_pitch_changes.md) |
| called_by | [qfn16_solve_warnings_empty_on_clean_sketch](/crates/oxide-app/tests/sketch_qfn16_smoke/qfn16_solve_warnings_empty_on_clean_sketch.md) |
