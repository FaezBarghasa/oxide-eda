---
okf_version: "0.2"
type: Function
title: move_anchor_via_sidecar
description: Look up an anchor by sidecar key and reposition the matching
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/move_anchor_via_sidecar
language: rust
---

# move_anchor_via_sidecar

Look up an anchor by sidecar key and reposition the matching

## Signature

```rust
fn move_anchor_via_sidecar(
    pad: &super::super::state::EditorPad,
    sketch: &mut SketchData,
    key: &str,
    target: (f64, f64),
)
```

## Docstring

Look up an anchor by sidecar key and reposition the matching
Point. Silently no-ops when the sidecar key is absent / unparseable
or the entity isn't a Point.

## Source
Lines 345–354 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solve](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.md) |
| calls | [sidecar_to_id](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/sidecar_to_id.md) |
| calls | [set_point_xy](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/set_point_xy.md) |
| called_by | [mirror_solve_to_chamfer_anchors](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_chamfer_anchors.md) |
| called_by | [mirror_solve_to_oval_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_oval_geometry.md) |
