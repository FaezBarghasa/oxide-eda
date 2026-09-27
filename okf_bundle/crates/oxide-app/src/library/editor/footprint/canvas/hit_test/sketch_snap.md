---
okf_version: "0.2"
type: Function
title: sketch_snap
description: Find the sketch Point whose screen position is within
resource: crates/oxide-app/src/library/editor/footprint/canvas/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/hit_test/sketch_snap
language: rust
---

# sketch_snap

Find the sketch Point whose screen position is within

## Signature

```rust
pub(super) fn sketch_snap(
    sketch: Option<&oxide_sketch::SketchData>,
    cstate: &FootprintCanvasState,
    click_world: (f64, f64),
) -> Option<SketchEntityId>
```

## Visibility

- `pub(super)`

## Docstring

Find the sketch Point whose screen position is within
`SKETCH_SNAP_RADIUS_PX` of the given world-mm click. Returns the
nearest-snap Point's `SketchEntityId`, or `None` if no Point is
in range. Used by the canvas to drive auto-Coincident behaviour
in multi-click drawing tools.

## Source
Lines 105–129 in `crates/oxide-app/src/library/editor/footprint/canvas/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/footprint/canvas/hit_test.md) |
| called_by | [pointer_move_world](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/pointer_move_world.md) |
| called_by | [primary_press_snap](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/primary_press_snap.md) |
| called_by | [released_sketch_click](/crates/oxide-app/src/library/editor/footprint/canvas/input/release/released_sketch_click.md) |
| called_by | [try_sketch_point_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_sketch_point_grab.md) |
