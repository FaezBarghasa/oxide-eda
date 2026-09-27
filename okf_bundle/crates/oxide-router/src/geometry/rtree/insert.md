---
okf_version: "0.2"
type: Function
title: insert
description: Insert an object into the spatial index.
resource: crates/oxide-router/src/geometry/rtree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/rtree/insert
language: rust
---

# insert

Insert an object into the spatial index.

## Signature

```rust
impl SpatialIndex { pub fn insert(&mut self, object: SpatialObject) }
```

## Visibility

- `pub`

## Docstring

Insert an object into the spatial index.

## Source
Lines 48–50 in `crates/oxide-router/src/geometry/rtree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rtree](/crates/oxide-router/src/geometry/rtree.md) |
