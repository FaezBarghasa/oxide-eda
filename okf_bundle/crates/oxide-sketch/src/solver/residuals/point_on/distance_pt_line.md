---
okf_version: "0.2"
type: Function
title: distance_pt_line
description: "DistancePtLine: signed perpendicular distance from `point` to the"
resource: crates/oxide-sketch/src/solver/residuals/point_on.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/point_on/distance_pt_line
language: rust
---

# distance_pt_line

DistancePtLine: signed perpendicular distance from `point` to the

## Signature

```rust
pub fn distance_pt_line(
    point: SketchEntityId,
    line: SketchEntityId,
    target_mm: f64,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

DistancePtLine: signed perpendicular distance from `point` to the
infinite line, minus `target_mm`. Zero when `point` is exactly
`target_mm` away on the line's right-hand side (cross-product
sign convention; left-hand side requires negative target).

At `target_mm = 0` this reduces exactly to [`point_on_line`].

## Source
Lines 110–121 in `crates/oxide-sketch/src/solver/residuals/point_on.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [point_on](/crates/oxide-sketch/src/solver/residuals/point_on.md) |
| calls | [point_and_line](/crates/oxide-sketch/src/solver/residuals/point_on/point_and_line.md) |
| calls | [signed_perp_distance](/crates/oxide-sketch/src/solver/residuals/point_on/signed_perp_distance.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
