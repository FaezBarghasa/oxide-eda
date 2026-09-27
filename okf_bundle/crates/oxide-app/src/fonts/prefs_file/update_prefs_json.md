---
okf_version: "0.2"
type: Function
title: update_prefs_json
description: "Update one key of `prefs.json` at `path` without clobbering the"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/update_prefs_json
language: rust
---

# update_prefs_json

Update one key of `prefs.json` at `path` without clobbering the

## Signature

```rust
pub(super) fn update_prefs_json(
    path: &Path,
    context: &str,
    mutator: impl FnOnce(&mut serde_json::Map<String, serde_json::Value>),
)
```

## Visibility

- `pub(super)`

## Docstring

Update one key of `prefs.json` at `path` without clobbering the
others, or — when the existing file cannot be loaded — without
touching the file at all (#594).

`context` is the preference key being written; it rides through to
`write_pref_atomic` and into the refusal report so a failure names
which knob the user was turning. Creates the parent dir if missing.

## Source
Lines 169–194 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [load_for_update](/crates/oxide-app/src/fonts/prefs_file/load_for_update.md) |
| calls | [report_refusal](/crates/oxide-app/src/fonts/prefs_file/report_refusal.md) |
| calls | [forget_refusal](/crates/oxide-app/src/fonts/prefs_file/forget_refusal.md) |
| calls | [write_pref_atomic](/crates/oxide-app/src/fonts/mod/write_pref_atomic.md) |
| called_by | [write_dock_layout](/crates/oxide-app/src/fonts/dock_layout/write_dock_layout.md) |
| called_by | [write_erc_severity_overrides](/crates/oxide-app/src/fonts/erc/write_erc_severity_overrides.md) |
| called_by | [write_pin_matrix_overrides](/crates/oxide-app/src/fonts/erc/write_pin_matrix_overrides.md) |
| called_by | [write_component_filter](/crates/oxide-app/src/fonts/misc/write_component_filter.md) |
| called_by | [write_first_run_tour_dismissed](/crates/oxide-app/src/fonts/misc/write_first_run_tour_dismissed.md) |
| called_by | [write_library_browser_search](/crates/oxide-app/src/fonts/misc/write_library_browser_search.md) |
| called_by | [write_component_classes_pref](/crates/oxide-app/src/fonts/mod/write_component_classes_pref.md) |
| called_by | [write_grid_size_mm_pref_at](/crates/oxide-app/src/fonts/mod/write_grid_size_mm_pref_at.md) |
| called_by | [write_grid_style_pref_at](/crates/oxide-app/src/fonts/mod/write_grid_style_pref_at.md) |
| called_by | [write_grid_visible_pref_at](/crates/oxide-app/src/fonts/mod/write_grid_visible_pref_at.md) |
| called_by | [write_label_style_pref_at](/crates/oxide-app/src/fonts/mod/write_label_style_pref_at.md) |
| called_by | [write_multisheet_style_pref_at](/crates/oxide-app/src/fonts/mod/write_multisheet_style_pref_at.md) |
| called_by | [write_pcb_gpu_render_pref_at](/crates/oxide-app/src/fonts/mod/write_pcb_gpu_render_pref_at.md) |
| called_by | [write_power_port_style_pref_at](/crates/oxide-app/src/fonts/mod/write_power_port_style_pref_at.md) |
| called_by | [write_snap_enabled_pref_at](/crates/oxide-app/src/fonts/mod/write_snap_enabled_pref_at.md) |
| called_by | [write_symbol_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/write_symbol_grid_size_mm_pref.md) |
| called_by | [write_symbol_grid_style_pref](/crates/oxide-app/src/fonts/mod/write_symbol_grid_style_pref.md) |
| called_by | [write_symbol_pin_selection_pref](/crates/oxide-app/src/fonts/mod/write_symbol_pin_selection_pref.md) |
| called_by | [write_theme_pref_at](/crates/oxide-app/src/fonts/mod/write_theme_pref_at.md) |
| called_by | [write_ui_font_pref_at](/crates/oxide-app/src/fonts/mod/write_ui_font_pref_at.md) |
| called_by | [write_unit_pref_at](/crates/oxide-app/src/fonts/mod/write_unit_pref_at.md) |
| called_by | [absent_prefs_file_is_created](/crates/oxide-app/src/fonts/prefs_file/absent_prefs_file_is_created.md) |
| called_by | [empty_prefs_file_is_treated_as_absent](/crates/oxide-app/src/fonts/prefs_file/empty_prefs_file_is_treated_as_absent.md) |
| called_by | [existing_keys_survive_an_unrelated_write](/crates/oxide-app/src/fonts/prefs_file/existing_keys_survive_an_unrelated_write.md) |
| called_by | [malformed_prefs_file_is_left_byte_identical](/crates/oxide-app/src/fonts/prefs_file/malformed_prefs_file_is_left_byte_identical.md) |
| called_by | [non_object_root_is_left_alone_and_does_not_panic](/crates/oxide-app/src/fonts/prefs_file/non_object_root_is_left_alone_and_does_not_panic.md) |
| called_by | [unreadable_prefs_file_is_left_alone](/crates/oxide-app/src/fonts/prefs_file/unreadable_prefs_file_is_left_alone.md) |
| called_by | [writes_resume_after_the_broken_file_is_moved_aside](/crates/oxide-app/src/fonts/prefs_file/writes_resume_after_the_broken_file_is_moved_aside.md) |
| called_by | [write_custom_filter_presets](/crates/oxide-app/src/fonts/presets/write_custom_filter_presets.md) |
| called_by | [write_footprint_filter_presets](/crates/oxide-app/src/fonts/presets/write_footprint_filter_presets.md) |
