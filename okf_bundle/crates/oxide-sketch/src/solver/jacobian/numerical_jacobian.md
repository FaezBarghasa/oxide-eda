---
okf_version: "0.2"
type: Function
title: numerical_jacobian
description: "Compute the (m × n) Jacobian of the total residual at `state`."
resource: crates/oxide-sketch/src/solver/jacobian.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/jacobian/numerical_jacobian
language: rust
---

# numerical_jacobian

Compute the (m × n) Jacobian of the total residual at `state`.

## Signature

```rust
pub fn numerical_jacobian(
    sketch: &SketchData,
    state: &[f64],
    index: &EntityIndex,
    params: &ResolvedParams,
) -> Result<Vec<Vec<f64>>, SketchError>
```

## Visibility

- `pub`

## Docstring

Compute the (m × n) Jacobian of the total residual at `state`.
`m = total_residual.len()`; `n = state.len()`. The returned matrix
is row-major (`j[row][col]`).

Uses central differences. Each column requires two `total_residual`
evaluations, so the cost is `2n` residual evaluations per call.

## Source
Lines 31–76 in `crates/oxide-sketch/src/solver/jacobian.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [jacobian](/crates/oxide-sketch/src/solver/jacobian.md) |
| calls | [total_residual](/crates/oxide-sketch/src/solver/residual/total_residual.md) |
| called_by | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| called_by | [solve](/crates/oxide-sketch/src/solver/mod/solve.md) |
| called_by | [dof_fully_constrained_marks_black](/crates/oxide-sketch/tests/dof/dof_fully_constrained_marks_black.md) |
| called_by | [dof_over_constrained_marks_red](/crates/oxide-sketch/tests/dof/dof_over_constrained_marks_red.md) |
| called_by | [dof_parametric_over_constraint_marks_red](/crates/oxide-sketch/tests/dof/dof_parametric_over_constraint_marks_red.md) |
| called_by | [dof_under_constrained_marks_blue](/crates/oxide-sketch/tests/dof/dof_under_constrained_marks_blue.md) |
| called_by | [jacobian_coincident_matches_analytical](/crates/oxide-sketch/tests/solver_basics/jacobian_coincident_matches_analytical.md) |
| called_by | [jacobian_distance_pt_pt_matches_analytical](/crates/oxide-sketch/tests/solver_basics/jacobian_distance_pt_pt_matches_analytical.md) |
| called_by | [jacobian_empty_sketch_is_zero_rows](/crates/oxide-sketch/tests/solver_basics/jacobian_empty_sketch_is_zero_rows.md) |
| called_by | [jacobian_horizontal_matches_analytical](/crates/oxide-sketch/tests/solver_basics/jacobian_horizontal_matches_analytical.md) |
