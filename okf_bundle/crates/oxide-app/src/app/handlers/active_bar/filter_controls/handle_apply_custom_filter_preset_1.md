---
okf_version: "0.2"
type: Function
title: handle_apply_custom_filter_preset
description: Apply a saved preset — replaces the active filter set.
resource: crates/oxide-app/src/app/handlers/active_bar/filter_controls.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/active_bar/filter_controls/handle_apply_custom_filter_preset_1
language: rust
---

# handle_apply_custom_filter_preset

Apply a saved preset — replaces the active filter set.

## Signature

```rust
pub(crate) fn handle_apply_custom_filter_preset(&mut self, idx: usize) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Apply a saved preset — replaces the active filter set.

## Source
Lines 34–42 in `crates/oxide-app/src/app/handlers/active_bar/filter_controls.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [filter_controls](/crates/oxide-app/src/app/handlers/active_bar/filter_controls.md) |
