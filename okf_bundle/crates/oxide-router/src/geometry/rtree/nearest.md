---
okf_version: "0.2"
type: Function
title: nearest
description: Find nearest object to a point.
resource: crates/oxide-router/src/geometry/rtree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/rtree/nearest
language: rust
---

# nearest

Find nearest object to a point.

## Signature

```rust
impl SpatialIndex { pub fn nearest(&self, point: Point2D) -> Option<&SpatialObject> }
```

## Visibility

- `pub`

## Docstring

Find nearest object to a point.

## Source
Lines 146–150 in `crates/oxide-router/src/geometry/rtree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rtree](/crates/oxide-router/src/geometry/rtree.md) |
