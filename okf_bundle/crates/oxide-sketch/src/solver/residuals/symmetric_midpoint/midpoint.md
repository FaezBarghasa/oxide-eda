---
okf_version: "0.2"
type: Function
title: midpoint
description: "Residual for point `P` being the midpoint of the line entity's"
resource: crates/oxide-sketch/src/solver/residuals/symmetric_midpoint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/midpoint
language: rust
---

# midpoint

Residual for point `P` being the midpoint of the line entity's

## Signature

```rust
pub fn midpoint(
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

Residual for point `P` being the midpoint of the line entity's
endpoints `A, B`.

Returns two scalars:
1. `P.x − (A.x + B.x) / 2`
2. `P.y − (A.y + B.y) / 2`

Order: `[mid_x_match, mid_y_match]`.

## Source
Lines 99–115 in `crates/oxide-sketch/src/solver/residuals/symmetric_midpoint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symmetric_midpoint](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| calls | [scale](/crates/oxide-sketch/src/solver/math/scale.md) |
| calls | [add](/crates/oxide-sketch/src/solver/math/add.md) |
| calls | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
