---
okf_version: "0.2"
type: Function
title: draw_filled_closed_loops
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/draw_filled_closed_loops
language: rust
---

# draw_filled_closed_loops

## Signature

```rust
pub(super) fn draw_filled_closed_loops(
    frame: &mut canvas::Frame,
    cstate: &FootprintCanvasState,
    sketch: &oxide_sketch::SketchData,
    state: &FootprintEditorState,
)
```

## Visibility

- `pub(super)`

## Source
Lines 161–405 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fills](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [point_pos](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/point_pos.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| called_by | [draw_sketch_overlay](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/draw_sketch_overlay.md) |
