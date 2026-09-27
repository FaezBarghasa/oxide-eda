---
okf_version: "0.2"
type: Function
title: query_region
description: "Items whose bbox overlaps `region`. Iterator-style so"
resource: crates/oxide-sketch/src/geom/aabb_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/aabb_index/query_region_1
language: rust
---

# query_region

Items whose bbox overlaps `region`. Iterator-style so

## Signature

```rust
pub fn query_region(&'a self, region: Aabb) -> impl Iterator<Item = &'a T> + 'a
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Items whose bbox overlaps `region`. Iterator-style so
callers that just want to walk hits don't pay the Vec
allocation.

## Source
Lines 130–138 in `crates/oxide-sketch/src/geom/aabb_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aabb_index](/crates/oxide-sketch/src/geom/aabb_index.md) |
