---
okf_version: "0.2"
type: Function
title: draw_line_preview
description: Two-click line placement preview.
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_line_preview_1
language: rust
---

# draw_line_preview

Two-click line placement preview.

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn draw_line_preview(
        &self,
        frame: &mut canvas::Frame,
        state: &CanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Two-click line placement preview.

## Source
Lines 111–140 in `crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
