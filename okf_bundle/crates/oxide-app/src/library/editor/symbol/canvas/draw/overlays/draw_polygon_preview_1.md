---
okf_version: "0.2"
type: Function
title: draw_polygon_preview
description: Click-collect polygon placement preview — an open polyline
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_polygon_preview_1
language: rust
---

# draw_polygon_preview

Click-collect polygon placement preview — an open polyline

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn draw_polygon_preview(
        &self,
        frame: &mut canvas::Frame,
        state: &CanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Click-collect polygon placement preview — an open polyline
through the committed vertices plus a rubber-band segment to
the live cursor. Once >= 3 vertices are collected, the first
vertex gets a highlighted ring marking it as the "click here
to close" affordance.

## Source
Lines 289–340 in `crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
