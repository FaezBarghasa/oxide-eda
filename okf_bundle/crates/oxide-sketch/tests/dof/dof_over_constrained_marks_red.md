---
okf_version: "0.2"
type: Function
title: dof_over_constrained_marks_red
description: "[test]"
resource: crates/oxide-sketch/tests/dof.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/dof/dof_over_constrained_marks_red
language: rust
---

# dof_over_constrained_marks_red

[test]

## Signature

```rust
fn dof_over_constrained_marks_red()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 115–201 in `crates/oxide-sketch/tests/dof.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dof](/crates/oxide-sketch/tests/dof.md) |
| calls | [pack](/crates/oxide-sketch/src/solver/state/pack.md) |
| calls | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| calls | [empty_params](/crates/oxide-sketch/tests/dof/empty_params.md) |
| calls | [total_residual](/crates/oxide-sketch/src/solver/residual/total_residual.md) |
| calls | [norm_vec](/crates/oxide-sketch/src/solver/math/norm_vec.md) |
| calls | [numerical_jacobian](/crates/oxide-sketch/src/solver/jacobian/numerical_jacobian.md) |
| calls | [over_constraint_ids](/crates/oxide-sketch/src/solver/dof/over_constraint_ids.md) |
| calls | [entity_colours](/crates/oxide-sketch/src/solver/dof/entity_colours.md) |
