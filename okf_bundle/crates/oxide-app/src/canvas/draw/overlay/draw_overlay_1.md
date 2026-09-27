---
okf_version: "0.2"
type: Function
title: draw_overlay
description: "Layer 4 — every-frame overlay: cursor HUD, in-progress previews,"
resource: crates/oxide-app/src/canvas/draw/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/draw/overlay/draw_overlay_1
language: rust
---

# draw_overlay

Layer 4 — every-frame overlay: cursor HUD, in-progress previews,

## Signature

```rust
pub(in crate::canvas) fn draw_overlay(
        &self,
        state: &CanvasState,
        renderer: &Renderer,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> canvas::Geometry
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Layer 4 — every-frame overlay: cursor HUD, in-progress previews,
placement ghosts, and drag guides, composed in the original order.

## Source
Lines 6–45 in `crates/oxide-app/src/canvas/draw/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/canvas/draw/overlay.md) |
