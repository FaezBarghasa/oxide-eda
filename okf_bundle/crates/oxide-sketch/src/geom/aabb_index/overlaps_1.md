---
okf_version: "0.2"
type: Function
title: overlaps
description: "`true` when this box overlaps `other` — inclusive on the"
resource: crates/oxide-sketch/src/geom/aabb_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/aabb_index/overlaps_1
language: rust
---

# overlaps

`true` when this box overlaps `other` — inclusive on the

## Signature

```rust
pub fn overlaps(&self, other: Aabb) -> bool
```

## Visibility

- `pub`

## Docstring

`true` when this box overlaps `other` — inclusive on the
shared boundary.

## Source
Lines 65–70 in `crates/oxide-sketch/src/geom/aabb_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aabb_index](/crates/oxide-sketch/src/geom/aabb_index.md) |
