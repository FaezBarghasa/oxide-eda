---
okf_version: "0.2"
type: Class
title: AabbIndex
description: "Flat-array AABB index. The user inserts `(item, bbox)` pairs;"
resource: crates/oxide-sketch/src/geom/aabb_index.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/aabb_index/AabbIndex
language: rust
---

# AabbIndex

Flat-array AABB index. The user inserts `(item, bbox)` pairs;

## Signature

```rust
pub struct AabbIndex
```

## Type Parameters

- `T`

## Decorators

- `derive(Debug, Default, Clone)`

## Visibility

- `pub`

## Docstring

Flat-array AABB index. The user inserts `(item, bbox)` pairs;
the index rebuilds after each batch of inserts. Queries return
the items whose bbox intersects the query region.
[derive(Debug, Default, Clone)]

## Methods

- `items`

## Source
Lines 86–88 in `crates/oxide-sketch/src/geom/aabb_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aabb_index](/crates/oxide-sketch/src/geom/aabb_index.md) |
