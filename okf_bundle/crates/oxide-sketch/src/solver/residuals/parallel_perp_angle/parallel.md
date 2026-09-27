---
okf_version: "0.2"
type: Function
title: parallel
description: "Parallel: `cross(d1, d2) = 0`."
resource: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/parallel
language: rust
---

# parallel

Parallel: `cross(d1, d2) = 0`.

## Signature

```rust
pub fn parallel(
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

Parallel: `cross(d1, d2) = 0`.

Zero whether the lines point the same way or are antiparallel; the
solver doesn't need to distinguish, since both cases satisfy the
"same line direction modulo sign" definition of parallelism.

## Source
Lines 43–53 in `crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_perp_angle](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.md) |
| calls | [line_dir](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/line_dir.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
