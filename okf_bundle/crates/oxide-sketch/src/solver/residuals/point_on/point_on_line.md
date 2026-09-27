---
okf_version: "0.2"
type: Function
title: point_on_line
description: "PointOnLine: signed perpendicular distance from `point` to the"
resource: crates/oxide-sketch/src/solver/residuals/point_on.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/point_on/point_on_line
language: rust
---

# point_on_line

PointOnLine: signed perpendicular distance from `point` to the

## Signature

```rust
pub fn point_on_line(
    point: SketchEntityId,
    line: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

PointOnLine: signed perpendicular distance from `point` to the
infinite line through `line`'s endpoints. Zero when `point` sits
on the line.

A degenerate line (`|B − A| < ε`) is treated as a malformed
entity and reported via `SketchError::EntityNotFound(line)`.

## Source
Lines 70–80 in `crates/oxide-sketch/src/solver/residuals/point_on.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [point_on](/crates/oxide-sketch/src/solver/residuals/point_on.md) |
| calls | [point_and_line](/crates/oxide-sketch/src/solver/residuals/point_on/point_and_line.md) |
| calls | [signed_perp_distance](/crates/oxide-sketch/src/solver/residuals/point_on/signed_perp_distance.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
