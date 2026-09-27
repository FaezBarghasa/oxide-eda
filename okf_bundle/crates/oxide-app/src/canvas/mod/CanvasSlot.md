---
okf_version: "0.2"
type: Class
title: CanvasSlot
description: The canvas program that handles input and rendering.
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/CanvasSlot
language: rust
---

# CanvasSlot

The canvas program that handles input and rendering.

## Signature

```rust
pub struct CanvasSlot
```

## Visibility

- `pub`

## Docstring

The canvas program that handles input and rendering.
Holds references to app state needed for drawing (theme colors, etc).

## Methods

- `bg_cache`
- `content_cache`
- `overlay_cache`
- `content_cache_camera`
- `camera`
- `render_cache`
- `selected`
- `pending_fit`
- `wire_preview`
- `drawing_mode`
- `tool_preview`
- `ghost_label`
- `ghost_symbol`
- `ghost_text`
- `placement_paused`
- `paper_width_mm`
- `paper_height_mm`
- `erc_markers`
- `pending_net_color`
- `lasso_polygon`
- `arc_points`
- `polyline_points`
- `reorder_picker_armed`
- `shape_anchor`

## Source
Lines 65–142 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
