---
okf_version: "0.2"
type: Module
title: pointer
description: Pointer mechanics — button-press / button-release / cursor-move
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer
language: rust
---

# pointer

Pointer mechanics — button-press / button-release / cursor-move

## Docstring

Pointer mechanics — button-press / button-release / cursor-move
dispatchers plus the classification helpers they share (snap
resolution, empty-press arming, drag-tick move publishing, and the
cursor-move cache/hover tail).

The press/release dispatchers walk their per-tool arms (in
`tools.rs` / `release.rs`) in the original top-to-bottom order; the
secondary-button (pan / context-menu) handling lives here.

## Relationships

| Type | Target |
|------|--------|
| related | [on_button_pressed](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_button_pressed.md) |
| related | [on_secondary_pressed](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_secondary_pressed.md) |
| related | [on_primary_pressed](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_primary_pressed.md) |
| related | [primary_press_snap](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/primary_press_snap.md) |
| related | [primary_press_empty](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/primary_press_empty.md) |
| related | [on_button_released](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_button_released.md) |
| related | [on_secondary_released](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_secondary_released.md) |
| related | [on_primary_released](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_primary_released.md) |
| related | [on_cursor_moved](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_cursor_moved.md) |
| related | [pointer_move_world](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/pointer_move_world.md) |
| related | [try_round_resize_tick](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/try_round_resize_tick.md) |
| related | [on_pointer_drag_tick](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_pointer_drag_tick.md) |
| related | [drag_tick_point](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/drag_tick_point.md) |
| related | [drag_tick_line](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/drag_tick_line.md) |
| related | [cursor_move_cache_and_hover](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/cursor_move_cache_and_hover.md) |
| related | [on_button_pressed](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_button_pressed.md) |
| related | [on_secondary_pressed](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_secondary_pressed.md) |
| related | [on_primary_pressed](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_primary_pressed.md) |
| related | [primary_press_snap](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/primary_press_snap.md) |
| related | [primary_press_empty](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/primary_press_empty.md) |
| related | [on_button_released](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_button_released.md) |
| related | [on_secondary_released](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_secondary_released.md) |
| related | [on_primary_released](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_primary_released.md) |
| related | [on_cursor_moved](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_cursor_moved.md) |
| related | [pointer_move_world](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/pointer_move_world.md) |
| related | [try_round_resize_tick](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/try_round_resize_tick.md) |
| related | [on_pointer_drag_tick](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_pointer_drag_tick.md) |
| related | [drag_tick_point](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/drag_tick_point.md) |
| related | [drag_tick_line](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/drag_tick_line.md) |
| related | [cursor_move_cache_and_hover](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/cursor_move_cache_and_hover.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
