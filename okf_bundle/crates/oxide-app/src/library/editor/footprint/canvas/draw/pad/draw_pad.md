---
okf_version: "0.2"
type: Function
title: draw_pad
description: "Render a single pad — copper outline, drilled hole, and pad number"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/pad/draw_pad
language: rust
---

# draw_pad

Render a single pad — copper outline, drilled hole, and pad number

## Signature

```rust
pub(super) fn draw_pad(
    frame: &mut canvas::Frame,
    cstate: &FootprintCanvasState,
    pad: &EditorPad,
    is_selected: bool,
)
```

## Visibility

- `pub(super)`

## Docstring

Render a single pad — copper outline, drilled hole, and pad number
(when zoomed in enough to read).

## Source
Lines 11–205 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/canvas/draw/pad.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| calls | [translate](/crates/oxide-app/src/app/view/translate/translate.md) |
| called_by | [draw_pads_layer](/crates/oxide-app/src/library/editor/footprint/canvas/draw/scene/draw_pads_layer.md) |
