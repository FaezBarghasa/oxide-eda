---
okf_version: "0.2"
type: Module
title: parallel_perp_angle
description: Task 2.4 — Parallel / Perpendicular / Angle residuals.
resource: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle
language: rust
---

# parallel_perp_angle

Task 2.4 — Parallel / Perpendicular / Angle residuals.

## Docstring

Task 2.4 — Parallel / Perpendicular / Angle residuals.

Each helper returns a single scalar residual (`Vec<f64>` of length 1)
that the Levenberg–Marquardt driver in Phase 3 will drive to zero.

All formulas are composed from primitives in
[`crate::solver::math`]:
- `cross` — two lines are parallel iff their direction-vector
cross is zero.
- `dot` — two lines are perpendicular iff their direction-vector
dot is zero.
- `wrap_to_pi` — the signed CCW angle from `d1` to `d2` is
`atan2(cross, dot)`; the residual is wrapped into `(−π, π]`
so a sketch crossing the ±π branch cut sees a continuous
derivative.

## Relationships

| Type | Target |
|------|--------|
| related | [line_dir](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/line_dir.md) |
| related | [parallel](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/parallel.md) |
| related | [perpendicular](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/perpendicular.md) |
| related | [angle](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/angle.md) |
