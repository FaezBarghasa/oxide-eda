---
okf_version: "0.2"
type: Function
title: sync_and_persist_custom_filter_presets
description: Mirror the in-memory preset list + active-tab index into the
resource: crates/oxide-app/src/app/handlers/active_bar/filter_controls.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/active_bar/filter_controls/sync_and_persist_custom_filter_presets
language: rust
---

# sync_and_persist_custom_filter_presets

Mirror the in-memory preset list + active-tab index into the

## Signature

```rust
impl Oxide { fn sync_and_persist_custom_filter_presets(&mut self) }
```

## Docstring

Mirror the in-memory preset list + active-tab index into the
panel context (so the Properties panel re-renders) and persist
to disk.

## Source
Lines 133–139 in `crates/oxide-app/src/app/handlers/active_bar/filter_controls.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [filter_controls](/crates/oxide-app/src/app/handlers/active_bar/filter_controls.md) |
| calls | [write_custom_filter_presets](/crates/oxide-app/src/fonts/presets/write_custom_filter_presets.md) |
