---
okf_version: "0.2"
type: Function
title: bbox_corner_points
description: "Mint the four bbox corner Points (`[ne, se, sw, nw]`) at the pad's"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/bbox_corner_points
language: rust
---

# bbox_corner_points

Mint the four bbox corner Points (`[ne, se, sw, nw]`) at the pad's

## Signature

```rust
pub(super) fn bbox_corner_points(
    sketch: &mut SketchData,
    plane_id: PlaneId,
    pad: &EditorPad,
) -> [SketchEntityId; 4]
```

## Visibility

- `pub(super)`

## Docstring

Mint the four bbox corner Points (`[ne, se, sw, nw]`) at the pad's
bbox extents. Used as the spine of every rectangular mint variant
(Rect / RoundRect / Oval / Chamfered).

## Source
Lines 103–110 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.md) |
| calls | [push_point](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_point.md) |
| called_by | [mint_chamfered_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_chamfered_pad_geometry.md) |
| called_by | [mint_oval_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_oval_pad_geometry.md) |
| called_by | [mint_round_rect_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_rect_pad_geometry.md) |
