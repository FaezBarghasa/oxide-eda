---
okf_version: "0.2"
type: Function
title: polygon_outline_hit
description: "v0.18.25 — `true` when the point lies within `tol` of any closed-"
resource: crates/oxide-app/src/library/editor/footprint/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/geometry/polygon_outline_hit
language: rust
---

# polygon_outline_hit

v0.18.25 — `true` when the point lies within `tol` of any closed-

## Signature

```rust
pub(super) fn polygon_outline_hit(px: f64, py: f64, vertices: &[[f64; 2]], tol: f64) -> bool
```

## Visibility

- `pub(super)`

## Docstring

v0.18.25 — `true` when the point lies within `tol` of any closed-
polygon edge (including the implicit last-to-first segment).

## Source
Lines 43–60 in `crates/oxide-app/src/library/editor/footprint/canvas/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/canvas/geometry.md) |
| calls | [point_to_segment_dist](/crates/oxide-app/src/library/editor/footprint/canvas/geometry/point_to_segment_dist.md) |
