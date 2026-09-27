---
okf_version: "0.2"
type: Function
title: symmetric_about_point
description: "Residual for `p1` and `p2` being mirror images about the centre"
resource: crates/oxide-sketch/src/solver/residuals/symmetric_midpoint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_point
language: rust
---

# symmetric_about_point

Residual for `p1` and `p2` being mirror images about the centre

## Signature

```rust
pub fn symmetric_about_point(
    p1: SketchEntityId,
    p2: SketchEntityId,
    center: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

Residual for `p1` and `p2` being mirror images about the centre
point `C` (i.e. `C` is exactly the midpoint of `p1, p2`).

Returns two scalars:
1. `(p1.x + p2.x) / 2 − C.x`
2. `(p1.y + p2.y) / 2 − C.y`

Order: `[mid_x_eq, mid_y_eq]`.

## Source
Lines 74–89 in `crates/oxide-sketch/src/solver/residuals/symmetric_midpoint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symmetric_midpoint](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| calls | [scale](/crates/oxide-sketch/src/solver/math/scale.md) |
| calls | [add](/crates/oxide-sketch/src/solver/math/add.md) |
| calls | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
