---
okf_version: "0.2"
type: Function
title: mint_oval_pad_geometry
description: "v0.24 Track A5 — mint an Oval pad's parametric geometry. An Oval is"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_oval_pad_geometry
language: rust
---

# mint_oval_pad_geometry

v0.24 Track A5 — mint an Oval pad's parametric geometry. An Oval is

## Signature

```rust
pub(super) fn mint_oval_pad_geometry(
    sketch: &mut SketchData,
    plane_id: PlaneId,
    pad: &mut EditorPad,
    centre_id: SketchEntityId,
) -> [SketchEntityId; 4]
```

## Visibility

- `pub(super)`

## Docstring

v0.24 Track A5 — mint an Oval pad's parametric geometry. An Oval is
a stadium / discorectangle.

## Source
Lines 191–311 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mint](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.md) |
| calls | [mint_pad_corner_outline](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_pad_corner_outline.md) |
| calls | [bbox_corner_points](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/bbox_corner_points.md) |
| calls | [push_point](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_point.md) |
| calls | [push_arc_ccw](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_arc_ccw.md) |
| calls | [bind_shape_param](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/bind_shape_param.md) |
| called_by | [mint_shape_geometry_for](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_shape_geometry_for.md) |
