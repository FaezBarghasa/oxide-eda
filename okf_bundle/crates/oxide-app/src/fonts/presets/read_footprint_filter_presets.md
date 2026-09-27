---
okf_version: "0.2"
type: Function
title: read_footprint_filter_presets
description: Read the user-defined footprint-editor filter presets. Returns an
resource: crates/oxide-app/src/fonts/presets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/presets/read_footprint_filter_presets
language: rust
---

# read_footprint_filter_presets

Read the user-defined footprint-editor filter presets. Returns an

## Signature

```rust
pub fn read_footprint_filter_presets() -> Vec<crate::active_bar::FootprintFilterPreset>
```

## Visibility

- `pub`

## Docstring

Read the user-defined footprint-editor filter presets. Returns an
empty `Vec` if the file is missing, malformed, or the key absent.
Capped to `CUSTOM_FILTER_PRESET_LIMIT` entries on read so a hand-
edited file with too many slots still loads cleanly. Parallel to
`read_custom_filter_presets` (schematic), but keyed on
`FootprintFilterPreset` (Task 6).

## Source
Lines 44–64 in `crates/oxide-app/src/fonts/presets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [presets](/crates/oxide-app/src/fonts/presets.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
| called_by | [handle_footprint_primitive_edit](/crates/oxide-app/src/app/dispatch/library/editor/handle_footprint_primitive_edit.md) |
| called_by | [apply_filter_preset](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply_filter_preset.md) |
| called_by | [capture_filter_preset](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/capture_filter_preset.md) |
