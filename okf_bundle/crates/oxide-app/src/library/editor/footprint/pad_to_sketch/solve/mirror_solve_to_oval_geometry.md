---
okf_version: "0.2"
type: Function
title: mirror_solve_to_oval_geometry
description: "v0.24 Phase 6 — Oval: rewrite the 4 anchor Points + 2 arc-centre"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_oval_geometry
language: rust
---

# mirror_solve_to_oval_geometry

v0.24 Phase 6 — Oval: rewrite the 4 anchor Points + 2 arc-centre

## Signature

```rust
pub fn mirror_solve_to_oval_geometry(
    state: &FootprintEditorState,
    sketch: &mut SketchData,
    resolved: &HashMap<String, f64>,
)
```

## Visibility

- `pub`

## Docstring

v0.24 Phase 6 — Oval: rewrite the 4 anchor Points + 2 arc-centre
Points based on the resolved width / height parameters.

## Source
Lines 264–332 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solve](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.md) |
| calls | [move_anchor_via_sidecar](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/move_anchor_via_sidecar.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
