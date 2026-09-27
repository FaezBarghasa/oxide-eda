---
okf_version: "0.2"
type: Function
title: capture
description: Snapshot the active filter set into a new preset with a default name.
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/capture_1
language: rust
---

# capture

Snapshot the active filter set into a new preset with a default name.

## Signature

```rust
pub fn capture(name: String, filters: &std::collections::HashSet<SelectionFilter>) -> Self
```

## Visibility

- `pub`

## Docstring

Snapshot the active filter set into a new preset with a default name.

## Source
Lines 183–192 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
