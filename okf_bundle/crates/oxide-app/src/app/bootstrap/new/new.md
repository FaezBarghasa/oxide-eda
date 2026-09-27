---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-app/src/app/bootstrap/new.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:31:59Z"
concept_id: crates/oxide-app/src/app/bootstrap/new/new
language: rust
---

# new

## Signature

```rust
impl Oxide { pub fn new() -> (Self, Task<Message>) }
```

## Visibility

- `pub`

## Source
Lines 11–457 in `crates/oxide-app/src/app/bootstrap/new.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [new](/crates/oxide-app/src/app/bootstrap/new.md) |
| calls | [read_dock_layout](/crates/oxide-app/src/fonts/dock_layout/read_dock_layout.md) |
| calls | [read_pcb_gpu_render_pref](/crates/oxide-app/src/fonts/mod/read_pcb_gpu_render_pref.md) |
| calls | [read_symbol_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/read_symbol_grid_size_mm_pref.md) |
| calls | [read_symbol_grid_style_pref](/crates/oxide-app/src/fonts/mod/read_symbol_grid_style_pref.md) |
| calls | [read_symbol_pin_selection_pref](/crates/oxide-app/src/fonts/mod/read_symbol_pin_selection_pref.md) |
| calls | [read_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/read_grid_size_mm_pref.md) |
| calls | [find_standard_symbols_dir](/crates/oxide-app/src/app/helpers/find_standard_symbols_dir.md) |
| calls | [load_profile_set](/crates/oxide-app/src/keymap/profile/load_profile_set.md) |
| calls | [check_prefs_file](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file.md) |
| calls | [read_theme_pref](/crates/oxide-app/src/fonts/mod/read_theme_pref.md) |
| calls | [read_unit_pref](/crates/oxide-app/src/fonts/mod/read_unit_pref.md) |
| calls | [read_grid_visible_pref](/crates/oxide-app/src/fonts/mod/read_grid_visible_pref.md) |
| calls | [read_snap_enabled_pref](/crates/oxide-app/src/fonts/mod/read_snap_enabled_pref.md) |
| calls | [read_ui_font_pref](/crates/oxide-app/src/fonts/mod/read_ui_font_pref.md) |
| calls | [read_component_classes_pref](/crates/oxide-app/src/fonts/mod/read_component_classes_pref.md) |
| calls | [read_first_run_tour_dismissed](/crates/oxide-app/src/fonts/misc/read_first_run_tour_dismissed.md) |
| calls | [read_power_port_style_pref](/crates/oxide-app/src/fonts/mod/read_power_port_style_pref.md) |
| calls | [read_label_style_pref](/crates/oxide-app/src/fonts/mod/read_label_style_pref.md) |
| calls | [read_multisheet_style_pref](/crates/oxide-app/src/fonts/mod/read_multisheet_style_pref.md) |
| calls | [read_grid_style_pref](/crates/oxide-app/src/fonts/mod/read_grid_style_pref.md) |
| calls | [existing_backup_profiles_path](/crates/oxide-app/src/keymap/profile/existing_backup_profiles_path.md) |
| calls | [read_erc_severity_overrides](/crates/oxide-app/src/fonts/erc/read_erc_severity_overrides.md) |
| calls | [read_pin_matrix_overrides](/crates/oxide-app/src/fonts/erc/read_pin_matrix_overrides.md) |
| calls | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| calls | [read_component_filter](/crates/oxide-app/src/fonts/misc/read_component_filter.md) |
| calls | [configured_level_label](/crates/oxide-app/src/diagnostics/configured_level_label.md) |
| calls | [recent_entries](/crates/oxide-app/src/diagnostics/recent_entries.md) |
| calls | [read_custom_filter_presets](/crates/oxide-app/src/fonts/presets/read_custom_filter_presets.md) |
| calls | [read_footprint_filter_presets](/crates/oxide-app/src/fonts/presets/read_footprint_filter_presets.md) |
| calls | [load_and_mount_all](/crates/oxide-app/src/panels/components_panel/global_prefs/load_and_mount_all.md) |
| calls | [bundled_window_icon](/crates/oxide-app/src/app/bootstrap/mod/bundled_window_icon.md) |
