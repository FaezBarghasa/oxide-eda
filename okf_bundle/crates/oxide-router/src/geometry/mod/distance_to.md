---
okf_version: "0.2"
type: Function
title: distance_to
description: Euclidean distance to another point in micrometers.
resource: crates/oxide-router/src/geometry/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:59:54Z"
concept_id: crates/oxide-router/src/geometry/mod/distance_to
language: rust
---

# distance_to

Euclidean distance to another point in micrometers.

## Signature

```rust
impl Point2D { pub fn distance_to(&self, other: Point2D) -> Microns }
```

## Visibility

- `pub`

## Docstring

Euclidean distance to another point in micrometers.

## Source
Lines 36–40 in `crates/oxide-router/src/geometry/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-router/src/geometry/mod.md) |
