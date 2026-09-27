---
okf_version: "0.2"
type: Function
title: query_point
description: "Items whose bbox contains `p`. Returns clones so callers"
resource: crates/oxide-sketch/src/geom/aabb_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/aabb_index/query_point_1
language: rust
---

# query_point

Items whose bbox contains `p`. Returns clones so callers

## Signature

```rust
pub fn query_point(&self, p: Point2) -> Vec<T>
```

## Visibility

- `pub`

## Docstring

Items whose bbox contains `p`. Returns clones so callers
can use the items after the index goes out of scope.

## Source
Lines 119–125 in `crates/oxide-sketch/src/geom/aabb_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aabb_index](/crates/oxide-sketch/src/geom/aabb_index.md) |
