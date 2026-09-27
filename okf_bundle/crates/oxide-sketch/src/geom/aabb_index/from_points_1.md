---
okf_version: "0.2"
type: Function
title: from_points
description: "Build an Aabb covering all `points`. Returns `None` for an"
resource: crates/oxide-sketch/src/geom/aabb_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/aabb_index/from_points_1
language: rust
---

# from_points

Build an Aabb covering all `points`. Returns `None` for an

## Signature

```rust
pub fn from_points(points: &[Point2]) -> Option<Self>
```

## Visibility

- `pub`

## Docstring

Build an Aabb covering all `points`. Returns `None` for an
empty slice.

## Source
Lines 38–57 in `crates/oxide-sketch/src/geom/aabb_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aabb_index](/crates/oxide-sketch/src/geom/aabb_index.md) |
