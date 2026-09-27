---
okf_version: "0.2"
type: Function
title: solve_lm
description: Levenberg–Marquardt solve.
resource: crates/oxide-sketch/src/solver/lm.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/lm/solve_lm
language: rust
---

# solve_lm

Levenberg–Marquardt solve.

## Signature

```rust
pub fn solve_lm(
    sketch: &SketchData,
    params: &ResolvedParams,
    timeout_ms: u64,
    tolerance: f64,
    max_iters: usize,
) -> Result<SolveResult, SolveError>
```

## Visibility

- `pub`

## Docstring

Levenberg–Marquardt solve.

`timeout_ms` is the wall-clock budget. `tolerance` is the linear
residual-norm threshold — `|r|² < tolerance²` declares convergence.
`max_iters` caps the iteration count before [`SolveError::DidNotConverge`]
is returned. The defaults baked into [`crate::solver::Solver`]
(`tolerance = 1e-12`, `max_iters = 100`) are appropriate for the
v0.13 sketch use case; callers can tighten them for unit tests or
for high-precision regression cases.

## Source
Lines 83–242 in `crates/oxide-sketch/src/solver/lm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lm](/crates/oxide-sketch/src/solver/lm.md) |
| calls | [pack](/crates/oxide-sketch/src/solver/state/pack.md) |
| calls | [total_residual](/crates/oxide-sketch/src/solver/residual/total_residual.md) |
| calls | [norm_sq](/crates/oxide-sketch/src/solver/math/norm_sq.md) |
| calls | [norm_vec](/crates/oxide-sketch/src/solver/math/norm_vec.md) |
| calls | [numerical_jacobian](/crates/oxide-sketch/src/solver/jacobian/numerical_jacobian.md) |
| calls | [matmul_ata](/crates/oxide-sketch/src/solver/math/matmul_ata.md) |
| calls | [add_diag](/crates/oxide-sketch/src/solver/math/add_diag.md) |
| calls | [matvec_t](/crates/oxide-sketch/src/solver/math/matvec_t.md) |
| calls | [ConstraintId](/crates/oxide-sketch/src/id/ConstraintId.md) |
| calls | [axpy](/crates/oxide-sketch/src/solver/math/axpy.md) |
| called_by | [solve](/crates/oxide-sketch/src/solver/mod/solve.md) |
| called_by | [isosceles_triangle_apex_60](/crates/oxide-sketch/tests/canonical_sketches/isosceles_triangle_apex_60.md) |
| called_by | [parallelogram_base10_side5_60deg](/crates/oxide-sketch/tests/canonical_sketches/parallelogram_base10_side5_60deg.md) |
| called_by | [rectangle_10_by_5](/crates/oxide-sketch/tests/canonical_sketches/rectangle_10_by_5.md) |
| called_by | [regular_hexagon_circumradius_10](/crates/oxide-sketch/tests/canonical_sketches/regular_hexagon_circumradius_10.md) |
| called_by | [dof_fully_constrained_marks_black](/crates/oxide-sketch/tests/dof/dof_fully_constrained_marks_black.md) |
| called_by | [dof_over_constrained_marks_red](/crates/oxide-sketch/tests/dof/dof_over_constrained_marks_red.md) |
| called_by | [dof_parametric_over_constraint_marks_red](/crates/oxide-sketch/tests/dof/dof_parametric_over_constraint_marks_red.md) |
| called_by | [dof_under_constrained_marks_blue](/crates/oxide-sketch/tests/dof/dof_under_constrained_marks_blue.md) |
| called_by | [lm_already_converged_returns_quickly](/crates/oxide-sketch/tests/lm_basic/lm_already_converged_returns_quickly.md) |
| called_by | [lm_no_constraints_returns_immediately](/crates/oxide-sketch/tests/lm_basic/lm_no_constraints_returns_immediately.md) |
| called_by | [lm_solves_anchored_distance_in_either_direction](/crates/oxide-sketch/tests/lm_basic/lm_solves_anchored_distance_in_either_direction.md) |
| called_by | [lm_solves_anchored_horizontal_distance](/crates/oxide-sketch/tests/lm_basic/lm_solves_anchored_horizontal_distance.md) |
| called_by | [lm_solves_offset_circle_via_distance_pt_circle](/crates/oxide-sketch/tests/lm_basic/lm_solves_offset_circle_via_distance_pt_circle.md) |
