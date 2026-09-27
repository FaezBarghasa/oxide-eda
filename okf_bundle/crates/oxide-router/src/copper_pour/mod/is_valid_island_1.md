---
okf_version: "0.2"
type: Function
title: is_valid_island
description: Check if a detected polygon island meets the minimum area requirement to remain.
resource: crates/oxide-router/src/copper_pour/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:53:15Z"
concept_id: crates/oxide-router/src/copper_pour/mod/is_valid_island_1
language: rust
---

# is_valid_island

Check if a detected polygon island meets the minimum area requirement to remain.

## Signature

```rust
pub fn is_valid_island(polygon: &[Point2D], min_area_sq_microns: i64) -> bool
```

## Visibility

- `pub`

## Docstring

Check if a detected polygon island meets the minimum area requirement to remain.

## Source
Lines 111–125 in `crates/oxide-router/src/copper_pour/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [copper_pour](/crates/oxide-router/src/copper_pour/mod.md) |
