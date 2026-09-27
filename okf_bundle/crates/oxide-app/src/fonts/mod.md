---
okf_version: "0.2"
type: Module
title: fonts
description: Font management for Oxide.
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod
language: rust
---

# fonts

Font management for Oxide.

## Docstring

Font management for Oxide.

Responsibilities:
- Enumerate system font families using fontdb (done once, cached).
- Provide the canonical canvas font constant (Iosevka).
- Read / write the UI font preference from a simple JSON config file.

Config file: OS-canonical config dir (`%APPDATA%\oxide\prefs.json`
on Windows, `~/Library/Application Support/oxide/prefs.json` on
macOS, `$XDG_CONFIG_HOME/oxide/prefs.json` on Linux).
Format: `{"ui_font": "Roboto"}`

Fallback: if the OS reports no config directory at all
(`dirs::config_dir()` returns `None` — rare, e.g. a daemon or
container with neither `$HOME` nor `$XDG_CONFIG_HOME`), preferences
resolve to a random-named per-process subdirectory of the OS temp
dir instead (see [`production_temp_fallback_path`]), and a
`tracing::error!` names the fact that they will not survive a
temp-directory sweep or a restart.

## Relationships

| Type | Target |
|------|--------|
| related | [write_pref_atomic](/crates/oxide-app/src/fonts/mod/write_pref_atomic.md) |
| related | [ComponentClassEntry](/crates/oxide-app/src/fonts/mod/ComponentClassEntry.md) |
| related | [default_component_classes](/crates/oxide-app/src/fonts/mod/default_component_classes.md) |
| related | [iced_font_for_family](/crates/oxide-app/src/fonts/mod/iced_font_for_family.md) |
| related | [system_font_families](/crates/oxide-app/src/fonts/mod/system_font_families.md) |
| related | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| related | [production_temp_fallback_path](/crates/oxide-app/src/fonts/mod/production_temp_fallback_path.md) |
| related | [migrate_legacy_prefs](/crates/oxide-app/src/fonts/mod/migrate_legacy_prefs.md) |
| related | [legacy_posix_prefs_path](/crates/oxide-app/src/fonts/mod/legacy_posix_prefs_path.md) |
| related | [read_ui_font_pref](/crates/oxide-app/src/fonts/mod/read_ui_font_pref.md) |
| related | [read_ui_font_pref_at](/crates/oxide-app/src/fonts/mod/read_ui_font_pref_at.md) |
| related | [write_ui_font_pref](/crates/oxide-app/src/fonts/mod/write_ui_font_pref.md) |
| related | [write_ui_font_pref_at](/crates/oxide-app/src/fonts/mod/write_ui_font_pref_at.md) |
| related | [read_component_classes_pref](/crates/oxide-app/src/fonts/mod/read_component_classes_pref.md) |
| related | [write_component_classes_pref](/crates/oxide-app/src/fonts/mod/write_component_classes_pref.md) |
| related | [read_power_port_style_pref](/crates/oxide-app/src/fonts/mod/read_power_port_style_pref.md) |
| related | [read_power_port_style_pref_at](/crates/oxide-app/src/fonts/mod/read_power_port_style_pref_at.md) |
| related | [write_power_port_style_pref](/crates/oxide-app/src/fonts/mod/write_power_port_style_pref.md) |
| related | [write_power_port_style_pref_at](/crates/oxide-app/src/fonts/mod/write_power_port_style_pref_at.md) |
| related | [read_label_style_pref](/crates/oxide-app/src/fonts/mod/read_label_style_pref.md) |
| related | [read_label_style_pref_at](/crates/oxide-app/src/fonts/mod/read_label_style_pref_at.md) |
| related | [write_label_style_pref](/crates/oxide-app/src/fonts/mod/write_label_style_pref.md) |
| related | [write_label_style_pref_at](/crates/oxide-app/src/fonts/mod/write_label_style_pref_at.md) |
| related | [read_multisheet_style_pref](/crates/oxide-app/src/fonts/mod/read_multisheet_style_pref.md) |
| related | [read_multisheet_style_pref_at](/crates/oxide-app/src/fonts/mod/read_multisheet_style_pref_at.md) |
| related | [write_multisheet_style_pref](/crates/oxide-app/src/fonts/mod/write_multisheet_style_pref.md) |
| related | [write_multisheet_style_pref_at](/crates/oxide-app/src/fonts/mod/write_multisheet_style_pref_at.md) |
| related | [read_grid_style_pref](/crates/oxide-app/src/fonts/mod/read_grid_style_pref.md) |
| related | [read_grid_style_pref_at](/crates/oxide-app/src/fonts/mod/read_grid_style_pref_at.md) |
| related | [write_grid_style_pref](/crates/oxide-app/src/fonts/mod/write_grid_style_pref.md) |
| related | [write_grid_style_pref_at](/crates/oxide-app/src/fonts/mod/write_grid_style_pref_at.md) |
| related | [read_prefs_json](/crates/oxide-app/src/fonts/mod/read_prefs_json.md) |
| related | [read_theme_pref](/crates/oxide-app/src/fonts/mod/read_theme_pref.md) |
| related | [read_theme_pref_at](/crates/oxide-app/src/fonts/mod/read_theme_pref_at.md) |
| related | [write_theme_pref](/crates/oxide-app/src/fonts/mod/write_theme_pref.md) |
| related | [write_theme_pref_at](/crates/oxide-app/src/fonts/mod/write_theme_pref_at.md) |
| related | [read_unit_pref](/crates/oxide-app/src/fonts/mod/read_unit_pref.md) |
| related | [read_unit_pref_at](/crates/oxide-app/src/fonts/mod/read_unit_pref_at.md) |
| related | [write_unit_pref](/crates/oxide-app/src/fonts/mod/write_unit_pref.md) |
| related | [write_unit_pref_at](/crates/oxide-app/src/fonts/mod/write_unit_pref_at.md) |
| related | [read_grid_visible_pref](/crates/oxide-app/src/fonts/mod/read_grid_visible_pref.md) |
| related | [read_grid_visible_pref_at](/crates/oxide-app/src/fonts/mod/read_grid_visible_pref_at.md) |
| related | [write_grid_visible_pref](/crates/oxide-app/src/fonts/mod/write_grid_visible_pref.md) |
| related | [write_grid_visible_pref_at](/crates/oxide-app/src/fonts/mod/write_grid_visible_pref_at.md) |
| related | [read_pcb_gpu_render_pref](/crates/oxide-app/src/fonts/mod/read_pcb_gpu_render_pref.md) |
| related | [read_pcb_gpu_render_pref_at](/crates/oxide-app/src/fonts/mod/read_pcb_gpu_render_pref_at.md) |
| related | [write_pcb_gpu_render_pref](/crates/oxide-app/src/fonts/mod/write_pcb_gpu_render_pref.md) |
| related | [write_pcb_gpu_render_pref_at](/crates/oxide-app/src/fonts/mod/write_pcb_gpu_render_pref_at.md) |
| related | [read_snap_enabled_pref](/crates/oxide-app/src/fonts/mod/read_snap_enabled_pref.md) |
| related | [read_snap_enabled_pref_at](/crates/oxide-app/src/fonts/mod/read_snap_enabled_pref_at.md) |
| related | [write_snap_enabled_pref](/crates/oxide-app/src/fonts/mod/write_snap_enabled_pref.md) |
| related | [write_snap_enabled_pref_at](/crates/oxide-app/src/fonts/mod/write_snap_enabled_pref_at.md) |
| related | [read_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/read_grid_size_mm_pref.md) |
| related | [read_grid_size_mm_pref_at](/crates/oxide-app/src/fonts/mod/read_grid_size_mm_pref_at.md) |
| related | [write_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/write_grid_size_mm_pref.md) |
| related | [write_grid_size_mm_pref_at](/crates/oxide-app/src/fonts/mod/write_grid_size_mm_pref_at.md) |
| related | [read_symbol_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/read_symbol_grid_size_mm_pref.md) |
| related | [write_symbol_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/write_symbol_grid_size_mm_pref.md) |
| related | [read_symbol_grid_style_pref](/crates/oxide-app/src/fonts/mod/read_symbol_grid_style_pref.md) |
| related | [write_symbol_grid_style_pref](/crates/oxide-app/src/fonts/mod/write_symbol_grid_style_pref.md) |
| related | [read_symbol_pin_selection_pref](/crates/oxide-app/src/fonts/mod/read_symbol_pin_selection_pref.md) |
| related | [write_symbol_pin_selection_pref](/crates/oxide-app/src/fonts/mod/write_symbol_pin_selection_pref.md) |
| related | [prefs_path_lives_under_the_shared_config_root](/crates/oxide-app/src/fonts/mod/prefs_path_lives_under_the_shared_config_root.md) |
| related | [temp_prefs](/crates/oxide-app/src/fonts/mod/temp_prefs.md) |
| related | [a_saved_pcb_gpu_render_value_overrides_the_compile_time_default](/crates/oxide-app/src/fonts/mod/a_saved_pcb_gpu_render_value_overrides_the_compile_time_default.md) |
| related | [an_absent_pcb_gpu_render_key_falls_back_to_the_compile_time_default](/crates/oxide-app/src/fonts/mod/an_absent_pcb_gpu_render_key_falls_back_to_the_compile_time_default.md) |
