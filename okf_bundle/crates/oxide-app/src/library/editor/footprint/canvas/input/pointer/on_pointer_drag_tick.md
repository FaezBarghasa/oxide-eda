---
okf_version: "0.2"
type: Function
title: on_pointer_drag_tick
description: Generic pad / sketch-point / sketch-line drag tick — updates the
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_pointer_drag_tick
language: rust
---

# on_pointer_drag_tick

Generic pad / sketch-point / sketch-line drag tick — updates the

## Signature

```rust
impl FootprintCanvas<'_> { fn on_pointer_drag_tick(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Docstring

Generic pad / sketch-point / sketch-line drag tick — updates the
drag-moved threshold, keeps the rubber-band endpoint in sync,
and publishes the per-tick move message.

## Source
Lines 454–504 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
