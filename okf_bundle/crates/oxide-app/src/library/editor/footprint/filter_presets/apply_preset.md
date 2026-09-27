---
okf_version: "0.2"
type: Function
title: apply_preset
description: "Replace the editor's active selection filter with exactly the"
resource: crates/oxide-app/src/library/editor/footprint/filter_presets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/filter_presets/apply_preset
language: rust
---

# apply_preset

Replace the editor's active selection filter with exactly the

## Signature

```rust
pub fn apply_preset(state: &mut FootprintEditorState, preset: &FootprintFilterPreset)
```

## Visibility

- `pub`

## Docstring

Replace the editor's active selection filter with exactly the
preset's kinds (everything else is switched off).

## Source
Lines 14–16 in `crates/oxide-app/src/library/editor/footprint/filter_presets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [filter_presets](/crates/oxide-app/src/library/editor/footprint/filter_presets.md) |
| called_by | [apply_filter_preset_sets_state_filter](/crates/oxide-app/src/library/editor/footprint/tests/apply_filter_preset_sets_state_filter.md) |
| called_by | [apply_filter_preset](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply_filter_preset.md) |
