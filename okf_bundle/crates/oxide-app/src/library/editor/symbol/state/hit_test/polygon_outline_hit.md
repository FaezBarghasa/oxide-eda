---
okf_version: "0.2"
type: Function
title: polygon_outline_hit
description: "`true` when `(x, y)` lies within `tol` of any closed-polygon edge"
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/polygon_outline_hit
language: rust
---

# polygon_outline_hit

`true` when `(x, y)` lies within `tol` of any closed-polygon edge

## Signature

```rust
fn polygon_outline_hit(x: f64, y: f64, vertices: &[[f64; 2]], tol: f64) -> bool
```

## Docstring

`true` when `(x, y)` lies within `tol` of any closed-polygon edge
(including the implicit last-to-first segment).

## Source
Lines 186–195 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
| calls | [point_to_segment_dist_sq](/crates/oxide-app/src/library/editor/symbol/state/hit_test/point_to_segment_dist_sq.md) |
| called_by | [hit_test_graphic_body](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_body.md) |
