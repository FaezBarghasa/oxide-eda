---
okf_version: "0.2"
type: Function
title: draw_dim_pill
description: v0.14.2 — live ghost preview for the multi-click sketch drawing
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_dim_pill
language: rust
---

# draw_dim_pill

v0.14.2 — live ghost preview for the multi-click sketch drawing

## Signature

```rust
fn draw_dim_pill(frame: &mut canvas::Frame, centre: Point, label: &str)
```

## Docstring

v0.14.2 — live ghost preview for the multi-click sketch drawing
tools. Reads `state.tool_pending` + `state.cursor_mm` and draws a
dashed semi-transparent overlay showing where the next click would
land:

- **Line tool, after click 1** → ghost line from first endpoint
to cursor.
- **Circle tool, after click 1** → ghost circle centred on click 1
with radius = distance(centre, cursor).
- **Arc tool, after click 1** → ghost line from centre to cursor
(cursor will become the start endpoint).
- **Arc tool, after click 2** → ghost arc from start through the
cursor angle, around the centre.

v0.27 — Fusion-style dimension pill chrome. Centred at
`centre` (screen coords), draws a soft grey rounded-look
rectangle behind a centred white label. Used by the live
dimension overlays during sketch tool placement so the user
sees the running length / width / height / angle as they
move the cursor.

## Source
Lines 31–33 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.md) |
| calls | [draw_dim_pill_styled](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_dim_pill_styled.md) |
| called_by | [draw_sketch_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_sketch_tool_preview.md) |
