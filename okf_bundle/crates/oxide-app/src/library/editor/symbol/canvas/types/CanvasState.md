---
okf_version: "0.2"
type: Class
title: CanvasState
description: Canvas-program ephemeral state — drag + pan tracking.
resource: crates/oxide-app/src/library/editor/symbol/canvas/types.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/types/CanvasState
language: rust
---

# CanvasState

Canvas-program ephemeral state — drag + pan tracking.

## Signature

```rust
pub struct CanvasState
```

## Decorators

- `derive(Debug, Default)`

## Visibility

- `pub`

## Docstring

Canvas-program ephemeral state — drag + pan tracking.
[derive(Debug, Default)]

## Methods

- `dragging`
- `dragging_handle`
- `drag_anchor_offset`
- `last_drag_world_pos`
- `panning`
- `last_pan_pos`
- `secondary_press_pos`
- `pan_moved`
- `box_select_origin`
- `box_select_current`
- `line_from`
- `line_cursor`
- `rect_from`
- `rect_cursor`
- `circle_center`
- `circle_cursor`
- `arc_center`
- `arc_radius_start`
- `arc_cursor`
- `arc_end_deg_unwrapped`
- `polygon_cursor`
- `polygon_last_click_time`
- `polygon_last_click_pos`

## Source
Lines 198–293 in `crates/oxide-app/src/library/editor/symbol/canvas/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-app/src/library/editor/symbol/canvas/types.md) |
