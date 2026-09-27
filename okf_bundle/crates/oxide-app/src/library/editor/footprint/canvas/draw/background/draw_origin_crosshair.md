---
okf_version: "0.2"
type: Function
title: draw_origin_crosshair
description: "Origin crosshair — Altium yellow on the dark Pads canvas, slate"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/background.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/background/draw_origin_crosshair
language: rust
---

# draw_origin_crosshair

Origin crosshair — Altium yellow on the dark Pads canvas, slate

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn draw_origin_crosshair(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    ) }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

Origin crosshair — Altium yellow on the dark Pads canvas, slate
grey on the Fusion-style white sketch canvas.

## Source
Lines 129–155 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/background.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [background](/crates/oxide-app/src/library/editor/footprint/canvas/draw/background.md) |
