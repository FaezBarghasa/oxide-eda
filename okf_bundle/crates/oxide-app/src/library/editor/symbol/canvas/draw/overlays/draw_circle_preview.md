---
okf_version: "0.2"
type: Function
title: draw_circle_preview
description: Two-click circle placement preview.
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_circle_preview
language: rust
---

# draw_circle_preview

Two-click circle placement preview.

## Signature

```rust
impl SymbolCanvas<'_> { pub(in crate::library::editor::symbol::canvas) fn draw_circle_preview(
        &self,
        frame: &mut canvas::Frame,
        state: &CanvasState,
    ) }
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Two-click circle placement preview.

## Source
Lines 143–183 in `crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
