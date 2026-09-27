---
okf_version: "0.2"
type: Module
title: canvas
description: Symbol-tab interactive canvas.
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod
language: rust
---

# canvas

Symbol-tab interactive canvas.

## Docstring

Symbol-tab interactive canvas.

The canvas reads the typed [`oxide_library::Symbol`] primitive
directly. The body rectangle is derived from `Symbol.graphics`
(first `Rectangle` graphic), or defaults to a
`[-5.08, -2.54] .. [5.08, 2.54]` rectangle when the primitive
carries no body geometry yet.

World-space convention mirrors the schematic editor: Standard y-axis
(positive going up; on screen y goes down so we flip). The
camera ([`crate::canvas::Camera`]) handles pan/zoom; the user
pans with right- or middle-button drag and zooms with the wheel.
Press Home (or click the Fit button) to fit the symbol bbox to
the viewport — also the implicit state on tab open.

Background colour, grid size + visibility, snap, and the cursor
coordinate readout follow the same Altium-parity surface as the
schematic canvas: bg + grid colour come from the active theme's
`CanvasColors`; grid spacing follows `panel_ctx.grid_size_mm`;
the unit ([`oxide_types::coord::Unit`]) drives the status
footer. Sheet colour is per-tab (Altium "Document Options")
and shifts the bg fill alpha so the user can pick Black / White
/ Dark Gray / Light Gray / Cream per-symbol library.

## Relationships

| Type | Target |
|------|--------|
| related | [SymbolCanvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod/SymbolCanvas.md) |
| related | [new](/crates/oxide-app/src/library/editor/symbol/canvas/mod/new.md) |
| related | [pin_visible_on_active_part](/crates/oxide-app/src/library/editor/symbol/canvas/mod/pin_visible_on_active_part.md) |
| related | [pin_hit_by_label](/crates/oxide-app/src/library/editor/symbol/canvas/mod/pin_hit_by_label.md) |
| related | [body_rect](/crates/oxide-app/src/library/editor/symbol/canvas/mod/body_rect.md) |
| related | [bbox](/crates/oxide-app/src/library/editor/symbol/canvas/mod/bbox.md) |
| related | [new](/crates/oxide-app/src/library/editor/symbol/canvas/mod/new.md) |
| related | [pin_visible_on_active_part](/crates/oxide-app/src/library/editor/symbol/canvas/mod/pin_visible_on_active_part.md) |
| related | [pin_hit_by_label](/crates/oxide-app/src/library/editor/symbol/canvas/mod/pin_hit_by_label.md) |
| related | [body_rect](/crates/oxide-app/src/library/editor/symbol/canvas/mod/body_rect.md) |
| related | [bbox](/crates/oxide-app/src/library/editor/symbol/canvas/mod/bbox.md) |
| related | [item_in_selection](/crates/oxide-app/src/library/editor/symbol/canvas/mod/item_in_selection.md) |
| related | [is_graphic_selected](/crates/oxide-app/src/library/editor/symbol/canvas/mod/is_graphic_selected.md) |
| related | [update](/crates/oxide-app/src/library/editor/symbol/canvas/mod/update.md) |
| related | [mouse_interaction](/crates/oxide-app/src/library/editor/symbol/canvas/mod/mouse_interaction.md) |
| related | [draw](/crates/oxide-app/src/library/editor/symbol/canvas/mod/draw.md) |
| related | [update](/crates/oxide-app/src/library/editor/symbol/canvas/mod/update.md) |
| related | [mouse_interaction](/crates/oxide-app/src/library/editor/symbol/canvas/mod/mouse_interaction.md) |
| related | [draw](/crates/oxide-app/src/library/editor/symbol/canvas/mod/draw.md) |
| related | [draw_symbol_with_renderer](/crates/oxide-app/src/library/editor/symbol/canvas/mod/draw_symbol_with_renderer.md) |
| related | [build_symbol_renderer_snapshot](/crates/oxide-app/src/library/editor/symbol/canvas/mod/build_symbol_renderer_snapshot.md) |
| related | [draw_symbol_with_renderer](/crates/oxide-app/src/library/editor/symbol/canvas/mod/draw_symbol_with_renderer.md) |
| related | [build_symbol_renderer_snapshot](/crates/oxide-app/src/library/editor/symbol/canvas/mod/build_symbol_renderer_snapshot.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
