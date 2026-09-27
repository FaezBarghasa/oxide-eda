---
okf_version: "0.2"
type: Function
title: warn_auto_match_by_name
description: Auto-match by pin name. Stubbed until the name-based heuristic ships;
resource: crates/oxide-app/src/library/component_preview/updates/pin_map.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/pin_map/warn_auto_match_by_name
language: rust
---

# warn_auto_match_by_name

Auto-match by pin name. Stubbed until the name-based heuristic ships;

## Signature

```rust
pub(super) fn warn_auto_match_by_name()
```

## Visibility

- `pub(super)`

## Docstring

Auto-match by pin name. Stubbed until the name-based heuristic ships;
emits a tracing warning and leaves the overrides untouched.

## Source
Lines 21–26 in `crates/oxide-app/src/library/component_preview/updates/pin_map.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_map](/crates/oxide-app/src/library/component_preview/updates/pin_map.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
