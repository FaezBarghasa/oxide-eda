---
okf_version: "0.2"
type: Function
title: draw_constraint_icons
description: v0.13.2 Phase 6.6 — render constraint glyphs above the sketch
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/draw_constraint_icons
language: rust
---

# draw_constraint_icons

v0.13.2 Phase 6.6 — render constraint glyphs above the sketch

## Signature

```rust
pub(super) fn draw_constraint_icons(
    frame: &mut canvas::Frame,
    cstate: &FootprintCanvasState,
    sketch: &oxide_sketch::SketchData,
    state: &FootprintEditorState,
)
```

## Visibility

- `pub(super)`

## Docstring

v0.13.2 Phase 6.6 — render constraint glyphs above the sketch
entities. Each constraint's centroid (geometric mean of the
entities it touches) gets a small Unicode glyph; over-constrained
constraints render in red.

## Source
Lines 15–353 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [circle_center_local](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/circle_center_local.md) |
| calls | [arc_refs_local](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/arc_refs_local.md) |
| calls | [translate](/crates/oxide-app/src/app/view/translate/translate.md) |
| called_by | [draw_sketch_overlay](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/draw_sketch_overlay.md) |
