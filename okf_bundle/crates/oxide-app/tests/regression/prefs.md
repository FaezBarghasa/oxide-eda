---
okf_version: "0.2"
type: Module
title: prefs
description: "`prefs.json` migration plus the read/write round-trip sweep."
resource: crates/oxide-app/tests/regression/prefs.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/prefs
language: rust
---

# prefs

`prefs.json` migration plus the read/write round-trip sweep.

## Docstring

`prefs.json` migration plus the read/write round-trip sweep.

## Relationships

| Type | Target |
|------|--------|
| related | [f1_legacy_prefs_path_copied_forward_when_canonical_empty](/crates/oxide-app/tests/regression/prefs/f1_legacy_prefs_path_copied_forward_when_canonical_empty.md) |
| related | [f1_canonical_present_blocks_legacy_copy](/crates/oxide-app/tests/regression/prefs/f1_canonical_present_blocks_legacy_copy.md) |
| related | [f1_no_legacy_no_canonical_is_a_clean_noop](/crates/oxide-app/tests/regression/prefs/f1_no_legacy_no_canonical_is_a_clean_noop.md) |
| related | [f3_stale_label_style_rewritten_to_standard](/crates/oxide-app/tests/regression/prefs/f3_stale_label_style_rewritten_to_standard.md) |
| related | [f3_canonical_label_style_left_alone](/crates/oxide-app/tests/regression/prefs/f3_canonical_label_style_left_alone.md) |
| related | [f3_label_style_case_variants_all_normalise](/crates/oxide-app/tests/regression/prefs/f3_label_style_case_variants_all_normalise.md) |
| related | [f3_garbage_json_doesnt_corrupt_file](/crates/oxide-app/tests/regression/prefs/f3_garbage_json_doesnt_corrupt_file.md) |
| related | [temp_prefs_path](/crates/oxide-app/tests/regression/prefs/temp_prefs_path.md) |
| related | [prefs_theme_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_theme_round_trip_through_json.md) |
| related | [prefs_unit_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_unit_round_trip_through_json.md) |
| related | [prefs_grid_visible_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_grid_visible_round_trip_through_json.md) |
| related | [prefs_snap_enabled_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_snap_enabled_round_trip_through_json.md) |
| related | [prefs_grid_size_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_grid_size_round_trip_through_json.md) |
| related | [prefs_writes_dont_clobber_neighboring_keys](/crates/oxide-app/tests/regression/prefs/prefs_writes_dont_clobber_neighboring_keys.md) |
| related | [prefs_garbage_json_falls_back_to_defaults](/crates/oxide-app/tests/regression/prefs/prefs_garbage_json_falls_back_to_defaults.md) |
| related | [prefs_ui_font_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_ui_font_round_trip_through_json.md) |
| related | [prefs_label_style_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_label_style_round_trip_through_json.md) |
| related | [prefs_power_port_style_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_power_port_style_round_trip_through_json.md) |
| related | [prefs_multisheet_style_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_multisheet_style_round_trip_through_json.md) |
| related | [prefs_grid_style_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_grid_style_round_trip_through_json.md) |
| related | [prefs_enum_case_insensitive_decode](/crates/oxide-app/tests/regression/prefs/prefs_enum_case_insensitive_decode.md) |
| related | [prefs_cross_pref_independence](/crates/oxide-app/tests/regression/prefs/prefs_cross_pref_independence.md) |
