---
okf_version: "0.2"
type: Function
title: insert
resource: crates/oxide-sketch/src/geom/aabb_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/aabb_index/insert
language: rust
---

# insert

## Signature

```rust
impl AabbIndex<T> { pub fn insert(&mut self, item: T, bbox: Aabb) }
```

## Type Parameters

- `T: Clone`

## Visibility

- `pub`

## Source
Lines 101–103 in `crates/oxide-sketch/src/geom/aabb_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aabb_index](/crates/oxide-sketch/src/geom/aabb_index.md) |
