---
okf_version: "0.2"
type: Function
title: apply_filter_preset_sets_state_filter
description: Task 6 — applying a footprint filter preset replaces the active
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests/apply_filter_preset_sets_state_filter
language: rust
---

# apply_filter_preset_sets_state_filter

Task 6 — applying a footprint filter preset replaces the active

## Signature

```rust
fn apply_filter_preset_sets_state_filter()
```

## Decorators

- `test`

## Docstring

Task 6 — applying a footprint filter preset replaces the active
selection-filter set with exactly the preset's kinds.
[test]

## Source
Lines 164–175 in `crates/oxide-app/src/library/editor/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/tests.md) |
| calls | [apply_preset](/crates/oxide-app/src/library/editor/footprint/filter_presets/apply_preset.md) |
