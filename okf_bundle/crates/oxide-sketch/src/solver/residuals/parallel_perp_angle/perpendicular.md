---
okf_version: "0.2"
type: Function
title: perpendicular
description: "Perpendicular: `dot(d1, d2) = 0`."
resource: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/perpendicular
language: rust
---

# perpendicular

Perpendicular: `dot(d1, d2) = 0`.

## Signature

```rust
pub fn perpendicular(
    l1: SketchEntityId,
    l2: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

Perpendicular: `dot(d1, d2) = 0`.

## Source
Lines 56–66 in `crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_perp_angle](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.md) |
| calls | [line_dir](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/line_dir.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
