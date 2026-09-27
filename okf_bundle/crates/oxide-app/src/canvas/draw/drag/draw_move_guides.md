---
okf_version: "0.2"
type: Function
title: draw_move_guides
description: Drag-to-move guide line + connection-point X markers.
resource: crates/oxide-app/src/canvas/draw/drag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/drag/draw_move_guides
language: rust
---

# draw_move_guides

Drag-to-move guide line + connection-point X markers.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_move_guides(
        &self,
        frame: &mut canvas::Frame,
        state: &CanvasState,
        bounds: Rectangle,
    ) }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Drag-to-move guide line + connection-point X markers.

## Source
Lines 5–203 in `crates/oxide-app/src/canvas/draw/drag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drag](/crates/oxide-app/src/canvas/draw/drag.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [instance_transform](/crates/oxide-app/src/schematic_runtime/mod/instance_transform.md) |
