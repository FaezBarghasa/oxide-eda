---
okf_version: "0.2"
type: Function
title: merge_colinear
description: Drop the middle vertex from any colinear consecutive triple
resource: crates/oxide-sketch/src/geom/simplify.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/simplify/merge_colinear
language: rust
---

# merge_colinear

Drop the middle vertex from any colinear consecutive triple

## Signature

```rust
pub fn merge_colinear(polygon: &[Point2]) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Drop the middle vertex from any colinear consecutive triple
(a, b, c) where b lies on segment a→c within `eps`. After this
pass the polygon has only "essential" corners — no spurious
midpoints on a straight edge.

## Source
Lines 67–83 in `crates/oxide-sketch/src/geom/simplify.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simplify](/crates/oxide-sketch/src/geom/simplify.md) |
| called_by | [merge_colinear_drops_midpoints](/crates/oxide-sketch/src/geom/simplify/merge_colinear_drops_midpoints.md) |
| called_by | [simplify_polygon](/crates/oxide-sketch/src/geom/simplify/simplify_polygon.md) |
