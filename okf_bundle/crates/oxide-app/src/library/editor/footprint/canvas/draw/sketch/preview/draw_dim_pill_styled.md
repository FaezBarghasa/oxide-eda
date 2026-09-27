---
okf_version: "0.2"
type: Function
title: draw_dim_pill_styled
description: "v0.14-footprint — dimension-pill chrome with an explicit `focused`"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_dim_pill_styled
language: rust
---

# draw_dim_pill_styled

v0.14-footprint — dimension-pill chrome with an explicit `focused`

## Signature

```rust
fn draw_dim_pill_styled(frame: &mut canvas::Frame, centre: Point, label: &str, focused: bool)
```

## Docstring

v0.14-footprint — dimension-pill chrome with an explicit `focused`
state. `focused` paints the accent-bordered "active field" variant
used while the user Tab-cycles the Line length / angle inputs; the
inactive field keeps the muted grey plate every passive live
dimension uses.

## Source
Lines 40–82 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| called_by | [draw_dim_pill](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_dim_pill.md) |
| called_by | [draw_sketch_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_sketch_tool_preview.md) |
