---
okf_version: "0.2"
type: Function
title: read_prefs_json
description: "Read `prefs.json` at `path` and parse to JSON value. Returns `None`"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_prefs_json
language: rust
---

# read_prefs_json

Read `prefs.json` at `path` and parse to JSON value. Returns `None`

## Signature

```rust
fn read_prefs_json(path: &Path) -> Option<serde_json::Value>
```

## Docstring

Read `prefs.json` at `path` and parse to JSON value. Returns `None`
when the file is absent OR malformed — same semantics every read
pref needs ("treat as missing, use default").

## Source
Lines 602–605 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| called_by | [read_grid_size_mm_pref_at](/crates/oxide-app/src/fonts/mod/read_grid_size_mm_pref_at.md) |
| called_by | [read_grid_style_pref_at](/crates/oxide-app/src/fonts/mod/read_grid_style_pref_at.md) |
| called_by | [read_grid_visible_pref_at](/crates/oxide-app/src/fonts/mod/read_grid_visible_pref_at.md) |
| called_by | [read_label_style_pref_at](/crates/oxide-app/src/fonts/mod/read_label_style_pref_at.md) |
| called_by | [read_multisheet_style_pref_at](/crates/oxide-app/src/fonts/mod/read_multisheet_style_pref_at.md) |
| called_by | [read_pcb_gpu_render_pref_at](/crates/oxide-app/src/fonts/mod/read_pcb_gpu_render_pref_at.md) |
| called_by | [read_power_port_style_pref_at](/crates/oxide-app/src/fonts/mod/read_power_port_style_pref_at.md) |
| called_by | [read_snap_enabled_pref_at](/crates/oxide-app/src/fonts/mod/read_snap_enabled_pref_at.md) |
| called_by | [read_symbol_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/read_symbol_grid_size_mm_pref.md) |
| called_by | [read_symbol_grid_style_pref](/crates/oxide-app/src/fonts/mod/read_symbol_grid_style_pref.md) |
| called_by | [read_symbol_pin_selection_pref](/crates/oxide-app/src/fonts/mod/read_symbol_pin_selection_pref.md) |
| called_by | [read_theme_pref_at](/crates/oxide-app/src/fonts/mod/read_theme_pref_at.md) |
| called_by | [read_ui_font_pref_at](/crates/oxide-app/src/fonts/mod/read_ui_font_pref_at.md) |
| called_by | [read_unit_pref_at](/crates/oxide-app/src/fonts/mod/read_unit_pref_at.md) |
