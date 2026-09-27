---
okf_version: "0.2"
type: Module
title: point_on
description: Task 2.5 — PointOnLine / PointOnArc / DistancePtLine residuals.
resource: crates/oxide-sketch/src/solver/residuals/point_on.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/point_on
language: rust
---

# point_on

Task 2.5 — PointOnLine / PointOnArc / DistancePtLine residuals.

## Docstring

Task 2.5 — PointOnLine / PointOnArc / DistancePtLine residuals.

Each helper returns a single scalar residual (`Vec<f64>` of length 1)
that the Levenberg–Marquardt driver in Phase 3 will drive to zero.

Formulas are composed from primitives in [`crate::solver::math`]:
- The signed perpendicular distance from `P` to the infinite
line through `A,B` is `cross(P − A, B − A) / |B − A|` —
`cross` for the signed area, `norm` for the segment length.
The 2D cross product gives the side of the line as well as
the magnitude.
- For an arc, the underlying circle has radius
`|start − center|`, so the point-on-arc residual reduces to
`|P − center| − |start − center|`.

Sign convention: the perpendicular-distance residual is signed
(the 2D cross product gives the "side" of the line as well as
the magnitude). The solver needs the sign so it can drive `P`
from either side of the line; bake/UI layers take the absolute
value when they need an unsigned distance.

## Relationships

| Type | Target |
|------|--------|
| related | [signed_perp_distance](/crates/oxide-sketch/src/solver/residuals/point_on/signed_perp_distance.md) |
| related | [point_and_line](/crates/oxide-sketch/src/solver/residuals/point_on/point_and_line.md) |
| related | [point_on_line](/crates/oxide-sketch/src/solver/residuals/point_on/point_on_line.md) |
| related | [point_on_arc](/crates/oxide-sketch/src/solver/residuals/point_on/point_on_arc.md) |
| related | [distance_pt_line](/crates/oxide-sketch/src/solver/residuals/point_on/distance_pt_line.md) |
| related | [distance_pt_circle](/crates/oxide-sketch/src/solver/residuals/point_on/distance_pt_circle.md) |
