---
okf_version: "0.2"
type: Function
title: mirror_solve_to_chamfer_anchors
description: "v0.24 Track A6 — Chamfered pads: re-derive the chamfer anchor"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_chamfer_anchors
language: rust
---

# mirror_solve_to_chamfer_anchors

v0.24 Track A6 — Chamfered pads: re-derive the chamfer anchor

## Signature

```rust
pub fn mirror_solve_to_chamfer_anchors(
    state: &FootprintEditorState,
    sketch: &mut SketchData,
    resolved: &HashMap<String, f64>,
)
```

## Visibility

- `pub`

## Docstring

v0.24 Track A6 — Chamfered pads: re-derive the chamfer anchor
Point coordinates from the resolved `chamfer_len_<slug>` parameter.

## Source
Lines 112–171 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solve](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.md) |
| calls | [move_anchor_via_sidecar](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/move_anchor_via_sidecar.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
