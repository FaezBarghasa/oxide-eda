---
okf_version: "0.2"
type: Function
title: point_pos
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/point_pos
language: rust
---

# point_pos

## Signature

```rust
fn point_pos(
        id: SketchEntityId,
        sketch: &oxide_sketch::SketchData,
        state: &FootprintEditorState,
    ) -> Option<(f64, f64)>
```

## Source
Lines 226–249 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fills](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [draw_filled_closed_loops](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/draw_filled_closed_loops.md) |
