---
okf_version: "0.2"
type: Function
title: output_for
description: "Attach `constraints` to `data`, flag them all over-constrained,"
resource: crates/oxide-app/src/app/runtime/footprint_summaries.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_summaries/output_for
language: rust
---

# output_for

Attach `constraints` to `data`, flag them all over-constrained,

## Signature

```rust
fn output_for(
        mut data: SketchData,
        constraints: Vec<Constraint>,
        params: ResolvedParams,
    ) -> (Footprint, FullSolveOutput)
```

## Docstring

Attach `constraints` to `data`, flag them all over-constrained,
and assemble the `FullSolveOutput` the panel reads.

Conflicting constraints make `solve_lm` legitimately return
`DidNotConverge`, so — exactly as `oxide-sketch/tests/dof.rs`
does — the output is assembled at the packed initial state
rather than through `Solver::solve`. Everything
`build_over_constraint_summaries` reads is real: the packed
state, the entity index, and the params the solve ran with.

## Source
Lines 262–288 in `crates/oxide-app/src/app/runtime/footprint_summaries.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries.md) |
| calls | [pack](/crates/oxide-sketch/src/solver/state/pack.md) |
| called_by | [parametric_residual_uses_the_params_the_solve_ran_with](/crates/oxide-app/src/app/runtime/footprint_summaries/parametric_residual_uses_the_params_the_solve_ran_with.md) |
| called_by | [unevaluable_residual_is_not_zero_and_sorts_first](/crates/oxide-app/src/app/runtime/footprint_summaries/unevaluable_residual_is_not_zero_and_sorts_first.md) |
