---
okf_version: "0.2"
type: Function
title: draw_select_rect
description: Drag-to-select (box-select) rectangle.
resource: crates/oxide-app/src/canvas/draw/drag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/drag/draw_select_rect
language: rust
---

# draw_select_rect

Drag-to-select (box-select) rectangle.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_select_rect(
        &self,
        frame: &mut canvas::Frame,
        state: &CanvasState,
        bounds: Rectangle,
    ) }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Drag-to-select (box-select) rectangle.

## Source
Lines 206–239 in `crates/oxide-app/src/canvas/draw/drag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drag](/crates/oxide-app/src/canvas/draw/drag.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
