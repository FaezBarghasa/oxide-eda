---
okf_version: "0.2"
type: Function
title: p
resource: crates/oxide-sketch/src/geom/curves.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/curves/p
language: rust
---

# p

## Signature

```rust
fn p(x: f64, y: f64) -> Point2
```

## Source
Lines 88–90 in `crates/oxide-sketch/src/geom/curves.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [curves](/crates/oxide-sketch/src/geom/curves.md) |
| called_by | [arc_arc_both_sweeps_contain_hit](/crates/oxide-sketch/src/geom/curves/arc_arc_both_sweeps_contain_hit.md) |
| called_by | [arc_arc_no_overlap_in_sweeps](/crates/oxide-sketch/src/geom/curves/arc_arc_no_overlap_in_sweeps.md) |
| called_by | [arc_circle_filters_outside_sweep](/crates/oxide-sketch/src/geom/curves/arc_circle_filters_outside_sweep.md) |
| called_by | [circles_concentric_returns_empty](/crates/oxide-sketch/src/geom/curves/circles_concentric_returns_empty.md) |
| called_by | [circles_disjoint_no_intersection](/crates/oxide-sketch/src/geom/curves/circles_disjoint_no_intersection.md) |
| called_by | [circles_nested_no_intersection](/crates/oxide-sketch/src/geom/curves/circles_nested_no_intersection.md) |
| called_by | [circles_tangent_single_point](/crates/oxide-sketch/src/geom/curves/circles_tangent_single_point.md) |
| called_by | [circles_two_intersection](/crates/oxide-sketch/src/geom/curves/circles_two_intersection.md) |
