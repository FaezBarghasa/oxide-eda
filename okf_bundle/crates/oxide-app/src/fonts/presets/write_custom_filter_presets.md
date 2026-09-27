---
okf_version: "0.2"
type: Function
title: write_custom_filter_presets
description: Persist the list of custom selection-filter presets without
resource: crates/oxide-app/src/fonts/presets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/presets/write_custom_filter_presets
language: rust
---

# write_custom_filter_presets

Persist the list of custom selection-filter presets without

## Signature

```rust
pub fn write_custom_filter_presets(presets: &[crate::active_bar::CustomFilterPreset])
```

## Visibility

- `pub`

## Docstring

Persist the list of custom selection-filter presets without
clobbering other preference keys.

## Source
Lines 30–36 in `crates/oxide-app/src/fonts/presets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [presets](/crates/oxide-app/src/fonts/presets.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| called_by | [sync_and_persist_custom_filter_presets](/crates/oxide-app/src/app/handlers/active_bar/filter_controls/sync_and_persist_custom_filter_presets.md) |
