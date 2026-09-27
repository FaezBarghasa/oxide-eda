---
okf_version: "0.2"
type: Function
title: read_custom_filter_presets
description: Read the user-defined custom selection-filter presets. Returns an
resource: crates/oxide-app/src/fonts/presets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/presets/read_custom_filter_presets
language: rust
---

# read_custom_filter_presets

Read the user-defined custom selection-filter presets. Returns an

## Signature

```rust
pub fn read_custom_filter_presets() -> Vec<crate::active_bar::CustomFilterPreset>
```

## Visibility

- `pub`

## Docstring

Read the user-defined custom selection-filter presets. Returns an
empty `Vec` if the file is missing, malformed, or the key absent.
Capped to `CUSTOM_FILTER_PRESET_LIMIT` entries on read so a hand-
edited file with too many slots still loads cleanly.

## Source
Lines 9–26 in `crates/oxide-app/src/fonts/presets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [presets](/crates/oxide-app/src/fonts/presets.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
