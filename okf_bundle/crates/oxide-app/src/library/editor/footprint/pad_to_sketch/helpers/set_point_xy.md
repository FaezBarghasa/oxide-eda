---
okf_version: "0.2"
type: Function
title: set_point_xy
description: "Set an existing Point entity's coordinates by ID. Returns `true`"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/set_point_xy
language: rust
---

# set_point_xy

Set an existing Point entity's coordinates by ID. Returns `true`

## Signature

```rust
pub(super) fn set_point_xy(sketch: &mut SketchData, id: SketchEntityId, x: f64, y: f64) -> bool
```

## Visibility

- `pub(super)`

## Docstring

Set an existing Point entity's coordinates by ID. Returns `true`
when the entity was found (and is a Point); `false` otherwise.
Shared between the move-mirror path and the post-solve mirrors.

## Source
Lines 115–124 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| called_by | [reassert_bbox_corners](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/reassert_bbox_corners.md) |
| called_by | [translate_profile_with_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/translate_profile_with_pad.md) |
| called_by | [mirror_solve_to_round_rect_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_round_rect_geometry.md) |
| called_by | [move_anchor_via_sidecar](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/move_anchor_via_sidecar.md) |
