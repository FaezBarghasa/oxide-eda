---
okf_version: "0.2"
type: Function
title: mirror_solve_to_round_rect_geometry
description: "v0.24 Phase 6 — RoundRect: rewrite the per-corner Arc-centre"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_round_rect_geometry
language: rust
---

# mirror_solve_to_round_rect_geometry

v0.24 Phase 6 — RoundRect: rewrite the per-corner Arc-centre

## Signature

```rust
pub fn mirror_solve_to_round_rect_geometry(
    state: &FootprintEditorState,
    sketch: &mut SketchData,
    resolved: &HashMap<String, f64>,
)
```

## Visibility

- `pub`

## Docstring

v0.24 Phase 6 — RoundRect: rewrite the per-corner Arc-centre
Point + the two adjacent anchor Points so the rendered geometry
matches the resolved radius.

## Source
Lines 176–260 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solve](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.md) |
| calls | [sidecar_to_id](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/sidecar_to_id.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [set_point_xy](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/set_point_xy.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
