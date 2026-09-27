---
okf_version: "0.2"
type: Function
title: apply_filter_preset
description: "Task 6 — apply footprint filter preset `idx` from the"
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply_filter_preset
language: rust
---

# apply_filter_preset

Task 6 — apply footprint filter preset `idx` from the

## Signature

```rust
fn apply_filter_preset(editor: &mut crate::app::FootprintEditorState, idx: usize)
```

## Docstring

Task 6 — apply footprint filter preset `idx` from the
persisted list. Re-read from disk on every apply so a
preset captured in a different tab/session is picked up
without needing an in-memory refresh.

## Source
Lines 77–84 in `crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/updates/active_bar.md) |
| calls | [read_footprint_filter_presets](/crates/oxide-app/src/fonts/presets/read_footprint_filter_presets.md) |
| calls | [apply_preset](/crates/oxide-app/src/library/editor/footprint/filter_presets/apply_preset.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply.md) |
