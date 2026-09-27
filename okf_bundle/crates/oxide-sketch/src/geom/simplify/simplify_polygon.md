---
okf_version: "0.2"
type: Function
title: simplify_polygon
description: "One-call pipeline: snap to grid, dedup adjacent duplicates,"
resource: crates/oxide-sketch/src/geom/simplify.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/simplify/simplify_polygon
language: rust
---

# simplify_polygon

One-call pipeline: snap to grid, dedup adjacent duplicates,

## Signature

```rust
pub fn simplify_polygon(polygon: &[Point2], snap_step: f64, dedup_eps: f64) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

One-call pipeline: snap to grid, dedup adjacent duplicates,
merge colinear runs. Pre-processes a polygon for the boolean
operations.

## Source
Lines 103–107 in `crates/oxide-sketch/src/geom/simplify.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simplify](/crates/oxide-sketch/src/geom/simplify.md) |
| calls | [snap_to_grid](/crates/oxide-sketch/src/geom/simplify/snap_to_grid.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
| calls | [merge_colinear](/crates/oxide-sketch/src/geom/simplify/merge_colinear.md) |
| called_by | [simplify_pipeline_combines_all_three](/crates/oxide-sketch/src/geom/simplify/simplify_pipeline_combines_all_three.md) |
