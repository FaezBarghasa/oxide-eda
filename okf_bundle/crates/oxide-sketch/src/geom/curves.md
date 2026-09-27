---
okf_version: "0.2"
type: Module
title: curves
description: "Curve × curve intersections — Circle × Circle, Arc × Circle,"
resource: crates/oxide-sketch/src/geom/curves.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/curves
language: rust
---

# curves

Curve × curve intersections — Circle × Circle, Arc × Circle,

## Docstring

Curve × curve intersections — Circle × Circle, Arc × Circle,
Arc × Arc.

Built on the Circle × Circle quadratic root-finder; arc filters
reuse the angular containment helpers on `Arc2`.

## Relationships

| Type | Target |
|------|--------|
| related | [circle_circle_intersections](/crates/oxide-sketch/src/geom/curves/circle_circle_intersections.md) |
| related | [arc_circle_intersections](/crates/oxide-sketch/src/geom/curves/arc_circle_intersections.md) |
| related | [arc_arc_intersections](/crates/oxide-sketch/src/geom/curves/arc_arc_intersections.md) |
| related | [p](/crates/oxide-sketch/src/geom/curves/p.md) |
| related | [close](/crates/oxide-sketch/src/geom/curves/close.md) |
| related | [circles_two_intersection](/crates/oxide-sketch/src/geom/curves/circles_two_intersection.md) |
| related | [circles_disjoint_no_intersection](/crates/oxide-sketch/src/geom/curves/circles_disjoint_no_intersection.md) |
| related | [circles_nested_no_intersection](/crates/oxide-sketch/src/geom/curves/circles_nested_no_intersection.md) |
| related | [circles_tangent_single_point](/crates/oxide-sketch/src/geom/curves/circles_tangent_single_point.md) |
| related | [circles_concentric_returns_empty](/crates/oxide-sketch/src/geom/curves/circles_concentric_returns_empty.md) |
| related | [arc_circle_filters_outside_sweep](/crates/oxide-sketch/src/geom/curves/arc_circle_filters_outside_sweep.md) |
| related | [arc_arc_both_sweeps_contain_hit](/crates/oxide-sketch/src/geom/curves/arc_arc_both_sweeps_contain_hit.md) |
| related | [arc_arc_no_overlap_in_sweeps](/crates/oxide-sketch/src/geom/curves/arc_arc_no_overlap_in_sweeps.md) |
