---
okf_version: "0.2"
type: Function
title: pointer_move_world
description: v0.18.8 / v0.27 — resolve the move-tick world position. Select
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/pointer_move_world_1
language: rust
---

# pointer_move_world

v0.18.8 / v0.27 — resolve the move-tick world position. Select

## Signature

```rust
fn pointer_move_world(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
    ) -> (f64, f64)
```

## Docstring

v0.18.8 / v0.27 — resolve the move-tick world position. Select
tools read the raw cursor EXCEPT while a drag is in flight
(edge-resize should respect Snap Options).

## Source
Lines 376–421 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
| calls | [sketch_snap](/crates/oxide-app/src/library/editor/footprint/canvas/hit_test/sketch_snap.md) |
| calls | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
