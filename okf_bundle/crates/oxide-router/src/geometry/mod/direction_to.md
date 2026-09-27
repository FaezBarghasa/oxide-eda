---
okf_version: "0.2"
type: Function
title: direction_to
description: Normalize vector to unit direction (f64).
resource: crates/oxide-router/src/geometry/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:59:54Z"
concept_id: crates/oxide-router/src/geometry/mod/direction_to
language: rust
---

# direction_to

Normalize vector to unit direction (f64).

## Signature

```rust
impl Point2D { pub fn direction_to(&self, other: Point2D) -> (f64, f64) }
```

## Visibility

- `pub`

## Docstring

Normalize vector to unit direction (f64).

## Source
Lines 50–59 in `crates/oxide-router/src/geometry/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-router/src/geometry/mod.md) |
