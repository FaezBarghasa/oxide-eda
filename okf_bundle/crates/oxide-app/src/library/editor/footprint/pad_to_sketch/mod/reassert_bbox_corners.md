---
okf_version: "0.2"
type: Function
title: reassert_bbox_corners
description: "Re-state the four bbox-corner Points ABSOLUTELY from `pad.bbox_mm()`"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/reassert_bbox_corners
language: rust
---

# reassert_bbox_corners

Re-state the four bbox-corner Points ABSOLUTELY from `pad.bbox_mm()`

## Signature

```rust
fn reassert_bbox_corners(
    sketch: &mut SketchData,
    pad: &EditorPad,
    already_translated: &HashSet<SketchEntityId>,
)
```

## Docstring

Re-state the four bbox-corner Points ABSOLUTELY from `pad.bbox_mm()`
after the delta pass.

A no-op on a healthy pad — the delta already landed them there. It
exists for the unhealthy one: the corners are the only owned Points
whose position is fully derivable from `Pad`, so any drift between
the pad's declared size and its sketch outline is repairable, and
repairing it on every move is what stops repeated moves from
accumulating f64 rounding in the outline forever.

Skips anything the profile trace already placed — a
`Custom(SketchProfile)` loop is authoritative over its own points
and owes nothing to the pad's bbox.

## Source
Lines 363–387 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [set_point_xy](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/set_point_xy.md) |
| called_by | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
