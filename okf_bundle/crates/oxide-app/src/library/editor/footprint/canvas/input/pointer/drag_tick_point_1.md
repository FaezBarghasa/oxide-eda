---
okf_version: "0.2"
type: Function
title: drag_tick_point
description: Sketch Point drag tick — per-tick delta since the last tick.
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/drag_tick_point_1
language: rust
---

# drag_tick_point

Sketch Point drag tick — per-tick delta since the last tick.

## Signature

```rust
fn drag_tick_point(
        &self,
        drag: &mut DragState,
        point_id: oxide_sketch::id::SketchEntityId,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

Sketch Point drag tick — per-tick delta since the last tick.

## Source
Lines 507–527 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
