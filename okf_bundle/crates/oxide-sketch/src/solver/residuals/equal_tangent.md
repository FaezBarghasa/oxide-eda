---
okf_version: "0.2"
type: Module
title: equal_tangent
description: Task 2.6 — EqualLength / EqualRadius / TangentLineArc / TangentArcArc
resource: crates/oxide-sketch/src/solver/residuals/equal_tangent.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/equal_tangent
language: rust
---

# equal_tangent

Task 2.6 — EqualLength / EqualRadius / TangentLineArc / TangentArcArc

## Docstring

Task 2.6 — EqualLength / EqualRadius / TangentLineArc / TangentArcArc
residuals.

Each helper returns a single scalar residual (`Vec<f64>` of length 1)
that the Levenberg–Marquardt driver in Phase 3 will drive to zero.

Formulas compose from primitives in [`crate::solver::math`]:
- Line length = `norm(end − start)`.
- Circle radius is stored explicitly; arc radius is
`distance(start, center)`.
- Line/arc tangency: the signed perpendicular distance from the
arc centre to the line equals the arc radius (in absolute
value, since the line can sit on either side of the centre).
- Arc/arc tangency:
external — `|C2 − C1| = r1 + r2`
internal — `|C2 − C1| = |r1 − r2|`

## Relationships

| Type | Target |
|------|--------|
| related | [line_length](/crates/oxide-sketch/src/solver/residuals/equal_tangent/line_length.md) |
| related | [entity_radius](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_radius.md) |
| related | [entity_center_xy](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_center_xy.md) |
| related | [equal_length](/crates/oxide-sketch/src/solver/residuals/equal_tangent/equal_length.md) |
| related | [equal_radius](/crates/oxide-sketch/src/solver/residuals/equal_tangent/equal_radius.md) |
| related | [tangent_line_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_line_arc.md) |
| related | [tangent_arc_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_arc_arc.md) |
