---
okf_version: "0.2"
type: Function
title: concave_contour_fills_exactly_its_own_area
description: Correctness — a concave contour (e.g. an L-shaped copper pour) must
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/concave_contour_fills_exactly_its_own_area
language: rust
---

# concave_contour_fills_exactly_its_own_area

Correctness — a concave contour (e.g. an L-shaped copper pour) must

## Signature

```rust
fn concave_contour_fills_exactly_its_own_area()
```

## Decorators

- `test`

## Docstring

Correctness — a concave contour (e.g. an L-shaped copper pour) must
fill EXACTLY its own area, no more and no less. A triangle-fan from
`points[0]` bridges triangles across the notch and over-fills; ear-clip
partitions the polygon exactly, so the fill vertices' total triangle
area must equal the contour's shoelace area. This is the regression
test for the GPU-paints-copper-where-the-pour-has-none bug.
[test]

## Source
Lines 448–482 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| calls | [triangulate_polygons](/crates/oxide-gfx/src/pipeline/polygon/triangulate_polygons.md) |
| calls | [shoelace_area](/crates/oxide-gfx/src/pipeline/polygon/shoelace_area.md) |
| calls | [triangle_area](/crates/oxide-gfx/src/pipeline/polygon/triangle_area.md) |
