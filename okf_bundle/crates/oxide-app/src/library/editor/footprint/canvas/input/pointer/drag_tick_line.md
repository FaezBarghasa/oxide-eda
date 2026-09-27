---
okf_version: "0.2"
type: Function
title: drag_tick_line
description: Sketch Line drag tick — the cursor delta is projected onto the
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/drag_tick_line
language: rust
---

# drag_tick_line

Sketch Line drag tick — the cursor delta is projected onto the

## Signature

```rust
impl FootprintCanvas<'_> { fn drag_tick_line(
        &self,
        drag: &mut DragState,
        line_id: oxide_sketch::id::SketchEntityId,
        sketch_ref: &oxide_sketch::SketchData,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Docstring

Sketch Line drag tick — the cursor delta is projected onto the
line's perpendicular so an edge only pushes in its natural
resize direction (Fusion-style).

## Source
Lines 532–603 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
