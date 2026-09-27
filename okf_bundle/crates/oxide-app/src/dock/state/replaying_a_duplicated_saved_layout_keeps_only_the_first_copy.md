---
okf_version: "0.2"
type: Function
title: replaying_a_duplicated_saved_layout_keeps_only_the_first_copy
description: A saved layout written before the fix holds duplicates.
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/replaying_a_duplicated_saved_layout_keeps_only_the_first_copy
language: rust
---

# replaying_a_duplicated_saved_layout_keeps_only_the_first_copy

A saved layout written before the fix holds duplicates.

## Signature

```rust
fn replaying_a_duplicated_saved_layout_keeps_only_the_first_copy()
```

## Decorators

- `test`

## Docstring

A saved layout written before the fix holds duplicates.
`fonts::read_dock_layout` replays it through `add_panel` region
by region, so the widened guard is also the migration.
[test]

## Source
Lines 372–390 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
