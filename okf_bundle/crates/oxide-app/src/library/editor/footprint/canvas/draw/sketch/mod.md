---
okf_version: "0.2"
type: Module
title: sketch
description: "Sketch-mode rendering — entity overlay, DOF arrows, snap glyph,"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/mod
language: rust
---

# sketch

Sketch-mode rendering — entity overlay, DOF arrows, snap glyph,

## Docstring

Sketch-mode rendering — entity overlay, DOF arrows, snap glyph,
constraint icons, filled closed loops, and the live ghost preview
for multi-click drawing tools.

Split by concern (the folder carries the namespace, so the children
keep no `sketch_`/`draw_` prefix):
- `constraints` — constraint-glyph overlay.
- `entities` — point / line / circle / arc entity overlay.
- `arrows` — DOF direction arrows.
- `snap` — inferred-constraint snap glyph.
- `fills` — filled closed loops plus the `ClosedLoop` records.
- `preview` — live ghost preview for multi-click drawing tools.

The z-order the layer methods paint in is unchanged; each concern
keeps its original primitive-push sequence byte-for-byte.
