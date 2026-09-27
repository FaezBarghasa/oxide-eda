---
okf_version: "0.2"
type: Function
title: build_over_constraint_summaries
description: v0.22 Phase E3+E4 — Build the per-over-constraint summary list
resource: crates/oxide-app/src/app/runtime/footprint_summaries.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_summaries/build_over_constraint_summaries
language: rust
---

# build_over_constraint_summaries

v0.22 Phase E3+E4 — Build the per-over-constraint summary list

## Signature

```rust
pub(super) fn build_over_constraint_summaries(
    fp: &oxide_library::primitive::footprint::Footprint,
    out: &oxide_sketch::solver::FullSolveOutput,
) -> Vec<crate::panels::OverConstraintSummary>
```

## Visibility

- `pub(super)`

## Docstring

v0.22 Phase E3+E4 — Build the per-over-constraint summary list
from the solver's `over_constraints` IDs. Resolves each
constraint's actual kind (label + first touched entity) so the
Properties panel can show meaningful rows + click-to-focus.

Rows whose residual cannot be evaluated come first, then the rest
descending by residual magnitude — see [`residual_magnitude_of`].

## Source
Lines 37–126 in `crates/oxide-app/src/app/runtime/footprint_summaries.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries.md) |
| calls | [residual_magnitude_of](/crates/oxide-app/src/app/runtime/footprint_summaries/residual_magnitude_of.md) |
| called_by | [build_footprint_editor_panel_ctx](/crates/oxide-app/src/app/runtime/footprint_ctx/build_footprint_editor_panel_ctx.md) |
| called_by | [parametric_residual_uses_the_params_the_solve_ran_with](/crates/oxide-app/src/app/runtime/footprint_summaries/parametric_residual_uses_the_params_the_solve_ran_with.md) |
| called_by | [unevaluable_residual_is_not_zero_and_sorts_first](/crates/oxide-app/src/app/runtime/footprint_summaries/unevaluable_residual_is_not_zero_and_sorts_first.md) |
