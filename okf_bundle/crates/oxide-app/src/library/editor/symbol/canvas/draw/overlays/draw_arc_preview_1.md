---
okf_version: "0.2"
type: Function
title: draw_arc_preview
description: Three-click arc placement preview.
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_arc_preview_1
language: rust
---

# draw_arc_preview

Three-click arc placement preview.

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn draw_arc_preview(
        &self,
        frame: &mut canvas::Frame,
        state: &CanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Three-click arc placement preview.

## Source
Lines 186–282 in `crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| calls | [normalize_arc_commit_deg](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_arc_commit_deg.md) |
| calls | [ccw_wrapped_sweep_rad](/crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad.md) |
