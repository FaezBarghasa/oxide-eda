---
okf_version: "0.2"
type: Function
title: query_radius
description: Query objects within a radius from center point.
resource: crates/oxide-router/src/geometry/rtree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/rtree/query_radius_1
language: rust
---

# query_radius

Query objects within a radius from center point.

## Signature

```rust
pub fn query_radius(&self, center: Point2D, radius: Microns) -> Vec<&SpatialObject>
```

## Visibility

- `pub`

## Docstring

Query objects within a radius from center point.

## Source
Lines 135–143 in `crates/oxide-router/src/geometry/rtree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rtree](/crates/oxide-router/src/geometry/rtree.md) |
