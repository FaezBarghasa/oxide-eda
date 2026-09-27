---
okf_version: "0.2"
type: Function
title: silk_f_hit_at
description: v0.18.18 — bounding-box hit test for silk-front graphics.
resource: crates/oxide-app/src/library/editor/footprint/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/mod/silk_f_hit_at
language: rust
---

# silk_f_hit_at

v0.18.18 — bounding-box hit test for silk-front graphics.

## Signature

```rust
pub(super) fn silk_f_hit_at(
    silk_f: &[oxide_library::primitive::footprint::FpGraphic],
    x: f64,
    y: f64,
    tolerance_mm: f64,
) -> Option<usize>
```

## Visibility

- `pub(super)`

## Docstring

v0.18.18 — bounding-box hit test for silk-front graphics.
Returns the index of the first graphic whose hit-test contains
`(x, y)` (in mm), with a small tolerance to make thin shapes
reachable. Iterates in reverse so the topmost (most recently
placed) graphic wins on overlap.

v0.18.25 — Line/Arc/Circle/Rectangle/Polygon use shape-tight
hit-tests rather than AABB. Filled variants still match the
interior; outlined variants only match within `tolerance_mm` of
the stroke. Text continues to use AABB (the bake step doesn't
expose per-glyph geometry yet).

## Source
Lines 483–605 in `crates/oxide-app/src/library/editor/footprint/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/footprint/canvas/mod.md) |
| calls | [point_to_segment_dist](/crates/oxide-app/src/library/editor/footprint/canvas/geometry/point_to_segment_dist.md) |
| called_by | [on_secondary_released](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_secondary_released.md) |
| called_by | [try_silk_select](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_silk_select.md) |
