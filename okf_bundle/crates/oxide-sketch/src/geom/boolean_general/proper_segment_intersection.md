---
okf_version: "0.2"
type: Function
title: proper_segment_intersection
description: Strict-interior segment×segment intersection. Returns the hit
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general/proper_segment_intersection
language: rust
---

# proper_segment_intersection

Strict-interior segment×segment intersection. Returns the hit

## Signature

```rust
fn proper_segment_intersection(
    a: Point2,
    b: Point2,
    c: Point2,
    d: Point2,
) -> Option<(Point2, f64, f64)>
```

## Docstring

Strict-interior segment×segment intersection. Returns the hit
point and the parametric `(t_subject, t_clip)` only when both
parameters lie strictly inside `(0, 1)` — endpoints don't
count as proper intersections in Greiner-Hormann.

## Source
Lines 459–480 in `crates/oxide-sketch/src/geom/boolean_general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean_general](/crates/oxide-sketch/src/geom/boolean_general.md) |
| called_by | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
