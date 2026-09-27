---
okf_version: "0.2"
type: Function
title: draw_sketch_reticle
description: v0.27 — Fusion-style sketch reticle painted at the SNAP target
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_reticle_1
language: rust
---

# draw_sketch_reticle

v0.27 — Fusion-style sketch reticle painted at the SNAP target

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_sketch_reticle(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
        cursor_screen: Option<Point>,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — Fusion-style sketch reticle painted at the SNAP target
(state.cursor_mm) while a placement tool is active. Hidden for
the Select tool + while the context menu is open.

## Source
Lines 20–60 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
