---
okf_version: "0.2"
type: Module
title: canvas
description: Footprint editor 2D canvas — pure CPU rendering via
resource: crates/oxide-app/src/library/editor/footprint/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/mod
language: rust
---

# canvas

Footprint editor 2D canvas — pure CPU rendering via

## Docstring

Footprint editor 2D canvas — pure CPU rendering via
`iced::widget::Canvas`. Pads are drawn as axis-aligned rectangles
coloured by their primary layer; courtyard renders as a yellow
outline; graphics (silk/fab) trace through their stored layer
colour.

Input model — middle/right-drag pans, scroll-wheel zooms (cursor
anchored), left-click on a pad selects it, left-drag moves the
selected pad, left-click on empty canvas adds a pad. Delete-key
handling lives in `library/editor/footprint/mod.rs`'s key event
since Canvas doesn't surface keyboard events.

Submodules:
- [`geometry`] — pure helpers (point-in-poly, segment distance).
- [`hit_test`] — sketch entity hit-test + `sketch_snap`.
- [`input`] — the `Program::update` event handlers.
- [`draw`] — the `Program::draw` layer methods plus the free
renderers they call: grid (fine + coarse), pad (copper / hole /
number + Pads-mode tool preview), silk (front + back graphics),
and sketch (entity overlay, DOF arrows, snap glyph, constraint
icons, filled closed loops, and the multi-click ghost preview).

## Relationships

| Type | Target |
|------|--------|
| related | [FootprintCanvasState](/crates/oxide-app/src/library/editor/footprint/canvas/mod/FootprintCanvasState.md) |
| related | [DragState](/crates/oxide-app/src/library/editor/footprint/canvas/mod/DragState.md) |
| related | [default](/crates/oxide-app/src/library/editor/footprint/canvas/mod/default.md) |
| related | [default](/crates/oxide-app/src/library/editor/footprint/canvas/mod/default.md) |
| related | [world_to_screen](/crates/oxide-app/src/library/editor/footprint/canvas/mod/world_to_screen.md) |
| related | [screen_to_world](/crates/oxide-app/src/library/editor/footprint/canvas/mod/screen_to_world.md) |
| related | [fit_to_bounds](/crates/oxide-app/src/library/editor/footprint/canvas/mod/fit_to_bounds.md) |
| related | [world_to_screen](/crates/oxide-app/src/library/editor/footprint/canvas/mod/world_to_screen.md) |
| related | [screen_to_world](/crates/oxide-app/src/library/editor/footprint/canvas/mod/screen_to_world.md) |
| related | [fit_to_bounds](/crates/oxide-app/src/library/editor/footprint/canvas/mod/fit_to_bounds.md) |
| related | [FootprintCanvas](/crates/oxide-app/src/library/editor/footprint/canvas/mod/FootprintCanvas.md) |
| related | [update](/crates/oxide-app/src/library/editor/footprint/canvas/mod/update.md) |
| related | [draw](/crates/oxide-app/src/library/editor/footprint/canvas/mod/draw.md) |
| related | [mouse_interaction](/crates/oxide-app/src/library/editor/footprint/canvas/mod/mouse_interaction.md) |
| related | [update](/crates/oxide-app/src/library/editor/footprint/canvas/mod/update.md) |
| related | [draw](/crates/oxide-app/src/library/editor/footprint/canvas/mod/draw.md) |
| related | [mouse_interaction](/crates/oxide-app/src/library/editor/footprint/canvas/mod/mouse_interaction.md) |
| related | [silk_f_hit_at](/crates/oxide-app/src/library/editor/footprint/canvas/mod/silk_f_hit_at.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
