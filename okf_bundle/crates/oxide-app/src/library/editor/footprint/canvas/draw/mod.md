---
okf_version: "0.2"
type: Module
title: draw
description: "Canvas draw layers — the `Program::draw` body decomposed by layer"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/mod
language: rust
---

# draw

Canvas draw layers — the `Program::draw` body decomposed by layer

## Docstring

Canvas draw layers — the `Program::draw` body decomposed by layer
into `impl FootprintCanvas` methods, plus the free-function
renderers those layer methods call. The trait `draw` (in the parent
`canvas` module) stays a thin sequence that calls the layer methods
in the ORIGINAL z-order (order is load-bearing). Behaviour is
byte-identical.

Layer-method modules (`impl FootprintCanvas`):
- [`background`] — background fill + grid, guides, origin crosshair.
- [`scene`] — silk graphics, courtyard, pads, array badges.
- [`ghosts`] — PlacePad / PlaceVia placement ghosts.
- [`overlays`] — sketch reticle, select cursor mark, touching-line
/ lasso ghosts, rubber-band rectangle, sketch-entity overlay.

Free-function renderers (called by the layer methods above and, for
a few items, by the parent `canvas` / `input` modules through the
re-exports below):
- [`grid`] — fine + coarse grid rendering.
- [`pad`] — pad copper / hole / number + Pads-mode tool preview.
- [`silk`] — silk-front + silk-back graphics renderer.
- [`sketch`] — sketch entity overlay, DOF arrows, snap glyph,
constraint icons, filled closed loops, and the multi-click ghost
preview for sketch drawing tools.
