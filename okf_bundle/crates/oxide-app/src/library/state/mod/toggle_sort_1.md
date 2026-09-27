---
okf_version: "0.2"
type: Function
title: toggle_sort
description: "Toggle the sort: if `key` matches the current sort column, flip"
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/toggle_sort_1
language: rust
---

# toggle_sort

Toggle the sort: if `key` matches the current sort column, flip

## Signature

```rust
pub fn toggle_sort(&mut self, key: String)
```

## Visibility

- `pub`

## Docstring

Toggle the sort: if `key` matches the current sort column, flip
direction; otherwise set ascending sort on `key`.

## Source
Lines 276–286 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
