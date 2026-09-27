---
okf_version: "0.2"
type: Function
title: capture_preset
description: "Snapshot the editor's currently-enabled filter kinds into a new"
resource: crates/oxide-app/src/library/editor/footprint/filter_presets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/filter_presets/capture_preset
language: rust
---

# capture_preset

Snapshot the editor's currently-enabled filter kinds into a new

## Signature

```rust
pub fn capture_preset(state: &FootprintEditorState, name: String) -> FootprintFilterPreset
```

## Visibility

- `pub`

## Docstring

Snapshot the editor's currently-enabled filter kinds into a new
named preset, ready to be appended to the persisted list.

## Source
Lines 20–25 in `crates/oxide-app/src/library/editor/footprint/filter_presets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [filter_presets](/crates/oxide-app/src/library/editor/footprint/filter_presets.md) |
| called_by | [capture_filter_preset](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/capture_filter_preset.md) |
