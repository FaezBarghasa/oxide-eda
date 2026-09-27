---
okf_version: "0.2"
type: Function
title: draw_box_select_overlay
description: Rubber-band box selection overlay (Window blue / Crossing green).
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_box_select_overlay_1
language: rust
---

# draw_box_select_overlay

Rubber-band box selection overlay (Window blue / Crossing green).

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn draw_box_select_overlay(
        &self,
        frame: &mut canvas::Frame,
        state: &CanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Rubber-band box selection overlay (Window blue / Crossing green).

## Source
Lines 12–59 in `crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
