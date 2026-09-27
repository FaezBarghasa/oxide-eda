---
okf_version: "0.2"
type: Function
title: temp_prefs_path
description: ─────────────────────────────────────────────────────────────────
resource: crates/oxide-app/tests/regression/prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/prefs/temp_prefs_path
language: rust
---

# temp_prefs_path

─────────────────────────────────────────────────────────────────

## Signature

```rust
fn temp_prefs_path() -> (TempDir, PathBuf)
```

## Docstring

─────────────────────────────────────────────────────────────────
§4.4 — Preferences persistence sweep

For each user-toggleable knob the checklist asks: "toggle, restart
the app, confirm the value is restored". We can't restart from a
single test process, but we can exercise the same write→read pair
through the same `prefs.json` JSON encoding the production code
uses. Tests inject a tempdir prefs file via the `_at(path)`
variants on each pref function so the user's real prefs.json is
never touched.
─────────────────────────────────────────────────────────────────

## Source
Lines 204–208 in `crates/oxide-app/tests/regression/prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs](/crates/oxide-app/tests/regression/prefs.md) |
| called_by | [prefs_cross_pref_independence](/crates/oxide-app/tests/regression/prefs/prefs_cross_pref_independence.md) |
| called_by | [prefs_enum_case_insensitive_decode](/crates/oxide-app/tests/regression/prefs/prefs_enum_case_insensitive_decode.md) |
| called_by | [prefs_garbage_json_falls_back_to_defaults](/crates/oxide-app/tests/regression/prefs/prefs_garbage_json_falls_back_to_defaults.md) |
| called_by | [prefs_grid_size_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_grid_size_round_trip_through_json.md) |
| called_by | [prefs_grid_style_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_grid_style_round_trip_through_json.md) |
| called_by | [prefs_grid_visible_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_grid_visible_round_trip_through_json.md) |
| called_by | [prefs_label_style_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_label_style_round_trip_through_json.md) |
| called_by | [prefs_multisheet_style_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_multisheet_style_round_trip_through_json.md) |
| called_by | [prefs_power_port_style_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_power_port_style_round_trip_through_json.md) |
| called_by | [prefs_snap_enabled_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_snap_enabled_round_trip_through_json.md) |
| called_by | [prefs_theme_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_theme_round_trip_through_json.md) |
| called_by | [prefs_ui_font_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_ui_font_round_trip_through_json.md) |
| called_by | [prefs_unit_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_unit_round_trip_through_json.md) |
| called_by | [prefs_writes_dont_clobber_neighboring_keys](/crates/oxide-app/tests/regression/prefs/prefs_writes_dont_clobber_neighboring_keys.md) |
