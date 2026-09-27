---
okf_version: "0.2"
type: Class
title: UiState
resource: crates/oxide-app/src/app/state/ui.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/state/ui/UiState
language: rust
---

# UiState

## Signature

```rust
pub struct UiState
```

## Visibility

- `pub`

## Methods

- `theme_id`
- `unit`
- `grid_visible`
- `snap_enabled`
- `cursor_x`
- `cursor_y`
- `zoom`
- `grid_size_mm`
- `visible_grid_mm`
- `snap_hotspots`
- `ui_font_name`
- `component_classes`
- `preferences_draft_component_classes`
- `keymap_profiles`
- `active_keymap`
- `keymap_pending_sequence`
- `keymap_pending_target`
- `canvas_font_name`
- `canvas_font_size`
- `canvas_font_bold`
- `canvas_font_italic`
- `left_width`
- `right_width`
- `bottom_height`
- `window_size`
- `main_window_scale`
- `panel_list_open`
- `preferences_open`
- `keyboard_shortcuts_open`
- `first_run_tour_open`
- `find_replace`
- `preferences_nav`
- `preferences_draft_theme`
- `preferences_draft_font`
- `preferences_theme_status`
- `power_port_style`
- `preferences_draft_power_port_style`
- `label_style`
- `preferences_draft_label_style`
- `multisheet_style`
- `preferences_draft_multisheet_style`
- `grid_style`
- `preferences_draft_grid_style`
- `pcb_gpu_render`
- `preferences_draft_pcb_gpu_render`
- `symbol_grid_size_mm`
- `preferences_draft_symbol_grid_size_mm`
- `symbol_grid_style`
- `preferences_draft_symbol_grid_style`
- `symbol_pin_selection`
- `preferences_draft_symbol_pin_selection`
- `preferences_keymap_editor`
- `preferences_keymap_status`
- `keymap_load_error`
- `keymap_backup`
- `prefs_load_error`
- `preferences_prefs_status`
- `preferences_keymap_search`
- `preferences_keymap_recorder`
- `preferences_dirty`
- `preferences_dirty_sticky`
- `custom_theme`
- `rename_dialog`
- `remove_dialog`
- `project_close_confirm`
- `app_quit_confirm`
- `project_options`
- `enable_version_control`
- `grid_properties`
- `selection_filter_custom`
- `erc_violations`
- `erc_violations_by_path`
- `erc_focus_global_index`
- `project_netlist`
- `erc_severity_override`
- `net_colors`
- `auto_focus`
- `annotate_dialog_open`
- `annotate_order`
- `erc_dialog_open`
- `annotate_reset_confirm`
- `modal_offsets`
- `modal_dragging`
- `tab_dragging`
- `move_selection`
- `net_color_palette_open`
- `parameter_manager_open`
- `reorder_picker`
- `pin_matrix_overrides`
- `annotate_locked`
- `selection_mode`
- `pending_net_color`
- `wire_color_overrides`
- `lasso_polygon`
- `net_color_undo`
- `net_color_custom`
- `main_window_id`
- `windows`
- `passive_calculator`
- `passive_calculator_open`
- `command_palette`

## Source
Lines 13–384 in `crates/oxide-app/src/app/state/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/app/state/ui.md) |
