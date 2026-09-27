---
okf_version: "0.2"
type: Function
title: prefs_path
description: "Canonical OS-native preferences-file location:"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/prefs_path
language: rust
---

# prefs_path

Canonical OS-native preferences-file location:

## Signature

```rust
fn prefs_path() -> PathBuf
```

## Docstring

Canonical OS-native preferences-file location:
- Windows: `%APPDATA%\oxide\prefs.json`
- macOS:   `~/Library/Application Support/oxide/prefs.json`
- Linux:   `$XDG_CONFIG_HOME/oxide/prefs.json` (or `~/.config/...`)

Computed once per process (via `OnceLock`) so the legacy-prefs
migration runs at most once.

The directory itself comes from [`crate::config_root::config_root`],
shared by the other three config-file resolvers. Under
`cfg(test)`/`test-prefs-redirect` that resolves to a per-process
tempdir, and this function returns *before* touching
`legacy_posix_prefs_path()` at all — so the legacy migration never
runs and never reads/writes the developer's real config directory.
Issue #437, hoisted in #440.

## Source
Lines 203–231 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [config_root](/crates/oxide-app/src/config_root/config_root.md) |
| calls | [is_test_redirect_active](/crates/oxide-app/src/config_root/is_test_redirect_active.md) |
| calls | [production_temp_fallback_path](/crates/oxide-app/src/fonts/mod/production_temp_fallback_path.md) |
| calls | [legacy_posix_prefs_path](/crates/oxide-app/src/fonts/mod/legacy_posix_prefs_path.md) |
| calls | [migrate_legacy_prefs](/crates/oxide-app/src/fonts/mod/migrate_legacy_prefs.md) |
| called_by | [read_component_classes_pref](/crates/oxide-app/src/fonts/mod/read_component_classes_pref.md) |
| called_by | [read_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/read_grid_size_mm_pref.md) |
| called_by | [read_grid_style_pref](/crates/oxide-app/src/fonts/mod/read_grid_style_pref.md) |
| called_by | [read_grid_visible_pref](/crates/oxide-app/src/fonts/mod/read_grid_visible_pref.md) |
| called_by | [read_label_style_pref](/crates/oxide-app/src/fonts/mod/read_label_style_pref.md) |
| called_by | [read_multisheet_style_pref](/crates/oxide-app/src/fonts/mod/read_multisheet_style_pref.md) |
| called_by | [read_pcb_gpu_render_pref](/crates/oxide-app/src/fonts/mod/read_pcb_gpu_render_pref.md) |
| called_by | [read_power_port_style_pref](/crates/oxide-app/src/fonts/mod/read_power_port_style_pref.md) |
| called_by | [read_snap_enabled_pref](/crates/oxide-app/src/fonts/mod/read_snap_enabled_pref.md) |
| called_by | [read_symbol_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/read_symbol_grid_size_mm_pref.md) |
| called_by | [read_symbol_grid_style_pref](/crates/oxide-app/src/fonts/mod/read_symbol_grid_style_pref.md) |
| called_by | [read_symbol_pin_selection_pref](/crates/oxide-app/src/fonts/mod/read_symbol_pin_selection_pref.md) |
| called_by | [read_theme_pref](/crates/oxide-app/src/fonts/mod/read_theme_pref.md) |
| called_by | [read_ui_font_pref](/crates/oxide-app/src/fonts/mod/read_ui_font_pref.md) |
| called_by | [read_unit_pref](/crates/oxide-app/src/fonts/mod/read_unit_pref.md) |
| called_by | [write_component_classes_pref](/crates/oxide-app/src/fonts/mod/write_component_classes_pref.md) |
| called_by | [write_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/write_grid_size_mm_pref.md) |
| called_by | [write_grid_style_pref](/crates/oxide-app/src/fonts/mod/write_grid_style_pref.md) |
| called_by | [write_grid_visible_pref](/crates/oxide-app/src/fonts/mod/write_grid_visible_pref.md) |
| called_by | [write_label_style_pref](/crates/oxide-app/src/fonts/mod/write_label_style_pref.md) |
| called_by | [write_multisheet_style_pref](/crates/oxide-app/src/fonts/mod/write_multisheet_style_pref.md) |
| called_by | [write_pcb_gpu_render_pref](/crates/oxide-app/src/fonts/mod/write_pcb_gpu_render_pref.md) |
| called_by | [write_power_port_style_pref](/crates/oxide-app/src/fonts/mod/write_power_port_style_pref.md) |
| called_by | [write_snap_enabled_pref](/crates/oxide-app/src/fonts/mod/write_snap_enabled_pref.md) |
| called_by | [write_symbol_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/write_symbol_grid_size_mm_pref.md) |
| called_by | [write_symbol_grid_style_pref](/crates/oxide-app/src/fonts/mod/write_symbol_grid_style_pref.md) |
| called_by | [write_symbol_pin_selection_pref](/crates/oxide-app/src/fonts/mod/write_symbol_pin_selection_pref.md) |
| called_by | [write_theme_pref](/crates/oxide-app/src/fonts/mod/write_theme_pref.md) |
| called_by | [write_ui_font_pref](/crates/oxide-app/src/fonts/mod/write_ui_font_pref.md) |
| called_by | [write_unit_pref](/crates/oxide-app/src/fonts/mod/write_unit_pref.md) |
