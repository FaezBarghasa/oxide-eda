---
okf_version: "0.2"
type: Module
title: geom
description: 2D computational-geometry primitives for the sketch crate.
resource: crates/oxide-sketch/src/geom/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/mod
language: rust
---

# geom

2D computational-geometry primitives for the sketch crate.

## Docstring

2D computational-geometry primitives for the sketch crate.

Submodules:
- [`predicates`] — orientation + signed area predicates with
epsilon-aware sign returns.
- [`segment`] — segment×segment, segment×circle, segment×arc
intersection. Drives the snap "Intersection" priority and
the constraint solver's residual computation for tangency /
intersection constraints.
- [`hull`] — convex hull, O(n log n) on the input size.
- [`triangulate`] — polygon triangulation for the sketch
overlay's filled-loop renderer.

Public entry point: re-exports below.

## Relationships

| Type | Target |
|------|--------|
| related | [Point2](/crates/oxide-sketch/src/geom/mod/Point2.md) |
| related | [new](/crates/oxide-sketch/src/geom/mod/new.md) |
| related | [distance_sq](/crates/oxide-sketch/src/geom/mod/distance_sq.md) |
| related | [distance](/crates/oxide-sketch/src/geom/mod/distance.md) |
| related | [new](/crates/oxide-sketch/src/geom/mod/new.md) |
| related | [distance_sq](/crates/oxide-sketch/src/geom/mod/distance_sq.md) |
| related | [distance](/crates/oxide-sketch/src/geom/mod/distance.md) |
| related | [from](/crates/oxide-sketch/src/geom/mod/from.md) |
| related | [from](/crates/oxide-sketch/src/geom/mod/from.md) |
| related | [from](/crates/oxide-sketch/src/geom/mod/from.md) |
| related | [from](/crates/oxide-sketch/src/geom/mod/from.md) |
| related | [from](/crates/oxide-sketch/src/geom/mod/from.md) |
| related | [from](/crates/oxide-sketch/src/geom/mod/from.md) |
| related | [from](/crates/oxide-sketch/src/geom/mod/from.md) |
| related | [from](/crates/oxide-sketch/src/geom/mod/from.md) |
