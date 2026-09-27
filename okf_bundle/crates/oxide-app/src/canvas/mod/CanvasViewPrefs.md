---
okf_version: "0.2"
type: Class
title: CanvasViewPrefs
description: "Everything the schematic `draw` path needs that is **not** per-window"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/CanvasViewPrefs
language: rust
---

# CanvasViewPrefs

Everything the schematic `draw` path needs that is **not** per-window

## Signature

```rust
pub struct CanvasViewPrefs
```

## Type Parameters

- `'a`

## Decorators

- `derive(Clone, Copy)`

## Visibility

- `pub`

## Docstring

Everything the schematic `draw` path needs that is **not** per-window
canvas state — settings, theme colours and app-state collections that
already have an owner in `UiState` / `DocumentState`.

#631 — these used to be fields on the canvas, written by scattered
`active_canvas_mut().x = …` assignments and kept equal to their real
owner by hand. They are now read straight from app state in `view`,
once per frame, so there is no second copy to drift. Scalars are
copied (they are `Copy` and cheaper to copy than to chase);
`wire_color_overrides` is borrowed, because cloning a `HashMap` every
frame is not a cost worth paying to avoid a lifetime.

Only settings with a single `UiState` owner that every window should
agree on live here. Per-window facts stay on [`CanvasSlot`] — the
window's paper size and render cache belong to *its* document, and
`erc_markers` / `lasso_polygon` / `pending_net_color` /
`reorder_picker_armed` are written to the active canvas alone today.
Promoting those would change what an undocked window renders, which
is not this change's job.
[derive(Clone, Copy)]

## Methods

- `grid_visible`
- `theme_bg`
- `theme_grid`
- `theme_paper`
- `canvas_colors`
- `snap_enabled`
- `snap_grid_mm`
- `visible_grid_mm`
- `grid_style`
- `auto_focus`
- `draw_mode`
- `wire_color_overrides`

## Source
Lines 289–302 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
