---
okf_version: "0.2"
type: Function
title: shoelace_area
description: Shoelace area of a closed contour — the ground truth a correct
resource: crates/oxide-gfx/src/scene/scenario_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/scene/scenario_tests/shoelace_area
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
Lines 242–250 in `crates/oxide-gfx/src/scene/scenario_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scenario_tests](/crates/oxide-gfx/src/scene/scenario_tests.md) |
| called_by | [concave_zone_fills_exactly_its_area](/crates/oxide-gfx/src/scene/scenario_tests/concave_zone_fills_exactly_its_area.md) |
| called_by | [triangle_area](/crates/oxide-gfx/src/scene/scenario_tests/triangle_area.md) |
