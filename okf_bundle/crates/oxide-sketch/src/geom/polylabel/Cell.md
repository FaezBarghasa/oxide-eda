---
okf_version: "0.2"
type: Class
title: Cell
description: "Centre point + half-diagonal for one quadtree cell, plus the"
resource: crates/oxide-sketch/src/geom/polylabel.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/polylabel/Cell
language: rust
---

# Cell

Centre point + half-diagonal for one quadtree cell, plus the

## Signature

```rust
struct Cell
```

## Decorators

- `derive(Debug, Clone, Copy)`

## Docstring

Centre point + half-diagonal for one quadtree cell, plus the
signed distance from `centre` to the polygon. Stored on the
priority queue so the next cell to subdivide is always the
one with the highest possible future improvement.
[derive(Debug, Clone, Copy)]

## Methods

- `centre`
- `half`
- `distance`
- `upper_bound`

## Source
Lines 34–41 in `crates/oxide-sketch/src/geom/polylabel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polylabel](/crates/oxide-sketch/src/geom/polylabel.md) |
