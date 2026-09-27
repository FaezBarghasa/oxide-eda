---
okf_version: "0.2"
type: Function
title: clip_against_edge
description: "Clip `subject` (a closed ring) against a single half-plane"
resource: crates/oxide-sketch/src/geom/boolean.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean/clip_against_edge
language: rust
---

# clip_against_edge

Clip `subject` (a closed ring) against a single half-plane

## Signature

```rust
fn clip_against_edge(subject: &[Point2], a: Point2, b: Point2) -> Vec<Point2>
```

## Docstring

Clip `subject` (a closed ring) against a single half-plane
defined by the directed edge `a → b`. Inside = left of the
edge (CCW convention). Returns a new ring.

## Source
Lines 64–96 in `crates/oxide-sketch/src/geom/boolean.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean](/crates/oxide-sketch/src/geom/boolean.md) |
| calls | [is_inside](/crates/oxide-sketch/src/geom/boolean/is_inside.md) |
| calls | [line_edge_intersection](/crates/oxide-sketch/src/geom/boolean/line_edge_intersection.md) |
| called_by | [intersect_convex_clip](/crates/oxide-sketch/src/geom/boolean/intersect_convex_clip.md) |
