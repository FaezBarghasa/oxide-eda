---
okf_version: "0.2"
type: Function
title: tangent_arc_arc
description: "TangentArcArc:"
resource: crates/oxide-sketch/src/solver/residuals/equal_tangent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_arc_arc
language: rust
---

# tangent_arc_arc

TangentArcArc:

## Signature

```rust
pub fn tangent_arc_arc(
    a1: SketchEntityId,
    a2: SketchEntityId,
    internal: bool,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

TangentArcArc:
external (`internal = false`): `|C2 − C1| − (r1 + r2) = 0`.
internal (`internal = true`):  `|C2 − C1| − |r1 − r2| = 0`.

Each entity may be Arc or Circle (dispatch via
[`entity_center_xy`] + [`entity_radius`]).

## Source
Lines 148–164 in `crates/oxide-sketch/src/solver/residuals/equal_tangent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [equal_tangent](/crates/oxide-sketch/src/solver/residuals/equal_tangent.md) |
| calls | [entity_center_xy](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_center_xy.md) |
| calls | [entity_radius](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_radius.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
