---
okf_version: "0.2"
type: Module
title: dof
description: DOF analysis — rank-based per-entity colour classification.
resource: crates/oxide-sketch/src/solver/dof.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-sketch/src/solver/dof
language: rust
---

# dof

DOF analysis — rank-based per-entity colour classification.

## Docstring

DOF analysis — rank-based per-entity colour classification.

After solve, examine the Jacobian to classify each entity:
- **Under**-constrained (blue) — the entity has free DoF the
constraint set didn't pin down.
- **Full** (black) — fully constrained.
- **Over**-constrained (red) — the entity participates in
redundant or conflicting constraints.

Strategy:
1. Compute the QR factorisation of the Jacobian J (m × n).
2. The numerical rank `r = rank(J, tol)` tells us how many of the
`n` state variables are pinned by the constraint set. Effective
free DoF = `n − r`.
3. For per-entity colouring, examine the columns of J belonging
to that entity. If those columns are full-column-rank in J,
the entity is fully constrained (black); otherwise it has
free DoF (blue).
4. For per-constraint over-detection: a constraint whose row is
in the rank-deficient null-space of J^T AND whose residual is
larger than `tol` after solve is over-constrained (red).

Reference: *Numerical Recipes* (Press et al., 3rd ed.) §2.10
("QR Decomposition") for the rank computation. Algorithm derived
from first principles — no third-party numerical-library or
constraint-solver source consulted.

## Relationships

| Type | Target |
|------|--------|
| related | [DofColor](/crates/oxide-sketch/src/solver/dof/DofColor.md) |
| related | [entity_colours](/crates/oxide-sketch/src/solver/dof/entity_colours.md) |
| related | [over_constraint_ids](/crates/oxide-sketch/src/solver/dof/over_constraint_ids.md) |
| related | [points_touched](/crates/oxide-sketch/src/solver/dof/points_touched.md) |
| related | [extend_with_entity_points](/crates/oxide-sketch/src/solver/dof/extend_with_entity_points.md) |
