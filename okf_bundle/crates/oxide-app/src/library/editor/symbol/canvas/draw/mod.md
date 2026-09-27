---
okf_version: "0.2"
type: Module
title: draw
description: "Canvas draw layers — the `Program::draw` body decomposed by layer"
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/mod
language: rust
---

# draw

Canvas draw layers — the `Program::draw` body decomposed by layer

## Docstring

Canvas draw layers — the `Program::draw` body decomposed by layer
into `impl SymbolCanvas` methods. The trait `draw` (in the parent
`canvas` module) stays a thin sequence that calls these in the
ORIGINAL z-order (bottom-to-top; the order is load-bearing).
Behaviour is byte-identical.

Each layer recomputes the world→screen transform (`w2s`) from
`self.camera` — the same closure the pre-split god-function built
once; the arithmetic is unchanged.

- [`background`] — background fill, adaptive grid, origin crosshair.
- [`scene`] — resize handles for the selected graphic(s).
- [`overlays`] — tool hint, rubber-band box selection, and the
line / circle / arc multi-click placement previews.
