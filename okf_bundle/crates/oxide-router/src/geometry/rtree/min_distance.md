---
okf_version: "0.2"
type: Function
title: min_distance
description: Calculate minimum distance between two spatial objects.
resource: crates/oxide-router/src/geometry/rtree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/rtree/min_distance
language: rust
---

# min_distance

Calculate minimum distance between two spatial objects.

## Signature

```rust
impl SpatialIndex { pub fn min_distance(&self, obj1: &SpatialObject, obj2: &SpatialObject) -> Microns }
```

## Visibility

- `pub`

## Docstring

Calculate minimum distance between two spatial objects.

## Source
Lines 174–176 in `crates/oxide-router/src/geometry/rtree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rtree](/crates/oxide-router/src/geometry/rtree.md) |
