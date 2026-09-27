---
okf_version: "0.2"
type: Function
title: pcb_renderer_events_for_message
resource: crates/oxide-app/src/app/pcb_dirty_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/pcb_dirty_adapter/pcb_renderer_events_for_message
language: rust
---

# pcb_renderer_events_for_message

## Signature

```rust
pub(crate) fn pcb_renderer_events_for_message(message: &Message) -> &'static [PcbAppEvent]
```

## Visibility

- `pub(crate)`

## Source
Lines 18–44 in `crates/oxide-app/src/app/pcb_dirty_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_dirty_adapter](/crates/oxide-app/src/app/pcb_dirty_adapter.md) |
| called_by | [apply_pcb_renderer_dirty_hint](/crates/oxide-app/src/app/pcb_dirty_adapter/apply_pcb_renderer_dirty_hint.md) |
| called_by | [cursor_events_map_to_camera_without_geometry_dirty](/crates/oxide-app/src/app/pcb_dirty_adapter/cursor_events_map_to_camera_without_geometry_dirty.md) |
| called_by | [move_selected_canvas_event_maps_to_footprint_move_dirty](/crates/oxide-app/src/app/pcb_dirty_adapter/move_selected_canvas_event_maps_to_footprint_move_dirty.md) |
| called_by | [theme_change_message_maps_to_theme_dirty_event](/crates/oxide-app/src/app/pcb_dirty_adapter/theme_change_message_maps_to_theme_dirty_event.md) |
| called_by | [unrelated_message_has_no_pcb_dirty_hint](/crates/oxide-app/src/app/pcb_dirty_adapter/unrelated_message_has_no_pcb_dirty_hint.md) |
