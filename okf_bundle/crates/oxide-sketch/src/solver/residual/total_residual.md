---
okf_version: "0.2"
type: Function
title: total_residual
description: Total residual vector for an entire sketch — concatenates the
resource: crates/oxide-sketch/src/solver/residual.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residual/total_residual
language: rust
---

# total_residual

Total residual vector for an entire sketch — concatenates the

## Signature

```rust
pub fn total_residual(
    sketch: &SketchData,
    state: &[f64],
    index: &EntityIndex,
    params: &ResolvedParams,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

Total residual vector for an entire sketch — concatenates the
per-constraint residual vectors in `sketch.constraints` order.

`total_residual` is the function the Levenberg–Marquardt iteration
(Phase 3) drives toward zero. The output length is the sum of
`ConstraintKind::residual_count()` across all constraints. The
state vector length is unrelated; the Jacobian is `(m × n)` where
`m = total_residual.len()` and `n = state.len()`.

## Source
Lines 105–116 in `crates/oxide-sketch/src/solver/residual.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [residual](/crates/oxide-sketch/src/solver/residual.md) |
| calls | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
| called_by | [numerical_jacobian](/crates/oxide-sketch/src/solver/jacobian/numerical_jacobian.md) |
| called_by | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| called_by | [total_residual_concatenates_per_constraint](/crates/oxide-sketch/tests/constraints_basics/total_residual_concatenates_per_constraint.md) |
| called_by | [total_residual_length_matches_constraint_kind_count_sum](/crates/oxide-sketch/tests/constraints_basics/total_residual_length_matches_constraint_kind_count_sum.md) |
| called_by | [dof_over_constrained_marks_red](/crates/oxide-sketch/tests/dof/dof_over_constrained_marks_red.md) |
| called_by | [dof_parametric_over_constraint_marks_red](/crates/oxide-sketch/tests/dof/dof_parametric_over_constraint_marks_red.md) |
