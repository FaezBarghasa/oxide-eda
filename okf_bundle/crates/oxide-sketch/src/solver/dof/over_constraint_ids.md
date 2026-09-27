---
okf_version: "0.2"
type: Function
title: over_constraint_ids
description: Constraints whose residual norm at the solved state exceeds
resource: crates/oxide-sketch/src/solver/dof.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-sketch/src/solver/dof/over_constraint_ids
language: rust
---

# over_constraint_ids

Constraints whose residual norm at the solved state exceeds

## Signature

```rust
pub fn over_constraint_ids(
    sketch: &SketchData,
    solve_result: &SolveResult,
    _jacobian: &[Vec<f64>],
    params: &crate::solver::residual::ResolvedParams,
) -> Vec<ConstraintId>
```

## Visibility

- `pub`

## Docstring

Constraints whose residual norm at the solved state exceeds
`RANK_TOL`. By construction LM drives `|r|² < tolerance²`
(default `1e-12² = 1e-24`), so any per-constraint residual still
above `RANK_TOL` after a successful solve indicates LM couldn't
satisfy that constraint — it's redundant or conflicting and should
be flagged red.

The Jacobian is accepted as a parameter for future per-row null-
space analysis (full rank-deficiency detection); the conservative
rule used here only needs the residual magnitude.

HI-14: `params` MUST be the resolved parameter map from the same
solve that produced `solve_result`. An empty map causes every
`DistancePtPt` / `Angle` / etc. constraint with a parametric target
to evaluate to `ExprError::Unknown`, get caught by `Err(_) =>
continue`, and silently miss real over-constraints.

## Source
Lines 170–214 in `crates/oxide-sketch/src/solver/dof.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dof](/crates/oxide-sketch/src/solver/dof.md) |
| calls | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
| called_by | [entity_colours](/crates/oxide-sketch/src/solver/dof/entity_colours.md) |
| called_by | [solve](/crates/oxide-sketch/src/solver/mod/solve.md) |
| called_by | [dof_over_constrained_marks_red](/crates/oxide-sketch/tests/dof/dof_over_constrained_marks_red.md) |
| called_by | [dof_parametric_over_constraint_marks_red](/crates/oxide-sketch/tests/dof/dof_parametric_over_constraint_marks_red.md) |
