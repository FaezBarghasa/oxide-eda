---
okf_version: "0.2"
type: Function
title: line_dir
description: "Resolve a line's direction vector `d = end − start` from the"
resource: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/line_dir
language: rust
---

# line_dir

Resolve a line's direction vector `d = end − start` from the

## Signature

```rust
fn line_dir(
    line: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec2, SketchError>
```

## Docstring

Resolve a line's direction vector `d = end − start` from the
current state vector. Returns `EntityNotFound` if the line itself
or either endpoint cannot be resolved.

## Source
Lines 26–36 in `crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_perp_angle](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| calls | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
| called_by | [angle](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/angle.md) |
| called_by | [parallel](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/parallel.md) |
| called_by | [perpendicular](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/perpendicular.md) |
