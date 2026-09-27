---
okf_version: "0.2"
type: Function
title: draw_pads_layer
description: Pads — render last of the content layers so they sit on top.
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/scene.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/scene/draw_pads_layer_1
language: rust
---

# draw_pads_layer

Pads — render last of the content layers so they sit on top.

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_pads_layer(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

Pads — render last of the content layers so they sit on top.

## Source
Lines 77–91 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/scene.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene](/crates/oxide-app/src/library/editor/footprint/canvas/draw/scene.md) |
| calls | [draw_pad](/crates/oxide-app/src/library/editor/footprint/canvas/draw/pad/draw_pad.md) |
