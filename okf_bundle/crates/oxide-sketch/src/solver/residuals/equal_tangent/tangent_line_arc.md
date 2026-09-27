---
okf_version: "0.2"
type: Function
title: tangent_line_arc
description: "TangentLineArc: perpendicular distance from the arc centre to the"
resource: crates/oxide-sketch/src/solver/residuals/equal_tangent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_line_arc
language: rust
---

# tangent_line_arc

TangentLineArc: perpendicular distance from the arc centre to the

## Signature

```rust
pub fn tangent_line_arc(
    line: SketchEntityId,
    arc: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

TangentLineArc: perpendicular distance from the arc centre to the
line equals the arc radius.

Residual = `|signed_perp_dist| − r_arc`. The line can sit on
either side of the centre and still be tangent, so the absolute
value is required. A degenerate (zero-length) line collapses to
`0 − r` so LM still has gradient information to grow the line.

## Source
Lines 117–140 in `crates/oxide-sketch/src/solver/residuals/equal_tangent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [equal_tangent](/crates/oxide-sketch/src/solver/residuals/equal_tangent.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| calls | [entity_center_xy](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_center_xy.md) |
| calls | [entity_radius](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_radius.md) |
| calls | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
| calls | [norm](/crates/oxide-sketch/src/solver/math/norm.md) |
| calls | [cross](/crates/oxide-sketch/src/solver/math/cross.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
