---
okf_version: "0.2"
type: Function
title: entity_colours
description: Per-entity DoF colour. Returns one entry per Point entity that
resource: crates/oxide-sketch/src/solver/dof.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-sketch/src/solver/dof/entity_colours
language: rust
---

# entity_colours

Per-entity DoF colour. Returns one entry per Point entity that

## Signature

```rust
pub fn entity_colours(
    sketch: &SketchData,
    solve_result: &SolveResult,
    jacobian: &[Vec<f64>],
    index: &EntityIndex,
    params: &crate::solver::residual::ResolvedParams,
) -> HashMap<SketchEntityId, DofColor>
```

## Visibility

- `pub`

## Docstring

Per-entity DoF colour. Returns one entry per Point entity that
participates in the solve — both free Points (in `index.points`)
AND Fixed Points (in `index.fixed`). Non-Point entities (lines,
arcs, circles) are not included; they inherit their endpoints'
colours at render time.

**Conservative coarse rule (acceptable for v0.13):** if the total
numerical rank of the Jacobian equals `state.len()`, every non-
Fixed Point is fully constrained; otherwise every non-Fixed Point
is under-constrained. Then any Point participating in an over-
constrained constraint (residual > `RANK_TOL` post-solve) is
upgraded to `Over`.

This is documented as intentional. The plan's three canonical
cases (`under` / `full` / `over`) all classify correctly under
this rule. A future revision can replace the coarse global
rank-vs-`n` test with per-column rank-deficiency detection
(rank-1 update / column-zeroing test) for finer per-entity
granularity.

`params` MUST be the resolved parameter map from the same solve
that produced `solve_result` — see the HI-14 note on
[`over_constraint_ids`], which this function delegates the `Over`
classification to.

## Source
Lines 80–152 in `crates/oxide-sketch/src/solver/dof.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dof](/crates/oxide-sketch/src/solver/dof.md) |
| calls | [over_constraint_ids](/crates/oxide-sketch/src/solver/dof/over_constraint_ids.md) |
| calls | [points_touched](/crates/oxide-sketch/src/solver/dof/points_touched.md) |
| called_by | [solve](/crates/oxide-sketch/src/solver/mod/solve.md) |
| called_by | [dof_fully_constrained_marks_black](/crates/oxide-sketch/tests/dof/dof_fully_constrained_marks_black.md) |
| called_by | [dof_over_constrained_marks_red](/crates/oxide-sketch/tests/dof/dof_over_constrained_marks_red.md) |
| called_by | [dof_parametric_over_constraint_marks_red](/crates/oxide-sketch/tests/dof/dof_parametric_over_constraint_marks_red.md) |
| called_by | [dof_under_constrained_marks_blue](/crates/oxide-sketch/tests/dof/dof_under_constrained_marks_blue.md) |
