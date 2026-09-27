---
okf_version: "0.2"
type: Function
title: draw_rect_preview
description: Two-click rectangle placement preview — a rubber-band outline
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_rect_preview_1
language: rust
---

# draw_rect_preview

Two-click rectangle placement preview — a rubber-band outline

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn draw_rect_preview(
        &self,
        frame: &mut canvas::Frame,
        state: &CanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Two-click rectangle placement preview — a rubber-band outline
spanning the committed first corner and the live cursor.

## Source
Lines 63–108 in `crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
