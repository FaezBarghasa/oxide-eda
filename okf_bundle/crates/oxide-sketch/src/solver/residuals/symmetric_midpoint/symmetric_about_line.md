---
okf_version: "0.2"
type: Function
title: symmetric_about_line
description: "Residual for `p1` and `p2` being mirror images of each other"
resource: crates/oxide-sketch/src/solver/residuals/symmetric_midpoint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_line
language: rust
---

# symmetric_about_line

Residual for `p1` and `p2` being mirror images of each other

## Signature

```rust
pub fn symmetric_about_line(
    p1: SketchEntityId,
    p2: SketchEntityId,
    line: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

Residual for `p1` and `p2` being mirror images of each other
across the infinite line through the line entity's endpoints
`A, B`. Returns two scalars:

1. **midpoint-on-line**: signed perpendicular distance from the
midpoint `M = (p1 + p2) / 2` to the line. Computed as the 2D
cross product `cross(M − A, d) / |d|`, where `d = B − A`.
Zero iff `M` lies on the line.
2. **perpendicular-to-line**: the dot product `dot(p2 − p1, d)`.
Zero iff the segment `p1p2` is perpendicular to the line
direction.

Together these two scalars vanish iff `p1` and `p2` are
reflections of each other across the line.

Order: `[midpoint_on_line, perp_to_line]`.

## Source
Lines 29–64 in `crates/oxide-sketch/src/solver/residuals/symmetric_midpoint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symmetric_midpoint](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| calls | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
| calls | [scale](/crates/oxide-sketch/src/solver/math/scale.md) |
| calls | [add](/crates/oxide-sketch/src/solver/math/add.md) |
| calls | [cross](/crates/oxide-sketch/src/solver/math/cross.md) |
| calls | [norm_sq_2](/crates/oxide-sketch/src/solver/math/norm_sq_2.md) |
| calls | [dot](/crates/oxide-sketch/src/solver/math/dot.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
