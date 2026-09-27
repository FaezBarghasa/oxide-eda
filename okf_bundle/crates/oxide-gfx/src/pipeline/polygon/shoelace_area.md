---
okf_version: "0.2"
type: Function
title: shoelace_area
description: Shoelace area of a closed contour — the ground truth a correct
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/shoelace_area
language: rust
---

# shoelace_area

Shoelace area of a closed contour — the ground truth a correct

## Signature

```rust
fn shoelace_area(points: &[[f32; 2]]) -> f64
```

## Docstring

Shoelace area of a closed contour — the ground truth a correct
triangulation's total triangle area must equal exactly, whether the
contour is convex or concave.

## Source
Lines 391–399 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| called_by | [concave_contour_fills_exactly_its_own_area](/crates/oxide-gfx/src/pipeline/polygon/concave_contour_fills_exactly_its_own_area.md) |
| called_by | [triangle_area](/crates/oxide-gfx/src/pipeline/polygon/triangle_area.md) |
