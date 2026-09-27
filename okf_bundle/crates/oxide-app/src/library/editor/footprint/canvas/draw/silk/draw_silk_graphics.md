---
okf_version: "0.2"
type: Function
title: draw_silk_graphics
description: "v0.18.16 — render the silk-layer graphics list. Each `FpGraphic`"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/silk.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/silk/draw_silk_graphics
language: rust
---

# draw_silk_graphics

v0.18.16 — render the silk-layer graphics list. Each `FpGraphic`

## Signature

```rust
pub(super) fn draw_silk_graphics(
    frame: &mut canvas::Frame,
    cstate: &FootprintCanvasState,
    graphics: &[oxide_library::primitive::footprint::FpGraphic],
    layer: FpLayer,
    selected_idx: Option<usize>,
)
```

## Visibility

- `pub(super)`

## Docstring

v0.18.16 — render the silk-layer graphics list. Each `FpGraphic`
becomes a single Path stroke / fill in the layer's colour.

## Source
Lines 13–181 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/silk.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [silk](/crates/oxide-app/src/library/editor/footprint/canvas/draw/silk.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| called_by | [draw_silk_layers](/crates/oxide-app/src/library/editor/footprint/canvas/draw/scene/draw_silk_layers.md) |
