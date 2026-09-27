---
okf_version: "0.2"
type: Function
title: dof_parametric_over_constraint_marks_red
description: "GH #599 — the parametric twin of `dof_over_constrained_marks_red`."
resource: crates/oxide-sketch/tests/dof.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/dof/dof_parametric_over_constraint_marks_red
language: rust
---

# dof_parametric_over_constraint_marks_red

GH #599 — the parametric twin of `dof_over_constrained_marks_red`.

## Signature

```rust
fn dof_parametric_over_constraint_marks_red()
```

## Decorators

- `test`

## Docstring

GH #599 — the parametric twin of `dof_over_constrained_marks_red`.

Same conflict, but both targets are `DimTarget::Expr` referencing
sketch parameters. `entity_colours` used to build its own empty
`ResolvedParams`, so every parametric target evaluated to
`ExprError::Unknown`, got dropped by the `Err(_) => continue`
filter in `over_constraint_ids`, and the conflict was never
attributed to a point — a false negative, not the false positive
the old in-code comment claimed. The overlay painted an
over-constrained sketch Full/Under.
[test]

## Source
Lines 214–289 in `crates/oxide-sketch/tests/dof.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dof](/crates/oxide-sketch/tests/dof.md) |
| calls | [pack](/crates/oxide-sketch/src/solver/state/pack.md) |
| calls | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| calls | [total_residual](/crates/oxide-sketch/src/solver/residual/total_residual.md) |
| calls | [norm_vec](/crates/oxide-sketch/src/solver/math/norm_vec.md) |
| calls | [numerical_jacobian](/crates/oxide-sketch/src/solver/jacobian/numerical_jacobian.md) |
| calls | [over_constraint_ids](/crates/oxide-sketch/src/solver/dof/over_constraint_ids.md) |
| calls | [entity_colours](/crates/oxide-sketch/src/solver/dof/entity_colours.md) |
