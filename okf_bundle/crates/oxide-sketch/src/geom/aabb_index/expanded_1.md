---
okf_version: "0.2"
type: Function
title: expanded
description: "Expand the box by `pad` in every direction. Used to query"
resource: crates/oxide-sketch/src/geom/aabb_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/aabb_index/expanded_1
language: rust
---

# expanded

Expand the box by `pad` in every direction. Used to query

## Signature

```rust
pub fn expanded(&self, pad: f64) -> Self
```

## Visibility

- `pub`

## Docstring

Expand the box by `pad` in every direction. Used to query
"anything within `pad` of this point" via point-vs-box.

## Source
Lines 74–79 in `crates/oxide-sketch/src/geom/aabb_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aabb_index](/crates/oxide-sketch/src/geom/aabb_index.md) |
