---
okf_version: "0.2"
type: Function
title: capture_filter_preset
description: "Task 6 — minimal capture affordance: snapshot the current"
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar/capture_filter_preset
language: rust
---

# capture_filter_preset

Task 6 — minimal capture affordance: snapshot the current

## Signature

```rust
fn capture_filter_preset(editor: &mut crate::app::FootprintEditorState)
```

## Docstring

Task 6 — minimal capture affordance: snapshot the current
filter as a new default-named preset and persist it. No
rename UI yet (deferred — see filter_presets.rs). Silently
ignores the capture once `CUSTOM_FILTER_PRESET_LIMIT` slots
are full rather than evicting an existing preset.

## Source
Lines 100–110 in `crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/updates/active_bar.md) |
| calls | [read_footprint_filter_presets](/crates/oxide-app/src/fonts/presets/read_footprint_filter_presets.md) |
| calls | [capture_preset](/crates/oxide-app/src/library/editor/footprint/filter_presets/capture_preset.md) |
| calls | [write_footprint_filter_presets](/crates/oxide-app/src/fonts/presets/write_footprint_filter_presets.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply.md) |
