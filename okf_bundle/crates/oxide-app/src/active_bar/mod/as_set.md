---
okf_version: "0.2"
type: Function
title: as_set
description: "Realize the preset's `Vec` into a `HashSet` for assignment back"
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/as_set
language: rust
---

# as_set

Realize the preset's `Vec` into a `HashSet` for assignment back

## Signature

```rust
impl CustomFilterPreset { pub fn as_set(&self) -> std::collections::HashSet<SelectionFilter> }
```

## Visibility

- `pub`

## Docstring

Realize the preset's `Vec` into a `HashSet` for assignment back
into `InteractionState::selection_filters`.

## Source
Lines 196–198 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
