---
okf_version: "0.2"
type: Function
title: solve_and_bake
description: "Resolve parameters, run LM, capture DOF, bake pads."
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake
language: rust
---

# solve_and_bake

Resolve parameters, run LM, capture DOF, bake pads.

## Signature

```rust
fn solve_and_bake(
    state: &mut FootprintEditorState,
    footprint: &mut Footprint,
) -> Result<(), SketchError>
```

## Docstring

Resolve parameters, run LM, capture DOF, bake pads.

v0.22 — solver is always live, no hysteresis, no pause state. All
`SolveError` variants (including `Timeout`) propagate as
`SketchError::SolveFailed` so the `_with_warnings` wrappers
surface them to the user in `state.solve_warnings`. Footprint
sketches stay small (tens-to-low-hundreds of entities); a
timeout indicates a real solver problem worth showing, not a
transient that can be silently swallowed.

## Source
Lines 362–589 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch.md) |
| calls | [bake_pads](/crates/oxide-bake/src/pad/bake_pads.md) |
| calls | [bake_arrays](/crates/oxide-bake/src/array/mod/bake_arrays.md) |
| calls | [bake_silk](/crates/oxide-bake/src/silk/bake_silk.md) |
| calls | [bake_courtyard](/crates/oxide-bake/src/courtyard/bake_courtyard.md) |
| calls | [bake_mask_openings](/crates/oxide-bake/src/mask/bake_mask_openings.md) |
| calls | [bake_mask_excludes](/crates/oxide-bake/src/mask/bake_mask_excludes.md) |
| calls | [bake_paste_apertures](/crates/oxide-bake/src/mask/bake_paste_apertures.md) |
| calls | [bake_pours](/crates/oxide-bake/src/pour/bake_pours.md) |
| calls | [bake_keepouts](/crates/oxide-bake/src/keepout/bake_keepouts.md) |
| calls | [bake_cutouts](/crates/oxide-bake/src/cutout/bake_cutouts.md) |
| calls | [bake_v_scores](/crates/oxide-bake/src/vscore/bake_v_scores.md) |
| calls | [bake_body3d](/crates/oxide-bake/src/body3d/bake_body3d.md) |
| calls | [mirror_solve_to_pad_stack](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_pad_stack.md) |
| calls | [mirror_solve_to_oval_size](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_oval_size.md) |
| calls | [mirror_solve_to_chamfer_anchors](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_chamfer_anchors.md) |
| calls | [mirror_solve_to_round_rect_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_round_rect_geometry.md) |
| calls | [mirror_solve_to_oval_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_oval_geometry.md) |
| called_by | [apply_sketch_edit](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit.md) |
| called_by | [apply_sketch_role](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_role.md) |
