---
okf_version: "0.2"
type: Class
title: InteractionState
resource: crates/oxide-app/src/app/state/interaction.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/interaction/InteractionState
language: rust
---

# InteractionState

## Signature

```rust
pub struct InteractionState
```

## Visibility

- `pub`

## Methods

- `current_tool`
- `canvas`
- `canvases`
- `pcb_canvas`
- `dragging`
- `drag_start_pos`
- `drag_start_size`
- `tab_drag_origin`
- `wire_points`
- `wire_drawing`
- `arc_points`
- `polyline_points`
- `shape_anchor`
- `clipboard_wires`
- `clipboard_buses`
- `clipboard_labels`
- `clipboard_symbols`
- `clipboard_junctions`
- `clipboard_no_connects`
- `clipboard_text_notes`
- `draw_mode`
- `editing_text`
- `context_menu`
- `project_tree_context_menu`
- `grid_picker`
- `tab_context_menu`
- `context_submenu`
- `pending_submenu`
- `submenu_launcher_hovered`
- `submenu_panel_hovered`
- `submenu_unhovered_since`
- `last_mouse_pos`
- `last_tree_click`
- `active_bar_menu`
- `selection_filters`
- `custom_filter_presets`
- `active_custom_filter_tab`
- `footprint_filter_presets`
- `selection_slots`
- `last_tool`
- `pending_power`
- `pending_port`
- `hover_symbol_uuid`
- `hover_started_at`
- `hover_screen_pos`

## Source
Lines 9–133 in `crates/oxide-app/src/app/state/interaction.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interaction](/crates/oxide-app/src/app/state/interaction.md) |
