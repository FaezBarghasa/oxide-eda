---
okf_version: "0.2"
type: Module
title: overlays
description: "Interaction overlays — the Sketch reticle, the Select-tool cursor"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays
language: rust
---

# overlays

Interaction overlays — the Sketch reticle, the Select-tool cursor

## Docstring

Interaction overlays — the Sketch reticle, the Select-tool cursor
mark (with the line-hover resize arrow), the Touching-Line / Lasso
ghosts, the rubber-band rectangle, and the sketch-entity overlay.
These sit at the top of the z-stack. Extracted verbatim from
`Program::draw`.

## Relationships

| Type | Target |
|------|--------|
| related | [draw_sketch_reticle](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_reticle.md) |
| related | [draw_select_cursor_mark](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_select_cursor_mark.md) |
| related | [draw_touching_line_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_touching_line_ghost.md) |
| related | [draw_lasso_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_lasso_ghost.md) |
| related | [draw_rubber_band](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_rubber_band.md) |
| related | [draw_sketch_overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_overlays.md) |
| related | [draw_sketch_reticle](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_reticle.md) |
| related | [draw_select_cursor_mark](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_select_cursor_mark.md) |
| related | [draw_touching_line_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_touching_line_ghost.md) |
| related | [draw_lasso_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_lasso_ghost.md) |
| related | [draw_rubber_band](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_rubber_band.md) |
| related | [draw_sketch_overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_overlays.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
