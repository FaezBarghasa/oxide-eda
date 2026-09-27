---
okf_version: "0.2"
type: Function
title: write_footprint_filter_presets
description: Persist the list of footprint-editor filter presets without
resource: crates/oxide-app/src/fonts/presets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/presets/write_footprint_filter_presets
language: rust
---

# write_footprint_filter_presets

Persist the list of footprint-editor filter presets without

## Signature

```rust
pub fn write_footprint_filter_presets(presets: &[crate::active_bar::FootprintFilterPreset])
```

## Visibility

- `pub`

## Docstring

Persist the list of footprint-editor filter presets without
clobbering other preference keys.

## Source
Lines 68–74 in `crates/oxide-app/src/fonts/presets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [presets](/crates/oxide-app/src/fonts/presets.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| called_by | [capture_filter_preset](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/capture_filter_preset.md) |
