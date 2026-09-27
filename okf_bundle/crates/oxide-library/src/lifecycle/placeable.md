---
okf_version: "0.2"
type: Function
title: placeable
description: Whether new placements are allowed (UI may still gate).
resource: crates/oxide-library/src/lifecycle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/lifecycle/placeable
language: rust
---

# placeable

Whether new placements are allowed (UI may still gate).

## Signature

```rust
impl LifecycleState { pub fn placeable(self) -> bool }
```

## Visibility

- `pub`

## Docstring

Whether new placements are allowed (UI may still gate).

## Source
Lines 21–23 in `crates/oxide-library/src/lifecycle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lifecycle](/crates/oxide-library/src/lifecycle.md) |
