---
okf_version: "0.2"
type: Class
title: CanvasEvent
description: "[derive(Debug, Clone)]"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/CanvasEvent
language: rust
---

# CanvasEvent

[derive(Debug, Clone)]

## Signature

```rust
pub enum CanvasEvent
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone)]

## Methods

- `x`
- `y`
- `zoom_pct`
- `world_x`
- `world_y`
- `world_x`
- `world_y`
- `world_x`
- `world_y`
- `screen_x`
- `screen_y`
- `x1`
- `y1`
- `x2`
- `y2`
- `dx`
- `dy`

## Source
Lines 712–749 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
| called_by | [resolve](/crates/oxide-app/src/app/command/bridge/resolve.md) |
| called_by | [cursor_events_map_to_camera_without_geometry_dirty](/crates/oxide-app/src/app/pcb_dirty_adapter/cursor_events_map_to_camera_without_geometry_dirty.md) |
| called_by | [move_selected_canvas_event_maps_to_footprint_move_dirty](/crates/oxide-app/src/app/pcb_dirty_adapter/move_selected_canvas_event_maps_to_footprint_move_dirty.md) |
| called_by | [update_pending_fit](/crates/oxide-app/src/canvas/input/camera/update_pending_fit.md) |
| called_by | [update_wheel_scrolled](/crates/oxide-app/src/canvas/input/camera/update_wheel_scrolled.md) |
| called_by | [update_cursor_moved](/crates/oxide-app/src/canvas/input/pointer/update_cursor_moved.md) |
| called_by | [update_left_pressed](/crates/oxide-app/src/canvas/input/pointer/update_left_pressed.md) |
| called_by | [update_left_released](/crates/oxide-app/src/canvas/input/pointer/update_left_released.md) |
| called_by | [update](/crates/oxide-app/src/pcb_canvas/update.md) |
